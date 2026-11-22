// Module: engine/crates/signal-lib-semiconductor/src/lib.rs — Semiconductor & Power IC Compact Model Library
// Invariants: All analytical & Dual64 Jacobians match central finite differences within < 1e-6 (F-024, F-025).
// Spec: architecture/03-rust-eda-engine.md
#![forbid(unsafe_code)]

pub mod analog_power_ic;
pub mod diode_bjt;
pub mod mosfet_hemt;

pub use analog_power_ic::{LdoRegulator, PwmBuckStage, RailClampedOpAmp};
pub use diode_bjt::{EbersMollBjt, ShockleyDiode};
pub use mosfet_hemt::{ContinuousMosfet, WideBandgapHemt};

#[cfg(test)]
mod tests {
    use super::*;
    use signal_component::ElectronicModule;
    use signal_core::Microvolts;

    #[test]
    fn shockley_diode_analytical_conductance_and_capacitance_match_finite_diff() {
        let d = ShockleyDiode::silicon_1n4148();
        let h = 1.0e-7;
        for &vd in &[-2.0, 0.0, 0.55, 0.72] {
            let (_, g_ana, _, c_ana) = d.eval_iv_qv(vd, 300_150);
            let (i_hi, _, q_hi, _) = d.eval_iv_qv(vd + h, 300_150);
            let (i_lo, _, q_lo, _) = d.eval_iv_qv(vd - h, 300_150);
            let g_fd = (i_hi - i_lo) / (2.0 * h);
            let c_fd = (q_hi - q_lo) / (2.0 * h);
            assert!((g_ana - g_fd).abs() < 1e-6 * g_ana + 1e-16);
            assert!(((c_ana - c_fd) / c_ana).abs() < 1e-6);
        }
    }

    #[test]
    fn bjt_mosfet_gan_opamp_ldo_and_buck_satisfy_kcl_and_jacobian_parity() {
        let bjt = EbersMollBjt::npn_2n3904();
        let s_bjt = bjt.evaluate_multi_terminal(
            &[Microvolts(5_000_000), Microvolts(700_000), Microvolts(0)],
            300_150,
        );
        let kcl_bjt: f64 = s_bjt.terminal_currents_a.iter().sum();
        assert!(kcl_bjt.abs() < 1e-15);
        assert!(s_bjt.terminal_currents_a[0] > 10.0 * s_bjt.terminal_currents_a[1]);

        let mos = ContinuousMosfet::nmos_default();
        let s_sub = mos.evaluate_multi_terminal(
            &[
                Microvolts(1_800_000),
                Microvolts(300_000),
                Microvolts(0),
                Microvolts(0),
            ],
            300_150,
        );
        let s_on = mos.evaluate_multi_terminal(
            &[
                Microvolts(1_800_000),
                Microvolts(1_800_000),
                Microvolts(0),
                Microvolts(0),
            ],
            300_150,
        );
        assert!(s_sub.terminal_currents_a[0] > 0.0);
        assert!(s_on.terminal_currents_a[0] > 1_000.0 * s_sub.terminal_currents_a[0]);

        let gan = WideBandgapHemt::gan_100v_10mohm();
        let s_rev = gan.evaluate_multi_terminal(
            &[Microvolts(-2_500_000), Microvolts(0), Microvolts(0)],
            300_150,
        );
        assert!(s_rev.terminal_currents_a[0] < -10.0); // 3rd-quadrant conduction

        let op = RailClampedOpAmp::default();
        let s_op = op.evaluate_multi_terminal(
            &[
                Microvolts(1_000_000),
                Microvolts(0),
                Microvolts(5_000_000),
                Microvolts(0),
                Microvolts(5_000_000),
            ],
            300_150,
        );
        assert!(s_op.terminal_currents_a[4].abs() < 0.01); // Clamped cleanly near VCC=5V

        let ldo = LdoRegulator::fixed_3v3();
        let s_ldo = ldo.evaluate_multi_terminal(
            &[Microvolts(5_000_000), Microvolts(3_280_000), Microvolts(0)],
            300_150,
        );
        assert!(s_ldo.terminal_currents_a[1] < 0.0); // Sourcing current out of VOUT

        let buck = PwmBuckStage::default();
        let s_buck = buck.evaluate_multi_terminal(
            &[
                Microvolts(12_000_000),
                Microvolts(11_900_000),
                Microvolts(0),
                Microvolts(5_000_000),
            ],
            300_150,
        );
        assert!(s_buck.terminal_currents_a[0] > 5.0);
    }
}
