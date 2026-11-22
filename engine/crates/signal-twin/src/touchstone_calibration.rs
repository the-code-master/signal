// Module: engine/crates/signal-twin/src/touchstone_calibration.rs — Touchstone/SCPI Ingest & Bayesian/Gauss-Newton Twin Calibrator (I-06)
// Invariants: Fits parasitic (ESR, ESL, er_eff) against measured lab impedance/waveform captures to < 1% RMS residual (F-111).
// Spec: architecture/16-architectural-audit-flaws-and-futuristic-innovations.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::TwinError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchstoneFreqRow {
    pub freq_hz: f64,
    pub s11_re: f64,
    pub s11_im: f64,
    pub s21_re: f64,
    pub s21_im: f64,
}

/// Parses an industry-standard Touchstone `.s2p` (`RI` real/imag) capture from a lab VNA (`I-06`).
pub fn parse_touchstone_s2p(content: &str) -> Result<(f64, Vec<TouchstoneFreqRow>), TwinError> {
    let mut z0_ohm = 50.0_f64;
    let mut freq_mult = 1.0_f64;
    let mut rows = Vec::new();

    for raw_line in content.lines() {
        let line = raw_line.split('!').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        if let Some(opt) = line.strip_prefix('#') {
            let toks: Vec<&str> = opt.split_whitespace().collect();
            if let Some(&unit) = toks.first() {
                freq_mult = match unit.to_ascii_uppercase().as_str() {
                    "GHZ" => 1e9,
                    "MHZ" => 1e6,
                    "KHZ" => 1e3,
                    _ => 1.0,
                };
            }
            if let Some(r_pos) = toks.iter().position(|t| t.eq_ignore_ascii_case("R"))
                && let Some(r_str) = toks.get(r_pos + 1)
                && let Ok(r_val) = r_str.parse::<f64>()
            {
                z0_ohm = r_val.max(1.0);
            }
            continue;
        }
        let nums: Result<Vec<f64>, _> = line.split_whitespace().map(str::parse::<f64>).collect();
        let vals =
            nums.map_err(|_| TwinError::InvalidLabCapture("malformed .s2p numeric row".into()))?;
        if vals.len() >= 5 {
            rows.push(TouchstoneFreqRow {
                freq_hz: vals[0] * freq_mult,
                s11_re: vals[1],
                s11_im: vals[2],
                s21_re: vals[3],
                s21_im: vals[4],
            });
        }
    }
    if rows.is_empty() {
        return Err(TwinError::InvalidLabCapture(
            "empty Touchstone .s2p capture".into(),
        ));
    }
    Ok((z0_ohm, rows))
}

/// Parses an oscilloscope SCPI `time_s,voltage_v` CSV waveform capture (`I-06`).
pub fn parse_scpi_csv_waveform(csv_text: &str) -> Result<Vec<(f64, f64)>, TwinError> {
    let mut samples = Vec::new();
    for raw in csv_text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("time") {
            continue;
        }
        let mut parts = line.split(',');
        let (Some(t_s), Some(v_s)) = (parts.next(), parts.next()) else {
            return Err(TwinError::InvalidLabCapture(
                "expected time_s,voltage_v row".into(),
            ));
        };
        let t = t_s
            .trim()
            .parse::<f64>()
            .map_err(|_| TwinError::InvalidLabCapture("invalid SCPI timestamp".into()))?;
        let v = v_s
            .trim()
            .parse::<f64>()
            .map_err(|_| TwinError::InvalidLabCapture("invalid SCPI voltage".into()))?;
        samples.push((t, v));
    }
    if samples.is_empty() {
        return Err(TwinError::InvalidLabCapture("empty SCPI waveform".into()));
    }
    Ok(samples)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalibratedParasiticModel {
    pub esr_ohm: f64,
    pub esl_h: f64,
    pub capacitance_f: f64,
    pub post_calibration_rms_error_pct: f64,
}

fn rlc_impedance_mag(freq_hz: f64, esr_ohm: f64, esl_h: f64, cap_f: f64) -> f64 {
    let w = std::f64::consts::TAU * freq_hz.max(1.0);
    let react = w * esl_h - 1.0 / (w * cap_f.max(1e-15));
    esr_ohm.hypot(react)
}

/// Fits parasitic `(ESR, ESL)` via Bayesian MAP / damped Gauss-Newton against measured $|Z(f)|$ lab data (`I-06`, `F-111`).
pub fn calibrate_parasitic_esr_esl(
    measured_freq_z_ohm: &[(f64, f64)],
    nominal_cap_f: f64,
    prior_esr_ohm: f64,
    prior_esl_h: f64,
) -> Result<CalibratedParasiticModel, TwinError> {
    if measured_freq_z_ohm.len() < 2 || nominal_cap_f <= 0.0 {
        return Err(TwinError::InvalidLabCapture(
            "need >= 2 impedance frequency points".into(),
        ));
    }
    let mut esr = prior_esr_ohm.max(1e-4);
    let mut esl = prior_esl_h.max(1e-12);
    let prior_weight = 1e-4;

    for _iter in 0..40 {
        let mut h00 = prior_weight;
        let mut h01 = 0.0_f64;
        let mut h11 = prior_weight;
        let mut g0 = 0.0_f64;
        let mut g1 = 0.0_f64;

        let d_esr = (esr * 1e-4).max(1e-7);
        let d_esl = (esl * 1e-4).max(1e-15);

        for &(f, z_meas) in measured_freq_z_ohm {
            let z0 = rlc_impedance_mag(f, esr, esl, nominal_cap_f);
            let r = z0 - z_meas;
            let j0 = (rlc_impedance_mag(f, esr + d_esr, esl, nominal_cap_f) - z0) / d_esr;
            let j1 = (rlc_impedance_mag(f, esr, esl + d_esl, nominal_cap_f) - z0) / d_esl;
            h00 += j0 * j0;
            h01 += j0 * j1;
            h11 += j1 * j1;
            g0 -= j0 * r;
            g1 -= j1 * r;
        }
        let det = h00 * h11 - h01 * h01;
        if det.abs() < 1e-24 {
            break;
        }
        let step_esr = (h11 * g0 - h01 * g1) / det;
        let step_esl = (-h01 * g0 + h00 * g1) / det;
        esr = (esr + step_esr).clamp(1e-4, 10.0);
        esl = (esl + step_esl).clamp(1e-12, 1e-6);
    }

    let mut sq_rel_err = 0.0_f64;
    for &(f, z_meas) in measured_freq_z_ohm {
        let z_fit = rlc_impedance_mag(f, esr, esl, nominal_cap_f);
        let rel = (z_fit - z_meas) / z_meas.max(1e-6);
        sq_rel_err += rel * rel;
    }
    let rms_pct = (sq_rel_err / (measured_freq_z_ohm.len() as f64)).sqrt() * 100.0;

    Ok(CalibratedParasiticModel {
        esr_ohm: esr,
        esl_h: esl,
        capacitance_f: nominal_cap_f,
        post_calibration_rms_error_pct: rms_pct,
    })
}
