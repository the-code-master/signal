// Module: engine/crates/signal-mcad/src/lib.rs — Parametric B-Rep CAD, 2D Dogleg Sketch, Sheet-Metal & ECAD Anchors
// Invariants: Rejects non-positive extrusion depths; enforces < 50 ms live ECAD-MCAD anchor sync & exact nm math (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
#![forbid(unsafe_code)]

use signal_proto::v1::McadFeatureNode;
use thiserror::Error;

pub mod brep_sheet_anchors;
pub mod sketch_solver;
pub mod writers_step_gltf_3mf;

pub use brep_sheet_anchors::{
    GasketCompressionResult, MAX_ECAD_MCAD_SYNC_LATENCY_PS, compute_sheet_metal_bend_allowance_nm,
    eval_rational_nurbs_surface_nm, evaluate_brep_feature_tree, verify_ip_gasket_compression,
};
pub use sketch_solver::{SketchConstraint, SketchPoint, SolvedSketch, solve_sketch_2d};
pub use writers_step_gltf_3mf::{
    write_3mf_model_bytes, write_gltf2_scene_bytes, write_step_ap242_with_pmi_bytes,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum McadError {
    #[error("MCAD feature `{0}` must specify positive primary_depth_nm")]
    NonPositiveFeatureDepth(String),
    #[error("MCAD feature `{0}` cannot specify negative fillet_radius_nm")]
    InvalidFilletRadius(String),
    #[error("MCAD feature tree cannot be empty")]
    EmptyFeatureTree,
    #[error("2D sketch solver did not converge (residual {0} nm > 0.5 nm)")]
    SketchDidNotConverge(i64),
    #[error("ECAD-MCAD anchor `{anchor_id}` sync latency ({latency_ps} ps) exceeded 50 ms SLA")]
    AnchorSyncSlaExceeded { anchor_id: String, latency_ps: i64 },
}

pub fn validate_feature_node(node: &McadFeatureNode) -> Result<(), McadError> {
    if node.primary_depth_nm <= 0 {
        return Err(McadError::NonPositiveFeatureDepth(node.feature_id.clone()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::Nanometers;
    use signal_geom::{BBox3Nm, Point3Nm};
    use signal_proto::v1::{
        EcadMcadAnchor, EvaluateBRepFeatureTreeRequest, McadFeatureOpKind, Point3dNm,
    };

    #[test]
    fn solves_2d_sketch_distance_perpendicular_and_tangent_constraints() {
        let pts = vec![
            SketchPoint {
                id: 0,
                x_nm: 0,
                y_nm: 0,
                fixed: true,
            },
            SketchPoint {
                id: 1,
                x_nm: 9_200_000,
                y_nm: 800_000,
                fixed: false,
            },
            SketchPoint {
                id: 2,
                x_nm: 500_000,
                y_nm: 5_400_000,
                fixed: false,
            },
            SketchPoint {
                id: 3,
                x_nm: 4_800_000,
                y_nm: 2_300_000,
                fixed: false,
            },
        ];
        let constraints = vec![
            SketchConstraint::Horizontal { p1: 0, p2: 1 },
            SketchConstraint::Distance {
                p1: 0,
                p2: 1,
                target_nm: 10_000_000,
            },
            SketchConstraint::Perpendicular {
                l1_p1: 0,
                l1_p2: 1,
                l2_p1: 0,
                l2_p2: 2,
            },
            SketchConstraint::Distance {
                p1: 0,
                p2: 2,
                target_nm: 6_000_000,
            },
            SketchConstraint::TangentCircleLine {
                center: 3,
                radius_nm: 2_000_000,
                line_p1: 0,
                line_p2: 1,
            },
        ];
        let solved = solve_sketch_2d(&pts, &constraints, 60).expect("sketch converged");
        assert_eq!(solved.points_nm[1].x.0, 10_000_000);
        assert_eq!(solved.points_nm[1].y.0, 0);
        assert_eq!(solved.points_nm[2].x.0, 0);
        assert_eq!(solved.points_nm[2].y.0.abs(), 6_000_000);
        assert_eq!(solved.points_nm[3].y.0.abs(), 2_000_000);
    }

    #[test]
    fn evaluates_brep_sheet_metal_live_anchors_ip67_gasket_and_writers() {
        let req = EvaluateBRepFeatureTreeRequest {
            fencing: None,
            anchors: vec![EcadMcadAnchor {
                anchor_id: "ANCHOR-MH1".into(),
                pcb_entity_id: "MH1".into(),
                pcb_origin_nm: Some(Point3dNm {
                    x_nm: 25_000_000,
                    y_nm: 15_000_000,
                    z_nm: 0,
                }),
                sync_latency_ps: 12_000_000_000, // 12 ms (< 50 ms SLA)
            }],
            features: vec![
                McadFeatureNode {
                    feature_id: "F-BOSS-1".into(),
                    op_kind: McadFeatureOpKind::Extrude as i32,
                    primary_depth_nm: 8_000_000,
                    fillet_radius_nm: 500_000,
                    linked_pcb_anchor_id: "ANCHOR-MH1".into(),
                    sheet_metal_k_factor: 0.0,
                },
                McadFeatureNode {
                    feature_id: "F-FLANGE-1".into(),
                    op_kind: McadFeatureOpKind::SheetMetalFlange as i32,
                    primary_depth_nm: 20_000_000,
                    fillet_radius_nm: 2_000_000,
                    linked_pcb_anchor_id: String::new(),
                    sheet_metal_k_factor: 0.44,
                },
            ],
        };
        let pcb = BBox3Nm {
            min: Point3Nm {
                x: Nanometers(0),
                y: Nanometers(0),
                z: Nanometers(0),
            },
            max: Point3Nm {
                x: Nanometers(50_000_000),
                y: Nanometers(30_000_000),
                z: Nanometers(1_600_000),
            },
        };
        let lid_rib = BBox3Nm {
            min: Point3Nm {
                x: Nanometers(10_000_000),
                y: Nanometers(10_000_000),
                z: Nanometers(3_600_000),
            },
            max: Point3Nm {
                x: Nanometers(20_000_000),
                y: Nanometers(20_000_000),
                z: Nanometers(8_000_000),
            },
        };

        let resp =
            evaluate_brep_feature_tree(&req, Some(pcb), &[lid_rib]).expect("brep tree valid");
        assert_eq!(resp.interference_collision_count, 0);
        assert_eq!(resp.min_pcb_enclosure_clearance_nm, 2_000_000);
        assert!(resp.unfolded_flat_length_nm > 20_000_000);

        let gasket = verify_ip_gasket_compression(2_400_000, 1_850_000);
        assert!(gasket.ip67_seal_pass);

        let bounds = resp.bounds_nm.expect("bounds");
        let step_bytes = write_step_ap242_with_pmi_bytes("ENC-HOUSING", &bounds, 50);
        let gltf_bytes = write_gltf2_scene_bytes("ENC-HOUSING", &bounds);
        let mf3_bytes = write_3mf_model_bytes("ENC-HOUSING", &bounds);
        assert!(
            std::str::from_utf8(&step_bytes)
                .expect("utf8")
                .contains("FLATNESS_TOLERANCE")
        );
        assert!(
            std::str::from_utf8(&gltf_bytes)
                .expect("utf8")
                .contains("\"version\": \"2.0\"")
        );
        assert!(
            std::str::from_utf8(&mf3_bytes)
                .expect("utf8")
                .contains("<triangles>")
        );
    }
}
