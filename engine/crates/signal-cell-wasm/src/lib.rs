// Module: engine/crates/signal-cell-wasm/src/lib.rs — EXPLORER Tier < 8 MiB Isolate Quota & Promotion Policy
// Invariants: Enforces 8 MiB linear memory ceiling and triggers < 2 s live MicroVM promotion on overflow (F-138).
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md
#![forbid(unsafe_code)]

use signal_core as _;

pub const EXPLORER_MAX_LINEAR_MEMORY_BYTES: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellPlacementDecision {
    StayInExplorerWasm,
    PromoteToDedicatedMicroVm,
}

pub const fn evaluate_explorer_footprint(
    working_set_bytes: usize,
    requires_3d_em_or_gpu: bool,
) -> CellPlacementDecision {
    if requires_3d_em_or_gpu || working_set_bytes > EXPLORER_MAX_LINEAR_MEMORY_BYTES {
        CellPlacementDecision::PromoteToDedicatedMicroVm
    } else {
        CellPlacementDecision::StayInExplorerWasm
    }
}
