// Module: engine/crates/signal-component/src/lib.rs — Canonical ElectronicModule Charge/Flux Trait, Dual64 AD & SOA
// Invariants: Models stamp exact conserved charge q(v) and flux Phi(i) with analytical + Dual64 Jacobians (F-024).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

pub mod dual;
pub mod soa_thermal;

pub use dual::{Dual64, forward_ad_jacobian};
pub use soa_thermal::{
    FaultInjectionMode, SoaFailureData, SoaSpec, apply_fault_injection, compute_junction_temp_mk,
    evaluate_soa_watchdog,
};

use signal_core::Microvolts;

/// Two-terminal conservative DC/transient stamp (`q(v)`, `Phi(i)`, `I(v)`, `dI/dV`, `F-024`).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ConservativeStamp {
    pub current_amps: f64,
    pub charge_coulombs: f64,
    pub flux_webers: f64,
    pub conductance_siemens: f64,
}

/// Multi-terminal conservative state & analytical Jacobian stamp for `N`-port devices (`BJT`, `MOSFET`, `OpAmp`, `Transformer`).
#[derive(Debug, Clone, PartialEq)]
pub struct MultiTerminalStamp {
    pub terminal_currents_a: Vec<f64>,
    pub terminal_charges_c: Vec<f64>,
    pub branch_fluxes_wb: Vec<f64>,
    pub g_jacobian: Vec<Vec<f64>>,
    pub c_jacobian: Vec<Vec<f64>>,
    pub junction_temp_mk: i64,
}

pub trait ElectronicModule: std::fmt::Debug + Send + Sync {
    fn model_urn(&self) -> &str;
    fn terminal_count(&self) -> usize;
    fn evaluate_conservative(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> ConservativeStamp;

    /// Evaluates full `N`-terminal currents, conserved charges `q(v)`, conductance Jacobian `dI/dV`, and capacitance Jacobian `dq/dV`.
    fn evaluate_multi_terminal(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> MultiTerminalStamp {
        let s = self.evaluate_conservative(terminal_bias, temp_mk);
        let v0 = terminal_bias.first().map_or(0.0, |u| u.to_si_f64());
        let v1 = terminal_bias.get(1).map_or(0.0, |u| u.to_si_f64());
        let tj = compute_junction_temp_mk(v0 - v1, s.current_amps, temp_mk, 100.0).0;
        MultiTerminalStamp {
            terminal_currents_a: vec![s.current_amps, -s.current_amps],
            terminal_charges_c: vec![s.charge_coulombs, -s.charge_coulombs],
            branch_fluxes_wb: if s.flux_webers != 0.0 {
                vec![s.flux_webers]
            } else {
                Vec::new()
            },
            g_jacobian: vec![
                vec![s.conductance_siemens, -s.conductance_siemens],
                vec![-s.conductance_siemens, s.conductance_siemens],
            ],
            c_jacobian: vec![vec![0.0; 2]; 2],
            junction_temp_mk: tj,
        }
    }

    fn evaluate_soa(
        &self,
        entity_id: &str,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> Option<SoaFailureData> {
        let s = self.evaluate_conservative(terminal_bias, temp_mk);
        let v_diff = terminal_bias.first().map_or(0.0, |u| u.to_si_f64())
            - terminal_bias.get(1).map_or(0.0, |u| u.to_si_f64());
        evaluate_soa_watchdog(
            entity_id,
            v_diff,
            s.current_amps,
            temp_mk,
            &SoaSpec::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::Nanometers;

    #[test]
    fn dual64_forward_ad_matches_analytical_transcendentals() {
        let (_, jac) = forward_ad_jacobian(&[1.2, 0.5], 1, |x| {
            vec![(x[0] * x[1]).tanh() + x[0].limexp(20.0)]
        });
        let h = 1.0e-7;
        let f = |a: f64, b: f64| signal_core::det_tanh(a * b) + signal_core::limexp_c2(a, 20.0).0;
        let fd0 = (f(1.2 + h, 0.5) - f(1.2 - h, 0.5)) / (2.0 * h);
        let fd1 = (f(1.2, 0.5 + h) - f(1.2, 0.5 - h)) / (2.0 * h);
        assert!(((jac[0][0] - fd0) / fd0).abs() < 1e-8);
        assert!(((jac[0][1] - fd1) / fd1).abs() < 1e-8);
    }

    #[test]
    fn soa_watchdog_emits_smoke_event_and_fault_injection_modifies_stamp() {
        let spec = SoaSpec {
            max_abs_voltage_uv: 25_000_000,
            max_abs_current_na: 1_000_000_000,
            max_power_uw: 250_000, // 0.25 W
            max_junction_temp_mk: 423_150,
            r_theta_jc_k_per_w: 150.0,
            smoke_anchor_nm: (Nanometers(100), Nanometers(200)),
            recommended_upgrade_mpn: "RC1206_1W".to_string(),
        };
        let fail =
            evaluate_soa_watchdog("R_LOAD", 12.0, 0.1, 298_150, &spec).expect("1.2W > 0.25W");
        assert_eq!(fail.failure_mode, "THERMAL_OVERPOWER_SMOKE");
        assert_eq!(fail.recommended_upgrade_mpn, "RC1206_1W");
        assert!(fail.junction_temp_mkelvin > 298_150);

        let base = ConservativeStamp {
            current_amps: 0.01,
            charge_coulombs: 0.0,
            flux_webers: 0.0,
            conductance_siemens: 0.001,
        };
        let shorted = apply_fault_injection(base, 5.0, FaultInjectionMode::Short);
        assert!(shorted.current_amps > 1000.0);
        let opened = apply_fault_injection(base, 5.0, FaultInjectionMode::Open);
        assert!(opened.conductance_siemens < 1e-9);
    }
}
