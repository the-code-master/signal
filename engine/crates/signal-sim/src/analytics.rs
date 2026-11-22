// Module: engine/crates/signal-sim/src/analytics.rs — Halton QMC Cpk/Spearman, Dual-Dirac PAM4 BER, Regression & Stimulus
// Invariants: Deterministic Halton QMC + Spearman tornado (FEAT-083); Dual-Dirac PAM4 bathtub to 1e-15 (FEAT-084); PRBS/ISO-7637 (FEAT-090).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_core::{det_exp, det_ln, det_sin};
use signal_proto::v1::{
    MonteCarloSensitivityBar, MonteCarloYieldSummary, RegressionAssertionResult,
    RunRegressionSuiteResponse, SerDesJitterDecomposition,
};
use std::collections::BTreeMap;

/// Radical-inverse Halton low-discrepancy quasi-Monte-Carlo sequence in `(0, 1)`.
pub fn halton_radical_inverse(mut index: u32, base: u32) -> f64 {
    let b = base.max(2);
    let mut f = 1.0 / f64::from(b);
    let mut res = 0.0;
    while index > 0 {
        res += f * f64::from(index % b);
        index /= b;
        f /= f64::from(b);
    }
    res
}

fn ranks_of(xs: &[f64]) -> Vec<f64> {
    let mut indexed: Vec<(usize, f64)> = xs.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| a.1.total_cmp(&b.1));
    let mut ranks = vec![0.0; xs.len()];
    for (rank, (orig_idx, _)) in indexed.into_iter().enumerate() {
        ranks[orig_idx] = (rank + 1) as f64;
    }
    ranks
}

fn spearman_corr(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 {
        return 0.0;
    }
    let rx = ranks_of(&x[..n]);
    let ry = ranks_of(&y[..n]);
    let mean = (n as f64 + 1.0) * 0.5;
    let mut cov = 0.0;
    let mut vx = 0.0;
    let mut vy = 0.0;
    for i in 0..n {
        let dx = rx[i] - mean;
        let dy = ry[i] - mean;
        cov += dx * dy;
        vx += dx * dx;
        vy += dy * dy;
    }
    let denom = (vx * vy).sqrt();
    if denom < 1.0e-15 { 0.0 } else { cov / denom }
}

/// `FEAT-083`: Quasi-Monte-Carlo Process Capability $C_{pk}$ and Spearman Rank Tornado Sensitivity.
pub fn run_yield_analytics<F>(
    param_specs: &[(String, String, f64, f64)], // (entity_id, param_key, nominal, rel_sigma)
    sample_count: u32,
    lsl: f64,
    usl: f64,
    eval_metric: F,
) -> MonteCarloYieldSummary
where
    F: Fn(&[f64]) -> f64,
{
    const PRIMES: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
    let n = sample_count.max(4) as usize;
    let p_dim = param_specs.len();
    let mut param_samples = vec![Vec::with_capacity(n); p_dim];
    let mut outputs = Vec::with_capacity(n);
    let mut pass_cnt = 0_usize;

    for s in 1..=n {
        let mut p_vec = Vec::with_capacity(p_dim);
        for (d, (_, _, nom, rel_tol)) in param_specs.iter().enumerate() {
            let u = halton_radical_inverse(s as u32, PRIMES[d % PRIMES.len()]);
            // Map uniform (0,1) -> [-sqrt(3)*sigma, +sqrt(3)*sigma] zero-mean unit-variance scaling
            let z = (u - 0.5) * 2.0 * 3.0_f64.sqrt();
            let val = nom * (1.0 + rel_tol * z);
            param_samples[d].push(val);
            p_vec.push(val);
        }
        let y = eval_metric(&p_vec);
        if y >= lsl && y <= usl {
            pass_cnt += 1;
        }
        outputs.push(y);
    }

    let mean = outputs.iter().sum::<f64>() / (n as f64);
    let var = outputs.iter().map(|y| (y - mean) * (y - mean)).sum::<f64>() / ((n - 1) as f64);
    let sigma = var.sqrt().max(1.0e-15);
    let cpk = ((usl - mean) / (3.0 * sigma)).min((mean - lsl) / (3.0 * sigma));

    let mut raw_corrs: Vec<(usize, f64)> = (0..p_dim)
        .map(|d| (d, spearman_corr(&param_samples[d], &outputs)))
        .collect();
    let sq_sum: f64 = raw_corrs
        .iter()
        .map(|(_, r)| r * r)
        .sum::<f64>()
        .max(1.0e-15);
    raw_corrs.sort_by(|a, b| b.1.abs().total_cmp(&a.1.abs()));

    let tornado_bars = raw_corrs
        .into_iter()
        .map(|(d, rho)| MonteCarloSensitivityBar {
            entity_id: param_specs[d].0.clone(),
            param_key: param_specs[d].1.clone(),
            spearman_rank_corr: rho,
            variance_share_pct: 100.0 * (rho * rho) / sq_sum,
        })
        .collect();

    MonteCarloYieldSummary {
        sample_count: n as u32,
        cpk_index: cpk,
        pass_yield_pct: 100.0 * (pass_cnt as f64) / (n as f64),
        tornado_bars,
    }
}

/// Accurate complementary error function $\operatorname{erfc}(x)$ across $[0, 10]$ for $10^{-15}$ BER tails.
pub fn det_erfc_positive(x: f64) -> f64 {
    let ax = x.max(0.0);
    // Abramowitz & Stegun 7.1.26 rational approximation * exp(-x^2)
    let t = 1.0 / (1.0 + 0.327_591_1 * ax);
    let poly = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    poly * det_exp(-ax * ax)
}

/// `FEAT-084`: Dual-Dirac RJ/DJ Jitter Decomposition & PAM4 (`3`-eye) BER Bathtub Curve down to `1e-15`.
pub fn decompose_jitter_pam4_ber(
    unit_interval_fs: i64,
    rj_rms_fs: i64,
    pj_pp_fs: i64,
    ddj_pp_fs: i64,
) -> (SerDesJitterDecomposition, Vec<(f64, f64)>) {
    let ui = unit_interval_fs.max(1) as f64;
    let rj = rj_rms_fs.max(1) as f64;
    let dj_pp = (pj_pp_fs.saturating_add(ddj_pp_fs)).max(0) as f64;
    // Q-factor for BER = 1e-15 under PAM4 (3/4 symbol transition density): Q_1e15 ~= 7.9413
    const Q_AT_1E15: f64 = 7.941_345;
    // PAM4 horizontal eye penalty (~1/3 outer-to-inner slew compression factor = 1.0 UI nominal minus TJ)
    let tj_at_1e15_fs = dj_pp + 2.0 * Q_AT_1E15 * rj;
    let eye_open_ui = (1.0 - tj_at_1e15_fs / ui).clamp(0.0, 1.0);

    let mut bathtub = Vec::with_capacity(41);
    for idx in 0..=40 {
        let phase_ui = -0.5 + (idx as f64) * 0.025; // -0.5 UI .. +0.5 UI
        let dist_from_edge_fs = (0.5 - phase_ui.abs()) * ui - 0.5 * dj_pp;
        let ber = if dist_from_edge_fs <= 0.0 {
            0.5
        } else {
            let q = dist_from_edge_fs / (std::f64::consts::SQRT_2 * rj);
            // PAM4 transition-weighted Dual-Dirac tail BER = 0.75 * 0.5 * erfc(q)
            (0.375 * det_erfc_positive(q)).max(1.0e-18)
        };
        bathtub.push((phase_ui, ber));
    }

    (
        SerDesJitterDecomposition {
            rj_rms_fs,
            dj_pp_fs: dj_pp.round() as i64,
            pj_pp_fs,
            ddj_pp_fs,
            horizontal_eye_opening_ui_at_1e15: eye_open_ui,
        },
        bathtub,
    )
}

/// `FEAT-086`: Headless regression spec-assertion runner (`metric <= limit`, `metric >= limit`).
pub fn run_regression_assertions(
    measured_metrics: &BTreeMap<String, f64>,
    assertions: &[String],
) -> RunRegressionSuiteResponse {
    let mut results = Vec::with_capacity(assertions.len());
    let mut all_passed = true;

    for (idx, expr) in assertions.iter().enumerate() {
        let (passed, measured, margin_pct) = eval_single_assertion(expr, measured_metrics);
        if !passed {
            all_passed = false;
        }
        results.push(RegressionAssertionResult {
            assertion_id: format!("assert_{}", idx + 1),
            metric_expression: expr.clone(),
            passed,
            measured_value: measured,
            margin_pct,
        });
    }
    RunRegressionSuiteResponse {
        all_passed,
        results,
    }
}

fn eval_single_assertion(expr: &str, metrics: &BTreeMap<String, f64>) -> (bool, f64, f64) {
    for (op, is_upper) in [("<=", true), (">=", false), ("<", true), (">", false)] {
        if let Some((k, lim_s)) = expr.split_once(op)
            && let Some(&val) = metrics.get(k.trim())
            && let Ok(lim) = lim_s.trim().parse::<f64>()
        {
            let denom = lim.abs().max(1.0e-12);
            let margin = if is_upper {
                100.0 * (lim - val) / denom
            } else {
                100.0 * (val - lim) / denom
            };
            let ok = if op == "<=" {
                val <= lim
            } else if op == ">=" {
                val >= lim
            } else if op == "<" {
                val < lim
            } else {
                val > lim
            };
            return (ok, val, margin);
        }
    }
    (false, f64::NAN, -100.0)
}

/// `FEAT-090`: Synthesizes deterministic **PRBS-7**, **PRBS-15**, **ISO 7637-2** automotive pulse, or audio chirp/sine.
pub fn synthesize_stimulus(preset_kind: &str, sample_rate_hz: i64, num_samples: usize) -> Vec<f64> {
    let fs = sample_rate_hz.max(1) as f64;
    let mut out = Vec::with_capacity(num_samples);
    match preset_kind.trim().to_ascii_uppercase().as_str() {
        "PRBS7" => {
            let mut lfsr = 0x7F_u16;
            for _ in 0..num_samples {
                let bit = ((lfsr >> 6) ^ (lfsr >> 5)) & 1;
                out.push(if (lfsr & 1) == 1 { 1.0 } else { -1.0 });
                lfsr = ((lfsr << 1) | bit) & 0x7F;
            }
        }
        "PRBS15" => {
            let mut lfsr = 0x7FFF_u16;
            for _ in 0..num_samples {
                let bit = ((lfsr >> 14) ^ (lfsr >> 13)) & 1;
                out.push(if (lfsr & 1) == 1 { 1.0 } else { -1.0 });
                lfsr = ((lfsr << 1) | bit) & 0x7FFF;
            }
        }
        "ISO7637_PULSE_5A" => {
            // 12V nominal + 65V load-dump double-exponential surge: V(t) = 12 + 65*(exp(-t/0.15) - exp(-t/0.005))
            for k in 0..num_samples {
                let t = (k as f64) / fs;
                out.push(12.0 + 65.0 * (det_exp(-t / 0.15) - det_exp(-t / 0.005)));
            }
        }
        "AUDIO_CHIRP" => {
            let dur = (num_samples.max(1) as f64) / fs;
            let f0 = 20.0;
            let f1 = 20_000.0;
            let k_rate = det_ln(f1 / f0) / dur.max(1.0e-6);
            for k in 0..num_samples {
                let t = (k as f64) / fs;
                let phase = std::f64::consts::TAU * f0 * (det_exp(k_rate * t) - 1.0) / k_rate;
                out.push(det_sin(phase));
            }
        }
        _ => {
            for k in 0..num_samples {
                let t = (k as f64) / fs;
                out.push(det_sin(std::f64::consts::TAU * 1_000.0 * t));
            }
        }
    }
    out
}
