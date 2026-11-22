// Module: engine/crates/signal-module-sdk/src/microvm_ipc.rs — Zero-Network Child-MicroVM Shared-Memory Ring IPC
// Invariants: Untrusted OSDI/IBIS-AMI .so models never dlopen in signal-pod; framed SPSC IPC with SHA-512 MAC (F-114).
// Spec: architecture/03-rust-eda-engine.md, architecture/15-application-security-and-project-isolation.md
use crate::SdkError;
use signal_core::{ct_eq_bytes, sha512};
use std::collections::VecDeque;

const OSDI_IPC_MAGIC: [u8; 4] = *b"OSDI";
const MAX_IPC_PAYLOAD_FLOATS: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxModelKind {
    OsdiCompactModel,
    IbisAmiSerDes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildMicroVmPolicy {
    pub network_namespace_none: bool,
    pub seccomp_strict_filter: bool,
    pub host_dlopen_forbidden: bool,
    pub max_memory_mib: u32,
}

impl ChildMicroVmPolicy {
    pub const STRICT_ZERO_NETWORK: Self = Self {
        network_namespace_none: true,
        seccomp_strict_filter: true,
        host_dlopen_forbidden: true,
        max_memory_mib: 128,
    };

    pub fn validate(&self) -> Result<(), SdkError> {
        if !self.network_namespace_none
            || !self.seccomp_strict_filter
            || !self.host_dlopen_forbidden
            || self.max_memory_mib == 0
            || self.max_memory_mib > 1024
        {
            return Err(SdkError::MicroVmSandboxViolation(
                "OSDI/IBIS-AMI child MicroVM must enforce zero-network + host_dlopen_forbidden"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OsdiStampRequestFrame {
    pub seq_id: u64,
    pub kind: SandboxModelKind,
    pub temp_mk: i64,
    pub terminal_voltages_v: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OsdiStampResponseFrame {
    pub seq_id: u64,
    pub residuals_a: Vec<f64>,
    pub charges_c: Vec<f64>,
    pub jacobian_flat: Vec<f64>,
}

/// Encodes an `OsdiStampRequestFrame` into deterministic shared-memory ring-buffer bytes + 16-byte SHA-512 integrity tag (`F-114`).
pub fn encode_osdi_ipc_request(
    policy: &ChildMicroVmPolicy,
    req: &OsdiStampRequestFrame,
) -> Result<Vec<u8>, SdkError> {
    policy.validate()?;
    if req.terminal_voltages_v.len() > MAX_IPC_PAYLOAD_FLOATS {
        return Err(SdkError::MicroVmSandboxViolation(
            "IPC float count > 256 cap".to_string(),
        ));
    }
    let mut buf = Vec::with_capacity(24 + req.terminal_voltages_v.len() * 8 + 16);
    buf.extend_from_slice(&OSDI_IPC_MAGIC);
    buf.push(match req.kind {
        SandboxModelKind::OsdiCompactModel => 1,
        SandboxModelKind::IbisAmiSerDes => 2,
    });
    buf.extend_from_slice(&req.seq_id.to_le_bytes());
    buf.extend_from_slice(&req.temp_mk.to_le_bytes());
    buf.extend_from_slice(&(req.terminal_voltages_v.len() as u32).to_le_bytes());
    for &v in &req.terminal_voltages_v {
        buf.extend_from_slice(&v.to_bits().to_le_bytes());
    }
    let tag = sha512(&buf);
    buf.extend_from_slice(&tag[..16]);
    Ok(buf)
}

pub fn decode_osdi_ipc_request(frame: &[u8]) -> Result<OsdiStampRequestFrame, SdkError> {
    if frame.len() < 41 || frame[..4] != OSDI_IPC_MAGIC {
        return Err(SdkError::MicroVmSandboxViolation(
            "invalid OSDI IPC header".to_string(),
        ));
    }
    let body_end = frame.len() - 16;
    let expected = sha512(&frame[..body_end]);
    if !ct_eq_bytes(&frame[body_end..], &expected[..16]) {
        return Err(SdkError::MicroVmSandboxViolation(
            "OSDI IPC integrity mismatch".to_string(),
        ));
    }
    let kind = match frame[4] {
        1 => SandboxModelKind::OsdiCompactModel,
        2 => SandboxModelKind::IbisAmiSerDes,
        _ => {
            return Err(SdkError::MicroVmSandboxViolation(
                "unknown model kind".to_string(),
            ));
        }
    };
    let seq_id = u64::from_le_bytes(
        frame[5..13]
            .try_into()
            .map_err(|_| SdkError::MicroVmSandboxViolation("bad seq_id".to_string()))?,
    );
    let temp_mk = i64::from_le_bytes(
        frame[13..21]
            .try_into()
            .map_err(|_| SdkError::MicroVmSandboxViolation("bad temp_mk".to_string()))?,
    );
    let count = u32::from_le_bytes(
        frame[21..25]
            .try_into()
            .map_err(|_| SdkError::MicroVmSandboxViolation("bad count".to_string()))?,
    ) as usize;

    if count > MAX_IPC_PAYLOAD_FLOATS || 25 + count * 8 != body_end {
        return Err(SdkError::MicroVmSandboxViolation(
            "OSDI IPC length mismatch".to_string(),
        ));
    }
    let mut vals = Vec::with_capacity(count);
    for i in 0..count {
        let off = 25 + i * 8;
        let bits = u64::from_le_bytes(
            frame[off..off + 8]
                .try_into()
                .map_err(|_| SdkError::MicroVmSandboxViolation("bad f64".to_string()))?,
        );
        vals.push(f64::from_bits(bits));
    }
    Ok(OsdiStampRequestFrame {
        seq_id,
        kind,
        temp_mk,
        terminal_voltages_v: vals,
    })
}

/// Bounded in-memory SPSC shared-memory ring-buffer channel modelling host<->child-MicroVM IPC (`F-114`).
#[derive(Debug)]
pub struct MicroVmShmRing {
    capacity_frames: usize,
    policy: ChildMicroVmPolicy,
    queue: VecDeque<Vec<u8>>,
}

impl MicroVmShmRing {
    pub fn new(policy: ChildMicroVmPolicy, capacity_frames: usize) -> Result<Self, SdkError> {
        policy.validate()?;
        Ok(Self {
            capacity_frames: capacity_frames.max(1),
            policy,
            queue: VecDeque::with_capacity(capacity_frames.max(1)),
        })
    }

    pub fn push_request(&mut self, req: &OsdiStampRequestFrame) -> Result<(), SdkError> {
        if self.queue.len() >= self.capacity_frames {
            return Err(SdkError::MicroVmSandboxViolation(
                "SPSC ring buffer full".to_string(),
            ));
        }
        let bytes = encode_osdi_ipc_request(&self.policy, req)?;
        self.queue.push_back(bytes);
        Ok(())
    }

    pub fn pop_request(&mut self) -> Result<Option<OsdiStampRequestFrame>, SdkError> {
        let Some(bytes) = self.queue.pop_front() else {
            return Ok(None);
        };
        Ok(Some(decode_osdi_ipc_request(&bytes)?))
    }
}
