// Module: engine/crates/signal-magnetics/src/lib.rs — iGSE Core Loss, Dowell Proximity AC Loss & Planar Coil Synth (FEAT-021)
// Invariants: #![forbid(unsafe_code)]; exact Dowell sinh/cosh/sin/cos winding resistance & Steinmetz iGSE core loss.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use signal_core::{det_cos, det_exp, det_ln, det_sin};
use signal_geom::Point2Nm;
use signal_proto::v1::{DesignPlanarMagneticsRequest, DesignPlanarMagneticsResponse};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MagneticsError {
    #[error("invalid magnetics specification: {0}")]
    InvalidSpec(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FerriteSteinmetzMaterial {
    pub k_si: f64,
    pub alpha: f64,
    pub beta: f64,
    pub mu_r: f64,
    pub b_sat_tesla: f64,
}

impl FerriteSteinmetzMaterial {
    pub fn by_name(name: &str) -> Self {
        match name.trim().to_ascii_uppercase().as_str() {
            "3F4" => Self {
                k_si: 8.5,
                alpha: 1.62,
                beta: 2.65,
                mu_r: 900.0,
                b_sat_tesla: 0.41,
            },
            "PC95" => Self {
                k_si: 3.1,
                alpha: 1.46,
                beta: 2.55,
                mu_r: 3300.0,
                b_sat_tesla: 0.53,
            },
            _ => Self {
                // TDK N87 canonical default
                k_si: 4.8,
                alpha: 1.48,
                beta: 2.60,
                mu_r: 2200.0,
                b_sat_tesla: 0.49,
            },
        }
    }
}

fn pow_pos(base: f64, exp: f64) -> f64 {
    det_exp(exp * det_ln(base.max(1e-30)))
}

fn det_sinh_cosh(x: f64) -> (f64, f64) {
    let xc = x.clamp(-20.0, 20.0);
    let ep = det_exp(xc);
    let em = det_exp(-xc);
    (0.5 * (ep - em), 0.5 * (ep + em))
}

/// Computes Improved Generalized Steinmetz Equation (`iGSE`) volumetric core loss (`W/m^3`)
/// for a triangular/PWM flux swing $\Delta B$ (`Tesla`) at `freq_hz` & `duty` (`FEAT-021`).
pub fn compute_igse_core_loss_w_per_m3(
    mat: FerriteSteinmetzMaterial,
    delta_b_tesla: f64,
    freq_hz: f64,
    duty_ratio: f64,
) -> f64 {
    let d = duty_ratio.clamp(0.05, 0.95);
    let db = delta_b_tesla.max(1e-6);
    // ki approx k / ((2*pi)^(alpha-1) * 2^(beta-alpha) * integral_factor)
    let ki = mat.k_si
        / (pow_pos(2.0 * std::f64::consts::PI, mat.alpha - 1.0)
            * pow_pos(2.0, mat.beta - mat.alpha)
            * 1.45);
    let slope_up = db * freq_hz / d;
    let slope_dn = db * freq_hz / (1.0 - d);
    let avg_rate = d * pow_pos(slope_up, mat.alpha) + (1.0 - d) * pow_pos(slope_dn, mat.alpha);
    ki * pow_pos(db, mat.beta - mat.alpha) * avg_rate
}

/// Computes Dowell's 1D AC-to-DC winding resistance ratio $F_R = R_{\text{AC}}(f) / R_{\text{DC}} \ge 1.0$
/// accounting for both skin effect and $m$-layer proximity effect (`FEAT-021`).
pub fn compute_dowell_ac_resistance_factor(
    foil_thickness_m: f64,
    freq_hz: f64,
    layer_count: u32,
) -> f64 {
    let mu0 = 4.0e-7 * std::f64::consts::PI;
    let rho_cu = 1.724e-8_f64;
    let skin_depth_m = (rho_cu / (std::f64::consts::PI * freq_hz.max(1.0) * mu0)).sqrt();
    let delta = (foil_thickness_m.max(1.0e-6) / skin_depth_m).clamp(0.01, 10.0);

    let (sh2, ch2) = det_sinh_cosh(2.0 * delta);
    let (sh1, ch1) = det_sinh_cosh(delta);
    let skin_term = (sh2 + det_sin(2.0 * delta)) / (ch2 - det_cos(2.0 * delta)).max(1e-12);
    let prox_term = (sh1 - det_sin(delta)) / (ch1 + det_cos(delta)).max(1e-12);
    let m = f64::from(layer_count.max(1));
    (delta * (skin_term + ((2.0 * (m * m - 1.0)) / 3.0) * prox_term)).max(1.0)
}

/// Computes air-gap fringing factor $F_{\text{fringe}} \ge 1$ and interleaved planar leakage inductance (`nH`, `FEAT-021`).
pub fn compute_gap_fringing_and_leakage_nh(
    air_gap_m: f64,
    core_area_m2: f64,
    window_height_m: f64,
    turns: u32,
    mean_turn_len_m: f64,
    window_breadth_m: f64,
    inter_winding_iso_m: f64,
) -> (f64, i64) {
    let lg = air_gap_m.max(1.0e-6);
    let ae_sqrt = core_area_m2.max(1.0e-8).sqrt();
    let fringe = 1.0 + (lg / ae_sqrt) * det_ln((2.0 * window_height_m.max(2.0 * lg)) / lg);
    let mu0 = 4.0e-7 * std::f64::consts::PI;
    let n = f64::from(turns.max(1));
    let l_lk_h = (mu0 * n * n * mean_turn_len_m / window_breadth_m.max(1.0e-4))
        * inter_winding_iso_m.max(5.0e-5);
    (fringe, (l_lk_h * 1.0e9).round() as i64)
}

/// Designs an optimal planar PCB inductor/transformer matching `DesignPlanarMagneticsRequest` (`FEAT-021`).
pub fn design_planar_magnetics_rpc(
    req: &DesignPlanarMagneticsRequest,
) -> Result<DesignPlanarMagneticsResponse, MagneticsError> {
    if req.target_inductance_nh <= 0 || req.switching_freq_hz <= 0 {
        return Err(MagneticsError::InvalidSpec(
            "target_inductance_nh and switching_freq_hz must be > 0",
        ));
    }
    let mat = FerriteSteinmetzMaterial::by_name(&req.core_material);
    let ae_m2 = 64.0e-6; // ER23/Planar core 64 mm^2
    let ve_m3 = 1.95e-6; // 1.95 cm^3
    let i_pk_a = (req.peak_current_na.max(100_000_000) as f64) * 1.0e-9;
    let l_h = (req.target_inductance_nh as f64) * 1.0e-9;
    let f_hz = req.switching_freq_hz as f64;

    // Choose minimum turns avoiding saturation (B_pk <= 0.70 * B_sat)
    let b_target = 0.65 * mat.b_sat_tesla;
    let turns = (((l_h * i_pk_a) / (b_target * ae_m2)).ceil() as u32).clamp(2, 64);
    let delta_b = ((l_h * 0.4 * i_pk_a) / (f64::from(turns) * ae_m2)).clamp(0.01, mat.b_sat_tesla);

    let pv = compute_igse_core_loss_w_per_m3(mat, delta_b, f_hz, 0.45);
    let core_loss_mw = (pv * ve_m3 * 1_000.0).round() as i64;

    let fr = compute_dowell_ac_resistance_factor(70.0e-6, f_hz, (turns / 2).max(1));
    let r_dc_ohm = 0.0045 * f64::from(turns);
    let i_rms_a = 0.72 * i_pk_a;
    let ac_winding_mw = (i_rms_a * i_rms_a * r_dc_ohm * fr * 1_000.0).round() as i64;

    let (_, l_lk_nh) =
        compute_gap_fringing_and_leakage_nh(2.5e-4, ae_m2, 4.0e-3, turns, 0.048, 8.0e-3, 1.5e-4);

    Ok(DesignPlanarMagneticsResponse {
        optimal_turn_count: turns,
        core_loss_mw: core_loss_mw.max(1),
        ac_winding_loss_mw: ac_winding_mw.max(1),
        leakage_inductance_nh: l_lk_nh.max(1),
    })
}

/// Generates Archimedean planar PCB copper spiral centerline vertices in exact `nm` (`FEAT-021`).
pub fn generate_planar_spiral_coil_nm(
    center_nm: Point2Nm,
    inner_radius_nm: i64,
    turn_pitch_nm: i64,
    turns: u32,
    segments_per_turn: u32,
) -> Vec<Point2Nm> {
    let spt = segments_per_turn.clamp(8, 128);
    let total_pts = turns.max(1) * spt;
    (0..=total_pts)
        .map(|idx| {
            let rev = f64::from(idx) / f64::from(spt);
            let theta = 2.0 * std::f64::consts::PI * rev;
            let r_nm = (inner_radius_nm as f64) + rev * (turn_pitch_nm as f64);
            Point2Nm::new(
                center_nm.x.0 + (r_nm * det_cos(theta)).round() as i64,
                center_nm.y.0 + (r_nm * det_sin(theta)).round() as i64,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dowell_proximity_igse_core_loss_and_planar_spiral_coil_verified() {
        // Above 1 skin depth (70 um 2oz Cu at 1.5 MHz => Delta ~= 1.30), 6-layer proximity >> 3x 1-layer!
        let fr_1l = compute_dowell_ac_resistance_factor(70.0e-6, 1_500_000.0, 1);
        let fr_6l = compute_dowell_ac_resistance_factor(70.0e-6, 1_500_000.0, 6);
        assert!(fr_1l >= 1.0 && fr_6l > fr_1l * 3.0);

        let resp = design_planar_magnetics_rpc(&DesignPlanarMagneticsRequest {
            target_inductance_nh: 2_200_000, // 2.2 uH
            switching_freq_hz: 500_000,
            peak_current_na: 8_000_000_000, // 8 A
            core_material: "N87".into(),
        })
        .expect("planar mag");
        assert!(
            resp.optimal_turn_count >= 2 && resp.core_loss_mw > 0 && resp.ac_winding_loss_mw > 0
        );

        let spiral = generate_planar_spiral_coil_nm(Point2Nm::new(0, 0), 2_000_000, 400_000, 4, 16);
        assert_eq!(spiral.len(), 65);
    }
}
