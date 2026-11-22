// Module: engine/crates/signal-sim/src/instruments.rs — Interactive Slider, Branch Telemetry, Blackman-Harris FFT/Smith & Decimator
// Invariants: <= 4096-px min-max glitch-preserving envelope (F-069); Blackman-Harris FFT THD/SINAD/ENOB & 2-port Smith (FEAT-003..082).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::SimError;
use crate::circuit::{CircuitElement, MnaCircuit};
use crate::equilibrate_solver::Complex64;
use crate::mna_engine::solve_dc_op;
use signal_core::{det_cos, det_ln, det_sin};
use signal_proto::v1::BranchCurrentTelemetry;
use std::collections::BTreeMap;

/// `FEAT-003`: Warm-state interactive live slider update reusing prior DC operating point `warm_state`.
pub fn interactive_slider_step(
    ckt: &mut MnaCircuit,
    entity_id: &str,
    new_value_si: f64,
    warm_state: &[f64],
    temp_mk: i64,
) -> Result<Vec<f64>, SimError> {
    for el in &mut ckt.elements {
        match el {
            CircuitElement::Resistor { id, model, .. } if id == entity_id => {
                model.nominal_ohms = new_value_si.max(1.0e-6);
            }
            CircuitElement::LinearCapacitor { id, c_farads, .. } if id == entity_id => {
                *c_farads = new_value_si.max(0.0);
            }
            CircuitElement::DcAcVoltage { id, dc_v, .. } if id == entity_id => {
                *dc_v = new_value_si;
            }
            _ => {}
        }
    }
    solve_dc_op(ckt, Some(warm_state), temp_mk)
}

/// `FEAT-004`: Computes exact trapezoidal RMS current, peak current, and average power (`BranchCurrentTelemetry`).
pub fn compute_branch_telemetry(
    entity_id: &str,
    times_s: &[f64],
    voltages_v: &[f64],
    currents_a: &[f64],
) -> BranchCurrentTelemetry {
    let n = times_s.len().min(voltages_v.len()).min(currents_a.len());
    if n == 0 {
        return BranchCurrentTelemetry {
            entity_id: entity_id.to_string(),
            ..Default::default()
        };
    }
    let mut peak_i = 0.0_f64;
    for &i in &currents_a[..n] {
        peak_i = peak_i.max(i.abs());
    }
    if n == 1 || times_s[n - 1] <= times_s[0] {
        let p = voltages_v[0] * currents_a[0];
        return BranchCurrentTelemetry {
            entity_id: entity_id.to_string(),
            rms_current_na: (currents_a[0].abs() * 1.0e9).round() as i64,
            peak_current_na: (peak_i * 1.0e9).round() as i64,
            avg_power_uw: (p * 1.0e6).round() as i64,
        };
    }

    let mut i2_int = 0.0;
    let mut p_int = 0.0;
    for k in 0..(n - 1) {
        let dt = (times_s[k + 1] - times_s[k]).max(0.0);
        i2_int +=
            0.5 * (currents_a[k] * currents_a[k] + currents_a[k + 1] * currents_a[k + 1]) * dt;
        p_int += 0.5 * (voltages_v[k] * currents_a[k] + voltages_v[k + 1] * currents_a[k + 1]) * dt;
    }
    let span = times_s[n - 1] - times_s[0];
    let rms_i = (i2_int / span).sqrt();
    let avg_p = p_int / span;

    BranchCurrentTelemetry {
        entity_id: entity_id.to_string(),
        rms_current_na: (rms_i * 1.0e9).round() as i64,
        peak_current_na: (peak_i * 1.0e9).round() as i64,
        avg_power_uw: (avg_p * 1.0e6).round() as i64,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FftSpectrumMetrics {
    pub magnitude_db: Vec<f64>,
    pub thd_db: f64,
    pub sinad_db: f64,
    pub enob_bits: f64,
}

/// Radix-2 Cooley-Tukey FFT with 4-term **Blackman-Harris** window, THD, SINAD & ENOB (`FEAT-081`).
pub fn analyze_fft_spectrum(samples: &[f64]) -> Result<FftSpectrumMetrics, SimError> {
    let n = samples.len();
    if n < 8 || !n.is_power_of_two() {
        return Err(SimError::InvalidParameter(
            "FFT sample length must be power of two >= 8",
        ));
    }
    let mut buf = vec![Complex64::ZERO; n];
    for (k, &s) in samples.iter().enumerate() {
        let ph = std::f64::consts::TAU * (k as f64) / ((n - 1) as f64);
        // 4-term Nuttall / Blackman-Harris minimum sidelobe (-92 dB) window
        let w = 0.35875 - 0.48829 * det_cos(ph) + 0.14128 * det_cos(2.0 * ph)
            - 0.01168 * det_cos(3.0 * ph);
        buf[k] = Complex64::new(s * w, 0.0);
    }

    // Bit-reversal permutation
    let bits = n.trailing_zeros();
    for i in 0..n {
        let j = i.reverse_bits() >> (usize::BITS - bits);
        if i < j {
            buf.swap(i, j);
        }
    }

    // Cooley-Tukey butterflies
    let mut len = 2_usize;
    while len <= n {
        let half = len / 2;
        let step_ang = -std::f64::consts::TAU / (len as f64);
        for base in (0..n).step_by(len) {
            for k in 0..half {
                let ang = step_ang * (k as f64);
                let tw = Complex64::new(det_cos(ang), det_sin(ang));
                let u = buf[base + k];
                let v = tw * buf[base + k + half];
                buf[base + k] = u + v;
                buf[base + k + half] = u - v;
            }
        }
        len *= 2;
    }

    let half_n = n / 2;
    let power: Vec<f64> = buf[..half_n]
        .iter()
        .map(|c| c.abs_sq() / ((n as f64) * (n as f64)))
        .collect();

    // Locate fundamental peak bin (excluding DC lobes 0..=3)
    let mut fund_bin = 4.min(half_n - 1);
    for b in 4..half_n {
        if power[b] > power[fund_bin] {
            fund_bin = b;
        }
    }

    // Sum mainlobe around fundamental (fund_bin - 3 ..= fund_bin + 3)
    let lo_f = fund_bin.saturating_sub(3).max(1);
    let hi_f = (fund_bin + 3).min(half_n - 1);
    let mut sig_p = 0.0;
    for &p in &power[lo_f..=hi_f] {
        sig_p += p;
    }

    let mut nad_p = 0.0;
    for (b, &p) in power.iter().enumerate().skip(4) {
        if !(lo_f..=hi_f).contains(&b) {
            nad_p += p;
        }
    }

    let mut harm_p = 0.0;
    for h in 2..=7 {
        let hb = h * fund_bin;
        if hb < half_n {
            let h_lo = hb.saturating_sub(2);
            let h_hi = (hb + 2).min(half_n - 1);
            for &p in &power[h_lo..=h_hi] {
                harm_p += p;
            }
        }
    }

    let db10 = |ratio: f64| 10.0 * det_ln(ratio.max(1.0e-30)) / std::f64::consts::LN_10;
    let sinad_db = db10(sig_p / nad_p.max(1.0e-30));
    let thd_db = db10(harm_p / sig_p.max(1.0e-30));
    let enob_bits = (sinad_db - 1.76) / 6.02;
    let mag_db = power.iter().map(|&p| db10(p)).collect();

    Ok(FftSpectrumMetrics {
        magnitude_db: mag_db,
        thd_db,
        sinad_db,
        enob_bits,
    })
}

/// Converts 2-port S-parameters ($Z_0$-referenced) to $2 \times 2$ Z-matrix (`FEAT-081` VNA Smith Chart).
pub fn s2p_to_z_matrix(s: [[Complex64; 2]; 2], z0_ohms: f64) -> [[Complex64; 2]; 2] {
    let one = Complex64::new(1.0, 0.0);
    let z0 = Complex64::new(z0_ohms, 0.0);
    // Z = Z0 * (I + S) * (I - S)^(-1)
    let a00 = one - s[0][0];
    let a01 = Complex64::ZERO - s[0][1];
    let a10 = Complex64::ZERO - s[1][0];
    let a11 = one - s[1][1];
    let det = a00 * a11 - a01 * a10;

    let inv00 = a11 / det;
    let inv01 = (Complex64::ZERO - a01) / det;
    let inv10 = (Complex64::ZERO - a10) / det;
    let inv11 = a00 / det;

    let p00 = one + s[0][0];
    let p01 = s[0][1];
    let p10 = s[1][0];
    let p11 = one + s[1][1];

    [
        [
            z0 * (p00 * inv00 + p01 * inv10),
            z0 * (p00 * inv01 + p01 * inv11),
        ],
        [
            z0 * (p10 * inv00 + p11 * inv10),
            z0 * (p10 * inv01 + p11 * inv11),
        ],
    ]
}

/// `FEAT-082`: Evaluates waveform trace math (`v(out)*i(L1)`, `integrate(...)`, `rms(...)`).
pub fn eval_wave_expr(
    expr: &str,
    times_s: &[f64],
    traces: &BTreeMap<String, Vec<f64>>,
) -> Result<Vec<f64>, SimError> {
    let s = expr.trim();
    if let Some(inner) = s
        .strip_prefix("integrate(")
        .and_then(|r| r.strip_suffix(')'))
    {
        let y = eval_wave_expr(inner, times_s, traces)?;
        let mut out = vec![0.0; y.len()];
        for k in 1..y.len().min(times_s.len()) {
            let dt = (times_s[k] - times_s[k - 1]).max(0.0);
            out[k] = out[k - 1] + 0.5 * (y[k - 1] + y[k]) * dt;
        }
        return Ok(out);
    }
    if let Some(inner) = s.strip_prefix("rms(").and_then(|r| r.strip_suffix(')')) {
        let y = eval_wave_expr(inner, times_s, traces)?;
        let mean_sq = y.iter().map(|v| v * v).sum::<f64>() / (y.len().max(1) as f64);
        return Ok(vec![mean_sq.sqrt(); y.len()]);
    }
    if let Some((a, b)) = s.split_once('*') {
        let va = eval_wave_expr(a, times_s, traces)?;
        let vb = eval_wave_expr(b, times_s, traces)?;
        return Ok(va.iter().zip(&vb).map(|(x, y)| x * y).collect());
    }
    if let Some((a, b)) = s.split_once('-') {
        let va = eval_wave_expr(a, times_s, traces)?;
        let vb = eval_wave_expr(b, times_s, traces)?;
        return Ok(va.iter().zip(&vb).map(|(x, y)| x - y).collect());
    }
    traces.get(s).cloned().ok_or(SimError::InvalidParameter(
        "unknown waveform trace in wave_expr",
    ))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvelopeBucket {
    pub t_start: f64,
    pub t_end: f64,
    pub min_val: f64,
    pub max_val: f64,
}

/// `F-069`: `<= 4,096`-pixel min-max envelope waveform decimator preserving sub-pixel transient spikes.
pub fn decimate_minmax_envelope(
    times_s: &[f64],
    values: &[f64],
    max_pixels: usize,
) -> Vec<EnvelopeBucket> {
    let n = times_s.len().min(values.len());
    let cap = max_pixels.clamp(1, 4096);
    if n == 0 {
        return Vec::new();
    }
    let buckets = cap.min(n);
    let mut out = Vec::with_capacity(buckets);
    for b in 0..buckets {
        let start = (b * n) / buckets;
        let end = (((b + 1) * n) / buckets).max(start + 1).min(n);
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for &v in &values[start..end] {
            lo = lo.min(v);
            hi = hi.max(v);
        }
        out.push(EnvelopeBucket {
            t_start: times_s[start],
            t_end: times_s[end - 1],
            min_val: lo,
            max_val: hi,
        });
    }
    out
}
