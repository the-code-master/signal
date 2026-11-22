// Module: engine/crates/signal-twin/src/lib.rs — Lab Bench Digital Twin (I-06), Factory Board-Debug & PVT Shmoo Suite
// Invariants: Calibrates parasitic ESR/ESL to Touchstone/SCPI lab data; maps exact SI PVT Shmoo pass boundaries.
// Spec: architecture/16-architectural-audit-flaws-and-futuristic-innovations.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod board_debug_shmoo;
pub mod touchstone_calibration;

pub use board_debug_shmoo::{
    BenchProbeDiagnostic, TestpointBringupSpec, calibrate_twin_and_run_shmoo,
    diagnose_factory_testpoint_probe, evaluate_pvt_shmoo_grid,
};
pub use touchstone_calibration::{
    CalibratedParasiticModel, TouchstoneFreqRow, calibrate_parasitic_esr_esl,
    parse_scpi_csv_waveform, parse_touchstone_s2p,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TwinError {
    #[error("invalid lab bench instrument capture: {0}")]
    InvalidLabCapture(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::{Microvolts, Milliohms};
    use signal_proto::v1::{CalibrateTwinAndRunShmooRequest, ShmooPointResult};

    #[test]
    fn parses_touchstone_scpi_calibrates_esr_esl_diagnoses_testpoint_and_runs_shmoo() {
        let s2p = "! Keysight VNA capture\n# GHz S RI R 50\n1.0 -0.05 0.02 0.92 -0.18 0.92 -0.18 -0.05 0.02\n2.5 -0.08 0.04 0.85 -0.31 0.85 -0.31 -0.08 0.04\n";
        let (z0, rows) = parse_touchstone_s2p(s2p).expect("s2p parsed");
        assert_eq!(z0, 50.0);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].freq_hz, 1.0e9);

        let scpi = "# Tektronix MSO64\ntime_s,voltage_v\n0.0e-9,0.01\n1.0e-9,1.82\n";
        let wf = parse_scpi_csv_waveform(scpi).expect("scpi parsed");
        assert_eq!(wf.len(), 2);

        let tp = TestpointBringupSpec {
            testpoint_id: "TP14".into(),
            net_name: "VDD_CORE_0V8".into(),
            nominal_dc: Microvolts(800_000),
            tolerance: Microvolts(24_000),
            min_impedance_to_gnd: Milliohms(5_000),
            expected_clock_hz: 0,
        };
        assert!(diagnose_factory_testpoint_probe(&tp, 805_000, 120_000).within_spec);
        assert!(diagnose_factory_testpoint_probe(&tp, 12_000, 450).likely_short_to_gnd);

        let shmoo = calibrate_twin_and_run_shmoo(&CalibrateTwinAndRunShmooRequest {
            fencing: None,
            instrument_capture_id: "CAP-VNA-001".into(),
            voltage_steps: 6,
            freq_steps: 6,
        })
        .expect("twin + shmoo");
        assert!(shmoo.post_calibration_rms_error_pct < 2.0);
        assert_eq!(shmoo.shmoo_points.len(), 36);
        assert!(shmoo.pass_region_fraction > 0.2 && shmoo.pass_region_fraction < 0.95);

        // Verify 64-bit string serde roundtrip on golden ShmooPointResult fixture!
        let raw_fixture =
            include_str!("../../../../contracts/fixtures/cosim_twin_shmoo.fixture.json");
        let root: serde_json::Value = serde_json::from_str(raw_fixture).expect("fixture json");
        let pt: ShmooPointResult =
            serde_json::from_value(root["shmoo_point"].clone()).expect("shmoo deser");
        assert_eq!(pt.clock_freq_hz, 9_007_199_254_740_997_i64);
        assert_eq!(pt.temp_mkelvin, 4_200);
    }
}
