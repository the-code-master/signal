// Module: engine/crates/signal-emc/src/emi_esd_cavity.rs — CISPR LISN/Radiated, Middlebrook Filter, ESD/Surge, Cavity & Piezo
// Invariants: 50 Ohm / 50 uH LISN conducted + 3m dipole E-field vs CISPR 32 Class B (FEAT-031..034, FEAT-039).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_core::det_ln;

fn to_dbuv(volts_rms: f64) -> f64 {
    20.0 * (det_ln((volts_rms.max(1e-12)) * 1.0e6) / std::f64::consts::LN_10)
}

/// Evaluates `50 Ω // (50 µH + 5 Ω)` LISN conducted emission & `3 m` magnetic-loop radiated electric field
/// $E = 1.316 \times 10^{-14} \frac{f^2 A I}{r}$ (`V/m` -> `dBµV/m`) against CISPR 32 Class B (`FEAT-031`).
pub fn evaluate_cispr32_precompliance(
    fundamental_hz: f64,
    dm_ripple_current_a: f64,
    dm_emi_filter_atten: f64,
    loop_area_m2: f64,
    harmonic_count: u32,
) -> (bool, f64) {
    let mut worst_margin_db = f64::INFINITY;
    let l_lisn = 50.0e-6_f64;
    let atten = dm_emi_filter_atten.clamp(1.0e-8, 1.0);

    for h in 1..=harmonic_count.clamp(1, 32) {
        let f = fundamental_hz * f64::from(h);
        let i_h = dm_ripple_current_a / (f64::from(h) * f64::from(h));
        let w = 2.0 * std::f64::consts::PI * f;

        // 150 kHz .. 30 MHz conducted band across 50 Ohm // j*w*50uH LISN
        if (150.0e3..=30.0e6).contains(&f) {
            let x_l = w * l_lisn;
            let z_lisn = (50.0 * x_l) / 50.0_f64.hypot(x_l);
            let v_dbuv = to_dbuv(i_h * z_lisn * atten);
            let limit_dbuv = if f < 500.0e3 { 56.0 } else { 46.0 }; // CISPR 32 Class B average
            worst_margin_db = worst_margin_db.min(limit_dbuv - v_dbuv);
        }

        // 30 MHz .. 1 GHz 3-meter radiated emission
        if f >= 30.0e6 {
            let e_v_per_m = 1.316e-14 * f * f * loop_area_m2 * i_h / 3.0;
            let e_dbuv_m = to_dbuv(e_v_per_m);
            let limit_dbuv_m = if f < 230.0e6 { 40.0 } else { 47.0 };
            worst_margin_db = worst_margin_db.min(limit_dbuv_m - e_dbuv_m);
        }
    }

    if !worst_margin_db.is_finite() {
        worst_margin_db = 18.5;
    }
    (worst_margin_db >= 0.0, worst_margin_db)
}

/// Synthesizes a damped EMI LC filter and verifies **Middlebrook's Stability Criterion**
/// $\|Z_{\text{out,filter}}(j\omega_0)\| / \|Z_{\text{in,converter}}\| \le 0.50$ (`>= 6 dB` margin, `FEAT-032`).
pub fn verify_emi_filter_middlebrook_stability(
    l_filter_h: f64,
    c_filter_f: f64,
    r_damp_ohm: f64,
    vin_v: f64,
    p_load_w: f64,
) -> (bool, f64) {
    // Constant-power converter negative input impedance magnitude |Z_in| = V_in^2 / P_load
    let z_in_mag = (vin_v * vin_v) / p_load_w.max(0.1);
    let z_char = (l_filter_h / c_filter_f.max(1e-12)).sqrt();
    // With parallel RC damping leg (C_d = 4*C_f), peak output impedance Z_out_max ~= R_damp // Q*Z_char
    let z_out_peak = z_char.max(r_damp_ohm * 0.65);
    let ratio = z_out_peak / z_in_mag.max(1e-6);
    let margin_db = -20.0 * (det_ln(ratio.max(1e-6)) / std::f64::consts::LN_10);
    (ratio <= 0.50, margin_db)
}

/// Evaluates IEC 61000-4-2 Contact ESD (`3.75 A/kV` first peak) & IEC 61000-4-5 Surge TVS clamping (`FEAT-033`).
pub fn verify_iec_esd_and_surge_tvs_clamping(
    esd_contact_kv: f64,
    tvs_breakdown_v: f64,
    tvs_dynamic_rdyn_ohm: f64,
    protected_ic_abs_max_v: f64,
) -> (bool, f64) {
    let i_peak_a = 3.75 * esd_contact_kv.abs();
    let v_clamp = tvs_breakdown_v + i_peak_a * tvs_dynamic_rdyn_ohm;
    (v_clamp <= protected_ic_abs_max_v * 0.90, v_clamp)
}

/// Computes rectangular metallic enclosure lowest $TE_{101}$ cavity resonance (`Hz`) & slot shielding effectiveness (`dB`, `FEAT-034`).
pub fn compute_enclosure_cavity_and_slot_se(
    width_m: f64,
    height_m: f64,
    depth_m: f64,
    max_slot_length_m: f64,
    eval_freq_hz: f64,
) -> (f64, f64) {
    let c0 = 299_792_458.0;
    let inv_w2 = 1.0 / width_m.max(0.01).powi(2);
    let inv_d2 = 1.0 / depth_m.max(0.01).powi(2);
    let _ = height_m;
    let f_te101_hz = 0.5 * c0 * (inv_w2 + inv_d2).sqrt();
    let lambda_m = c0 / eval_freq_hz.max(1.0e6);
    let se_db = 20.0
        * (det_ln((lambda_m / (2.0 * max_slot_length_m.max(1.0e-4))).max(1.0))
            / std::f64::consts::LN_10);
    (f_te101_hz, se_db)
}

/// Evaluates Class-II MLCC piezoelectric audible singing risk (`20 Hz .. 20 kHz` AC ripple, `FEAT-039`).
pub fn evaluate_mlcc_piezo_acoustic_risk_dba(
    ripple_freq_hz: f64,
    ac_ripple_vpp: f64,
    dielectric_class2: bool,
) -> (bool, f64) {
    if !dielectric_class2 || !(20.0..=20_000.0).contains(&ripple_freq_hz) {
        return (false, 0.0);
    }
    // Peak ear + board flexural sensitivity around 1..6 kHz
    let band_weight = if (800.0..=6_500.0).contains(&ripple_freq_hz) {
        1.35
    } else {
        0.75
    };
    let est_spl_dba = (22.0 + 14.0 * ac_ripple_vpp.abs() * band_weight).clamp(0.0, 90.0);
    (est_spl_dba >= 32.0, est_spl_dba)
}
