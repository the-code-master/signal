// Module: engine/crates/signal-diff-sim/src/fast_sweep.rs — Taylor + Saltation-Adjoint Fast What-If Scrubber (FEAT-085)
// Invariants: Combines exact Saltation-Adjoint first-order tangent expansion with multi-point batch sweep (< 15 ms).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::DiffSimError;
use crate::revolve_adjoint::SwitchedBuckChopper;

#[derive(Debug, Clone, PartialEq)]
pub struct WhatIfCandidateCurve {
    pub candidate_param_si: i64,
    pub taylor_predicted_loss: f64,
    pub exact_evaluated_loss: f64,
    pub vout_end_volts: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FastWhatIfSweepSummary {
    pub base_param_si: i64,
    pub base_loss: f64,
    pub saltation_gradient_per_si: f64,
    pub curves: Vec<WhatIfCandidateCurve>,
}

/// Evaluates first-order Taylor + Saltation-Adjoint sensitivity expansion alongside multi-point batch sweep (`FEAT-085`).
/// Here `candidate_inductances_ph` are in picohenries (`1 pH = 1e-12 H`).
pub fn run_fast_what_if_inductance_sweep(
    base_model: &SwitchedBuckChopper,
    x0: [f64; 2],
    steps: usize,
    candidate_inductances_ph: &[i64],
) -> Result<FastWhatIfSweepSummary, DiffSimError> {
    let base_l_ph = (base_model.l_henries * 1.0e12).round() as i64;
    let base_loss = base_model.simulate_loss(x0, steps);
    let grad_dl_per_h = base_model.adjoint_gradient_dl(x0, steps, true)?;
    let grad_per_ph = grad_dl_per_h * 1.0e-12;

    let mut curves = Vec::with_capacity(candidate_inductances_ph.len());
    for &cand_ph in candidate_inductances_ph {
        let delta_h = ((cand_ph - base_l_ph) as f64) * 1.0e-12;
        let taylor_loss = (base_loss + grad_dl_per_h * delta_h).max(0.0);

        let mut cand_model = *base_model;
        cand_model.l_henries = (cand_ph.max(1) as f64) * 1.0e-12;
        let exact_loss = cand_model.simulate_loss(x0, steps);
        let vout_end = cand_model.target_vout_v + (2.0 * exact_loss).sqrt();

        curves.push(WhatIfCandidateCurve {
            candidate_param_si: cand_ph,
            taylor_predicted_loss: taylor_loss,
            exact_evaluated_loss: exact_loss,
            vout_end_volts: vout_end,
        });
    }

    Ok(FastWhatIfSweepSummary {
        base_param_si: base_l_ph,
        base_loss,
        saltation_gradient_per_si: grad_per_ph,
        curves,
    })
}
