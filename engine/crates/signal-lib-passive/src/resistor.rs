// Module: engine/crates/signal-lib-passive/src/resistor.rs — Linear & TCR1/TCR2 Electro-Thermal Resistor
// Invariants: Temperature-dependent R(T) = R0 * (1 + tcr1*dT + tcr2*dT^2); supports SOA & fault injection.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_component::{
    ConservativeStamp, ElectronicModule, FaultInjectionMode, SoaFailureData, SoaSpec,
    apply_fault_injection, evaluate_soa_watchdog,
};
use signal_core::Microvolts;

const TNOM_MK: i64 = 300_150; // 27.0 C nominal SPICE reference

#[derive(Debug, Clone)]
pub struct LinearResistor {
    pub urn: String,
    pub nominal_ohms: f64,
    pub tcr1_per_k: f64,
    pub tcr2_per_k2: f64,
    pub fault: FaultInjectionMode,
    pub soa: SoaSpec,
}

impl LinearResistor {
    pub fn new(nominal_ohms: f64) -> Self {
        Self {
            urn: "urn:signal:passive:resistor:v1".to_string(),
            nominal_ohms: nominal_ohms.max(1.0e-6),
            tcr1_per_k: 0.0,
            tcr2_per_k2: 0.0,
            fault: FaultInjectionMode::None,
            soa: SoaSpec::default(),
        }
    }

    pub fn effective_resistance_ohms(&self, temp_mk: i64) -> f64 {
        let dt = ((temp_mk - TNOM_MK) as f64) * 1.0e-3;
        let factor = (1.0 + self.tcr1_per_k * dt + self.tcr2_per_k2 * dt * dt).max(0.05);
        (self.nominal_ohms * factor).max(1.0e-6)
    }
}

impl ElectronicModule for LinearResistor {
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
        let v_branch = v_p - v_n;
        let g = 1.0 / self.effective_resistance_ohms(temp_mk);
        let base = ConservativeStamp {
            current_amps: g * v_branch,
            charge_coulombs: 0.0,
            flux_webers: 0.0,
            conductance_siemens: g,
        };
        apply_fault_injection(base, v_branch, self.fault)
    }

    fn evaluate_soa(
        &self,
        entity_id: &str,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> Option<SoaFailureData> {
        let s = self.evaluate_conservative(terminal_bias, temp_mk);
        let v = terminal_bias.first().map_or(0.0, |u| u.to_si_f64())
            - terminal_bias.get(1).map_or(0.0, |u| u.to_si_f64());
        evaluate_soa_watchdog(entity_id, v, s.current_amps, temp_mk, &self.soa)
    }
}
