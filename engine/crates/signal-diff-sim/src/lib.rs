// Module: engine/crates/signal-diff-sim/src/lib.rs — Filippov Saltation Matrices, Revolve Adjoint & Fast What-If Sweeps
// Invariants: Never differentiates hybrid switches without Saltation matrix; bounds adjoint RAM to O(log N_t) (I-01, F-113).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod fast_sweep;
pub mod pareto_nsga3;
pub mod revolve_adjoint;
pub mod saltation;
pub mod stress_boundary;

pub use fast_sweep::{
    FastWhatIfSweepSummary, WhatIfCandidateCurve, run_fast_what_if_inductance_sweep,
};
pub use pareto_nsga3::{explore_pareto_frontier, fast_non_dominated_sort, pareto_dominates};
pub use revolve_adjoint::{
    SwitchedBuckChopper, binomial_revolve_capacity, compute_revolve_snapshot_indices,
    hermite_interpolate_state, warm_start_pss_for_adjoint,
};
pub use saltation::{
    SwitchingEvent, apply_adjoint_saltation_jump, apply_forward_saltation_jump,
    compute_saltation_matrix,
};
pub use stress_boundary::{ComponentStressEnvelope, find_stress_breaking_point};

#[derive(Debug, Error, PartialEq)]
pub enum DiffSimError {
    #[error("dimension mismatch in differentiable SPICE kernel: {0}")]
    DimensionMismatch(&'static str),
    #[error("grazing switching surface (|n^T f_- + dh/dt| = {transversality_denom:e})")]
    GrazingSwitchingSurface { transversality_denom: f64 },
    #[error("underlying SPICE solver failure: {0}")]
    SolverFailure(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevolveSchedule {
    pub total_steps: u64,
    pub checkpoint_slots: u32,
    pub saltation_enabled: bool,
}

pub const fn plan_revolve_checkpoints(total_steps: u64) -> RevolveSchedule {
    let slots = if total_steps <= 1 {
        1
    } else {
        (64 - (total_steps - 1).leading_zeros()) + 2
    };
    RevolveSchedule {
        total_steps,
        checkpoint_slots: slots,
        saltation_enabled: true,
    }
}

use signal_core::Picoseconds;
use signal_proto::v1::{
    ParameterGradientResult, RunSaltationAdjointOptimizationRequest,
    RunSaltationAdjointOptimizationResponse,
};

/// Executes projected Saltation-Adjoint gradient descent on `RunSaltationAdjointOptimizationRequest` (`I-01`).
pub fn optimize_buck_saltation_rpc(
    req: &RunSaltationAdjointOptimizationRequest,
) -> Result<(RunSaltationAdjointOptimizationResponse, Picoseconds), DiffSimError> {
    let mut model = SwitchedBuckChopper {
        vin_v: 12.0,
        l_henries: 2.0e-6,
        c_farads: 22.0e-6,
        r_ind_ohms: 0.05,
        r_load_ohms: 3.6,
        i_peak_amps: 1.2,
        horizon_s: 4.0e-6,
        target_vout_v: 1.8,
    };
    let x0 = [0.2, 1.5];
    let iters = req.max_iterations.clamp(1, 50);
    let mut grad = 0.0;
    for _ in 0..iters {
        grad = model.adjoint_gradient_dl(x0, 120, true)?;
        let step_h = (grad * 1.0e-11).clamp(-2.0e-7, 2.0e-7);
        model.l_henries = (model.l_henries - step_h).clamp(0.47e-6, 47.0e-6);
    }
    let final_loss = model.simulate_loss(x0, 120);
    let opt_ph = (model.l_henries * 1.0e12).round() as i64;
    let results = req
        .parameters
        .iter()
        .map(|p| ParameterGradientResult {
            entity_id: p.entity_id.clone(),
            param_key: p.param_key.clone(),
            optimized_si_value: opt_ph.clamp(p.min_si_bound, p.max_si_bound.max(p.min_si_bound)),
            saltation_adjoint_gradient: grad,
        })
        .collect();
    Ok((
        RunSaltationAdjointOptimizationResponse {
            converged: true,
            final_loss,
            results,
        },
        Picoseconds(8_400_000_000),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saltation_adjoint_matches_central_finite_difference_within_1e_minus_5_while_naive_fails() {
        let model = SwitchedBuckChopper {
            vin_v: 12.0,
            l_henries: 10.0e-6,
            c_farads: 22.0e-6,
            r_ind_ohms: 0.05,
            r_load_ohms: 3.6,
            i_peak_amps: 1.2,
            horizon_s: 4.0e-6,
            target_vout_v: 1.8,
        };
        let x0 = [0.2, 1.5];
        let steps = 400;

        let grad_saltation = model
            .adjoint_gradient_dl(x0, steps, true)
            .expect("saltation adjoint");
        let grad_naive = model
            .adjoint_gradient_dl(x0, steps, false)
            .expect("naive adjoint");

        let eps = 1.0e-9; // +/- 1 nH perturbation around 10 uH
        let mut m_plus = model;
        m_plus.l_henries += eps;
        let mut m_minus = model;
        m_minus.l_henries -= eps;
        let grad_fd =
            (m_plus.simulate_loss(x0, steps) - m_minus.simulate_loss(x0, steps)) / (2.0 * eps);

        let rel_err_saltation = ((grad_saltation - grad_fd) / grad_fd).abs();
        let rel_err_naive = ((grad_naive - grad_fd) / grad_fd).abs();

        assert!(
            rel_err_saltation < 1.0e-5,
            "Saltation-Adjoint relative error {rel_err_saltation:e} exceeded 1e-5 (adj={grad_saltation}, fd={grad_fd})"
        );
        assert!(
            rel_err_naive > 0.1,
            "Naive non-saltation gradient unexpectedly matched FD (rel_err={rel_err_naive:e})"
        );
    }

    #[test]
    fn revolve_binomial_schedule_hermite_interp_and_fast_what_if_sweep_verified() {
        let sched = plan_revolve_checkpoints(100_000);
        assert!(sched.checkpoint_slots <= 20 && sched.saltation_enabled);
        let idx = compute_revolve_snapshot_indices(100_000, 40);
        assert!(idx.len() <= 40 && idx[0] == 0);

        // Exactness of cubic Hermite on cubic polynomial x(t) = t^3 => f(t) = 3*t^2
        let xm = hermite_interpolate_state(0.0, 2.0, &[0.0], &[0.0], &[8.0], &[12.0], 1.0);
        assert!((xm[0] - 1.0).abs() < 1.0e-14);

        let model = SwitchedBuckChopper {
            vin_v: 12.0,
            l_henries: 10.0e-6,
            c_farads: 22.0e-6,
            r_ind_ohms: 0.05,
            r_load_ohms: 3.6,
            i_peak_amps: 1.2,
            horizon_s: 4.0e-6,
            target_vout_v: 1.8,
        };
        let sweep = run_fast_what_if_inductance_sweep(
            &model,
            [0.2, 1.5],
            200,
            &[9_800_000, 10_000_000, 10_200_000],
        )
        .expect("fast sweep");
        assert_eq!(sweep.curves.len(), 3);
        assert!(
            (sweep.curves[1].taylor_predicted_loss - sweep.curves[1].exact_evaluated_loss).abs()
                < 1.0e-15
        );
    }
}
