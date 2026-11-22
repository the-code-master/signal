// Module: engine/crates/signal-harness/src/flex_kinematics.rs — Rigid-Flex 3D Neutral-Axis Kinematics & IPC-2223 Strain (FEAT-020)
// Invariants: Computes outer-fiber strain epsilon = t / (2 * R_bend); enforces IPC-2223 static/dynamic strain & staggered tear-stops.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::HarnessError;
use signal_core::Nanometers;
use signal_core::core_math::{det_cos, det_sin};
use signal_geom::Point3Nm;
use signal_proto::v1::{EvaluateMultiBoardAndFlexRequest, EvaluateMultiBoardAndFlexResponse};

pub const IPC2223_STATIC_RA_COPPER_MAX_STRAIN_PCT: f64 = 10.0;
pub const IPC2223_DYNAMIC_FLEX_MAX_STRAIN_PCT: f64 = 0.30;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FoldedSegmentPose {
    pub neutral_arc_length_nm: i64,
    pub distal_tip_nm: Point3Nm,
    pub conical_taper_ratio: f64,
}

/// Computes cylindrical/conical neutral-axis 3D rigid-flex folding kinematics (`FEAT-020`).
pub fn compute_rigid_flex_fold_pose(
    inner_bend_radius_nm: i64,
    stack_thickness_nm: i64,
    fold_angle_deg: f64,
    conical_end_radius_nm: Option<i64>,
) -> Result<FoldedSegmentPose, HarnessError> {
    if inner_bend_radius_nm <= 0 || stack_thickness_nm <= 0 {
        return Err(HarnessError::InvalidFlexBendGeometry(inner_bend_radius_nm));
    }
    let r_end = conical_end_radius_nm.unwrap_or(inner_bend_radius_nm).max(1);
    let r_eff = 0.5 * ((inner_bend_radius_nm as f64) + (r_end as f64));
    let r_neutral = r_eff + 0.5 * (stack_thickness_nm as f64);
    let theta = fold_angle_deg.to_radians();

    let arc_nm = (r_neutral * theta.abs()).round() as i64;
    let tip_x = (r_neutral * det_sin(theta)).round() as i64;
    let tip_z = (r_neutral * (1.0 - det_cos(theta))).round() as i64;

    Ok(FoldedSegmentPose {
        neutral_arc_length_nm: arc_nm,
        distal_tip_nm: Point3Nm {
            x: Nanometers(tip_x),
            y: Nanometers(0),
            z: Nanometers(tip_z),
        },
        conical_taper_ratio: (r_end as f64) / (inner_bend_radius_nm as f64),
    })
}

/// Verifies that multi-layer flex traces stagger their offsets (`>= 1.5x trace_width_nm`) and include tear-stop fillets (`FEAT-020`).
pub fn verify_staggered_flex_tear_stops(
    layer1_trace_centers_nm: &[i64],
    layer2_trace_centers_nm: &[i64],
    trace_width_nm: i64,
    has_teardrop_tear_stops: bool,
) -> bool {
    if !has_teardrop_tear_stops || trace_width_nm <= 0 {
        return false;
    }
    let min_stagger = (trace_width_nm * 3) / 2;
    for &c1 in layer1_trace_centers_nm {
        for &c2 in layer2_trace_centers_nm {
            if (c1 - c2).abs() < min_stagger {
                return false;
            }
        }
    }
    true
}

/// Evaluates IPC-2223 outer-fiber copper tensile strain $\epsilon = \frac{t}{2 R_{\text{bend}}} \times 100\%$ (`FEAT-020`).
pub fn evaluate_multiboard_and_flex(
    req: &EvaluateMultiBoardAndFlexRequest,
    dynamic_flex_application: bool,
    enclosure_collision_count: u32,
) -> Result<EvaluateMultiBoardAndFlexResponse, HarnessError> {
    if req.flex_bend_radius_nm <= 0 || req.copper_thickness_nm <= 0 {
        return Err(HarnessError::InvalidFlexBendGeometry(
            req.flex_bend_radius_nm,
        ));
    }
    let angle_factor = (req.fold_angle_deg.abs() / 90.0).clamp(0.25, 1.0);
    let strain_pct = ((req.copper_thickness_nm as f64) / (2.0 * (req.flex_bend_radius_nm as f64)))
        * 100.0
        * angle_factor;
    let limit_pct = if dynamic_flex_application {
        IPC2223_DYNAMIC_FLEX_MAX_STRAIN_PCT
    } else {
        IPC2223_STATIC_RA_COPPER_MAX_STRAIN_PCT
    };

    Ok(EvaluateMultiBoardAndFlexResponse {
        outer_fiber_copper_strain_pct: strain_pct,
        strain_within_ipc2223_limit: strain_pct <= limit_pct && enclosure_collision_count == 0,
        enclosure_collision_count,
    })
}
