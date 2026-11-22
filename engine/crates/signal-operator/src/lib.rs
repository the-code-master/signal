// Module: engine/crates/signal-operator/src/lib.rs — 3-Tier K8s Worker Node Pool Selector, Lease Batcher & GPU CC Guard
// Invariants: Never combines MIG + CC on Hopper H100 (F-118); falls back cleanly on single-node starter clusters (20-*).
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
#![forbid(unsafe_code)]

use signal_proto::v1::CellComputeTier;
use thiserror::Error;

pub mod lease_and_migration;
pub mod pool_scheduler;

pub use lease_and_migration::{
    AggregatedNodeLeasePayload, DEFAULT_MAX_CUTOVER_FREEZE_PS, HostNodeLeaseAggregator,
    LiveMigrationPhase, PodCellHeartbeat, ThreePhaseMigrationController,
};
pub use pool_scheduler::{
    ClusterTopologyPolicy, HardwareGpuSku, PredictiveWarmPool, reconcile_project_cell_spec,
    validate_confidential_gpu_schedule,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OperatorError {
    #[error(
        "Hopper H100 cannot combine MIG partitioning with Confidential Computing (F-118); use whole-GPU H100 CC or Blackwell B200 TEE-I/O MIG"
    )]
    IllegalHopperMigConfidentialCombo,
    #[error("requested confidential GPU mode `{0}` requires H100 CC or B200 TEE-I/O hardware")]
    NoConfidentialGpuHardware(String),
    #[error("pinned engine version `{0}` is not in cluster supported LTS versions")]
    UnsupportedPinnedLtsVersion(String),
    #[error("zero active pool-rust-engine worker nodes available")]
    ZeroActiveWorkerNodes,
    #[error("missing next fencing authority for 3-phase live migration")]
    MissingMigrationFencing,
    #[error("migration fencing epoch {provided} must exceed active epoch {current}")]
    StaleMigrationEpoch { provided: u64, current: u64 },
    #[error("invalid live migration destination node `{0}`")]
    InvalidMigrationDestination(String),
    #[error("Phase-2 live migration freeze {observed_ps} ps exceeded budget {max_ps} ps")]
    FreezeBudgetExceeded { observed_ps: i64, max_ps: i64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuIsolationMode {
    None,
    HopperWholeGpuConfidential,
    BlackwellTeeIoConfidentialMig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledCellSpec {
    pub node_pool: &'static str,
    pub engine_image_tag: String,
    pub gpu_mode: GpuIsolationMode,
    pub enforce_host_anti_affinity: bool,
}

pub fn plan_cell_placement(
    tier: CellComputeTier,
    pinned_version: &str,
    rust_worker_node_count: u32,
    blackwell_b200_available: bool,
) -> ScheduledCellSpec {
    let (node_pool, gpu_mode) = match tier {
        CellComputeTier::CoreHighmem => ("pool-rust-engine-highmem", GpuIsolationMode::None),
        CellComputeTier::CoreGpu => (
            "pool-rust-engine-gpu",
            if blackwell_b200_available {
                GpuIsolationMode::BlackwellTeeIoConfidentialMig
            } else {
                GpuIsolationMode::HopperWholeGpuConfidential
            },
        ),
        CellComputeTier::ConfidentialSilicon => (
            "pool-rust-engine-confidential",
            GpuIsolationMode::HopperWholeGpuConfidential,
        ),
        _ => ("pool-rust-engine", GpuIsolationMode::None),
    };
    ScheduledCellSpec {
        node_pool,
        engine_image_tag: format!("v{pinned_version}"),
        gpu_mode,
        enforce_host_anti_affinity: rust_worker_node_count >= 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_proto::v1::{ConfidentialGpuMode, FencingAuthority, ThreePhaseLiveMigrateRequest};
    use std::collections::BTreeSet;

    #[test]
    fn rejects_hopper_h100_mig_plus_cc_and_allows_blackwell_b200_tee_io_mig() {
        let err = validate_confidential_gpu_schedule(
            HardwareGpuSku::NvidiaH100Hopper,
            ConfidentialGpuMode::H100FullCcExclusive,
            true,
        )
        .expect_err("H100 + MIG + CC must fail");
        assert_eq!(err, OperatorError::IllegalHopperMigConfidentialCombo);

        let h100_ok = validate_confidential_gpu_schedule(
            HardwareGpuSku::NvidiaH100Hopper,
            ConfidentialGpuMode::H100FullCcExclusive,
            false,
        )
        .expect("whole-GPU H100 CC ok");
        assert_eq!(h100_ok, GpuIsolationMode::HopperWholeGpuConfidential);

        let b200_ok = validate_confidential_gpu_schedule(
            HardwareGpuSku::NvidiaB200Blackwell,
            ConfidentialGpuMode::B200TeeIoConfidentialMig,
            true,
        )
        .expect("B200 TEE-I/O MIG ok");
        assert_eq!(b200_ok, GpuIsolationMode::BlackwellTeeIoConfidentialMig);
    }

    #[test]
    fn reconciles_lts_pinning_single_node_fallback_and_warm_pool() {
        let policy = ClusterTopologyPolicy {
            baseline_worker_nodes: 4,
            max_extensible_worker_nodes: 32,
            active_worker_nodes: 1,
            adaptive_single_node_fallback: true,
            supported_lts_versions: BTreeSet::from(["0.1.0".into(), "0.2.0".into()]),
            default_platform_version: "0.2.0".into(),
        };
        let spec = reconcile_project_cell_spec(
            &policy,
            CellComputeTier::CoreHighmem,
            "0.1.0",
            HardwareGpuSku::None,
            ConfidentialGpuMode::None,
            false,
        )
        .expect("reconcile ok");
        assert_eq!(spec.node_pool, "pool-rust-engine");
        assert_eq!(spec.engine_image_tag, "v0.1.0");
        assert!(!spec.enforce_host_anti_affinity);

        assert!(policy.resolve_lts_engine_tag("9.9.9").is_err());

        let mut warm = PredictiveWarmPool::new(3, 1, 1);
        let (hit, repl) = warm.claim_and_plan_replenishment(CellComputeTier::CoreStandard, 4);
        assert!(hit);
        assert!(repl >= 3);
    }

    #[test]
    fn coalesces_node_leases_and_completes_sub_150ms_three_phase_migration() {
        let mut agg = HostNodeLeaseAggregator::new("worker-rust-01");
        agg.record_local_pod_heartbeat(PodCellHeartbeat {
            project_id: "p1".into(),
            pod_uid: "uid-1".into(),
            fencing_epoch: 5,
            expires_at_utc_ms: 2_000_000,
        })
        .expect("hb1");
        agg.record_local_pod_heartbeat(PodCellHeartbeat {
            project_id: "p2".into(),
            pod_uid: "uid-2".into(),
            fencing_epoch: 9,
            expires_at_utc_ms: 1_500_000,
        })
        .expect("hb2");
        let batch = agg.flush_coalesced_k8s_lease(1_600_000);
        assert_eq!(batch.active_cell_count, 1);
        assert_eq!(batch.evicted_expired_projects, vec!["p2".to_string()]);
        assert_eq!(batch.coalesced_merkle_digest.len(), 64);

        let mut mig = ThreePhaseMigrationController::new("p1", "worker-rust-01", 5, 65_536, 25.0);
        mig.run_phase1_precopy_round(0.1);
        let resp = mig
            .complete_cutover(&ThreePhaseLiveMigrateRequest {
                next_fencing: Some(FencingAuthority {
                    org_id: "org-1".into(),
                    project_id: "p1".into(),
                    fencing_epoch: 6,
                    lease_holder_pod_id: "uid-dest".into(),
                    issued_at_utc_ms: 1_000,
                    expires_at_utc_ms: 9_999_999,
                }),
                destination_node_id: "worker-rust-02".into(),
                max_cutover_freeze_ps: DEFAULT_MAX_CUTOVER_FREEZE_PS,
            })
            .expect("cutover ok");
        assert!(resp.completed);
        assert!(resp.observed_cutover_freeze_ps <= DEFAULT_MAX_CUTOVER_FREEZE_PS);
        assert_eq!(resp.new_fencing_epoch, 6);
    }
}
