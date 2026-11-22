// Module: engine/crates/signal-sim/src/elements.rs — MNA Circuit Element Enum & Assembled DAE State
// Invariants: Strongly-typed branch/node topologies for passive, semiconductor & switched MNA elements (F-024).
// Spec: architecture/03-rust-eda-engine.md
use signal_lib_passive::{LinearResistor, MlccCapacitor, SaturatingInductor};
use signal_lib_semiconductor::{ContinuousMosfet, ShockleyDiode};

#[derive(Debug, Clone)]
pub enum CircuitElement {
    Resistor {
        id: String,
        np: usize,
        nn: usize,
        model: LinearResistor,
    },
    LinearCapacitor {
        id: String,
        np: usize,
        nn: usize,
        c_farads: f64,
    },
    Mlcc {
        id: String,
        np: usize,
        nn: usize,
        model: MlccCapacitor,
    },
    LinearInductor {
        id: String,
        np: usize,
        nn: usize,
        l_henries: f64,
        r_series_ohms: f64,
        branch_idx: usize,
    },
    SatInductor {
        id: String,
        np: usize,
        nn: usize,
        model: SaturatingInductor,
        branch_idx: usize,
    },
    DcAcVoltage {
        id: String,
        np: usize,
        nn: usize,
        dc_v: f64,
        ac_mag_v: f64,
        ac_phase_rad: f64,
        branch_idx: usize,
    },
    SineVoltage {
        id: String,
        np: usize,
        nn: usize,
        offset_v: f64,
        amp_v: f64,
        freq_hz: f64,
        branch_idx: usize,
    },
    StepPulseVoltage {
        id: String,
        np: usize,
        nn: usize,
        v_initial: f64,
        v_pulsed: f64,
        delay_s: f64,
        rise_s: f64,
        branch_idx: usize,
    },
    CurrentSource {
        id: String,
        np: usize,
        nn: usize,
        dc_a: f64,
        ac_mag_a: f64,
    },
    Diode {
        id: String,
        na: usize,
        nc: usize,
        model: ShockleyDiode,
    },
    Mosfet {
        id: String,
        nd: usize,
        ng: usize,
        ns: usize,
        nb: usize,
        model: ContinuousMosfet,
    },
    PwmSwitch {
        id: String,
        np: usize,
        nn: usize,
        r_on: f64,
        r_off: f64,
        period_s: f64,
        duty: f64,
        high_side: bool,
    },
}

#[derive(Debug, Clone)]
pub struct AssembledMnaState {
    pub f_static: Vec<f64>,
    pub q_dynamic: Vec<f64>,
    pub g_jacobian: Vec<Vec<f64>>,
    pub c_jacobian: Vec<Vec<f64>>,
}
