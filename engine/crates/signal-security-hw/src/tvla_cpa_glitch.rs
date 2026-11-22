// Module: engine/crates/signal-security-hw/src/tvla_cpa_glitch.rs — CMOS Hamming Power, Welch's TVLA, CPA & Glitch Bypass (FEAT-050)
// Invariants: Flags Welch's two-sample |t| > 4.5 side-channel leakage; detects single-bit Hamming-1 FSM auth bypasses (FEAT-050).
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/15-application-security-and-project-isolation.md
use crate::HwSecurityError;
use signal_core::sha512;

pub const TVLA_WELCH_T_THRESHOLD: f64 = 4.5;

const AES_TOY_SBOX: [u8; 16] = [
    0xC, 0x5, 0x6, 0xB, 0x9, 0x0, 0xA, 0xD, 0x3, 0xE, 0xF, 0x8, 0x4, 0x7, 0x1, 0x2,
];

/// Simulates dynamic CMOS switching power traces $I_{DD}(t)$ proportional to register Hamming-distance transitions (`FEAT-050`).
pub fn simulate_cmos_hamming_power_traces(
    secret_key_byte: u8,
    trace_count: usize,
    fixed_input_mode: bool,
    boolean_first_order_masked: bool,
) -> Result<Vec<f64>, HwSecurityError> {
    if trace_count < 4 {
        return Err(HwSecurityError::InsufficientPowerTraces(trace_count));
    }
    let mut traces = Vec::with_capacity(trace_count);
    for idx in 0..trace_count {
        let seed_buf = (idx as u64).to_le_bytes();
        let prng = sha512(&seed_buf);
        let pt = if fixed_input_mode { 0x5A_u8 } else { prng[0] };
        let mask = prng[1];
        let noise = ((f64::from(prng[2]) / 255.0) - 0.5) * 0.35;

        let sbox_lo = AES_TOY_SBOX[((pt ^ secret_key_byte) & 0x0F) as usize];
        let sbox_hi = AES_TOY_SBOX[(((pt ^ secret_key_byte) >> 4) & 0x0F) as usize];
        let raw_state = (sbox_hi << 4) | sbox_lo;

        let switching_hw = if boolean_first_order_masked {
            // 2-share Boolean masking randomizes physical register transitions with uniform mask!
            let share0 = raw_state ^ mask;
            let prev_reg = prng[3];
            f64::from((share0 ^ prev_reg).count_ones())
        } else {
            f64::from(raw_state.count_ones())
        };
        traces.push(switching_hw * 12.5 + noise);
    }
    Ok(traces)
}

/// Computes Welch's two-sample $t$-statistic between Fixed and Random trace sets (`|t| > 4.5` => TVLA fail, `FEAT-050`).
pub fn compute_welch_tvla_t_stat(
    fixed_traces: &[f64],
    random_traces: &[f64],
) -> Result<f64, HwSecurityError> {
    if fixed_traces.len() < 2 || random_traces.len() < 2 {
        return Err(HwSecurityError::InsufficientPowerTraces(
            fixed_traces.len().min(random_traces.len()),
        ));
    }
    let mean_var = |xs: &[f64]| -> (f64, f64) {
        let n = xs.len() as f64;
        let mean = xs.iter().sum::<f64>() / n;
        let var = xs.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
        (mean, var)
    };
    let (m0, v0) = mean_var(fixed_traces);
    let (m1, v1) = mean_var(random_traces);
    let denom = ((v0 / (fixed_traces.len() as f64)) + (v1 / (random_traces.len() as f64)))
        .max(1e-15)
        .sqrt();
    Ok((m0 - m1) / denom)
}

/// Computes Pearson Correlation Power Analysis (`CPA`) score for a candidate nibble key hypothesis (`FEAT-050`).
pub fn compute_cpa_key_correlation(
    plaintexts: &[u8],
    power_samples: &[f64],
    candidate_nibble: u8,
) -> f64 {
    let n = plaintexts.len().min(power_samples.len());
    if n < 2 {
        return 0.0;
    }
    let hyp: Vec<f64> = plaintexts[..n]
        .iter()
        .map(|&p| f64::from(AES_TOY_SBOX[((p ^ candidate_nibble) & 0x0F) as usize].count_ones()))
        .collect();
    let mean_h = hyp.iter().sum::<f64>() / (n as f64);
    let mean_p = power_samples[..n].iter().sum::<f64>() / (n as f64);

    let mut cov = 0.0_f64;
    let mut var_h = 0.0_f64;
    let mut var_p = 0.0_f64;
    for i in 0..n {
        let dh = hyp[i] - mean_h;
        let dp = power_samples[i] - mean_p;
        cov += dh * dp;
        var_h += dh * dh;
        var_p += dp * dp;
    }
    let denom = (var_h * var_p).sqrt();
    if denom < 1e-12 {
        0.0
    } else {
        (cov / denom).abs()
    }
}

/// Counts FSM state encodings vulnerable to single-bit laser/voltage-glitch jump into privileged states (`FEAT-050`).
pub fn count_glitch_vulnerable_fsm_states(
    non_privileged_encodings: &[u32],
    privileged_encodings: &[u32],
) -> u32 {
    let mut vulnerable = 0_u32;
    for &s_norm in non_privileged_encodings {
        // Minimum Hamming distance >= 4 required so neither 1-bit laser flip nor 2-bit clock glitch reaches UNLOCKED!
        let min_hd = privileged_encodings
            .iter()
            .map(|&s_priv| (s_norm ^ s_priv).count_ones())
            .min()
            .unwrap_or(32);
        if min_hd <= 2 {
            vulnerable += 1;
        }
    }
    vulnerable
}
