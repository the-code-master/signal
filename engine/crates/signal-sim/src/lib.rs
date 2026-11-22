// Module: engine/crates/signal-sim/src/lib.rs — Hybrid CPU KLU <-> GPU cuDSS MNA Dispatcher
// Invariants: Enforces 25_000-node crossover threshold (F-116) and tracks PHYSICS_DETERMINISM_EPOCH (20-*).
// Spec: architecture/03-rust-eda-engine.md
#![forbid(unsafe_code)]

use signal_component as _;

pub const PHYSICS_DETERMINISM_EPOCH: u32 = 1;
pub const GPU_CUDSS_NODE_CROSSOVER: usize = 25_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinearSolverBackend {
    CpuKluRuizEquilibrated,
    GpuCuDssMixedPrecision,
}

pub const fn select_mna_backend(node_count: usize, gpu_available: bool) -> LinearSolverBackend {
    if gpu_available && node_count >= GPU_CUDSS_NODE_CROSSOVER {
        LinearSolverBackend::GpuCuDssMixedPrecision
    } else {
        LinearSolverBackend::CpuKluRuizEquilibrated
    }
}
