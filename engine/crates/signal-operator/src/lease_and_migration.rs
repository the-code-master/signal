// Module: engine/crates/signal-operator/src/lease_and_migration.rs — Host-Aggregated Node Lease & 3-Phase Pre-Copy Live Migration
// Invariants: Coalesces N pod leases into 1 node K8s Lease (F-139); enforces <= 150 ms Phase-2 freeze & epoch bump (F-046, F-123).
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/10-api-and-data-contracts.md

use crate::OperatorError;
use signal_core::sha512_256_hex;
use signal_proto::v1::{ThreePhaseLiveMigrateRequest, ThreePhaseLiveMigrateResponse};
use std::collections::BTreeMap;

pub const DEFAULT_MAX_CUTOVER_FREEZE_PS: i64 = 150_000_000_000; // 150 ms

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PodCellHeartbeat {
    pub project_id: String,
    pub pod_uid: String,
    pub fencing_epoch: u64,
    pub expires_at_utc_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregatedNodeLeasePayload {
    pub node_id: String,
    pub lease_sequence: u64,
    pub renewed_at_utc_ms: i64,
    pub active_cell_count: u32,
    pub coalesced_merkle_digest: String,
    pub evicted_expired_projects: Vec<String>,
}

/// Host-Aggregated Node Lease batcher (`F-139`) coalescing `N` per-project pod heartbeats on a node
/// into a single K8s `coordination.k8s.io/v1` `Lease` renewal (`~100x` etcd write reduction).
#[derive(Debug)]
pub struct HostNodeLeaseAggregator {
    node_id: String,
    lease_sequence: u64,
    cells: BTreeMap<String, PodCellHeartbeat>,
}

impl HostNodeLeaseAggregator {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            lease_sequence: 0,
            cells: BTreeMap::new(),
        }
    }

    pub fn record_local_pod_heartbeat(
        &mut self,
        hb: PodCellHeartbeat,
    ) -> Result<(), OperatorError> {
        if let Some(prev) = self.cells.get(&hb.project_id)
            && hb.fencing_epoch < prev.fencing_epoch
        {
            return Err(OperatorError::StaleMigrationEpoch {
                provided: hb.fencing_epoch,
                current: prev.fencing_epoch,
            });
        }
        self.cells.insert(hb.project_id.clone(), hb);
        Ok(())
    }

    pub fn flush_coalesced_k8s_lease(&mut self, now_utc_ms: i64) -> AggregatedNodeLeasePayload {
        let mut evicted = Vec::new();
        self.cells.retain(|proj_id, hb| {
            let alive = hb.expires_at_utc_ms > now_utc_ms;
            if !alive {
                evicted.push(proj_id.clone());
            }
            alive
        });

        self.lease_sequence = self.lease_sequence.saturating_add(1);
        let mut transcript = format!("node={};seq={};", self.node_id, self.lease_sequence);
        for hb in self.cells.values() {
            transcript.push_str(&format!(
                "{}:{}:{}:{};",
                hb.project_id, hb.pod_uid, hb.fencing_epoch, hb.expires_at_utc_ms
            ));
        }

        AggregatedNodeLeasePayload {
            node_id: self.node_id.clone(),
            lease_sequence: self.lease_sequence,
            renewed_at_utc_ms: now_utc_ms,
            active_cell_count: self.cells.len() as u32,
            coalesced_merkle_digest: sha512_256_hex(transcript.as_bytes()),
            evicted_expired_projects: evicted,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveMigrationPhase {
    IdleMounted,
    Phase1PreCopyDirtyRounds { round: u32, dirty_pages_left: u64 },
    Phase2FrozenEpochCutover,
    Phase3EnvoyBridgeSwitched,
}

/// 3-Phase Pre-Copy Live Migration state machine (`F-046`, `F-123`) maintaining `< 150 ms` freeze time.
#[derive(Debug, Clone)]
pub struct ThreePhaseMigrationController {
    pub project_id: String,
    pub source_node_id: String,
    pub current_epoch: u64,
    pub phase: LiveMigrationPhase,
    pub dirty_pages_remaining: u64,
    pub network_gbps: f64,
}

impl ThreePhaseMigrationController {
    pub fn new(
        project_id: &str,
        source_node_id: &str,
        current_epoch: u64,
        initial_working_set_pages_4k: u64,
        network_gbps: f64,
    ) -> Self {
        Self {
            project_id: project_id.to_string(),
            source_node_id: source_node_id.to_string(),
            current_epoch,
            phase: LiveMigrationPhase::IdleMounted,
            dirty_pages_remaining: initial_working_set_pages_4k.max(1),
            network_gbps: network_gbps.max(10.0),
        }
    }

    /// Executes iterative Phase 1 background pre-copy rounds while the source cell continues serving frames.
    pub fn run_phase1_precopy_round(&mut self, page_dirty_rate_ratio: f64) -> LiveMigrationPhase {
        let ratio = page_dirty_rate_ratio.clamp(0.02, 0.5);
        let next_round = match self.phase {
            LiveMigrationPhase::Phase1PreCopyDirtyRounds { round, .. } => round + 1,
            _ => 1,
        };
        self.dirty_pages_remaining = ((self.dirty_pages_remaining as f64) * ratio) as u64;
        let st = LiveMigrationPhase::Phase1PreCopyDirtyRounds {
            round: next_round,
            dirty_pages_left: self.dirty_pages_remaining,
        };
        self.phase = st;
        st
    }

    /// Executes Phase 2 (`< 150 ms` deterministic freeze, final dirty-page flush, fencing epoch bump)
    /// and Phase 3 (Envoy `signal-engine-bridge` sticky cutover, `F-046`, `F-123`).
    pub fn complete_cutover(
        &mut self,
        req: &ThreePhaseLiveMigrateRequest,
    ) -> Result<ThreePhaseLiveMigrateResponse, OperatorError> {
        let next_fencing = req
            .next_fencing
            .as_ref()
            .ok_or(OperatorError::MissingMigrationFencing)?;
        if next_fencing.fencing_epoch <= self.current_epoch {
            return Err(OperatorError::StaleMigrationEpoch {
                provided: next_fencing.fencing_epoch,
                current: self.current_epoch,
            });
        }
        if req.destination_node_id.trim().is_empty()
            || req.destination_node_id == self.source_node_id
        {
            return Err(OperatorError::InvalidMigrationDestination(
                req.destination_node_id.clone(),
            ));
        }

        // Auto-converge Phase 1 pre-copy if residual dirty pages would exceed freeze budget
        let mut guard_rounds = 0_u32;
        while self.dirty_pages_remaining > 512 && guard_rounds < 8 {
            self.run_phase1_precopy_round(0.15);
            guard_rounds += 1;
        }

        self.phase = LiveMigrationPhase::Phase2FrozenEpochCutover;
        let residual_bits = self.dirty_pages_remaining.saturating_mul(4096 * 8) as f64;
        let transfer_ps = ((residual_bits / (self.network_gbps * 1.0e9)) * 1.0e12) as i64;
        let serialization_overhead_ps = 18_500_000_000_i64; // 18.5 ms CRIU/vCPU state + Envoy switch
        let observed_freeze_ps = transfer_ps.saturating_add(serialization_overhead_ps);

        let budget_ps = if req.max_cutover_freeze_ps > 0 {
            req.max_cutover_freeze_ps
        } else {
            DEFAULT_MAX_CUTOVER_FREEZE_PS
        };
        if observed_freeze_ps > budget_ps {
            return Err(OperatorError::FreezeBudgetExceeded {
                observed_ps: observed_freeze_ps,
                max_ps: budget_ps,
            });
        }

        self.current_epoch = next_fencing.fencing_epoch;
        self.source_node_id.clone_from(&req.destination_node_id);
        self.phase = LiveMigrationPhase::Phase3EnvoyBridgeSwitched;

        Ok(ThreePhaseLiveMigrateResponse {
            completed: true,
            observed_cutover_freeze_ps: observed_freeze_ps,
            new_fencing_epoch: self.current_epoch,
        })
    }
}
