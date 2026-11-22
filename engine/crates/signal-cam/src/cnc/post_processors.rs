// Module: engine/crates/signal-cam/src/cnc/post_processors.rs — In-Memory Multi-Controller G-Code Post-Processors
// Invariants: Emits deterministic G-code strictly in RAM with SHA-256 digest for Step-Up 2FA SFTP egress (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::cnc::toolpath_dexel::{
    CncToolAssemblySpec, CncWaypoint5Axis, optimize_feedrate_nm_per_min,
    plan_adaptive_trochoidal_pocket, simulate_dexel_material_removal,
};
use crate::{CamError, validate_cnc_request};
use signal_core::{Nanometers, sha512_256_hex};
use signal_geom::{BBox2Nm, BBox3Nm, Point2Nm, Point3Nm};
use signal_proto::v1::{CncAxisStrategy, GenerateCncToolpathRequest, GenerateCncToolpathResponse};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostProcessorDialect {
    Fanuc31i,
    Siemens840d,
    HaasNgc,
    HeidenhainTnc640,
    GrblLinuxCnc,
}

impl PostProcessorDialect {
    pub fn parse_profile(profile: &str) -> Result<Self, CamError> {
        match profile.trim().to_ascii_uppercase().as_str() {
            "" | "FANUC_31I" | "FANUC" => Ok(Self::Fanuc31i),
            "SIEMENS_840D" => Ok(Self::Siemens840d),
            "HAAS_NGC" => Ok(Self::HaasNgc),
            "HEIDENHAIN_TNC640" => Ok(Self::HeidenhainTnc640),
            "GRBL_LINUXCNC" | "GRBL" => Ok(Self::GrblLinuxCnc),
            other => Err(CamError::UnsupportedPostProcessor(other.to_string())),
        }
    }
}

/// Formats 3-axis / 5-axis CNC waypoints into controller-native in-memory G-code bytes (`Vec<u8>`).
pub fn emit_post_processed_gcode_bytes(
    dialect: PostProcessorDialect,
    spindle_rpm: i64,
    five_axis_tcp: bool,
    waypoints: &[CncWaypoint5Axis],
) -> Vec<u8> {
    let mut out = String::with_capacity(waypoints.len() * 64 + 256);
    match dialect {
        PostProcessorDialect::Fanuc31i => {
            out.push_str("%\nO1001 (SIGNAL CAM FANUC 31I)\nG90 G94 G17 G49 G40 G80 G21\n");
            out.push_str(&format!("T1 M06\nS{} M03\n", spindle_rpm.max(1000)));
            if five_axis_tcp {
                out.push_str("G43.4 H01 (FANUC RTCP 5-AXIS)\n");
            }
        }
        PostProcessorDialect::Siemens840d => {
            out.push_str("; SIGNAL CAM SIEMENS SINUMERIK 840D\nG90 G94 G710\nCYCLE832(0.01, 1)\n");
            if five_axis_tcp {
                out.push_str("TRAORI\n");
            }
            out.push_str(&format!("T1 D1\nM06\nS{} M03\n", spindle_rpm.max(1000)));
        }
        PostProcessorDialect::HaasNgc => {
            out.push_str("%\nO02001 (SIGNAL CAM HAAS NGC)\nG90 G94 G17 G21\nG187 P2\n");
            if five_axis_tcp {
                out.push_str("G234 H01 (HAAS TCPC)\n");
            }
            out.push_str(&format!("T1 M06\nS{} M03\n", spindle_rpm.max(1000)));
        }
        PostProcessorDialect::HeidenhainTnc640 => {
            out.push_str("0 BEGIN PGM SIGNAL_CAM MM\n1 TOOL CALL 1 Z S");
            out.push_str(&format!("{}\n", spindle_rpm.max(1000)));
            if five_axis_tcp {
                out.push_str("2 FUNCTION TCPM F TCP AXIS POS PATHCTRL AXIS\n");
            }
        }
        PostProcessorDialect::GrblLinuxCnc => {
            out.push_str("(SIGNAL CAM GRBL / LINUXCNC)\nG21 G90 G94 G17\n");
            out.push_str(&format!("M3 S{}\n", spindle_rpm.max(1000)));
        }
    }

    for wp in waypoints {
        let x_mm = (wp.tip_nm.x.0 as f64) * 1e-6;
        let y_mm = (wp.tip_nm.y.0 as f64) * 1e-6;
        let z_mm = (wp.tip_nm.z.0 as f64) * 1e-6;
        let f_mm_min = ((wp.feedrate_nm_per_min as f64) * 1e-6).max(10.0);
        if five_axis_tcp {
            out.push_str(&format!(
                "G01 X{x_mm:.4} Y{y_mm:.4} Z{z_mm:.4} I{:.5} J{:.5} K{:.5} F{f_mm_min:.1}\n",
                wp.axis_unit[0], wp.axis_unit[1], wp.axis_unit[2]
            ));
        } else {
            out.push_str(&format!(
                "G01 X{x_mm:.4} Y{y_mm:.4} Z{z_mm:.4} F{f_mm_min:.1}\n"
            ));
        }
    }

    match dialect {
        PostProcessorDialect::HeidenhainTnc640 => out.push_str("999 END PGM SIGNAL_CAM MM\n"),
        PostProcessorDialect::Siemens840d => out.push_str("TRAFOOF\nM05\nM30\n"),
        _ => out.push_str("M05\nM30\n%\n"),
    }
    out.into_bytes()
}

/// Executes full CNC toolpath generation, Dexel gouge verification, and G-code staging (`GenerateCncToolpath`).
pub fn generate_cnc_toolpath(
    req: &GenerateCncToolpathRequest,
) -> Result<(GenerateCncToolpathResponse, Vec<u8>), CamError> {
    validate_cnc_request(req)?;
    let dialect = PostProcessorDialect::parse_profile(&req.post_processor_profile)?;
    let strategy =
        CncAxisStrategy::try_from(req.strategy).unwrap_or(CncAxisStrategy::Adaptive3axis);
    let five_axis = matches!(
        strategy,
        CncAxisStrategy::Indexed3plus2 | CncAxisStrategy::Simultaneous5axis
    );

    let first_op = req.operations.first();
    let rpm = first_op.map(|o| o.spindle_rpm).unwrap_or(18_000).max(1_000);
    let stepover_nm = first_op
        .map(|o| o.stepover_nm)
        .filter(|&s| s > 0)
        .unwrap_or(req.tool_diameter_nm / 8);
    let feed_nm_min = first_op
        .map(|o| o.feedrate_nm_per_min)
        .filter(|&f| f > 0)
        .unwrap_or_else(|| {
            optimize_feedrate_nm_per_min(rpm, 3, 40_000, req.tool_diameter_nm, stepover_nm)
        });

    let pocket = BBox2Nm {
        min: Point2Nm::new(0, 0),
        max: Point2Nm::new(req.tool_diameter_nm * 8, req.tool_diameter_nm * 6),
    };
    let lead_lean = if five_axis { Some((8.0, 4.0)) } else { None };
    let waypoints = plan_adaptive_trochoidal_pocket(
        pocket,
        -req.tool_diameter_nm,
        req.tool_diameter_nm,
        stepover_nm,
        feed_nm_min,
        lead_lean,
    )?;

    let stock = BBox3Nm {
        min: Point3Nm {
            x: Nanometers(0),
            y: Nanometers(0),
            z: Nanometers(-req.tool_diameter_nm * 3),
        },
        max: Point3Nm {
            x: Nanometers(req.tool_diameter_nm * 8),
            y: Nanometers(req.tool_diameter_nm * 6),
            z: Nanometers(0),
        },
    };
    let dexel = simulate_dexel_material_removal(
        stock,
        24,
        CncToolAssemblySpec {
            cutter_diameter_nm: req.tool_diameter_nm,
            flute_length_nm: req.tool_diameter_nm * 3,
            holder_diameter_nm: req.tool_diameter_nm * 4,
            overall_stickout_nm: req.tool_diameter_nm * 5,
        },
        &waypoints,
        &[],
    );

    let mut total_dist_nm = 0.0_f64;
    for pair in waypoints.windows(2) {
        let dx = (pair[1].tip_nm.x.0 - pair[0].tip_nm.x.0) as f64;
        let dy = (pair[1].tip_nm.y.0 - pair[0].tip_nm.y.0) as f64;
        let dz = (pair[1].tip_nm.z.0 - pair[0].tip_nm.z.0) as f64;
        total_dist_nm += (dx * dx + dy * dy + dz * dz).sqrt();
    }
    let cycle_minutes = total_dist_nm / (feed_nm_min.max(1) as f64);
    let cycle_time_ps = (cycle_minutes * 60.0 * 1e12).round() as i64;

    let gcode_bytes = emit_post_processed_gcode_bytes(dialect, rpm, five_axis, &waypoints);
    let digest = sha512_256_hex(&gcode_bytes);

    Ok((
        GenerateCncToolpathResponse {
            toolpath_job_id: format!("CNC-{}", &digest[..12]),
            estimated_cycle_time_ps: cycle_time_ps.max(1_000_000_000),
            holder_gouge_free: dexel.holder_gouge_free,
            gcode_sftp_staging_digest: digest,
        },
        gcode_bytes,
    ))
}
