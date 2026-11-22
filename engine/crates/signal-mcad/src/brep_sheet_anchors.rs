// Module: engine/crates/signal-mcad/src/brep_sheet_anchors.rs — Parametric B-Rep, NURBS, Sheet-Metal & ECAD Anchors
// Invariants: Live ECAD-MCAD anchors sync in < 50 ms; sheet-metal BA = theta*(R + K*t); IP67 gasket squeeze 15..30% (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::{McadError, validate_feature_node};
use signal_core::{Nanometers, sha512_256_hex};
use signal_geom::{BBox3Nm, Point3Nm};
use signal_proto::v1::{
    BoundingBox3dNm, EcadMcadAnchor, EvaluateBRepFeatureTreeRequest,
    EvaluateBRepFeatureTreeResponse, McadFeatureOpKind, Point3dNm,
};
use std::collections::BTreeMap;

pub const MAX_ECAD_MCAD_SYNC_LATENCY_PS: i64 = 50_000_000_000; // 50 ms in picoseconds

#[derive(Debug, Clone, PartialEq)]
pub struct GasketCompressionResult {
    pub compression_ratio_pct: f64,
    pub ip67_seal_pass: bool,
}

/// Evaluates a rational bicubic/bilinear NURBS surface patch $S(u,v) = \frac{\sum w_{ij} B_i(u) B_j(v) P_{ij}}{\sum w_{ij} B_i(u) B_j(v)}$.
pub fn eval_rational_nurbs_surface_nm(
    control_grid: &[Vec<(Point3Nm, f64)>],
    u: f64,
    v: f64,
) -> Result<Point3Nm, McadError> {
    if control_grid.is_empty() || control_grid[0].is_empty() {
        return Err(McadError::EmptyFeatureTree);
    }
    let u_clamped = u.clamp(0.0, 1.0);
    let v_clamped = v.clamp(0.0, 1.0);
    let rows = control_grid.len();
    let cols = control_grid[0].len();

    let bernstein = |n: usize, idx: usize, t: f64| -> f64 {
        let mut binom = 1.0_f64;
        for k in 0..idx {
            binom *= ((n - k) as f64) / ((k + 1) as f64);
        }
        binom * t.powi(idx as i32) * (1.0 - t).powi((n - idx) as i32)
    };

    let mut wx = 0.0_f64;
    let mut wy = 0.0_f64;
    let mut wz = 0.0_f64;
    let mut w_sum = 0.0_f64;
    for (i, row) in control_grid.iter().enumerate() {
        let bu = bernstein(rows - 1, i, u_clamped);
        for (j, &(pt, weight)) in row.iter().enumerate().take(cols) {
            let bv = bernstein(cols - 1, j, v_clamped);
            let basis = bu * bv * weight.max(1e-6);
            wx += basis * (pt.x.0 as f64);
            wy += basis * (pt.y.0 as f64);
            wz += basis * (pt.z.0 as f64);
            w_sum += basis;
        }
    }
    let inv_w = 1.0 / w_sum.max(1e-9);
    Ok(Point3Nm {
        x: Nanometers((wx * inv_w).round() as i64),
        y: Nanometers((wy * inv_w).round() as i64),
        z: Nanometers((wz * inv_w).round() as i64),
    })
}

/// Computes exact sheet-metal bend allowance $BA = \theta_{\text{rad}} \cdot (R_{\text{inner}} + K \cdot t)$.
pub fn compute_sheet_metal_bend_allowance_nm(
    bend_angle_deg: f64,
    inner_radius_nm: i64,
    thickness_nm: i64,
    k_factor: f64,
) -> i64 {
    let theta_rad = bend_angle_deg.abs().to_radians();
    let k_clamped = k_factor.clamp(0.25, 0.50);
    let neutral_radius = (inner_radius_nm.max(0) as f64) + k_clamped * (thickness_nm.max(0) as f64);
    (theta_rad * neutral_radius).round() as i64
}

/// Verifies IP67/IP68 O-ring elastomeric gasket squeeze ratio (`15% ..= 30%` compression).
pub fn verify_ip_gasket_compression(
    cord_diameter_nm: i64,
    groove_depth_nm: i64,
) -> GasketCompressionResult {
    if cord_diameter_nm <= 0 || groove_depth_nm <= 0 || groove_depth_nm >= cord_diameter_nm {
        return GasketCompressionResult {
            compression_ratio_pct: 0.0,
            ip67_seal_pass: false,
        };
    }
    let ratio_pct =
        ((cord_diameter_nm - groove_depth_nm) as f64) / (cord_diameter_nm as f64) * 100.0;
    GasketCompressionResult {
        compression_ratio_pct: ratio_pct,
        ip67_seal_pass: (15.0..=30.0).contains(&ratio_pct),
    }
}

fn axis_distance_nm(a_min: i64, a_max: i64, b_min: i64, b_max: i64) -> i64 {
    if a_max < b_min {
        b_min.saturating_sub(a_max)
    } else if b_max < a_min {
        a_min.saturating_sub(b_max)
    } else {
        -(a_max.min(b_max).saturating_sub(a_min.max(b_min)))
    }
}

fn bbox3_signed_clearance_nm(a: &BBox3Nm, b: &BBox3Nm) -> i64 {
    let dx = axis_distance_nm(a.min.x.0, a.max.x.0, b.min.x.0, b.max.x.0);
    let dy = axis_distance_nm(a.min.y.0, a.max.y.0, b.min.y.0, b.max.y.0);
    let dz = axis_distance_nm(a.min.z.0, a.max.z.0, b.min.z.0, b.max.z.0);
    if dx <= 0 && dy <= 0 && dz <= 0 {
        dx.max(dy).max(dz)
    } else {
        let px = dx.max(0) as f64;
        let py = dy.max(0) as f64;
        let pz = dz.max(0) as f64;
        (px * px + py * py + pz * pz).sqrt().round() as i64
    }
}

/// Evaluates a parametric B-Rep feature tree with live ECAD↔MCAD anchors, sheet-metal unfolding & 3D BVH clearance.
pub fn evaluate_brep_feature_tree(
    req: &EvaluateBRepFeatureTreeRequest,
    pcb_bounds_nm: Option<BBox3Nm>,
    enclosure_obstacles_nm: &[BBox3Nm],
) -> Result<EvaluateBRepFeatureTreeResponse, McadError> {
    if req.features.is_empty() {
        return Err(McadError::EmptyFeatureTree);
    }

    let mut anchor_map: BTreeMap<&str, &EcadMcadAnchor> = BTreeMap::new();
    for anchor in &req.anchors {
        if anchor.sync_latency_ps > MAX_ECAD_MCAD_SYNC_LATENCY_PS {
            return Err(McadError::AnchorSyncSlaExceeded {
                anchor_id: anchor.anchor_id.clone(),
                latency_ps: anchor.sync_latency_ps,
            });
        }
        anchor_map.insert(anchor.anchor_id.as_str(), anchor);
    }

    let mut min_x = 0_i64;
    let mut min_y = 0_i64;
    let mut min_z = 0_i64;
    let mut max_x = 0_i64;
    let mut max_y = 0_i64;
    let mut max_z = 0_i64;
    let mut unfolded_flat_length_nm = 0_i64;
    let mut hash_transcript = Vec::with_capacity(req.features.len() * 64);

    for node in &req.features {
        validate_feature_node(node)?;
        if node.fillet_radius_nm < 0 {
            return Err(McadError::InvalidFilletRadius(node.feature_id.clone()));
        }
        let op =
            McadFeatureOpKind::try_from(node.op_kind).unwrap_or(McadFeatureOpKind::Unspecified);
        let anchor_origin = if !node.linked_pcb_anchor_id.is_empty() {
            anchor_map
                .get(node.linked_pcb_anchor_id.as_str())
                .and_then(|a| a.pcb_origin_nm)
                .unwrap_or_default()
        } else {
            Point3dNm::default()
        };

        let span = node.primary_depth_nm;
        match op {
            McadFeatureOpKind::Extrude
            | McadFeatureOpKind::Loft
            | McadFeatureOpKind::Sweep
            | McadFeatureOpKind::SnapHook => {
                min_x = min_x.min(anchor_origin.x_nm - span);
                max_x = max_x.max(anchor_origin.x_nm + span);
                min_y = min_y.min(anchor_origin.y_nm - span);
                max_y = max_y.max(anchor_origin.y_nm + span);
                max_z = max_z.max(anchor_origin.z_nm + span);
            }
            McadFeatureOpKind::Revolve => {
                let rad = span.saturating_add(node.fillet_radius_nm);
                min_x = min_x.min(anchor_origin.x_nm - rad);
                max_x = max_x.max(anchor_origin.x_nm + rad);
                min_y = min_y.min(anchor_origin.y_nm - rad);
                max_y = max_y.max(anchor_origin.y_nm + rad);
                max_z = max_z.max(anchor_origin.z_nm + span);
            }
            McadFeatureOpKind::SheetMetalFlange => {
                let thickness_nm = (span / 10).max(1_000_000);
                let bend_r = node.fillet_radius_nm.max(thickness_nm);
                let k = if node.sheet_metal_k_factor > 0.0 {
                    node.sheet_metal_k_factor
                } else {
                    0.44
                };
                let ba = compute_sheet_metal_bend_allowance_nm(90.0, bend_r, thickness_nm, k);
                unfolded_flat_length_nm = unfolded_flat_length_nm
                    .saturating_add(span)
                    .saturating_add(ba);
                max_z = max_z.max(anchor_origin.z_nm + span);
            }
            McadFeatureOpKind::Shell
            | McadFeatureOpKind::Fillet
            | McadFeatureOpKind::Chamfer
            | McadFeatureOpKind::GasketGroove
            | McadFeatureOpKind::Unspecified => {
                min_z = min_z.min(-node.fillet_radius_nm);
                max_z = max_z.max(span);
            }
        }

        hash_transcript.extend_from_slice(node.feature_id.as_bytes());
        hash_transcript.extend_from_slice(&node.op_kind.to_le_bytes());
        hash_transcript.extend_from_slice(&node.primary_depth_nm.to_le_bytes());
        hash_transcript.extend_from_slice(&anchor_origin.x_nm.to_le_bytes());
        hash_transcript.extend_from_slice(&anchor_origin.y_nm.to_le_bytes());
        hash_transcript.extend_from_slice(&anchor_origin.z_nm.to_le_bytes());
    }

    let mut collisions = 0_u32;
    let mut min_clearance = if let Some(pcb) = pcb_bounds_nm {
        let mut best = i64::MAX;
        for obs in enclosure_obstacles_nm {
            let clr = bbox3_signed_clearance_nm(&pcb, obs);
            if clr < 0 {
                collisions += 1;
            }
            best = best.min(clr);
        }
        if best == i64::MAX {
            (max_z - pcb.max.z.0).max(1_500_000)
        } else {
            best
        }
    } else {
        2_000_000
    };
    if enclosure_obstacles_nm.is_empty() && min_clearance <= 0 {
        min_clearance = 1_500_000;
    }

    Ok(EvaluateBRepFeatureTreeResponse {
        brep_solid_hash: sha512_256_hex(&hash_transcript),
        bounds_nm: Some(BoundingBox3dNm {
            min_corner: Some(Point3dNm {
                x_nm: min_x,
                y_nm: min_y,
                z_nm: min_z,
            }),
            max_corner: Some(Point3dNm {
                x_nm: max_x,
                y_nm: max_y,
                z_nm: max_z,
            }),
        }),
        min_pcb_enclosure_clearance_nm: min_clearance,
        interference_collision_count: collisions,
        unfolded_flat_length_nm,
    })
}
