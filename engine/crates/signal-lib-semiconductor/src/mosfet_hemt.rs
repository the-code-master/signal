// Module: engine/crates/signal-lib-semiconductor/src/mosfet_hemt.rs — Continuous EKV MOSFET & Wide-Bandgap SiC/GaN HEMT
// Invariants: Smooth C^infinity subthreshold-to-strong-inversion EKV channel + 3rd-quadrant GaN reverse conduction (F-024).
// Spec: architecture/03-rust-eda-engine.md
use crate::diode_bjt::ShockleyDiode;
use signal_component::{
    ConservativeStamp, Dual64, ElectronicModule, MultiTerminalStamp, compute_junction_temp_mk,
    forward_ad_jacobian,
};
use signal_core::Microvolts;

/// 4-terminal (`D`, `G`, `S`, `B`) continuous EKV / Shichman-Hodges MOSFET (`NMOS` / `PMOS`).
#[derive(Debug, Clone)]
pub struct ContinuousMosfet {
    pub urn: String,
    pub is_nmos: bool,
    pub kp_a_per_v2: f64,
    pub w_over_l: f64,
    pub vth_volts: f64,
    pub subthreshold_n: f64,
    pub lambda_per_v: f64,
    pub cgs_farads: f64,
    pub cgd_farads: f64,
}

impl ContinuousMosfet {
    pub fn nmos_default() -> Self {
        Self {
            urn: "urn:signal:semi:mosfet_ekv:v1".to_string(),
            is_nmos: true,
            kp_a_per_v2: 200.0e-6,
            w_over_l: 10.0,
            vth_volts: 0.7,
            subthreshold_n: 1.35,
            lambda_per_v: 0.02,
            cgs_farads: 25.0e-15,
            cgd_farads: 8.0e-15,
        }
    }

    pub fn eval_ids_dual(&self, vd: Dual64, vg: Dual64, vs: Dual64, temp_mk: i64) -> Dual64 {
        let pol = Dual64::constant(if self.is_nmos { 1.0 } else { -1.0 });
        let vgs = pol * (vg - vs);
        let vgd = pol * (vg - vd);
        let vds = pol * (vd - vs);
        let vt = ShockleyDiode::thermal_voltage(temp_mk);
        let two_n_vt = Dual64::constant(2.0 * self.subthreshold_n * vt);
        let vth = Dual64::constant(self.vth_volts);

        // EKV continuous forward & reverse inversion currents:
        // I_spec = 2 * n * beta * Vt^2, I_f = I_spec * ln^2(1 + exp((Vgs - Vth)/(2*n*Vt)))
        let beta = self.kp_a_per_v2 * self.w_over_l;
        let i_spec = Dual64::constant(2.0 * self.subthreshold_n * beta * vt * vt);
        let uf = ((vgs - vth) / two_n_vt).ln1p_exp_softplus();
        let ur = ((vgd - vth) / two_n_vt).ln1p_exp_softplus();
        let vds_eff = (vds * vds + Dual64::constant(1.0e-6)).sqrt();
        let clm = Dual64::constant(1.0) + Dual64::constant(self.lambda_per_v) * vds_eff;
        pol * i_spec * (uf * uf - ur * ur) * clm
    }
}

impl ElectronicModule for ContinuousMosfet {
    fn model_urn(&self) -> &str {
        &self.urn
    }

    fn terminal_count(&self) -> usize {
        4
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
        let vd = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let vg = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let vs = terminal_bias.get(2).map_or(0.0, |v| v.to_si_f64());
        let vb = terminal_bias.get(3).map_or(vs, |v| v.to_si_f64());

        let (currents, g_jac) = forward_ad_jacobian(&[vd, vg, vs, vb], 4, |v| {
            let ids = self.eval_ids_dual(v[0], v[1], v[2], temp_mk);
            vec![ids, Dual64::constant(0.0), -ids, Dual64::constant(0.0)]
        });

        let qgd = self.cgd_farads * (vg - vd);
        let qgs = self.cgs_farads * (vg - vs);
        let c_jac = vec![
            vec![self.cgd_farads, -self.cgd_farads, 0.0, 0.0],
            vec![
                -self.cgd_farads,
                self.cgd_farads + self.cgs_farads,
                -self.cgs_farads,
                0.0,
            ],
            vec![0.0, -self.cgs_farads, self.cgs_farads, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ];
        let tj = compute_junction_temp_mk(vd - vs, currents[0], temp_mk, 45.0).0;

        MultiTerminalStamp {
            terminal_currents_a: currents,
            terminal_charges_c: vec![-qgd, qgd + qgs, -qgs, 0.0],
            branch_fluxes_wb: Vec::new(),
            g_jacobian: g_jac,
            c_jacobian: c_jac,
            junction_temp_mk: tj,
        }
    }
}

/// 3-terminal (`D`, `G`, `S`) Wide-Bandgap SiC / eGaN HEMT with zero-Qrr 3rd-quadrant reverse conduction.
#[derive(Debug, Clone)]
pub struct WideBandgapHemt {
    pub urn: String,
    pub rds_on_ohms: f64,
    pub vth_volts: f64,
    pub v_rev_knee_volts: f64,
    pub coss_0_farads: f64,
}

impl WideBandgapHemt {
    pub fn gan_100v_10mohm() -> Self {
        Self {
            urn: "urn:signal:semi:gan_hemt:v1".to_string(),
            rds_on_ohms: 0.010,
            vth_volts: 1.5,
            v_rev_knee_volts: 1.8,
            coss_0_farads: 350.0e-12,
        }
    }
}

impl ElectronicModule for WideBandgapHemt {
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
        let vd = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let vg = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let vs = terminal_bias.get(2).map_or(0.0, |v| v.to_si_f64());
        let g_on = 1.0 / self.rds_on_ohms.max(1.0e-4);

        let (currents, g_jac) = forward_ad_jacobian(&[vd, vg, vs], 3, |v| {
            let vgs = v[1] - v[2];
            let vds = v[0] - v[2];
            let vsd = -vds;
            // Smooth gate modulation + 3rd-quadrant 2DEG reverse conduction when Vsd > V_rev_knee
            let gate_open = Dual64::constant(0.5)
                * (Dual64::constant(1.0)
                    + ((vgs - Dual64::constant(self.vth_volts)) / Dual64::constant(0.25)).tanh());
            let i_ch = Dual64::constant(g_on) * gate_open * vds;
            let rev_excess = ((vsd - Dual64::constant(self.v_rev_knee_volts))
                / Dual64::constant(0.1))
            .ln1p_exp_softplus()
                * Dual64::constant(0.1);
            let i_rev = Dual64::constant(0.5 * g_on) * rev_excess;
            let id = i_ch - i_rev;
            vec![id, Dual64::constant(0.0), -id]
        });

        let (q_vec, c_jac) = forward_ad_jacobian(&[vd, vg, vs], 3, |v| {
            let vds = v[0] - v[2];
            let qoss =
                Dual64::constant(self.coss_0_farads * 15.0) * (vds / Dual64::constant(15.0)).tanh();
            vec![qoss, Dual64::constant(0.0), -qoss]
        });
        let tj = compute_junction_temp_mk(vd - vs, currents[0], temp_mk, 12.0).0;

        MultiTerminalStamp {
            terminal_currents_a: currents,
            terminal_charges_c: q_vec,
            branch_fluxes_wb: Vec::new(),
            g_jacobian: g_jac,
            c_jacobian: c_jac,
            junction_temp_mk: tj,
        }
    }
}
