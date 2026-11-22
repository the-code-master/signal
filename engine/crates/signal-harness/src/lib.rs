// Module: engine/crates/signal-harness/src/lib.rs — Multi-Board 3D Assembly, Rigid-Flex & Wire-Harness Suite
// Invariants: Pure-Rust exact nm 3D mates, IPC-2223 copper bend strain, and IPC/WHMA-A-620 harness verification (19-*).
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/19-cad-cam-3d-printing-and-plm-operations.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod assembly_3d;
pub mod flex_kinematics;
pub mod nailboard;

pub use assembly_3d::{
    AssemblyMateConstraint, ConnectorMatePinPair, MultiBoardAssemblyReport, SubBoardInstance,
    slice_assembly_z_clipping_plane, solve_multiboard_mates, trace_multiboard_connector_nets,
};
pub use flex_kinematics::{
    FoldedSegmentPose, IPC2223_DYNAMIC_FLEX_MAX_STRAIN_PCT,
    IPC2223_STATIC_RA_COPPER_MAX_STRAIN_PCT, compute_rigid_flex_fold_pose,
    evaluate_multiboard_and_flex, verify_staggered_flex_tear_stops,
};
pub use nailboard::{
    FlattenedNailboardPlan, WireCutStripCrimpRow, awg_area_m2_and_ampacity,
    generate_harness_nailboard_plan,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HarnessError {
    #[error("multi-board assembly must contain at least one board")]
    EmptyAssembly,
    #[error("unknown board `{0}` referenced in assembly mate constraint")]
    UnknownBoard(String),
    #[error("unrouted connector pin `{0}` in board-to-board interface")]
    UnroutedConnectorPin(u32),
    #[error("invalid rigid-flex bend radius or thickness ({0} nm)")]
    InvalidFlexBendGeometry(i64),
    #[error("unsupported wire gauge `{0}` AWG")]
    UnsupportedAwgGauge(u8),
    #[error("harness bundle requires >= 2 3D waypoints and >= 1 conductor")]
    InsufficientHarnessWaypoints,
    #[error("3D spline routing failed: {0}")]
    SplineRoutingError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::Nanometers;
    use signal_geom::Point3Nm;
    use signal_proto::v1::{EvaluateMultiBoardAndFlexRequest, GenerateHarnessNailboardRequest};

    #[test]
    fn solves_multiboard_mezzanine_mates_clipping_plane_and_net_continuity() {
        let boards = vec![
            SubBoardInstance {
                board_id: "MAIN-PCB".into(),
                local_size_nm: Point3Nm {
                    x: Nanometers(80_000_000),
                    y: Nanometers(60_000_000),
                    z: Nanometers(1_600_000),
                },
                origin_nm: Point3Nm {
                    x: Nanometers(0),
                    y: Nanometers(0),
                    z: Nanometers(0),
                },
            },
            SubBoardInstance {
                board_id: "MEZZ-RF".into(),
                local_size_nm: Point3Nm {
                    x: Nanometers(30_000_000),
                    y: Nanometers(20_000_000),
                    z: Nanometers(1_200_000),
                },
                origin_nm: Point3Nm {
                    x: Nanometers(0),
                    y: Nanometers(0),
                    z: Nanometers(0),
                },
            },
        ];
        let mates = vec![
            AssemblyMateConstraint::CoplanarZ {
                target_board_id: "MEZZ-RF".into(),
                reference_board_id: "MAIN-PCB".into(),
                standoff_z_nm: 5_000_000,
            },
            AssemblyMateConstraint::CoaxialXY {
                target_board_id: "MEZZ-RF".into(),
                center_x_nm: 40_000_000,
                center_y_nm: 30_000_000,
            },
        ];
        let report = solve_multiboard_mates(&boards, &mates, &[]).expect("mates solved");
        assert_eq!(report.board_to_board_collision_count, 0);
        let mezz_box = report.solved_board_bounds.get("MEZZ-RF").expect("mezz");
        assert_eq!(mezz_box.min.z.0, 6_600_000);

        let clipped = slice_assembly_z_clipping_plane(&report.solved_board_bounds, 7_000_000);
        assert_eq!(clipped.len(), 1);
        assert_eq!(clipped[0].0, "MEZZ-RF");

        let nets = trace_multiboard_connector_nets(&[ConnectorMatePinPair {
            from_board: "MAIN-PCB".into(),
            from_net: "SPI0_MOSI".into(),
            to_board: "MEZZ-RF".into(),
            to_net: "RF_SPI_SDI".into(),
            pin_number: 14,
        }])
        .expect("traced");
        assert_eq!(nets.get("MAIN-PCB:SPI0_MOSI").expect("net").len(), 1);
    }

    #[test]
    fn verifies_ipc2223_rigid_flex_strain_and_staggered_tear_stops() {
        let pose = compute_rigid_flex_fold_pose(2_500_000, 150_000, 90.0, Some(3_000_000))
            .expect("fold pose");
        assert!(pose.neutral_arc_length_nm > 4_000_000);

        let ok_static = evaluate_multiboard_and_flex(
            &EvaluateMultiBoardAndFlexRequest {
                project_id: "proj-wearable".into(),
                flex_bend_radius_nm: 2_000_000,
                copper_thickness_nm: 18_000, // 0.5 oz RA copper -> 0.45% strain
                fold_angle_deg: 90.0,
            },
            false,
            0,
        )
        .expect("static flex");
        assert!(ok_static.strain_within_ipc2223_limit);

        let fail_dynamic = evaluate_multiboard_and_flex(
            &EvaluateMultiBoardAndFlexRequest {
                project_id: "proj-hinge".into(),
                flex_bend_radius_nm: 1_500_000,
                copper_thickness_nm: 35_000, // 1.17% > 0.30% dynamic limit!
                fold_angle_deg: 90.0,
            },
            true,
            0,
        )
        .expect("dynamic flex");
        assert!(!fail_dynamic.strain_within_ipc2223_limit);
        assert!(verify_staggered_flex_tear_stops(
            &[0, 300_000],
            &[150_000, 450_000],
            100_000,
            true
        ));
    }

    #[test]
    fn generates_1to1_nailboard_and_flags_whma_a620_overcurrent() {
        let waypoints = [
            Point3Nm {
                x: Nanometers(0),
                y: Nanometers(0),
                z: Nanometers(0),
            },
            Point3Nm {
                x: Nanometers(150_000_000),
                y: Nanometers(50_000_000),
                z: Nanometers(30_000_000),
            },
            Point3Nm {
                x: Nanometers(300_000_000),
                y: Nanometers(0),
                z: Nanometers(60_000_000),
            },
        ];
        let pass_plan = generate_harness_nailboard_plan(
            &GenerateHarnessNailboardRequest {
                project_id: "proj-ev".into(),
                harness_bundle_id: "HARN-LV-01".into(),
                bundle_current_na: 4_000_000_000, // 4 A across 2x 20 AWG wires (2 A/wire <= 6 A derated)
            },
            &waypoints,
            20,
            2,
            250_000, // 250 mV max drop
        )
        .expect("nailboard ok");
        assert!(pass_plan.response.whma_a620_ampacity_pass);
        assert!(pass_plan.response.total_cut_length_nm > 315_000_000);
        assert_eq!(pass_plan.cut_strip_crimp_schedule.len(), 2);

        let overload_plan = generate_harness_nailboard_plan(
            &GenerateHarnessNailboardRequest {
                project_id: "proj-ev".into(),
                harness_bundle_id: "HARN-OVER".into(),
                bundle_current_na: 30_000_000_000, // 30 A across 2x 24 AWG -> fails ampacity & drop!
            },
            &waypoints,
            24,
            2,
            250_000,
        )
        .expect("nailboard evaluated");
        assert!(!overload_plan.response.whma_a620_ampacity_pass);
    }
}
