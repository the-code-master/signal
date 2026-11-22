// Module: engine/crates/signal-lib-passive/src/capacitor.rs — Charge-Conserving DC-Bias & Temp-Derated MLCC Capacitor
// Invariants: Stamps exact q(v, T) = beta(T)*C0*Vsat*tanh(v/Vsat) + Cmin*v (never naive C(V)dV/dt, F-024).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use signal_component::{
    ConservativeStamp, ElectronicModule, MultiTerminalStamp, compute_junction_temp_mk,
};
use signal_core::{Microvolts, det_tanh};

/// Class-II (X5R/X7R) voltage- and temperature-derated MLCC capacitor (`F-024`).
#[derive(Debug, Clone)]
pub struct MlccCapacitor {
    pub urn: String,
    pub c0_farads: f64,
    pub c_min_farads: f64,
    pub v_sat_volts: f64,
    pub temp_derate_per_k2: f64,
    pub leak_g_siemens: f64,
}

impl MlccCapacitor {
    pub fn new(c0_farads: f64, v_sat_volts: f64) -> Self {
        Self {
            urn: "urn:signal:passive:mlcc_capacitor:v1".to_string(),
            c0_farads: c0_farads.max(0.0),
            c_min_farads: (0.15 * c0_farads).max(1.0e-18),
            v_sat_volts: v_sat_volts.max(0.1),
            temp_derate_per_k2: 1.5e-5, // X7R parabolic roll-off
            leak_g_siemens: 1.0e-12,
        }
    }

    fn temp_factor(&self, temp_mk: i64) -> f64 {
        let dt = ((temp_mk - 298_150) as f64) * 1.0e-3;
        (1.0 / (1.0 + self.temp_derate_per_k2 * dt * dt)).clamp(0.2, 1.0)
    }

    /// Evaluates exact conserved charge $q(v, T)$ and differential capacitance $\partial q / \partial v$ (`F-024`).
    pub fn evaluate_charge_and_diff_cap(&self, v_volts: f64, temp_mk: i64) -> (f64, f64) {
        let beta = self.temp_factor(temp_mk);
        let c_nl = beta * self.c0_farads;
        let u = v_volts / self.v_sat_volts;
        let th = det_tanh(u);
        let sech_sq = 1.0 - th * th;
        let q = c_nl * self.v_sat_volts * th + self.c_min_farads * v_volts;
        let dq_dv = c_nl * sech_sq + self.c_min_farads;
        (q, dq_dv)
    }
}

impl ElectronicModule for MlccCapacitor {
    fn model_urn(&self) -> &str {
        &self.urn
    }

    fn terminal_count(&self) -> usize {
        2
    }

    fn evaluate_conservative(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> ConservativeStamp {
        let v_p = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let v_n = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let v = v_p - v_n;
        let (q, _) = self.evaluate_charge_and_diff_cap(v, temp_mk);
        ConservativeStamp {
            current_amps: self.leak_g_siemens * v,
            charge_coulombs: q,
            flux_webers: 0.0,
            conductance_siemens: self.leak_g_siemens,
        }
    }

    fn evaluate_multi_terminal(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> MultiTerminalStamp {
        let v_p = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let v_n = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let v = v_p - v_n;
        let (q, c_diff) = self.evaluate_charge_and_diff_cap(v, temp_mk);
        let i_leak = self.leak_g_siemens * v;
        let tj = compute_junction_temp_mk(v, i_leak, temp_mk, 80.0).0;
        MultiTerminalStamp {
            terminal_currents_a: vec![i_leak, -i_leak],
            terminal_charges_c: vec![q, -q],
            branch_fluxes_wb: Vec::new(),
            g_jacobian: vec![
                vec![self.leak_g_siemens, -self.leak_g_siemens],
                vec![-self.leak_g_siemens, self.leak_g_siemens],
            ],
            c_jacobian: vec![vec![c_diff, -c_diff], vec![-c_diff, c_diff]],
            junction_temp_mk: tj,
        }
    }
}
