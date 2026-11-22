// Module: engine/crates/signal-lib-semiconductor/src/diode_bjt.rs — Shockley Diode (C^2 Limexp + Qj) & Ebers-Moll BJT
// Invariants: C^2-continuous limexp_c2 clamping (F-025); conserved depletion + transit-time junction charge (F-024).
// Spec: architecture/03-rust-eda-engine.md
use signal_component::{
    ConservativeStamp, Dual64, ElectronicModule, MultiTerminalStamp, compute_junction_temp_mk,
    forward_ad_jacobian,
};
use signal_core::{Microvolts, limexp_c2};

const K_OVER_Q: f64 = 8.617_333_262e-5; // V / K
const LIMEXP_MAX: f64 = 40.0;

#[derive(Debug, Clone)]
pub struct ShockleyDiode {
    pub urn: String,
    pub is_amps: f64,
    pub emission_n: f64,
    pub cj0_farads: f64,
    pub vj_volts: f64,
    pub transit_time_s: f64,
    pub gmin_siemens: f64,
}

impl Default for ShockleyDiode {
    fn default() -> Self {
        Self::silicon_1n4148()
    }
}

impl ShockleyDiode {
    pub fn silicon_1n4148() -> Self {
        Self {
            urn: "urn:signal:semi:diode_shockley:v1".to_string(),
            is_amps: 2.52e-9,
            emission_n: 1.752,
            cj0_farads: 4.0e-12,
            vj_volts: 0.7,
            transit_time_s: 4.0e-9,
            gmin_siemens: 1.0e-12,
        }
    }

    pub fn thermal_voltage(temp_mk: i64) -> f64 {
        let t_k = ((temp_mk as f64) * 1.0e-3).max(4.0);
        K_OVER_Q * t_k
    }

    /// Evaluates `(current_a, conductance_s, charge_c, cap_f)` at branch voltage `vd_volts`.
    pub fn eval_iv_qv(&self, vd_volts: f64, temp_mk: i64) -> (f64, f64, f64, f64) {
        let nvt = self.emission_n * Self::thermal_voltage(temp_mk);
        let (ex, dex, _) = limexp_c2(vd_volts / nvt, LIMEXP_MAX);
        let id = self.is_amps * (ex - 1.0) + self.gmin_siemens * vd_volts;
        let gd = (self.is_amps / nvt) * dex + self.gmin_siemens;

        // Smooth C^1 depletion charge linearized above fc * Vj = 0.5 * Vj
        let v_fc = 0.5 * self.vj_volts;
        let (q_dep, c_dep) = if vd_volts <= v_fc {
            let one_m = 1.0 - vd_volts / self.vj_volts;
            let root = one_m.sqrt();
            (
                2.0 * self.vj_volts * self.cj0_farads * (1.0 - root),
                self.cj0_farads / root,
            )
        } else {
            let s_half = 0.5_f64.sqrt();
            let q0 = 2.0 * self.vj_volts * self.cj0_farads * (1.0 - s_half);
            let c0 = self.cj0_farads / s_half;
            let slope = 0.5 * c0 / (0.5 * self.vj_volts);
            let dv = vd_volts - v_fc;
            (q0 + c0 * dv + 0.5 * slope * dv * dv, c0 + slope * dv)
        };

        let q_tot = self.transit_time_s * id + q_dep;
        let c_tot = self.transit_time_s * gd + c_dep;
        (id, gd, q_tot, c_tot)
    }
}

impl ElectronicModule for ShockleyDiode {
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
        let vp = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let vn = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let (id, gd, qd, _) = self.eval_iv_qv(vp - vn, temp_mk);
        ConservativeStamp {
            current_amps: id,
            charge_coulombs: qd,
            flux_webers: 0.0,
            conductance_siemens: gd,
        }
    }

    fn evaluate_multi_terminal(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> MultiTerminalStamp {
        let vp = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let vn = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let (id, gd, qd, cd) = self.eval_iv_qv(vp - vn, temp_mk);
        let tj = compute_junction_temp_mk(vp - vn, id, temp_mk, 180.0).0;
        MultiTerminalStamp {
            terminal_currents_a: vec![id, -id],
            terminal_charges_c: vec![qd, -qd],
            branch_fluxes_wb: Vec::new(),
            g_jacobian: vec![vec![gd, -gd], vec![-gd, gd]],
            c_jacobian: vec![vec![cd, -cd], vec![-cd, cd]],
            junction_temp_mk: tj,
        }
    }
}

/// 3-terminal (`C`, `B`, `E`) transport Ebers-Moll BJT with Early effect (`NPN` / `PNP`).
#[derive(Debug, Clone)]
pub struct EbersMollBjt {
    pub urn: String,
    pub is_npn: bool,
    pub is_amps: f64,
    pub beta_f: f64,
    pub beta_r: f64,
    pub early_vaf_volts: f64,
    pub cje_farads: f64,
    pub cjc_farads: f64,
}

impl EbersMollBjt {
    pub fn npn_2n3904() -> Self {
        Self {
            urn: "urn:signal:semi:bjt_ebers_moll:v1".to_string(),
            is_npn: true,
            is_amps: 1.0e-14,
            beta_f: 150.0,
            beta_r: 4.0,
            early_vaf_volts: 100.0,
            cje_farads: 4.5e-12,
            cjc_farads: 3.5e-12,
        }
    }
}

impl ElectronicModule for EbersMollBjt {
    fn model_urn(&self) -> &str {
        &self.urn
    }

    fn terminal_count(&self) -> usize {
        3
    }

    fn evaluate_conservative(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> ConservativeStamp {
        let m = self.evaluate_multi_terminal(terminal_bias, temp_mk);
        ConservativeStamp {
            current_amps: m.terminal_currents_a[0],
            charge_coulombs: m.terminal_charges_c[0],
            flux_webers: 0.0,
            conductance_siemens: m.g_jacobian[0][0],
        }
    }

    fn evaluate_multi_terminal(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> MultiTerminalStamp {
        let vc = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let vb = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let ve = terminal_bias.get(2).map_or(0.0, |v| v.to_si_f64());
        let pol = if self.is_npn { 1.0 } else { -1.0 };
        let vt = ShockleyDiode::thermal_voltage(temp_mk);

        let (currents, g_jac) = forward_ad_jacobian(&[vc, vb, ve], 3, |v| {
            let vbe = Dual64::constant(pol) * (v[1] - v[2]);
            let vbc = Dual64::constant(pol) * (v[1] - v[0]);
            let vce = Dual64::constant(pol) * (v[0] - v[2]);
            let ifwd = Dual64::constant(self.is_amps)
                * ((vbe / Dual64::constant(vt)).limexp(LIMEXP_MAX) - Dual64::constant(1.0));
            let irev = Dual64::constant(self.is_amps)
                * ((vbc / Dual64::constant(vt)).limexp(LIMEXP_MAX) - Dual64::constant(1.0));
            let early = Dual64::constant(1.0)
                + (vce / Dual64::constant(self.early_vaf_volts)).tanh() * Dual64::constant(0.25);
            let ict = (ifwd - irev) * early;
            let ib = ifwd / Dual64::constant(self.beta_f) + irev / Dual64::constant(self.beta_r);
            let ic = ict - irev / Dual64::constant(self.beta_r);
            let ie = -(ic + ib);
            vec![
                Dual64::constant(pol) * ic,
                Dual64::constant(pol) * ib,
                Dual64::constant(pol) * ie,
            ]
        });

        let qbe = self.cje_farads * (vb - ve);
        let qbc = self.cjc_farads * (vb - vc);
        let c_jac = vec![
            vec![self.cjc_farads, -self.cjc_farads, 0.0],
            vec![
                -self.cjc_farads,
                self.cje_farads + self.cjc_farads,
                -self.cje_farads,
            ],
            vec![0.0, -self.cje_farads, self.cje_farads],
        ];
        let tj = compute_junction_temp_mk(vc - ve, currents[0], temp_mk, 120.0).0;

        MultiTerminalStamp {
            terminal_currents_a: currents,
            terminal_charges_c: vec![-qbc, qbe + qbc, -qbe],
            branch_fluxes_wb: Vec::new(),
            g_jacobian: g_jac,
            c_jacobian: c_jac,
            junction_temp_mk: tj,
        }
    }
}
