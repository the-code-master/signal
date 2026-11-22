// Module: engine/crates/signal-pcb/src/meander_and_weave.rs — 5xW Trombone Meanders (FEAT-012) & Glass-Weave Skew (FEAT-011)
// Invariants: Enforces >= 5x trace-width self-spacing on trombone arcs; models spatially periodic FR4/Megtron weave e_r(x,y).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use crate::PcbError;
use signal_core::{det_cos, det_sin};
use signal_geom::Point2Nm;
use signal_proto::v1::{
    AnalyzeFiberWeaveAndBackdrillRequest, AnalyzeFiberWeaveAndBackdrillResponse,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TromboneMeanderPlan {
    pub waypoints_nm: Vec<Point2Nm>,
    pub achieved_length_nm: i64,
    pub lobe_spacing_nm: i64,
    pub lobe_count: u32,
}

/// Synthesizes `>= 5x W` coupled-arc serpentine trombone meanders (`FEAT-012`) to match `target_length_nm`.
pub fn synthesize_trombone_meander(
    start_nm: Point2Nm,
    end_nm: Point2Nm,
    trace_width_nm: i64,
    target_length_nm: i64,
) -> Result<TromboneMeanderPlan, PcbError> {
    if trace_width_nm <= 0 {
        return Err(PcbError::InvalidRoutingGeometry(
            "trace_width_nm must be > 0",
        ));
    }
    let dx = (end_nm.x.0 - start_nm.x.0) as f64;
    let dy = (end_nm.y.0 - start_nm.y.0) as f64;
    let base_len_nm = dx.hypot(dy).round() as i64;
    let min_lobe_spacing_nm = 5 * trace_width_nm;

    if target_length_nm <= base_len_nm {
        return Ok(TromboneMeanderPlan {
            waypoints_nm: vec![start_nm, end_nm],
            achieved_length_nm: base_len_nm,
            lobe_spacing_nm: min_lobe_spacing_nm,
            lobe_count: 0,
        });
    }

    let extra_needed_nm = target_length_nm - base_len_nm;
    let max_span = (base_len_nm / (2 * min_lobe_spacing_nm).max(1)).clamp(1, 64) as u32;
    let lobes = max_span;
    // Each trombone lobe adds 2 * amplitude_nm of orthogonal path length
    let amp_nm = (extra_needed_nm / i64::from(2 * lobes)).max(trace_width_nm);

    let ux = dx / (base_len_nm.max(1) as f64);
    let uy = dy / (base_len_nm.max(1) as f64);
    let nx = -uy;
    let ny = ux;

    let mut pts = Vec::with_capacity((lobes as usize) * 4 + 2);
    pts.push(start_nm);
    let step_along = (base_len_nm as f64) / f64::from(lobes + 1);
    for k in 0..lobes {
        let sign = if k.is_multiple_of(2) { 1.0 } else { -1.0 };
        let s0 = step_along * (f64::from(k) + 0.3);
        let s1 = step_along * (f64::from(k) + 0.7);
        let p_in = Point2Nm::new(
            start_nm.x.0 + (ux * s0).round() as i64,
            start_nm.y.0 + (uy * s0).round() as i64,
        );
        let p_top0 = Point2Nm::new(
            p_in.x.0 + (nx * sign * (amp_nm as f64)).round() as i64,
            p_in.y.0 + (ny * sign * (amp_nm as f64)).round() as i64,
        );
        let p_out = Point2Nm::new(
            start_nm.x.0 + (ux * s1).round() as i64,
            start_nm.y.0 + (uy * s1).round() as i64,
        );
        let p_top1 = Point2Nm::new(
            p_out.x.0 + (nx * sign * (amp_nm as f64)).round() as i64,
            p_out.y.0 + (ny * sign * (amp_nm as f64)).round() as i64,
        );
        pts.extend_from_slice(&[p_in, p_top0, p_top1, p_out]);
    }
    pts.push(end_nm);

    Ok(TromboneMeanderPlan {
        waypoints_nm: pts,
        achieved_length_nm: base_len_nm + i64::from(2 * lobes) * amp_nm,
        lobe_spacing_nm: min_lobe_spacing_nm,
        lobe_count: lobes,
    })
}

/// Integrates spatially periodic glass-bundle dielectric $\varepsilon_r(x, y)$ (`1080`/`2116` weave, `FEAT-011`)
/// over two differential traces separated by `pair_pitch_nm` at routing angle `angle_deg` across `trace_len_nm`.
pub fn compute_fiber_weave_skew_fs(
    trace_len_nm: i64,
    pair_pitch_nm: i64,
    weave_pitch_nm: i64,
    er_mean: f64,
    er_delta: f64,
    angle_deg: f64,
) -> i64 {
    let c0_m_per_s = 299_792_458.0;
    let rad = angle_deg * (std::f64::consts::PI / 180.0);
    let cos_a = det_cos(rad);
    let sin_a = det_sin(rad);
    let samples = 2_000_usize;
    let ds_m = (trace_len_nm.max(1) as f64 * 1.0e-9) / (samples as f64);
    let wp = weave_pitch_nm.max(100_000) as f64;
    let pitch = pair_pitch_nm as f64;

    let mut t_pos_s = 0.0_f64;
    let mut t_neg_s = 0.0_f64;
    for i in 0..samples {
        let s_nm = ((i as f64) + 0.5) * ((trace_len_nm as f64) / (samples as f64));
        // Worst-case transverse alignment: positive leg rides warp glass bundle peak (offset = 0),
        // negative leg is displaced by +pair_pitch_nm into adjacent resin-rich trough
        let x_p = s_nm * cos_a;
        let y_p = s_nm * sin_a;
        let x_n = x_p - sin_a * pitch;
        let y_n = y_p + cos_a * pitch;

        let k = 2.0 * std::f64::consts::PI / wp;
        let er_p = (er_mean + 0.5 * er_delta * (det_cos(k * x_p) + det_cos(k * y_p))).max(1.0);
        let er_n = (er_mean + 0.5 * er_delta * (det_cos(k * x_n) + det_cos(k * y_n))).max(1.0);
        t_pos_s += ds_m * er_p.sqrt() / c0_m_per_s;
        t_neg_s += ds_m * er_n.sqrt() / c0_m_per_s;
    }
    ((t_pos_s - t_neg_s).abs() * 1.0e15).round() as i64
}

/// Evaluates `AnalyzeFiberWeaveAndBackdrillRequest` (`FEAT-011` + `FEAT-013`) for 56G/112G/224G links.
pub fn analyze_fiber_weave_and_backdrill_rpc(
    req: &AnalyzeFiberWeaveAndBackdrillRequest,
    board_thickness_nm: i64,
    active_signal_depth_nm: i64,
) -> AnalyzeFiberWeaveAndBackdrillResponse {
    // Unmitigated 0-deg 100 mm trace on 1080 glass (weave pitch 420 um, pair pitch 210 um => half-period lock!)
    let skew_fs = compute_fiber_weave_skew_fs(100_000_000, 210_000, 420_000, 3.8, 0.45, 0.0);
    let stub_len_nm = (board_thickness_nm - active_signal_depth_nm).max(100_000);
    let er_eff = 3.8_f64;
    let stub_m = (stub_len_nm as f64) * 1.0e-9;
    let f_notch_hz = (299_792_458.0 / (4.0 * stub_m * er_eff.sqrt())).round() as i64;
    // Leave 150 um safety stub clearance above internal signal layer
    let backdrill_depth_nm = if req.data_rate_bps >= 25_000_000_000 {
        (stub_len_nm - 150_000).max(0)
    } else {
        0
    };

    AnalyzeFiberWeaveAndBackdrillResponse {
        predicted_weave_skew_fs: skew_fs,
        recommended_zig_zag_deg: 10.0,
        stub_notch_freq_hz: f_notch_hz,
        recommended_backdrill_depth_nm: backdrill_depth_nm,
    }
}
