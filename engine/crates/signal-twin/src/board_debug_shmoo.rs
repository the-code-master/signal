// Module: engine/crates/signal-twin/src/board_debug_shmoo.rs — Factory Bring-Up Board Map (FEAT-070) & PVT Shmoo Mapper (FEAT-088)
// Invariants: Evaluates alpha-power critical-path PVT Shmoo grids in exact uV/Hz/mK & checks technician testpoints.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::TwinError;
use crate::touchstone_calibration::calibrate_parasitic_esr_esl;
use signal_core::{Microvolts, Milliohms};
use signal_proto::v1::{
    CalibrateTwinAndRunShmooRequest, CalibrateTwinAndRunShmooResponse, ShmooPointResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestpointBringupSpec {
    pub testpoint_id: String,
    pub net_name: String,
    pub nominal_dc: Microvolts,
    pub tolerance: Microvolts,
    pub min_impedance_to_gnd: Milliohms,
    pub expected_clock_hz: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchProbeDiagnostic {
    pub within_spec: bool,
    pub likely_short_to_gnd: bool,
    pub voltage_error_uv: i64,
}

/// Evaluates a technician's live multimeter/oscilloscope bench probe against design nominal expectations (`FEAT-070`).
pub fn diagnose_factory_testpoint_probe(
    spec: &TestpointBringupSpec,
    measured_dc_uv: i64,
    measured_z_gnd_mohm: i64,
) -> BenchProbeDiagnostic {
    let v_err = measured_dc_uv.saturating_sub(spec.nominal_dc.0);
    let short_to_gnd = measured_z_gnd_mohm < spec.min_impedance_to_gnd.0;
    let v_ok = v_err.abs() <= spec.tolerance.0;

    BenchProbeDiagnostic {
        within_spec: v_ok && !short_to_gnd,
        likely_short_to_gnd: short_to_gnd,
        voltage_error_uv: v_err,
    }
}

/// Evaluates a 2D/3D $(V_{DD}, f_{\text{clk}}, T)$ PVT Shmoo grid using the Sakurai alpha-power CMOS delay law (`FEAT-088`).
pub fn evaluate_pvt_shmoo_grid(
    voltages_uv: &[i64],
    clock_freqs_hz: &[i64],
    temps_mkelvin: &[i64],
    vth_nominal_uv: i64,
    fmax_at_1v_300k_hz: f64,
) -> Vec<ShmooPointResult> {
    let mut grid =
        Vec::with_capacity(voltages_uv.len() * clock_freqs_hz.len() * temps_mkelvin.len());
    for &temp_mk in temps_mkelvin {
        let temp_k = (temp_mk as f64) * 1e-3;
        // High temperature degrades carrier mobility (~ (300/T)^1.3) while slightly lowering V_th
        let mobility_factor = (300.0 / temp_k.max(20.0)).powf(0.35).clamp(0.55, 1.45);
        let vth_v = ((vth_nominal_uv as f64) * 1e-6 - 0.0007 * (temp_k - 300.0)).clamp(0.15, 0.55);

        for &vdd_uv in voltages_uv {
            let vdd_v = (vdd_uv as f64) * 1e-6;
            let overdrive = (vdd_v - vth_v).max(0.0);
            let ref_overdrive = (1.0 - 0.30_f64).max(0.05);
            // Sakurai alpha-power f_max(V_DD, T) = f_ref * mobility * ((V_DD - V_th)^1.3 / V_DD) / ((1.0 - 0.3)^1.3 / 1.0)
            let fmax_hz = if overdrive <= 0.02 {
                0.0
            } else {
                fmax_at_1v_300k_hz * mobility_factor * (overdrive.powf(1.3) / vdd_v.max(0.1))
                    / ref_overdrive.powf(1.3)
            };

            for &f_hz in clock_freqs_hz {
                grid.push(ShmooPointResult {
                    supply_voltage_uv: vdd_uv,
                    clock_freq_hz: f_hz,
                    temp_mkelvin: temp_mk,
                    passed: (f_hz as f64) <= fmax_hz && vdd_uv <= 1_250_000,
                });
            }
        }
    }
    grid
}

/// Top-level RPC entrypoint calibrating the Lab Digital Twin (`I-06`) and sweeping the PVT Shmoo plot (`FEAT-088`).
pub fn calibrate_twin_and_run_shmoo(
    req: &CalibrateTwinAndRunShmooRequest,
) -> Result<CalibrateTwinAndRunShmooResponse, TwinError> {
    let v_steps = req.voltage_steps.clamp(2, 32) as usize;
    let f_steps = req.freq_steps.clamp(2, 32) as usize;

    // Reference VNA decap sweep at 1 MHz, 15 MHz, and 100 MHz around true (ESR=12 mOhm, ESL=420 pH, C=1 uF)
    let synth_lab = [
        (1.0e6_f64, 0.159_50_f64),
        (7.76e6_f64, 0.012_05_f64),
        (50.0e6_f64, 0.129_30_f64),
        (100.0e6_f64, 0.262_58_f64),
    ];
    let cal = calibrate_parasitic_esr_esl(&synth_lab, 1.0e-6, 0.030, 800e-12)?;

    let voltages: Vec<i64> = (0..v_steps)
        .map(|i| 650_000 + ((i as i64) * 450_000) / ((v_steps - 1).max(1) as i64))
        .collect();
    let freqs: Vec<i64> = (0..f_steps)
        .map(|j| 400_000_000 + ((j as i64) * 1_600_000_000) / ((f_steps - 1).max(1) as i64))
        .collect();
    let points = evaluate_pvt_shmoo_grid(&voltages, &freqs, &[300_000], 300_000, 1.8e9);
    let pass_cnt = points.iter().filter(|p| p.passed).count();
    let pass_frac = (pass_cnt as f64) / (points.len().max(1) as f64);

    Ok(CalibrateTwinAndRunShmooResponse {
        post_calibration_rms_error_pct: cal.post_calibration_rms_error_pct,
        pass_region_fraction: pass_frac,
        shmoo_points: points,
    })
}
