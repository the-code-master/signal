// Module: engine/crates/signal-sim/src/circuit.rs — Charge/Flux-Conserving MNA Circuit Builder
// Invariants: Unified DAE d(q(x))/dt + f(x, t) = 0; delegates stamping to stamp.rs & deck parsing to spice_loader.rs (F-024..029).
// Spec: architecture/03-rust-eda-engine.md
pub use crate::elements::{AssembledMnaState, CircuitElement};
use crate::equilibrate_solver::Complex64;
use crate::spice_loader::load_mna_from_spice_deck;
pub use crate::spice_loader::pnjlim;
use crate::stamp::assemble_mna_system;
use signal_lib_passive::{LinearResistor, MlccCapacitor, SaturatingInductor};
use signal_lib_semiconductor::{ContinuousMosfet, ShockleyDiode};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct MnaCircuit {
    pub num_nodes: usize,
    pub num_branches: usize,
    pub elements: Vec<CircuitElement>,
}

impl MnaCircuit {
    pub const fn new(num_nodes: usize) -> Self {
        Self {
            num_nodes,
            num_branches: 0,
            elements: Vec::new(),
        }
    }

    pub const fn total_unknowns(&self) -> usize {
        self.num_nodes + self.num_branches
    }

    fn alloc_branch(&mut self) -> usize {
        let idx = self.num_nodes + self.num_branches;
        self.num_branches += 1;
        idx
    }

    #[inline]
    pub fn node_v(x: &[f64], node: usize) -> f64 {
        if node == 0 { 0.0 } else { x[node - 1] }
    }

    pub fn add_resistor(&mut self, id: &str, np: usize, nn: usize, r_ohms: f64) {
        self.elements.push(CircuitElement::Resistor {
            id: id.to_string(),
            np,
            nn,
            model: LinearResistor::new(r_ohms),
        });
    }

    pub fn add_capacitor(&mut self, id: &str, np: usize, nn: usize, c_farads: f64) {
        self.elements.push(CircuitElement::LinearCapacitor {
            id: id.to_string(),
            np,
            nn,
            c_farads,
        });
    }

    pub fn add_mlcc(&mut self, id: &str, np: usize, nn: usize, model: MlccCapacitor) {
        self.elements.push(CircuitElement::Mlcc {
            id: id.to_string(),
            np,
            nn,
            model,
        });
    }

    pub fn add_inductor(
        &mut self,
        id: &str,
        np: usize,
        nn: usize,
        l_henries: f64,
        r_series_ohms: f64,
    ) -> usize {
        let b = self.alloc_branch();
        self.elements.push(CircuitElement::LinearInductor {
            id: id.to_string(),
            np,
            nn,
            l_henries,
            r_series_ohms,
            branch_idx: b,
        });
        b
    }

    pub fn add_sat_inductor(
        &mut self,
        id: &str,
        np: usize,
        nn: usize,
        model: SaturatingInductor,
    ) -> usize {
        let b = self.alloc_branch();
        self.elements.push(CircuitElement::SatInductor {
            id: id.to_string(),
            np,
            nn,
            model,
            branch_idx: b,
        });
        b
    }

    pub fn add_dc_ac_vsource(
        &mut self,
        id: &str,
        np: usize,
        nn: usize,
        dc_v: f64,
        ac_mag_v: f64,
    ) -> usize {
        let b = self.alloc_branch();
        self.elements.push(CircuitElement::DcAcVoltage {
            id: id.to_string(),
            np,
            nn,
            dc_v,
            ac_mag_v,
            ac_phase_rad: 0.0,
            branch_idx: b,
        });
        b
    }

    pub fn add_sine_vsource(
        &mut self,
        id: &str,
        np: usize,
        nn: usize,
        offset_v: f64,
        amp_v: f64,
        freq_hz: f64,
    ) -> usize {
        let b = self.alloc_branch();
        self.elements.push(CircuitElement::SineVoltage {
            id: id.to_string(),
            np,
            nn,
            offset_v,
            amp_v,
            freq_hz,
            branch_idx: b,
        });
        b
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_step_vsource(
        &mut self,
        id: &str,
        np: usize,
        nn: usize,
        v0: f64,
        v1: f64,
        delay_s: f64,
        rise_s: f64,
    ) -> usize {
        let b = self.alloc_branch();
        self.elements.push(CircuitElement::StepPulseVoltage {
            id: id.to_string(),
            np,
            nn,
            v_initial: v0,
            v_pulsed: v1,
            delay_s,
            rise_s: rise_s.max(1.0e-12),
            branch_idx: b,
        });
        b
    }

    pub fn add_current_source(&mut self, id: &str, np: usize, nn: usize, dc_a: f64, ac_mag_a: f64) {
        self.elements.push(CircuitElement::CurrentSource {
            id: id.to_string(),
            np,
            nn,
            dc_a,
            ac_mag_a,
        });
    }

    pub fn add_diode(&mut self, id: &str, na: usize, nc: usize, model: ShockleyDiode) {
        self.elements.push(CircuitElement::Diode {
            id: id.to_string(),
            na,
            nc,
            model,
        });
    }

    pub fn add_mosfet(
        &mut self,
        id: &str,
        nd: usize,
        ng: usize,
        ns: usize,
        nb: usize,
        model: ContinuousMosfet,
    ) {
        self.elements.push(CircuitElement::Mosfet {
            id: id.to_string(),
            nd,
            ng,
            ns,
            nb,
            model,
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_pwm_switch(
        &mut self,
        id: &str,
        np: usize,
        nn: usize,
        r_on: f64,
        r_off: f64,
        period_s: f64,
        duty: f64,
        high_side: bool,
    ) {
        self.elements.push(CircuitElement::PwmSwitch {
            id: id.to_string(),
            np,
            nn,
            r_on,
            r_off,
            period_s,
            duty,
            high_side,
        });
    }

    pub fn from_spice_deck(src: &str) -> Result<(Self, BTreeMap<String, usize>), String> {
        load_mna_from_spice_deck(src)
    }

    pub fn assemble(
        &self,
        x: &[f64],
        t_sec: f64,
        temp_mk: i64,
        gmin: f64,
        src_scale: f64,
    ) -> AssembledMnaState {
        assemble_mna_system(self, x, t_sec, temp_mk, gmin, src_scale)
    }

    pub fn assemble_ac_rhs(&self) -> Vec<Complex64> {
        let mut rhs = vec![Complex64::ZERO; self.total_unknowns()];
        for el in &self.elements {
            match el {
                CircuitElement::DcAcVoltage {
                    ac_mag_v,
                    ac_phase_rad,
                    branch_idx,
                    ..
                } if *ac_mag_v != 0.0 => {
                    rhs[*branch_idx] = Complex64::new(
                        ac_mag_v * ac_phase_rad.cos(),
                        ac_mag_v * ac_phase_rad.sin(),
                    );
                }
                CircuitElement::CurrentSource {
                    np, nn, ac_mag_a, ..
                } if *ac_mag_a != 0.0 => {
                    if *np > 0 {
                        rhs[*np - 1] = rhs[*np - 1] - Complex64::new(*ac_mag_a, 0.0);
                    }
                    if *nn > 0 {
                        rhs[*nn - 1] = rhs[*nn - 1] + Complex64::new(*ac_mag_a, 0.0);
                    }
                }
                _ => {}
            }
        }
        rhs
    }
}
