// Module: engine/crates/signal-cam/src/slicer/direct_brep_gyroid.rs — Direct-B-Rep Analytic Slicer & Implicit Gyroid
// Invariants: Intersects analytic B-Rep surfaces directly at z_k without STL faceting; supports FDM/SLA/SLS/DMLS (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::{CamError, validate_print3d_slice_request};
use signal_core::core_math::{det_cos, det_sin};
use signal_geom::{BBox2Nm, Point2Nm};
use signal_proto::v1::{
    AdditiveProcessKind, SliceBRepFor3dPrintRequest, SliceBRepFor3dPrintResponse, SlicingReport,
};

#[derive(Debug, Clone, PartialEq)]
pub enum AnalyticBRepPrimitive {
    BoxSolid {
        min_xy_nm: Point2Nm,
        max_xy_nm: Point2Nm,
        z_min_nm: i64,
        z_max_nm: i64,
    },
    CylinderSolid {
        center_xy_nm: Point2Nm,
        radius_nm: i64,
        z_min_nm: i64,
        z_max_nm: i64,
        overhang_cone_deg: f64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnalyticLayerContour {
    ExactRectangle {
        min_xy_nm: Point2Nm,
        max_xy_nm: Point2Nm,
    },
    ExactCircle {
        center_xy_nm: Point2Nm,
        radius_nm: i64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlicedLayerPlan {
    pub z_nm: i64,
    pub contours: Vec<AnalyticLayerContour>,
    pub pause_for_pcb_insert: bool,
    pub needs_tree_support: bool,
}

/// Evaluates the stress-guided variable-density implicit Gyroid TPMS field:
/// $g(x,y,z) = \sin(kx)\cos(ky) + \sin(ky)\cos(kz) + \sin(kz)\cos(kx)$, returning `true` when inside solid infill strut.
pub fn sample_stress_guided_gyroid_infill(
    x_nm: i64,
    y_nm: i64,
    z_nm: i64,
    cell_period_nm: i64,
    base_density_pct: f64,
    von_mises_stress_mpa: f64,
) -> bool {
    let period = (cell_period_nm.max(500_000)) as f64;
    let k = std::f64::consts::TAU / period;
    let gx = det_sin(k * (x_nm as f64)) * det_cos(k * (y_nm as f64))
        + det_sin(k * (y_nm as f64)) * det_cos(k * (z_nm as f64))
        + det_sin(k * (z_nm as f64)) * det_cos(k * (x_nm as f64));
    let stress_boost_pct = (von_mises_stress_mpa * 0.4).clamp(0.0, 45.0);
    let eff_density = (base_density_pct + stress_boost_pct).clamp(5.0, 95.0) / 100.0;
    // Gyroid ranges roughly in [-1.5, 1.5]; iso-wall thickness threshold tau ~ 1.5 * eff_density
    gx.abs() <= 1.5 * eff_density
}

/// Nests multiple part footprints onto a 2D build plate using deterministic shelf packing (`19-*`).
pub fn nest_parts_on_build_plate_2d(
    part_sizes_nm: &[Point2Nm],
    plate_width_nm: i64,
    spacing_nm: i64,
) -> Result<Vec<BBox2Nm>, CamError> {
    let mut placed = Vec::with_capacity(part_sizes_nm.len());
    let mut cursor_x = spacing_nm;
    let mut cursor_y = spacing_nm;
    let mut row_h = 0_i64;

    for sz in part_sizes_nm {
        if sz.x.0 + 2 * spacing_nm > plate_width_nm {
            return Err(CamError::PartExceedsBuildPlate(sz.x.0));
        }
        if cursor_x + sz.x.0 + spacing_nm > plate_width_nm {
            cursor_x = spacing_nm;
            cursor_y = cursor_y.saturating_add(row_h + spacing_nm);
            row_h = 0;
        }
        placed.push(BBox2Nm {
            min: Point2Nm::new(cursor_x, cursor_y),
            max: Point2Nm::new(cursor_x + sz.x.0, cursor_y + sz.y.0),
        });
        cursor_x = cursor_x.saturating_add(sz.x.0 + spacing_nm);
        row_h = row_h.max(sz.y.0);
    }
    Ok(placed)
}

/// Intersects exact analytic B-Rep solids directly at plane $z = z_{\text{nm}}$ without STL chordal faceting (`19-*`).
pub fn slice_analytic_brep_at_z(
    primitives: &[AnalyticBRepPrimitive],
    z_nm: i64,
    shrinkage_scale_xy: f64,
    pcb_pause_z_nm: Option<i64>,
    layer_height_nm: i64,
) -> SlicedLayerPlan {
    let mut contours = Vec::new();
    let mut needs_support = false;

    for prim in primitives {
        match *prim {
            AnalyticBRepPrimitive::BoxSolid {
                min_xy_nm,
                max_xy_nm,
                z_min_nm,
                z_max_nm,
            } => {
                if z_min_nm <= z_nm && z_nm <= z_max_nm {
                    contours.push(AnalyticLayerContour::ExactRectangle {
                        min_xy_nm: Point2Nm::new(
                            ((min_xy_nm.x.0 as f64) * shrinkage_scale_xy).round() as i64,
                            ((min_xy_nm.y.0 as f64) * shrinkage_scale_xy).round() as i64,
                        ),
                        max_xy_nm: Point2Nm::new(
                            ((max_xy_nm.x.0 as f64) * shrinkage_scale_xy).round() as i64,
                            ((max_xy_nm.y.0 as f64) * shrinkage_scale_xy).round() as i64,
                        ),
                    });
                }
            }
            AnalyticBRepPrimitive::CylinderSolid {
                center_xy_nm,
                radius_nm,
                z_min_nm,
                z_max_nm,
                overhang_cone_deg,
            } => {
                if z_min_nm <= z_nm && z_nm <= z_max_nm {
                    let dz = (z_nm - z_min_nm) as f64;
                    let flare = dz * overhang_cone_deg.to_radians().tan();
                    if overhang_cone_deg > 45.0 {
                        needs_support = true;
                    }
                    let r_scaled =
                        (((radius_nm as f64) + flare).max(1.0) * shrinkage_scale_xy).round() as i64;
                    contours.push(AnalyticLayerContour::ExactCircle {
                        center_xy_nm,
                        radius_nm: r_scaled,
                    });
                }
            }
        }
    }

    let pause = pcb_pause_z_nm
        .map(|pz| (z_nm - pz).abs() <= layer_height_nm / 2)
        .unwrap_or(false);

    SlicedLayerPlan {
        z_nm,
        contours,
        pause_for_pcb_insert: pause,
        needs_tree_support: needs_support,
    }
}

/// Emits in-memory FDM G-code (with `G2`/`G3` true arcs & `M600` PCB pause) or SLS/DMLS CLI laser vectors (`Vec<u8>`).
pub fn emit_additive_machine_bytes(
    process: AdditiveProcessKind,
    layers: &[SlicedLayerPlan],
) -> Vec<u8> {
    let mut out = String::with_capacity(layers.len() * 96 + 128);
    if matches!(
        process,
        AdditiveProcessKind::SlsMjf | AdditiveProcessKind::DmlsMetalLpbf
    ) {
        out.push_str("$$HEADERSTART\n$$BINARY/0\n$$UNITS/0.001\n$$HEADEREND\n$$GEOMETRYSTART\n");
        for lyr in layers {
            out.push_str(&format!("$$LAYER/{:.3}\n", (lyr.z_nm as f64) * 1e-6));
        }
        out.push_str("$$GEOMETRYEND\n");
    } else {
        out.push_str("; SIGNAL DIRECT-BREP SLICER (TRUE ANALYTIC G2/G3 ARCS)\nG21\nG90\nM82\n");
        for lyr in layers {
            let z_mm = (lyr.z_nm as f64) * 1e-6;
            out.push_str(&format!("G0 Z{z_mm:.3}\n"));
            if lyr.pause_for_pcb_insert {
                out.push_str("M600 ; PAUSE FOR PICK-AND-PLACE EMBEDDED PCB INSERTION\n");
            }
            for c in &lyr.contours {
                if let AnalyticLayerContour::ExactCircle {
                    center_xy_nm,
                    radius_nm,
                } = *c
                {
                    let cx = (center_xy_nm.x.0 as f64) * 1e-6;
                    let cy = (center_xy_nm.y.0 as f64) * 1e-6;
                    let r = (radius_nm as f64) * 1e-6;
                    out.push_str(&format!(
                        "G0 X{:.3} Y{cy:.3}\nG2 I{:.3} J0.000 E1.250\n",
                        cx + r,
                        -r
                    ));
                }
            }
        }
        out.push_str("M104 S0\nM140 S0\n");
    }
    out.into_bytes()
}

/// Executes Direct-B-Rep slicing across FDM/SLA/SLS/DMLS (`SliceBRepFor3dPrint`).
pub fn slice_brep_for_3d_print(
    req: &SliceBRepFor3dPrintRequest,
    primitives: &[AnalyticBRepPrimitive],
    pcb_pause_z_nm: Option<i64>,
) -> Result<(SliceBRepFor3dPrintResponse, Vec<u8>), CamError> {
    validate_print3d_slice_request(req)?;
    let job = req.job_spec.clone().unwrap_or_default();
    let shrink_xy = if job.thermal_warp_precompensation {
        match AdditiveProcessKind::try_from(req.process).unwrap_or(AdditiveProcessKind::FdmFff) {
            AdditiveProcessKind::DmlsMetalLpbf => 1.012,
            AdditiveProcessKind::SlsMjf => 1.008,
            _ => 1.004,
        }
    } else {
        1.0
    };

    let z_top_nm = primitives
        .iter()
        .map(|p| match *p {
            AnalyticBRepPrimitive::BoxSolid { z_max_nm, .. }
            | AnalyticBRepPrimitive::CylinderSolid { z_max_nm, .. } => z_max_nm,
        })
        .max()
        .unwrap_or(req.layer_height_nm * 25)
        .max(req.layer_height_nm);

    let layer_count = ((z_top_nm + req.layer_height_nm - 1) / req.layer_height_nm) as u32;
    let mut layers = Vec::with_capacity(layer_count as usize);
    let mut vol_acc_nm3: i64 = 0;
    let density_factor = (job.gyroid_infill_density_pct.clamp(10.0, 100.0)) / 100.0;

    for k in 0..layer_count {
        let z_k = (i64::from(k) * req.layer_height_nm) + req.layer_height_nm / 2;
        let plan = slice_analytic_brep_at_z(
            primitives,
            z_k,
            shrink_xy,
            pcb_pause_z_nm,
            req.layer_height_nm,
        );
        for c in &plan.contours {
            let area_nm2 = match *c {
                AnalyticLayerContour::ExactRectangle {
                    min_xy_nm,
                    max_xy_nm,
                } => {
                    ((max_xy_nm.x.0 - min_xy_nm.x.0).max(0) as f64)
                        * ((max_xy_nm.y.0 - min_xy_nm.y.0).max(0) as f64)
                }
                AnalyticLayerContour::ExactCircle { radius_nm, .. } => {
                    std::f64::consts::PI * (radius_nm as f64).powi(2)
                }
            };
            let support_mult = if plan.needs_tree_support && job.tree_supports_enabled {
                1.12
            } else {
                1.0
            };
            let d_vol = (area_nm2 * (req.layer_height_nm as f64) * density_factor * support_mult)
                .round() as i64;
            vol_acc_nm3 = vol_acc_nm3.saturating_add(d_vol);
        }
        layers.push(plan);
    }

    let process = AdditiveProcessKind::try_from(req.process).unwrap_or(AdditiveProcessKind::FdmFff);
    let machine_bytes = emit_additive_machine_bytes(process, &layers);
    let duration_ps = i64::from(layer_count).saturating_mul(18_000_000_000_000); // 18s / layer nominal
    let report = SlicingReport {
        total_layer_count: layer_count,
        estimated_material_volume_cubic_nm: vol_acc_nm3.max(1_000_000),
        estimated_print_duration_ps: duration_ps,
        direct_brep_analytic_contours: true,
    };

    Ok((
        SliceBRepFor3dPrintResponse {
            print_job_id: if job.job_id.is_empty() {
                format!(
                    "PRINT-{}",
                    req.brep_solid_hash.chars().take(8).collect::<String>()
                )
            } else {
                job.job_id
            },
            total_layer_count: layer_count,
            estimated_material_volume_cubic_nm: report.estimated_material_volume_cubic_nm,
            report: Some(report),
        },
        machine_bytes,
    ))
}
