// Module: engine/crates/signal-cam/src/lib.rs — Subtractive CNC CAM & Direct-B-Rep Additive 3D Print Slicer
// Invariants: Rejects zero/negative cutter diameters or out-of-range layer heights; stages G-code/CLI in RAM (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
#![forbid(unsafe_code)]

use signal_proto::v1::{GenerateCncToolpathRequest, SliceBRepFor3dPrintRequest};
use thiserror::Error;

pub mod cnc;
pub mod slicer;

pub use cnc::{
    CncToolAssemblySpec, CncWaypoint5Axis, DexelSimulationReport, PostProcessorDialect,
    compute_5axis_tool_vector, compute_scallop_stepover_nm, emit_post_processed_gcode_bytes,
    generate_cnc_toolpath, optimize_feedrate_nm_per_min, plan_adaptive_trochoidal_pocket,
    plan_pcb_isolation_or_rest_passes, simulate_dexel_material_removal,
};
pub use slicer::{
    AnalyticBRepPrimitive, AnalyticLayerContour, SlicedLayerPlan, emit_additive_machine_bytes,
    nest_parts_on_build_plate_2d, sample_stress_guided_gyroid_infill, slice_analytic_brep_at_z,
    slice_brep_for_3d_print,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CamError {
    #[error("CNC tool diameter ({0} nm) must be strictly positive")]
    InvalidToolDiameter(i64),
    #[error("3D print layer height ({0} nm) must be between 5_000 nm and 1_000_000 nm")]
    InvalidLayerHeight(i64),
    #[error("pocket span is smaller than cutter diameter ({0} nm)")]
    PocketSmallerThanCutter(i64),
    #[error("unsupported CNC G-code post-processor profile `{0}`")]
    UnsupportedPostProcessor(String),
    #[error("part width ({0} nm) exceeds additive build-plate width")]
    PartExceedsBuildPlate(i64),
}

pub fn validate_cnc_request(req: &GenerateCncToolpathRequest) -> Result<(), CamError> {
    if req.tool_diameter_nm <= 0 {
        return Err(CamError::InvalidToolDiameter(req.tool_diameter_nm));
    }
    Ok(())
}

pub fn validate_print3d_slice_request(req: &SliceBRepFor3dPrintRequest) -> Result<(), CamError> {
    if !(5_000..=1_000_000).contains(&req.layer_height_nm) {
        return Err(CamError::InvalidLayerHeight(req.layer_height_nm));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::Nanometers;
    use signal_geom::{BBox3Nm, Point2Nm, Point3Nm};
    use signal_proto::v1::{
        AdditiveProcessKind, CncAxisStrategy, GenerateCncToolpathRequest, Print3dJob,
        SliceBRepFor3dPrintRequest,
    };

    #[test]
    fn generates_5axis_cnc_toolpaths_across_post_processors_and_detects_holder_gouge() {
        for profile in [
            "FANUC_31I",
            "SIEMENS_840D",
            "HAAS_NGC",
            "HEIDENHAIN_TNC640",
            "GRBL_LINUXCNC",
        ] {
            let (resp, gcode) = generate_cnc_toolpath(&GenerateCncToolpathRequest {
                fencing: None,
                brep_solid_hash: "deadbeef01234567".into(),
                strategy: CncAxisStrategy::Simultaneous5axis as i32,
                tool_diameter_nm: 6_000_000,
                post_processor_profile: profile.into(),
                operations: vec![],
            })
            .expect("cnc toolpath ok");
            assert!(resp.holder_gouge_free);
            assert_eq!(resp.gcode_sftp_staging_digest.len(), 64);
            assert!(!gcode.is_empty());
        }

        // Verify Dexel collision flags short flute / clamp gouge!
        let stock = BBox3Nm {
            min: Point3Nm {
                x: Nanometers(0),
                y: Nanometers(0),
                z: Nanometers(-20_000_000),
            },
            max: Point3Nm {
                x: Nanometers(20_000_000),
                y: Nanometers(20_000_000),
                z: Nanometers(0),
            },
        };
        let gouge = simulate_dexel_material_removal(
            stock,
            16,
            CncToolAssemblySpec {
                cutter_diameter_nm: 4_000_000,
                flute_length_nm: 5_000_000, // shorter than 12 mm cut!
                holder_diameter_nm: 16_000_000,
                overall_stickout_nm: 10_000_000,
            },
            &[CncWaypoint5Axis {
                tip_nm: Point3Nm {
                    x: Nanometers(10_000_000),
                    y: Nanometers(10_000_000),
                    z: Nanometers(-12_000_000),
                },
                axis_unit: [0.0, 0.0, 1.0],
                feedrate_nm_per_min: 500_000_000,
            }],
            &[],
        );
        assert!(!gouge.holder_gouge_free);
    }

    #[test]
    fn slices_direct_brep_with_true_g2_arcs_pcb_pause_and_stress_gyroid() {
        let prims = vec![AnalyticBRepPrimitive::CylinderSolid {
            center_xy_nm: Point2Nm::new(25_000_000, 25_000_000),
            radius_nm: 12_000_000,
            z_min_nm: 0,
            z_max_nm: 4_000_000,
            overhang_cone_deg: 50.0,
        }];
        let (resp, gcode) = slice_brep_for_3d_print(
            &SliceBRepFor3dPrintRequest {
                fencing: None,
                brep_solid_hash: "cafebabe99887766".into(),
                process: AdditiveProcessKind::FdmFff as i32,
                layer_height_nm: 200_000,
                job_spec: Some(Print3dJob {
                    job_id: "JOB-ENC-FDM".into(),
                    process: AdditiveProcessKind::FdmFff as i32,
                    layer_height_nm: 200_000,
                    gyroid_infill_density_pct: 25.0,
                    tree_supports_enabled: true,
                    thermal_warp_precompensation: true,
                }),
            },
            &prims,
            Some(2_100_000),
        )
        .expect("direct brep slice");
        assert_eq!(resp.total_layer_count, 20);
        assert!(resp.report.expect("report").direct_brep_analytic_contours);
        let text = std::str::from_utf8(&gcode).expect("utf8 gcode");
        assert!(text.contains("G2 I-"));
        assert!(text.contains("M600 ; PAUSE FOR PICK-AND-PLACE EMBEDDED PCB INSERTION"));

        let nested = nest_parts_on_build_plate_2d(
            &[
                Point2Nm::new(40_000_000, 30_000_000),
                Point2Nm::new(40_000_000, 30_000_000),
            ],
            80_000_000,
            5_000_000,
        )
        .expect("nested");
        assert_eq!(nested.len(), 2);
        assert!(nested[1].min.y.0 > nested[0].min.y.0);

        let mut high_stress_hits = 0;
        let mut low_stress_hits = 0;
        for step in 0..32 {
            let x = step * 250_000;
            if sample_stress_guided_gyroid_infill(x, x / 2, 500_000, 4_000_000, 20.0, 120.0) {
                high_stress_hits += 1;
            }
            if sample_stress_guided_gyroid_infill(x, x / 2, 500_000, 4_000_000, 20.0, 0.0) {
                low_stress_hits += 1;
            }
        }
        assert!(high_stress_hits >= low_stress_hits);
    }
}
