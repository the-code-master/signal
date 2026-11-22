// Module: engine/crates/signal-sim/src/lib.rs — Hybrid CPU KLU <-> GPU cuDSS MNA SPICE, PSS/HB & Virtual Bench
// Invariants: Enforces 25_000-node crossover threshold (F-116) and tracks PHYSICS_DETERMINISM_EPOCH (20-*).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod analytics;
pub mod anomaly_attribution;
pub mod circuit;
pub mod elements;
pub mod equilibrate_solver;
pub mod instruments;
pub mod instruments_analytics;
pub mod mna_engine;
pub mod pss_hb;
pub mod spice_loader;
pub mod stamp;

pub use anomaly_attribution::{ParasiticLoopCandidate, attribute_waveform_anomaly};
pub use circuit::{AssembledMnaState, CircuitElement, MnaCircuit, pnjlim};
pub use equilibrate_solver::{
    Complex64, GPU_MONTE_CARLO_BATCH_CROSSOVER, LinearSolverDispatcher, RuizEquilibratedSystem,
    neumaier_dot, ruiz_equilibrate, solve_complex_lu, solve_equilibrated_lu,
};
pub use instruments_analytics::*;
pub use mna_engine::{
    AcPointResult, TransientTrajectory, solve_ac_sweep, solve_dc_op, solve_transient_tr_bdf2,
    step_tr_bdf2,
};
pub use pss_hb::{HarmonicBalanceResult, PssResult, solve_harmonic_balance, solve_shooting_pss};

pub const PHYSICS_DETERMINISM_EPOCH: u32 = 1;
pub const GPU_CUDSS_NODE_CROSSOVER: usize = 25_000;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SimError {
    #[error("singular or ill-conditioned MNA Jacobian at pivot {pivot_index}")]
    SingularMatrix { pivot_index: usize },
    #[error("Newton-Raphson solver failed to converge: {0}")]
    NewtonNonConvergence(&'static str),
    #[error("invalid simulation parameter: {0}")]
    InvalidParameter(&'static str),
}

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
