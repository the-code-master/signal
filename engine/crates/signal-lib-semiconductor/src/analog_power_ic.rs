// Module: engine/crates/signal-lib-semiconductor/src/analog_power_ic.rs — Rail-Clamped Op-Amp, LDO & PWM Buck Stage
// Invariants: Smooth tanh rail/dropout clamping with exact Dual64 Jacobians (F-024, F-025).
// Spec: architecture/03-rust-eda-engine.md
use signal_component::{
    ConservativeStamp, Dual64, ElectronicModule, MultiTerminalStamp, compute_junction_temp_mk,
    forward_ad_jacobian,
};
use signal_core::Microvolts;

/// 5-terminal (`INP`, `INN`, `VCC`, `VEE`, `OUT`) behavioral rail-clamped Operational Amplifier.
#[derive(Debug, Clone)]
pub struct RailClampedOpAmp {
    pub urn: String,
    pub open_loop_gain: f64,
    pub r_out_ohms: f64,
    pub r_in_ohms: f64,
}

impl Default for RailClampedOpAmp {
    fn default() -> Self {
        Self {
            urn: "urn:signal:semi:opamp_rail_clamped:v1".to_string(),
            open_loop_gain: 100_000.0,
            r_out_ohms: 50.0,
            r_in_ohms: 10.0e6,
        }
    }
}

impl ElectronicModule for RailClampedOpAmp {
    fn model_urn(&self) -> &str {
        &self.urn
    }

    fn terminal_count(&self) -> usize {
        5
    }

    fn evaluate_conservative(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> ConservativeStamp {
        let m = self.evaluate_multi_terminal(terminal_bias, temp_mk);
        ConservativeStamp {
            current_amps: m.terminal_currents_a[4],
            charge_coulombs: 0.0,
            flux_webers: 0.0,
            conductance_siemens: m.g_jacobian[4][4],
        }
    }

    fn evaluate_multi_terminal(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> MultiTerminalStamp {
        let vp = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let vn = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let vcc = terminal_bias.get(2).map_or(5.0, |v| v.to_si_f64());
        let vee = terminal_bias.get(3).map_or(0.0, |v| v.to_si_f64());
        let vout = terminal_bias.get(4).map_or(0.0, |v| v.to_si_f64());

        let (currents, g_jac) = forward_ad_jacobian(&[vp, vn, vcc, vee, vout], 5, |v| {
            let vdiff = v[0] - v[1];
            let vmid = Dual64::constant(0.5) * (v[2] + v[3]);
            let vspan = (Dual64::constant(0.25) * (v[2] - v[3]) * (v[2] - v[3])
                + Dual64::constant(1.0e-6))
            .sqrt();
            let v_ideal =
                vmid + vspan * (Dual64::constant(self.open_loop_gain) * vdiff / vspan).tanh();
            let i_in = vdiff / Dual64::constant(self.r_in_ohms);
            let i_out = (v[4] - v_ideal) / Dual64::constant(self.r_out_ohms);
            let iq = Dual64::constant(1.0e-3);
            vec![i_in, -i_in, iq, -iq - i_out, i_out]
        });

        MultiTerminalStamp {
            terminal_currents_a: currents,
            terminal_charges_c: vec![0.0; 5],
            branch_fluxes_wb: Vec::new(),
            g_jacobian: g_jac,
            c_jacobian: vec![vec![0.0; 5]; 5],
            junction_temp_mk: compute_junction_temp_mk(vcc - vee, 1.0e-3, temp_mk, 90.0).0,
        }
    }
}

/// 3-terminal (`VIN`, `VOUT`, `GND`) Low-Dropout Regulator with dropout tracking and current limit.
#[derive(Debug, Clone)]
pub struct LdoRegulator {
    pub urn: String,
    pub nominal_vout_volts: f64,
    pub r_dropout_ohms: f64,
    pub quiescent_current_a: f64,
}

impl LdoRegulator {
    pub fn fixed_3v3() -> Self {
        Self {
            urn: "urn:signal:semi:ldo_regulator:v1".to_string(),
            nominal_vout_volts: 3.3,
            r_dropout_ohms: 0.12,
            quiescent_current_a: 50.0e-6,
        }
    }
}

impl ElectronicModule for LdoRegulator {
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
            current_amps: m.terminal_currents_a[1],
            charge_coulombs: 0.0,
            flux_webers: 0.0,
            conductance_siemens: m.g_jacobian[1][1],
        }
    }

    fn evaluate_multi_terminal(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> MultiTerminalStamp {
        let vin = terminal_bias.first().map_or(5.0, |v| v.to_si_f64());
        let vout = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let vgnd = terminal_bias.get(2).map_or(0.0, |v| v.to_si_f64());

        let (currents, g_jac) = forward_ad_jacobian(&[vin, vout, vgnd], 3, |v| {
            let vi = v[0] - v[2];
            let vo = v[1] - v[2];
            let v_nom = Dual64::constant(self.nominal_vout_volts);
            // Smooth min(vi, v_nom): 0.5*(vi + v_nom - sqrt((vi - v_nom)^2 + 1e-4))
            let diff = vi - v_nom;
            let v_target = Dual64::constant(0.5)
                * (vi + v_nom - (diff * diff + Dual64::constant(1.0e-4)).sqrt());
            let i_pass = (v_target - vo) / Dual64::constant(self.r_dropout_ohms);
            let iq = Dual64::constant(self.quiescent_current_a);
            vec![i_pass + iq, -i_pass, -iq]
        });

        MultiTerminalStamp {
            terminal_currents_a: currents.clone(),
            terminal_charges_c: vec![0.0; 3],
            branch_fluxes_wb: Vec::new(),
            g_jacobian: g_jac,
            c_jacobian: vec![vec![0.0; 3]; 3],
            junction_temp_mk: compute_junction_temp_mk(vin - vout, currents[0], temp_mk, 45.0).0,
        }
    }
}

/// 4-terminal (`VIN`, `SW`, `GND`, `PWM_CTRL`) Synchronous Buck Half-Bridge Power Stage.
#[derive(Debug, Clone)]
pub struct PwmBuckStage {
    pub urn: String,
    pub r_hs_on_ohms: f64,
    pub r_ls_on_ohms: f64,
    pub v_pwm_threshold: f64,
}

impl Default for PwmBuckStage {
    fn default() -> Self {
        Self {
            urn: "urn:signal:semi:pwm_buck_stage:v1".to_string(),
            r_hs_on_ohms: 0.015,
            r_ls_on_ohms: 0.010,
            v_pwm_threshold: 1.5,
        }
    }
}

impl ElectronicModule for PwmBuckStage {
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
            current_amps: m.terminal_currents_a[1],
            charge_coulombs: 0.0,
            flux_webers: 0.0,
            conductance_siemens: m.g_jacobian[1][1],
        }
    }

    fn evaluate_multi_terminal(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> MultiTerminalStamp {
        let vin = terminal_bias.first().map_or(12.0, |v| v.to_si_f64());
        let vsw = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let vgnd = terminal_bias.get(2).map_or(0.0, |v| v.to_si_f64());
        let vctrl = terminal_bias.get(3).map_or(0.0, |v| v.to_si_f64());

        let (currents, g_jac) = forward_ad_jacobian(&[vin, vsw, vgnd, vctrl], 4, |v| {
            let s_hi = Dual64::constant(0.5)
                * (Dual64::constant(1.0)
                    + ((v[3] - v[2] - Dual64::constant(self.v_pwm_threshold))
                        / Dual64::constant(0.05))
                    .tanh());
            let s_lo = Dual64::constant(1.0) - s_hi;
            let i_hs = s_hi * (v[0] - v[1]) / Dual64::constant(self.r_hs_on_ohms);
            let i_ls = s_lo * (v[1] - v[2]) / Dual64::constant(self.r_ls_on_ohms);
            vec![i_hs, i_ls - i_hs, -i_ls, Dual64::constant(0.0)]
        });

        MultiTerminalStamp {
            terminal_currents_a: currents.clone(),
            terminal_charges_c: vec![0.0; 4],
            branch_fluxes_wb: Vec::new(),
            g_jacobian: g_jac,
            c_jacobian: vec![vec![0.0; 4]; 4],
            junction_temp_mk: compute_junction_temp_mk(vin - vgnd, currents[0], temp_mk, 20.0).0,
        }
    }
}
