// Module: engine/crates/signal-lib-passive/src/lib.rs — Passive Component Models (Resistor, MLCC, Saturating Inductor, Transformer)
// Invariants: Analytical charge/flux Jacobians match Dual64 / finite-difference within < 1e-9 (F-024).
// Spec: architecture/03-rust-eda-engine.md
#![forbid(unsafe_code)]

pub mod capacitor;
pub mod inductor_transformer;
pub mod resistor;

pub use capacitor::MlccCapacitor;
pub use inductor_transformer::{CoupledTransformer, SaturatingInductor};
pub use resistor::LinearResistor;

#[cfg(test)]
mod tests {
    use super::*;
    use signal_component::ElectronicModule;
    use signal_core::Microvolts;

    #[test]
    fn mlcc_and_saturating_inductor_jacobians_match_central_differences() {
        let cap = MlccCapacitor::new(10.0e-6, 6.3);
        let h = 1.0e-6;
        for &v in &[-5.0, 0.0, 3.3, 12.0] {
            let (_, c_ana) = cap.evaluate_charge_and_diff_cap(v, 310_000);
            let (q_hi, _) = cap.evaluate_charge_and_diff_cap(v + h, 310_000);
            let (q_lo, _) = cap.evaluate_charge_and_diff_cap(v - h, 310_000);
            let c_fd = (q_hi - q_lo) / (2.0 * h);
            assert!(((c_ana - c_fd) / c_ana).abs() < 1e-8);
        }
        assert!(cap.evaluate_charge_and_diff_cap(12.0, 298_150).1 < cap.c0_farads * 0.5);

        let ind = SaturatingInductor::new(4.7e-6, 3.0, 0.02);
        for &i in &[0.0, 1.5, 4.5] {
            let (_, l_ana) = ind.evaluate_flux_and_diff_inductance(i);
            let (p_hi, _) = ind.evaluate_flux_and_diff_inductance(i + h);
            let (p_lo, _) = ind.evaluate_flux_and_diff_inductance(i - h);
            let l_fd = (p_hi - p_lo) / (2.0 * h);
            assert!(((l_ana - l_fd) / l_ana).abs() < 1e-8);
        }

        let r = LinearResistor {
            tcr1_per_k: 100e-6,
            ..LinearResistor::new(1_000.0)
        };
        let s = r.evaluate_conservative(&[Microvolts(5_000_000), Microvolts(0)], 350_150);
        assert!(s.current_amps < 0.005); // Positive TCR increases R above 27C

        let xfmr = CoupledTransformer::new(100e-6, 25e-6, 0.98);
        let ([phi1, phi2], lmat) = xfmr.evaluate_winding_fluxes(2.0, -1.0);
        assert!((lmat[0][1] - 49e-6).abs() < 1e-12);
        assert!(phi1 > 0.0 && phi2 > 0.0);
    }
}
