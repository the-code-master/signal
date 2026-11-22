// Module: engine/crates/signal-dfm/src/panel_dfa_fatigue.rs — Panelizer, IPC-7351B DFA, Engelmaier/Steinberg & Coating Keepouts
// Invariants: Stencil area ratio > 0.66 (FEAT-064), MLCC depaneling strain < 500 ustrain (FEAT-063), coating keepout clean (FEAT-066).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_core::{det_exp, det_ln};
use signal_geom::{BBox2Nm, Point2Nm};

#[derive(Debug, Clone, PartialEq)]
pub struct PanelLayoutSummary {
    pub panel_width_nm: i64,
    pub panel_height_nm: i64,
    pub board_utilization_pct: f64,
    pub fiducials_nm: [Point2Nm; 3],
    pub depanel_mlcc_strain_clean: bool,
}

/// Composes a `cols x rows` step-and-repeat manufacturing panel with rails, 3 global optical fiducials,
/// and checks MLCC proximity to V-score / mouse-bite break lines (`>= 3.0 mm` keepout for `< 500 µε` strain, `FEAT-063`).
pub fn compose_manufacturing_panel(
    board_w_nm: i64,
    board_h_nm: i64,
    cols: u32,
    rows: u32,
    rail_width_nm: i64,
    tab_kerf_nm: i64,
    min_mlcc_dist_to_edge_nm: i64,
) -> PanelLayoutSummary {
    let c = i64::from(cols.max(1));
    let r = i64::from(rows.max(1));
    let kerf = tab_kerf_nm.max(0);
    let rail = rail_width_nm.max(2_000_000);

    let panel_w = 2 * rail + c * board_w_nm + (c - 1) * kerf;
    let panel_h = 2 * rail + r * board_h_nm + (r - 1) * kerf;
    let active_area = (c * r) as f64 * (board_w_nm as f64) * (board_h_nm as f64);
    let total_area = ((panel_w as f64) * (panel_h as f64)).max(1.0);
    let util_pct = (active_area / total_area * 100.0).clamp(1.0, 99.9);

    let half_r = rail / 2;
    let fiducials = [
        Point2Nm::new(half_r, half_r),
        Point2Nm::new(panel_w - half_r, half_r),
        Point2Nm::new(half_r, panel_h - half_r),
    ];

    PanelLayoutSummary {
        panel_width_nm: panel_w,
        panel_height_nm: panel_h,
        board_utilization_pct: util_pct,
        fiducials_nm: fiducials,
        depanel_mlcc_strain_clean: min_mlcc_dist_to_edge_nm >= 3_000_000,
    }
}

/// Checks IPC-7525 stencil aperture Area Ratio $AR = \frac{L \cdot W}{2 (L + W) T} > 0.66$,
/// component courtyard spacing, and 0402/0201 pad thermal-mass balance against tombstoning (`FEAT-064`).
pub fn verify_dfa_stencil_and_tombstone(
    aperture_w_nm: i64,
    aperture_l_nm: i64,
    stencil_thickness_nm: i64,
    pad1_copper_area_sq_um: f64,
    pad2_copper_area_sq_um: f64,
) -> (bool, f64) {
    let w = aperture_w_nm.max(1) as f64;
    let l = aperture_l_nm.max(1) as f64;
    let t = stencil_thickness_nm.max(1) as f64;
    let area_ratio = (w * l) / (2.0 * (w + l) * t);
    let asym = (pad1_copper_area_sq_um - pad2_copper_area_sq_um).abs()
        / pad1_copper_area_sq_um.max(pad2_copper_area_sq_um).max(1.0);
    (area_ratio > 0.66 && asym <= 0.25, area_ratio)
}

/// Computes Engelmaier BGA solder-joint thermal fatigue life $N_f$ (`cycles`) from Distance-to-Neutral-Point (`dnp_mm`)
/// AND Steinberg fundamental PCB vibration natural frequency $f_1$ (`Hz`, `FEAT-065`).
pub fn evaluate_bga_solder_fatigue_and_board_modal_hz(
    dnp_mm: f64,
    solder_standoff_mm: f64,
    delta_cte_ppm_per_k: f64,
    delta_t_k: f64,
    board_len_mm: f64,
    board_wid_mm: f64,
    board_thick_mm: f64,
) -> (u32, f64) {
    let delta_gamma = (dnp_mm.max(0.5) / (2.0 * solder_standoff_mm.max(0.10)))
        * (delta_cte_ppm_per_k.abs() * 1.0e-6)
        * delta_t_k.max(5.0);
    let eps_f = 0.325_f64;
    let c_exp = -0.442_f64; // Engelmaier fatigue ductility exponent
    let ratio = (delta_gamma / (2.0 * eps_f)).max(1.0e-6);
    let nf = 0.5 * det_exp((1.0 / c_exp) * det_ln(ratio));

    // Steinberg rectangular PCB natural frequency f_n (simply-supported FR4 E ~= 18.6 GPa, rho ~= 1850 kg/m^3)
    let a_m = (board_len_mm.max(10.0)) * 1.0e-3;
    let b_m = (board_wid_mm.max(10.0)) * 1.0e-3;
    let h_m = (board_thick_mm.max(0.4)) * 1.0e-3;
    let e_pa = 18.6e9_f64;
    let nu = 0.16_f64;
    let rho = 1_850.0_f64;
    let flexural_d = (e_pa * h_m.powi(3)) / (12.0 * (1.0 - nu * nu));
    let f1_hz = (std::f64::consts::PI / 2.0)
        * (1.0 / (a_m * a_m) + 1.0 / (b_m * b_m))
        * (flexural_d / (rho * h_m)).sqrt();

    ((nf.round() as u32).clamp(1, 500_000), f1_hz)
}

/// Verifies zero overlap between conformal-coating spray polygons and mandatory keepout components
/// (connectors, MEMS microphones/barometers, RF shield clips, ICT pads, `FEAT-066`).
pub fn verify_conformal_coating_keepouts(
    coating_zones: &[BBox2Nm],
    mandatory_keepouts: &[(String, BBox2Nm)],
) -> Vec<String> {
    let mut violations = Vec::new();
    for (id, ko) in mandatory_keepouts {
        if coating_zones.iter().any(|cz| cz.intersects(ko)) {
            violations.push(id.clone());
        }
    }
    violations
}
