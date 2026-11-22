// Module: engine/crates/signal-component/src/lib.rs — Canonical ElectronicModule Charge/Flux Stamp Trait
// Invariants: Models stamp exact conserved charge q(v) and flux Phi(i) with analytical Jacobians (F-024).
// Spec: architecture/03-rust-eda-engine.md
#![forbid(unsafe_code)]

use signal_core::Microvolts;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConservativeStamp {
    pub current_amps: f64,
    pub charge_coulombs: f64,
    pub flux_webers: f64,
    pub conductance_siemens: f64,
}

pub trait ElectronicModule: std::fmt::Debug + Send + Sync {
    fn model_urn(&self) -> &str;
    fn terminal_count(&self) -> usize;
    fn evaluate_conservative(
        &self,
        terminal_bias: &[Microvolts],
        temp_mk: i64,
    ) -> ConservativeStamp;
}
