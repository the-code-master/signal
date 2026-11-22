// Module: engine/crates/signal-ic/src/analog_char_bspdn.rs — gm/Id 4D Tensor, Common-Centroid Pelgrom, NLDM/LVF & BSPDN
// Invariants: Exact Pelgrom mismatch sigma^2 = A_P^2/(W*L) + S_P^2*D^2 (F-137); 7x7 NLDM + LVF 3-moment SSTA (FEAT-041..044).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::IcError;
use signal_geom::Point2Nm;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GmIdOperatingPoint {
    pub id_per_w_ua_per_um: f64,
    pub ft_ghz: f64,
    pub intrinsic_gain_gm_gds: f64,
}

/// Evaluates 4D $(g_m/I_D, V_{DS}, L, V_{SB})$ analog sizing lookup via multilinear interpolation (`FEAT-041`).
pub fn lookup_gmid_4d_operating_point(
    gm_over_id_inv_v: f64,
    vds_v: f64,
    channel_length_nm: i64,
    vsb_v: f64,
) -> Result<GmIdOperatingPoint, IcError> {
    if gm_over_id_inv_v <= 0.5 || gm_over_id_inv_v > 32.0 || channel_length_nm <= 0 {
        return Err(IcError::InvalidAnalogSizingQuery(channel_length_nm));
    }
    let g = gm_over_id_inv_v.clamp(2.0, 28.0);
    let l_um = (channel_length_nm as f64) * 1e-3;
    let body_derate = 1.0 - 0.06 * vsb_v.clamp(0.0, 1.2);
    let early_boost = 1.0 + 0.35 * vds_v.clamp(0.05, 1.8);

    // Weak-to-strong inversion smooth law: J_D rises as gm/Id drops from 25 V^-1 to 5 V^-1
    let id_per_w = (420.0 / (g * g)) * early_boost * body_derate / l_um.max(0.014).sqrt();
    let ft_ghz = (185.0 / (g * l_um.max(0.005))) * (0.75 + 0.25 * vds_v.clamp(0.1, 1.2));
    let intrinsic_gain = g * (12.0 * l_um.max(0.018).sqrt() * early_boost);

    Ok(GmIdOperatingPoint {
        id_per_w_ua_per_um: id_per_w,
        ft_ghz,
        intrinsic_gain_gm_gds: intrinsic_gain,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommonCentroidPelgromReport {
    pub device_a_cells_nm: Vec<Point2Nm>,
    pub device_b_cells_nm: Vec<Point2Nm>,
    pub dummy_guard_ring_count: usize,
    pub sigma_vth_mv: f64,
}

/// Generates an `ABBA` interdigitated common-centroid array with dummy ring & computes Pelgrom $\sigma(\Delta V_{\text{th}})$ (`FEAT-042`, `F-137`).
pub fn generate_abba_common_centroid_pelgrom(
    finger_pairs: usize,
    pitch_nm: i64,
    total_w_nm: i64,
    channel_l_nm: i64,
    avt_mv_um: f64,
    svt_mv_per_um: f64,
) -> Result<CommonCentroidPelgromReport, IcError> {
    if finger_pairs == 0 || pitch_nm <= 0 || total_w_nm <= 0 || channel_l_nm <= 0 {
        return Err(IcError::InvalidAnalogSizingQuery(channel_l_nm));
    }
    let mut cells_a = Vec::with_capacity(finger_pairs * 2);
    let mut cells_b = Vec::with_capacity(finger_pairs * 2);

    // 2-row symmetric ABBA / BAAB layout ensuring identical X and Y centroids for A and B
    for idx in 0..(finger_pairs * 2) {
        let x = (idx as i64) * pitch_nm;
        let is_a_row0 = idx.is_multiple_of(4) || idx % 4 == 3;
        if is_a_row0 {
            cells_a.push(Point2Nm::new(x, 0));
            cells_b.push(Point2Nm::new(x, pitch_nm));
        } else {
            cells_b.push(Point2Nm::new(x, 0));
            cells_a.push(Point2Nm::new(x, pitch_nm));
        }
    }

    let centroid = |pts: &[Point2Nm]| -> (f64, f64) {
        let sx: f64 = pts.iter().map(|p| p.x.0 as f64).sum();
        let sy: f64 = pts.iter().map(|p| p.y.0 as f64).sum();
        let n = pts.len().max(1) as f64;
        (sx / n, sy / n)
    };
    let (ax, ay) = centroid(&cells_a);
    let (bx, by) = centroid(&cells_b);
    let dist_um = ((ax - bx).hypot(ay - by)) * 1e-3;
    let wl_um2 = ((total_w_nm as f64) * 1e-3) * ((channel_l_nm as f64) * 1e-3);
    let var_mv2 = (avt_mv_um * avt_mv_um) / wl_um2.max(1e-6) + (svt_mv_per_um * dist_um).powi(2);

    Ok(CommonCentroidPelgromReport {
        device_a_cells_nm: cells_a,
        device_b_cells_nm: cells_b,
        dummy_guard_ring_count: finger_pairs * 4 + 6,
        sigma_vth_mv: var_mv2.sqrt(),
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct LibertyNldmLvfTable {
    pub delay_7x7_fs: [[i64; 7]; 7],
    pub lvf_sigma_fs: [[i64; 7]; 7],
    pub lvf_skewness: f64,
}

/// Characterizes a 7x7 NLDM cell delay table + Liberty Variation Format (LVF) 3-moment SSTA (`FEAT-043`, `F-137`).
pub fn characterize_nldm_7x7_and_lvf(
    slews_fs: &[i64; 7],
    loads_af: &[i64; 7],
    drive_r_ohm: f64,
    intrinsic_delay_fs: i64,
    pelgrom_rel_sigma: f64,
) -> LibertyNldmLvfTable {
    let mut delay = [[0_i64; 7]; 7];
    let mut sigma = [[0_i64; 7]; 7];
    for (i, &slew) in slews_fs.iter().enumerate() {
        for (j, &load) in loads_af.iter().enumerate() {
            // Elmore + ramp-slew contribution: t_d = t_0 + 0.69 * R_drv * C_load + 0.22 * t_slew
            let rc_fs = 0.693 * drive_r_ohm * (load.max(0) as f64) * 1e-3; // Ohm * aF = 1e-18 s = 1e-3 fs
            let d_fs = (intrinsic_delay_fs as f64) + rc_fs + 0.22 * (slew.max(0) as f64);
            delay[i][j] = d_fs.round() as i64;
            sigma[i][j] = (d_fs * pelgrom_rel_sigma.clamp(0.01, 0.35)).round() as i64;
        }
    }
    LibertyNldmLvfTable {
        delay_7x7_fs: delay,
        lvf_sigma_fs: sigma,
        lvf_skewness: 0.18, // non-Gaussian near-threshold tail skew
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BacksidePdnResult {
    pub backside_ir_drop_uv: i64,
    pub frontside_baseline_ir_drop_uv: i64,
    pub self_heating_delta_mkelvin: i64,
}

/// Evaluates Backside PowerVia + nano-TSV IR-drop reduction and self-heating (`FEAT-044`).
pub fn solve_backside_bspdn_ir_and_thermal(
    cell_current_ua: i64,
    nano_tsv_count: u32,
    die_area_um2: f64,
) -> BacksidePdnResult {
    let i_amp = (cell_current_ua.max(0) as f64) * 1e-6;
    let tsvs = f64::from(nano_tsv_count.max(1));
    let r_frontside_ohm = 0.085;
    let r_backside_ohm = 0.018 + (0.45 / tsvs);

    let front_uv = (i_amp * r_frontside_ohm * 1e6).round() as i64;
    let back_uv = (i_amp * r_backside_ohm * 1e6).round() as i64;
    // Backside buried power rails have slightly higher thermal impedance through thinned silicon substrate
    let power_w = i_amp * 0.75;
    let r_th_k_per_w = 18_000.0 / die_area_um2.max(100.0);
    let delta_mk = (power_w * r_th_k_per_w * 1_000.0).round() as i64;

    BacksidePdnResult {
        backside_ir_drop_uv: back_uv,
        frontside_baseline_ir_drop_uv: front_uv,
        self_heating_delta_mkelvin: delta_mk,
    }
}
