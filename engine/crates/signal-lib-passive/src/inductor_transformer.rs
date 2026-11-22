// Module: engine/crates/signal-lib-passive/src/inductor_transformer.rs — Flux-Conserving Saturating Inductor & Coupled Transformer
// Invariants: Exact anhysteretic Phi(i) = L_sat*i + (L0 - L_sat)*I_sat*tanh(i/I_sat) and coupled flux conservation (F-024).
// Spec: architecture/03-rust-eda-engine.md
use signal_component::{ConservativeStamp, ElectronicModule};
use signal_core::{Microvolts, det_tanh};

/// Anhysteretic Jiles-Atherton / `tanh` saturating power inductor (`F-024`).
#[derive(Debug, Clone)]
pub struct SaturatingInductor {
    pub urn: String,
    pub l0_henries: f64,
    pub l_sat_henries: f64,
    pub i_sat_amps: f64,
    pub dcr_ohms: f64,
}

impl SaturatingInductor {
    pub fn new(l0_henries: f64, i_sat_amps: f64, dcr_ohms: f64) -> Self {
        Self {
            urn: "urn:signal:passive:saturating_inductor:v1".to_string(),
            l0_henries: l0_henries.max(1.0e-12),
            l_sat_henries: (0.15 * l0_henries).max(1.0e-13),
            i_sat_amps: i_sat_amps.max(1.0e-4),
            dcr_ohms: dcr_ohms.max(1.0e-4),
        }
    }

    /// Evaluates conserved magnetic flux $\Phi(i)$ and differential inductance $L_{\text{diff}}(i) = \partial \Phi / \partial i$.
    pub fn evaluate_flux_and_diff_inductance(&self, i_amps: f64) -> (f64, f64) {
        let u = i_amps / self.i_sat_amps;
        let th = det_tanh(u);
        let sech_sq = 1.0 - th * th;
        let dl = (self.l0_henries - self.l_sat_henries).max(0.0);
        let phi = self.l_sat_henries * i_amps + dl * self.i_sat_amps * th;
        let l_diff = self.l_sat_henries + dl * sech_sq;
        (phi, l_diff)
    }
}

impl ElectronicModule for SaturatingInductor {
    fn model_urn(&self) -> &str {
        &self.urn
    }

    fn terminal_count(&self) -> usize {
        2
    }

    fn evaluate_conservative(
        &self,
        terminal_bias: &[Microvolts],
        _temp_mk: i64,
    ) -> ConservativeStamp {
        let v_p = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let v_n = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let g_dc = 1.0 / self.dcr_ohms;
        let i_dc = (v_p - v_n) * g_dc;
        let (phi, _) = self.evaluate_flux_and_diff_inductance(i_dc);
        ConservativeStamp {
            current_amps: i_dc,
            charge_coulombs: 0.0,
            flux_webers: phi,
            conductance_siemens: g_dc,
        }
    }
}

/// 4-terminal coupled magnetic transformer with mutual inductance $M = k \sqrt{L_p L_s}$ and leakage flux conservation.
#[derive(Debug, Clone)]
pub struct CoupledTransformer {
    pub urn: String,
    pub l_primary_h: f64,
    pub l_secondary_h: f64,
    pub coupling_k: f64,
    pub r_primary_ohms: f64,
    pub r_secondary_ohms: f64,
}

impl CoupledTransformer {
    pub fn new(l_primary_h: f64, l_secondary_h: f64, coupling_k: f64) -> Self {
        Self {
            urn: "urn:signal:passive:coupled_transformer:v1".to_string(),
            l_primary_h: l_primary_h.max(1.0e-12),
            l_secondary_h: l_secondary_h.max(1.0e-12),
            coupling_k: coupling_k.clamp(0.0, 0.999_999),
            r_primary_ohms: 0.05,
            r_secondary_ohms: 0.05,
        }
    }

    pub fn mutual_inductance_h(&self) -> f64 {
        self.coupling_k * (self.l_primary_h * self.l_secondary_h).sqrt()
    }

    /// Evaluates conserved primary/secondary fluxes $(\Phi_1, \Phi_2)$ and $2 \times 2$ inductance matrix.
    pub fn evaluate_winding_fluxes(
        &self,
        i_primary_a: f64,
        i_secondary_a: f64,
    ) -> ([f64; 2], [[f64; 2]; 2]) {
        let m = self.mutual_inductance_h();
        let phi1 = self.l_primary_h * i_primary_a + m * i_secondary_a;
        let phi2 = m * i_primary_a + self.l_secondary_h * i_secondary_a;
        (
            [phi1, phi2],
            [[self.l_primary_h, m], [m, self.l_secondary_h]],
        )
    }
}

impl ElectronicModule for CoupledTransformer {
    fn model_urn(&self) -> &str {
        &self.urn
    }

    fn terminal_count(&self) -> usize {
        4
    }

    fn evaluate_conservative(
        &self,
        terminal_bias: &[Microvolts],
        _temp_mk: i64,
    ) -> ConservativeStamp {
        let vp1 = terminal_bias.first().map_or(0.0, |v| v.to_si_f64());
        let vp2 = terminal_bias.get(1).map_or(0.0, |v| v.to_si_f64());
        let vs1 = terminal_bias.get(2).map_or(0.0, |v| v.to_si_f64());
        let vs2 = terminal_bias.get(3).map_or(0.0, |v| v.to_si_f64());
        let ip = (vp1 - vp2) / self.r_primary_ohms;
        let isec = (vs1 - vs2) / self.r_secondary_ohms;
        let ([phi1, _], _) = self.evaluate_winding_fluxes(ip, isec);
        ConservativeStamp {
            current_amps: ip,
            charge_coulombs: 0.0,
            flux_webers: phi1,
            conductance_siemens: 1.0 / self.r_primary_ohms,
        }
    }
}
