// Module: engine/crates/signal-operator/src/lib.rs — 3-Tier K8s Worker Node Pool Selector & GPU CC Guard
// Invariants: Never combines MIG + CC on Hopper H100 (F-118); falls back cleanly on single-node starter clusters (20-*).
// Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
#![forbid(unsafe_code)]

use signal_proto::v1::CellComputeTier;

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
