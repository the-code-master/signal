// Module: engine/crates/signal-operator/src/pool_scheduler.rs — 1-4->N Rust Pool Reconciler, LTS Pinning, Warm Pool & GPU CC Scheduler
// Invariants: Rejects H100 MIG + CC combination (F-118); enforces .signal/platform.lock.json LTS pinning & 1-node fallback (20-*).
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/20-versioning-release-monorepo-build-and-k8s-topology.md

use crate::{GpuIsolationMode, OperatorError, ScheduledCellSpec};
use signal_proto::v1::{CellComputeTier, ConfidentialGpuMode};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareGpuSku {
    None,
    NvidiaH100Hopper,
    NvidiaB200Blackwell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterTopologyPolicy {
    pub baseline_worker_nodes: u32,
    pub max_extensible_worker_nodes: u32,
    pub active_worker_nodes: u32,
    pub adaptive_single_node_fallback: bool,
    pub supported_lts_versions: BTreeSet<String>,
    pub default_platform_version: String,
}

impl ClusterTopologyPolicy {
    /// Resolves a project's `.signal/platform.lock.json` `pinned_engine_version` against supported LTS releases (`20-*`).
    pub fn resolve_lts_engine_tag(&self, pinned_version: &str) -> Result<String, OperatorError> {
        let ver = if pinned_version.trim().is_empty() {
            self.default_platform_version.as_str()
        } else {
            pinned_version.trim().trim_start_matches('v')
        };
        if !self.supported_lts_versions.contains(ver) {
            return Err(OperatorError::UnsupportedPinnedLtsVersion(ver.to_string()));
        }
        Ok(format!("v{ver}"))
    }
}

/// Validates Confidential GPU scheduling hardware invariants (`F-079`, `F-118`, `I-11`):
/// - Hopper `H100` supports `H100FullCcExclusive` (whole-GPU CC) ONLY; simultaneous `MIG + CC` on `H100` is physically unsupported and rejected!
/// - Blackwell `B200` supports `B200TeeIoConfidentialMig` hardware-isolated Confidential MIG slices.
pub fn validate_confidential_gpu_schedule(
    sku: HardwareGpuSku,
    requested_mode: ConfidentialGpuMode,
    mig_slice_requested: bool,
) -> Result<GpuIsolationMode, OperatorError> {
    match (sku, requested_mode) {
        (_, ConfidentialGpuMode::Unspecified | ConfidentialGpuMode::None) => {
            Ok(GpuIsolationMode::None)
        }
        (HardwareGpuSku::NvidiaH100Hopper, ConfidentialGpuMode::H100FullCcExclusive) => {
            if mig_slice_requested {
                return Err(OperatorError::IllegalHopperMigConfidentialCombo);
            }
            Ok(GpuIsolationMode::HopperWholeGpuConfidential)
        }
        (HardwareGpuSku::NvidiaH100Hopper, ConfidentialGpuMode::B200TeeIoConfidentialMig) => {
            Err(OperatorError::IllegalHopperMigConfidentialCombo)
        }
        (HardwareGpuSku::NvidiaB200Blackwell, ConfidentialGpuMode::B200TeeIoConfidentialMig) => {
            Ok(GpuIsolationMode::BlackwellTeeIoConfidentialMig)
        }
        (HardwareGpuSku::NvidiaB200Blackwell, ConfidentialGpuMode::H100FullCcExclusive) => {
            if mig_slice_requested {
                return Err(OperatorError::IllegalHopperMigConfidentialCombo);
            }
            Ok(GpuIsolationMode::HopperWholeGpuConfidential)
        }
        (HardwareGpuSku::None, mode) => Err(OperatorError::NoConfidentialGpuHardware(format!(
            "{mode:?}"
        ))),
    }
}

/// Reconciles `1 Project = 1 Cell` pod placement across `1–4 -> N` extensible `pool-rust-engine` pools (`20-*`).
pub fn reconcile_project_cell_spec(
    policy: &ClusterTopologyPolicy,
    tier: CellComputeTier,
    pinned_version: &str,
    gpu_sku: HardwareGpuSku,
    requested_gpu_mode: ConfidentialGpuMode,
    mig_slice_requested: bool,
) -> Result<ScheduledCellSpec, OperatorError> {
    if policy.active_worker_nodes == 0 {
        return Err(OperatorError::ZeroActiveWorkerNodes);
    }
    let engine_image_tag = policy.resolve_lts_engine_tag(pinned_version)?;

    let effective_gpu_req = match (tier, requested_gpu_mode) {
        (
            CellComputeTier::CoreGpu | CellComputeTier::ConfidentialSilicon,
            ConfidentialGpuMode::Unspecified,
        ) => match gpu_sku {
            HardwareGpuSku::NvidiaB200Blackwell => ConfidentialGpuMode::B200TeeIoConfidentialMig,
            HardwareGpuSku::NvidiaH100Hopper => ConfidentialGpuMode::H100FullCcExclusive,
            HardwareGpuSku::None => {
                return Err(OperatorError::NoConfidentialGpuHardware(format!(
                    "{tier:?}"
                )));
            }
        },
        (_, mode) => mode,
    };
    let gpu_mode =
        validate_confidential_gpu_schedule(gpu_sku, effective_gpu_req, mig_slice_requested)?;

    let single_pool_fallback =
        policy.adaptive_single_node_fallback || policy.active_worker_nodes == 1;
    let node_pool = if single_pool_fallback {
        "pool-rust-engine"
    } else {
        match tier {
            CellComputeTier::CoreHighmem => "pool-rust-engine-highmem",
            CellComputeTier::CoreGpu => "pool-rust-engine-gpu",
            CellComputeTier::ConfidentialSilicon => "pool-rust-engine-confidential",
            _ => "pool-rust-engine",
        }
    };

    Ok(ScheduledCellSpec {
        node_pool,
        engine_image_tag,
        gpu_mode,
        enforce_host_anti_affinity: !single_pool_fallback && policy.active_worker_nodes >= 2,
    })
}

/// Predictive un-hydrated MicroVM Warm Pool controller (`F-039`) maintaining instant-hydration standby slots.
#[derive(Debug, Clone)]
pub struct PredictiveWarmPool {
    idle_slots: BTreeMap<i32, u32>,
    target_headroom: BTreeMap<i32, u32>,
}

impl PredictiveWarmPool {
    pub fn new(standard_target: u32, highmem_target: u32, gpu_target: u32) -> Self {
        let mut target_headroom = BTreeMap::new();
        target_headroom.insert(CellComputeTier::CoreStandard as i32, standard_target);
        target_headroom.insert(CellComputeTier::CoreHighmem as i32, highmem_target);
        target_headroom.insert(CellComputeTier::CoreGpu as i32, gpu_target);
        Self {
            idle_slots: target_headroom.clone(),
            target_headroom,
        }
    }

    /// Claims a pre-warmed un-hydrated MicroVM if available (`true` = `< 250 ms` warm hit),
    /// and returns the replenishment count needed to restore predictive headroom (`F-039`).
    pub fn claim_and_plan_replenishment(
        &mut self,
        tier: CellComputeTier,
        recent_attach_spike: u32,
    ) -> (bool, u32) {
        let key = tier as i32;
        let current = self.idle_slots.get(&key).copied().unwrap_or(0);
        let warm_hit = current > 0;
        let remaining = current.saturating_sub(1);
        let base_target = self.target_headroom.get(&key).copied().unwrap_or(2);
        let adaptive_target = base_target.saturating_add(recent_attach_spike / 2);
        let replenish_needed = adaptive_target.saturating_sub(remaining);
        self.idle_slots
            .insert(key, remaining.saturating_add(replenish_needed));
        (warm_hit, replenish_needed)
    }
}
