// Module: engine/crates/signal-em3d/src/via_return_rf.rs — Via Stub, Split Return, RF Synth, 3D Fields & TDR Layer-Peeling
// Invariants: Closed-form Hammerstad-Jensen microstrip, Wilkinson/patch RF synth (FEAT-019), and exact layer-peeling Z0(d) (FEAT-089).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_core::{det_cos, det_ln, det_sin};
use signal_geom::Point2Nm;

/// Quarter-wave via stub resonance $f_{\text{notch}} = \frac{c_0}{4 L_{\text{stub}} \sqrt{\varepsilon_{\text{eff}}}}$ & backdrill solver (`FEAT-013`).
pub fn compute_via_stub_notch_and_backdrill(
    stub_length_nm: i64,
    er_eff: f64,
    max_nyquist_hz: f64,
) -> (f64, i64) {
    let c0 = 299_792_458.0;
    let l_m = (stub_length_nm.max(1_000) as f64) * 1.0e-9;
    let f_notch_hz = c0 / (4.0 * l_m * er_eff.max(1.0).sqrt());
    // Target stub notch >= 2.5x Nyquist so insertion-loss suckout stays well above signal band
    let max_safe_stub_m = c0 / (4.0 * (2.5 * max_nyquist_hz.max(1.0e9)) * er_eff.max(1.0).sqrt());
    let max_safe_stub_nm = (max_safe_stub_m * 1.0e9).round() as i64;
    let drill_remove_nm = (stub_length_nm - max_safe_stub_nm).max(0);
    (f_notch_hz, drill_remove_nm)
}

/// Computes return-current detour loop inductance $\Delta L$ (`nH`) when a trace crosses a split reference plane (`FEAT-016`).
pub fn compute_split_plane_detour_inductance_nh(
    slot_detour_perimeter_nm: i64,
    trace_width_nm: i64,
    dielectric_height_nm: i64,
) -> f64 {
    let d_m = (slot_detour_perimeter_nm.max(0) as f64) * 1.0e-9;
    if d_m <= 0.0 {
        return 0.0;
    }
    let w_m = (trace_width_nm.max(10_000) as f64) * 1.0e-9;
    let h_m = (dielectric_height_nm.max(10_000) as f64) * 1.0e-9;
    let mu0_over_2pi = 2.0e-7_f64;
    let ratio = ((2.0 * (d_m + h_m)) / w_m).max(1.2);
    (mu0_over_2pi * d_m * det_ln(ratio)) * 1.0e9
}

#[derive(Debug, Clone, PartialEq)]
pub struct RfWilkinsonAndPatchSpec {
    pub microstrip_z0_ohm: f64,
    pub wilkinson_arm_z_ohm: f64,
    pub wilkinson_arm_length_nm: i64,
    pub wilkinson_isolation_r_ohm: f64,
    pub patch_width_nm: i64,
    pub patch_length_nm: i64,
}

/// Hammerstad-Jensen microstrip $Z_0$ + 2-way Wilkinson divider + rectangular patch antenna synthesizer (`FEAT-019`).
pub fn synthesize_rf_microstrip_wilkinson_patch(
    width_nm: i64,
    height_nm: i64,
    er: f64,
    center_freq_hz: f64,
) -> RfWilkinsonAndPatchSpec {
    let u = (width_nm.max(1_000) as f64) / (height_nm.max(1_000) as f64);
    let er_clamped = er.max(1.0);
    let er_eff = 0.5 * (er_clamped + 1.0) + 0.5 * (er_clamped - 1.0) / (1.0 + 12.0 / u).sqrt();
    let z0 = if u <= 1.0 {
        (60.0 / er_eff.sqrt()) * det_ln(8.0 / u + 0.25 * u)
    } else {
        (120.0 * std::f64::consts::PI) / (er_eff.sqrt() * (u + 1.393 + 0.667 * det_ln(u + 1.444)))
    };

    let c0 = 299_792_458.0;
    let f = center_freq_hz.max(1.0e6);
    let lambda_g_m = c0 / (f * er_eff.sqrt());
    let arm_len_nm = ((0.25 * lambda_g_m) * 1.0e9).round() as i64;

    // Rectangular microstrip patch W & Hammerstad fringing-corrected L
    let patch_w_m = (c0 / (2.0 * f)) * (2.0 / (er_clamped + 1.0)).sqrt();
    let h_m = (height_nm.max(1_000) as f64) * 1.0e-9;
    let w_over_h = patch_w_m / h_m;
    let delta_l_m =
        0.412 * h_m * ((er_eff + 0.3) * (w_over_h + 0.264)) / ((er_eff - 0.258) * (w_over_h + 0.8));
    let patch_l_m = (0.5 * lambda_g_m - 2.0 * delta_l_m).max(1.0e-4);

    RfWilkinsonAndPatchSpec {
        microstrip_z0_ohm: z0,
        wilkinson_arm_z_ohm: 2.0_f64.sqrt() * z0,
        wilkinson_arm_length_nm: arm_len_nm,
        wilkinson_isolation_r_ohm: 2.0 * z0,
        patch_width_nm: (patch_w_m * 1.0e9).round() as i64,
        patch_length_nm: (patch_l_m * 1.0e9).round() as i64,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmFieldSample {
    pub pos_nm: Point2Nm,
    pub e_mag_v_per_m: f64,
    pub h_mag_a_per_m: f64,
    pub poynting_w_per_m2: f64,
}

/// Samples animated harmonic $\vec{E}$, $\vec{H}$, and Poynting $\vec{S} = \vec{E} \times \vec{H}$ along a waveguide (`FEAT-087`).
pub fn sample_em3d_field_frame(
    start_nm: Point2Nm,
    end_nm: Point2Nm,
    wavelength_nm: i64,
    phase_rad: f64,
    num_samples: usize,
) -> Vec<EmFieldSample> {
    let n = num_samples.clamp(2, 512);
    let eta = 376.730_313_668 / 3.5_f64.sqrt();
    let lam = wavelength_nm.max(1_000) as f64;
    (0..n)
        .map(|i| {
            let t = (i as f64) / ((n - 1) as f64);
            let x = start_nm.x.0 + (((end_nm.x.0 - start_nm.x.0) as f64) * t).round() as i64;
            let y = start_nm.y.0 + (((end_nm.y.0 - start_nm.y.0) as f64) * t).round() as i64;
            let dist_nm = (((x - start_nm.x.0) as f64).hypot((y - start_nm.y.0) as f64)).max(0.0);
            let theta = (2.0 * std::f64::consts::PI * dist_nm / lam) - phase_rad;
            let e_v_m = 100.0 * det_cos(theta).abs() + 20.0 * det_sin(theta).abs();
            let h_a_m = e_v_m / eta;
            EmFieldSample {
                pos_nm: Point2Nm::new(x, y),
                e_mag_v_per_m: e_v_m,
                h_mag_a_per_m: h_a_m,
                poynting_w_per_m2: e_v_m * h_a_m,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TdrSpatialStep {
    pub step_index: u32,
    pub board_pos_nm: Point2Nm,
    pub impedance_mohm: i64,
}

/// Layer-peeling discrete TDR impedance reconstruction $Z_{k+1} = Z_k \frac{1 + \Gamma_k}{1 - \Gamma_k}$
/// mapped directly onto physical `(x_nm, y_nm)` trace coordinates (`FEAT-089`).
pub fn peel_tdr_impedance_profile(
    z_source_ohm: f64,
    local_reflections_gamma: &[f64],
    trace_start_nm: Point2Nm,
    trace_end_nm: Point2Nm,
) -> Vec<TdrSpatialStep> {
    let mut z_curr = z_source_ohm.max(1.0);
    let total = local_reflections_gamma.len().max(1);
    let mut out = Vec::with_capacity(local_reflections_gamma.len());

    for (idx, &gamma_raw) in local_reflections_gamma.iter().enumerate() {
        let g = gamma_raw.clamp(-0.95, 0.95);
        z_curr *= (1.0 + g) / (1.0 - g);
        let frac = ((idx + 1) as f64) / (total as f64);
        let px = trace_start_nm.x.0
            + (((trace_end_nm.x.0 - trace_start_nm.x.0) as f64) * frac).round() as i64;
        let py = trace_start_nm.y.0
            + (((trace_end_nm.y.0 - trace_start_nm.y.0) as f64) * frac).round() as i64;
        out.push(TdrSpatialStep {
            step_index: idx as u32,
            board_pos_nm: Point2Nm::new(px, py),
            impedance_mohm: (z_curr * 1_000.0).round() as i64,
        });
    }
    out
}
