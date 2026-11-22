// Module: engine/crates/signal-harness/src/nailboard.rs — 3D Spline Harness Router, 1:1 Nailboard & WHMA-A-620 Verifier (FEAT-062)
// Invariants: Flattens 3D spline arc lengths 1:1 onto 2D formboard; verifies DeltaV = 2*I*L*rho/A & bundle ampacity (FEAT-062).
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::HarnessError;
use signal_geom::{Point2Nm, Point3Nm, sample_catmull_rom_spline};
use signal_proto::v1::{GenerateHarnessNailboardRequest, GenerateHarnessNailboardResponse};

const COPPER_RESISTIVITY_OHM_M: f64 = 1.724e-8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireCutStripCrimpRow {
    pub wire_id: String,
    pub awg_gauge: u8,
    pub cut_length_nm: i64,
    pub strip_left_nm: i64,
    pub strip_right_nm: i64,
    pub terminal_crimp_mpn: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlattenedNailboardPlan {
    pub formboard_waypoints_2d_nm: Vec<Point2Nm>,
    pub cut_strip_crimp_schedule: Vec<WireCutStripCrimpRow>,
    pub response: GenerateHarnessNailboardResponse,
}

/// Returns nominal copper cross-sectional area ($m^2$) and single-wire free-air ampacity ($A$) per IPC/WHMA-A-620.
pub fn awg_area_m2_and_ampacity(awg: u8) -> Result<(f64, f64), HarnessError> {
    match awg {
        12 => Ok((3.31e-6, 25.0)),
        14 => Ok((2.08e-6, 18.0)),
        16 => Ok((1.31e-6, 13.0)),
        18 => Ok((0.823e-6, 10.0)),
        20 => Ok((0.518e-6, 7.5)),
        22 => Ok((0.326e-6, 5.0)),
        24 => Ok((0.205e-6, 3.5)),
        26 => Ok((0.129e-6, 2.2)),
        28 => Ok((0.081e-6, 1.4)),
        _ => Err(HarnessError::UnsupportedAwgGauge(awg)),
    }
}

fn bundle_derating_factor(wire_count: usize) -> f64 {
    match wire_count {
        0 | 1 => 1.0,
        2..=3 => 0.80,
        4..=6 => 0.70,
        7..=15 => 0.60,
        _ => 0.50,
    }
}

/// Routes a 3D wire-harness spline, flattens 1:1 onto a 2D formboard, and evaluates IPC/WHMA-A-620 voltage-drop & ampacity (`FEAT-062`).
pub fn generate_harness_nailboard_plan(
    req: &GenerateHarnessNailboardRequest,
    waypoints_3d_nm: &[Point3Nm],
    awg_gauge: u8,
    conductor_count: usize,
    max_allowed_drop_uv: i64,
) -> Result<FlattenedNailboardPlan, HarnessError> {
    if waypoints_3d_nm.len() < 2 || conductor_count == 0 {
        return Err(HarnessError::InsufficientHarnessWaypoints);
    }
    let (area_m2, single_ampacity_a) = awg_area_m2_and_ampacity(awg_gauge)?;

    let xy_pts: Vec<Point2Nm> = waypoints_3d_nm
        .iter()
        .map(|p| Point2Nm::new(p.x.0, p.y.0))
        .collect();
    let spline_xy = sample_catmull_rom_spline(&xy_pts, 8)
        .map_err(|e| HarnessError::SplineRoutingError(e.to_string()))?;

    let z_total_span = (waypoints_3d_nm[waypoints_3d_nm.len() - 1].z.0 - waypoints_3d_nm[0].z.0)
        as f64
        / ((spline_xy.len() - 1).max(1) as f64);

    let mut formboard_pts = Vec::with_capacity(spline_xy.len());
    formboard_pts.push(Point2Nm::new(0, 0));
    let mut cumulative_arc_nm = 0.0_f64;
    for pair in spline_xy.windows(2) {
        let dx = (pair[1].x.0 - pair[0].x.0) as f64;
        let dy = (pair[1].y.0 - pair[0].y.0) as f64;
        let ds_3d = (dx * dx + dy * dy + z_total_span * z_total_span).sqrt();
        cumulative_arc_nm += ds_3d;
        formboard_pts.push(Point2Nm::new(cumulative_arc_nm.round() as i64, 0));
    }

    // Add 15 mm service-loop + crimp termination allowance per IPC/WHMA-A-620.
    let service_allowance_nm = 15_000_000_i64;
    let cut_len_nm = (cumulative_arc_nm.round() as i64).saturating_add(service_allowance_nm);
    let length_m = (cut_len_nm as f64) * 1e-9;

    let total_current_a = (req.bundle_current_na.max(0) as f64) * 1e-9;
    let per_wire_current_a = total_current_a / (conductor_count as f64);

    // Round-trip loop voltage drop DeltaV = 2 * I_wire * L * rho_Cu / A_awg
    let drop_v = 2.0 * per_wire_current_a * length_m * COPPER_RESISTIVITY_OHM_M / area_m2;
    let worst_drop_uv = (drop_v * 1e6).round() as i64;

    let derated_ampacity_a = single_ampacity_a * bundle_derating_factor(conductor_count);
    let whma_pass =
        per_wire_current_a <= derated_ampacity_a && worst_drop_uv <= max_allowed_drop_uv;

    let mut schedule = Vec::with_capacity(conductor_count);
    for idx in 0..conductor_count {
        schedule.push(WireCutStripCrimpRow {
            wire_id: format!("{}-W{:02}", req.harness_bundle_id, idx + 1),
            awg_gauge,
            cut_length_nm: cut_len_nm,
            strip_left_nm: 3_500_000,
            strip_right_nm: 3_500_000,
            terminal_crimp_mpn: format!("MOLEX-CRIMP-{awg_gauge}AWG"),
        });
    }

    Ok(FlattenedNailboardPlan {
        formboard_waypoints_2d_nm: formboard_pts,
        cut_strip_crimp_schedule: schedule,
        response: GenerateHarnessNailboardResponse {
            total_cut_length_nm: cut_len_nm,
            worst_voltage_drop_uv: worst_drop_uv,
            whma_a620_ampacity_pass: whma_pass,
        },
    })
}
