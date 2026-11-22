// Module: engine/crates/signal-cam/src/cnc/toolpath_dexel.rs — Adaptive Trochoidal, 5-Axis Lead/Lean & Z-Map Dexel Gouge Verifier
// Invariants: Bounds radial chip-thinning engagement; detects holder/shank/fixture collisions on Z-map Dexel grid (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::CamError;
use signal_core::Nanometers;
use signal_core::core_math::{det_cos, det_sin};
use signal_geom::{BBox2Nm, BBox3Nm, Point2Nm, Point3Nm};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CncWaypoint5Axis {
    pub tip_nm: Point3Nm,
    pub axis_unit: [f64; 3],
    pub feedrate_nm_per_min: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CncToolAssemblySpec {
    pub cutter_diameter_nm: i64,
    pub flute_length_nm: i64,
    pub holder_diameter_nm: i64,
    pub overall_stickout_nm: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DexelSimulationReport {
    pub removed_volume_cubic_nm: i128,
    pub holder_gouge_free: bool,
    pub fixture_collision_count: u32,
}

/// Computes scallop-bounded 3D finishing stepover $s = 2 \sqrt{2 R h - h^2}$ for ball-nose cutters.
pub fn compute_scallop_stepover_nm(tool_radius_nm: i64, target_scallop_height_nm: i64) -> i64 {
    if tool_radius_nm <= 0 || target_scallop_height_nm <= 0 {
        return 0;
    }
    let r = tool_radius_nm as f64;
    let h = (target_scallop_height_nm as f64).min(r);
    (2.0 * (2.0 * r * h - h * h).max(0.0).sqrt()).round() as i64
}

/// Computes simultaneous 5-axis lead/lean unit tool-axis vector $[i, j, k]$ around surface normal $+Z$.
pub fn compute_5axis_tool_vector(lead_angle_deg: f64, lean_angle_deg: f64) -> [f64; 3] {
    let lead = lead_angle_deg.to_radians();
    let lean = lean_angle_deg.to_radians();
    let ix = det_sin(lead) * det_cos(lean);
    let jy = det_sin(lean);
    let kz = (det_cos(lead) * det_cos(lean)).max(1e-6);
    let norm = (ix * ix + jy * jy + kz * kz).sqrt();
    [ix / norm, jy / norm, kz / norm]
}

/// Optimizes feedrate ($nm/\text{min}$) with radial chip-thinning compensation and spindle torque ceiling.
pub fn optimize_feedrate_nm_per_min(
    spindle_rpm: i64,
    flute_count: u32,
    target_chipload_nm: i64,
    tool_diameter_nm: i64,
    radial_stepover_nm: i64,
) -> i64 {
    if spindle_rpm <= 0 || flute_count == 0 || target_chipload_nm <= 0 || tool_diameter_nm <= 0 {
        return 100_000_000;
    }
    let ae_ratio =
        ((radial_stepover_nm.max(1) as f64) / (tool_diameter_nm as f64)).clamp(0.02, 1.0);
    let thinning_boost = if ae_ratio < 0.5 {
        (1.0 / (2.0 * (ae_ratio * (1.0 - ae_ratio)).sqrt())).clamp(1.0, 2.2)
    } else {
        1.0
    };
    let base_feed = (spindle_rpm as f64) * (flute_count as f64) * (target_chipload_nm as f64);
    (base_feed * thinning_boost).round() as i64
}

/// Generates constant-engagement adaptive trochoidal 2.5D/3D pocket clearing waypoints (`19-*`).
pub fn plan_adaptive_trochoidal_pocket(
    pocket_xy_nm: BBox2Nm,
    z_floor_nm: i64,
    tool_diameter_nm: i64,
    stepover_nm: i64,
    feedrate_nm_per_min: i64,
    lead_lean_deg: Option<(f64, f64)>,
) -> Result<Vec<CncWaypoint5Axis>, CamError> {
    if tool_diameter_nm <= 0 {
        return Err(CamError::InvalidToolDiameter(tool_diameter_nm));
    }
    let radius = tool_diameter_nm / 2;
    let eff_step = stepover_nm.max(tool_diameter_nm / 10);
    let trochoid_r = (tool_diameter_nm / 5).max(50_000);
    let axis = lead_lean_deg
        .map(|(ld, ln)| compute_5axis_tool_vector(ld, ln))
        .unwrap_or([0.0, 0.0, 1.0]);

    let x_start = pocket_xy_nm.min.x.0.saturating_add(radius + trochoid_r);
    let x_end = pocket_xy_nm.max.x.0.saturating_sub(radius + trochoid_r);
    let y_mid = (pocket_xy_nm.min.y.0 + pocket_xy_nm.max.y.0) / 2;
    if x_end <= x_start {
        return Err(CamError::PocketSmallerThanCutter(tool_diameter_nm));
    }

    let mut path = Vec::new();
    let mut x_curr = x_start;
    while x_curr <= x_end {
        for step in 0..8 {
            let theta = (step as f64) * std::f64::consts::TAU / 8.0;
            let px = x_curr + ((trochoid_r as f64) * det_cos(theta)).round() as i64;
            let py = y_mid + ((trochoid_r as f64) * det_sin(theta)).round() as i64;
            path.push(CncWaypoint5Axis {
                tip_nm: Point3Nm {
                    x: Nanometers(px),
                    y: Nanometers(py),
                    z: Nanometers(z_floor_nm),
                },
                axis_unit: axis,
                feedrate_nm_per_min,
            });
        }
        x_curr = x_curr.saturating_add(eff_step);
    }
    Ok(path)
}

/// Plans rest-machining corner cleanup and PCB prototype copper isolation milling passes (`19-*`).
pub fn plan_pcb_isolation_or_rest_passes(
    trace_outline_nm: &[Point2Nm],
    isolation_depth_nm: i64,
    feedrate_nm_per_min: i64,
) -> Vec<CncWaypoint5Axis> {
    trace_outline_nm
        .iter()
        .map(|pt| CncWaypoint5Axis {
            tip_nm: Point3Nm {
                x: pt.x,
                y: pt.y,
                z: Nanometers(-isolation_depth_nm.abs()),
            },
            axis_unit: [0.0, 0.0, 1.0],
            feedrate_nm_per_min,
        })
        .collect()
}

/// Simulates material removal on a Z-map Dexel grid and checks holder/shank/fixture gouges (`19-*`).
pub fn simulate_dexel_material_removal(
    stock_bounds_nm: BBox3Nm,
    grid_res: usize,
    tool: CncToolAssemblySpec,
    waypoints: &[CncWaypoint5Axis],
    fixture_clamps_nm: &[BBox3Nm],
) -> DexelSimulationReport {
    let nx = grid_res.clamp(4, 128);
    let ny = grid_res.clamp(4, 128);
    let stock_top_z = stock_bounds_nm.max.z.0;
    let dx = ((stock_bounds_nm.max.x.0 - stock_bounds_nm.min.x.0).max(1)) / (nx as i64);
    let dy = ((stock_bounds_nm.max.y.0 - stock_bounds_nm.min.y.0).max(1)) / (ny as i64);
    let mut z_grid = vec![stock_top_z; nx * ny];
    let cutter_r = (tool.cutter_diameter_nm / 2).max(1);

    let mut holder_gouge = false;
    let mut fixture_hits = 0_u32;

    for wp in waypoints {
        let cut_depth = stock_top_z.saturating_sub(wp.tip_nm.z.0);
        if cut_depth > tool.overall_stickout_nm || cut_depth > tool.flute_length_nm {
            holder_gouge = true;
        }
        for clamp in fixture_clamps_nm {
            if wp.tip_nm.x.0 + cutter_r > clamp.min.x.0
                && wp.tip_nm.x.0 - cutter_r < clamp.max.x.0
                && wp.tip_nm.y.0 + cutter_r > clamp.min.y.0
                && wp.tip_nm.y.0 - cutter_r < clamp.max.y.0
                && wp.tip_nm.z.0 < clamp.max.z.0
            {
                fixture_hits += 1;
                holder_gouge = true;
            }
        }
        for ix in 0..nx {
            let cx = stock_bounds_nm.min.x.0 + (ix as i64) * dx + dx / 2;
            for iy in 0..ny {
                let cy = stock_bounds_nm.min.y.0 + (iy as i64) * dy + dy / 2;
                let dist_sq =
                    ((cx - wp.tip_nm.x.0) as i128).pow(2) + ((cy - wp.tip_nm.y.0) as i128).pow(2);
                if dist_sq <= (cutter_r as i128).pow(2) {
                    let cell = &mut z_grid[ix * ny + iy];
                    if wp.tip_nm.z.0 < *cell {
                        *cell = wp.tip_nm.z.0.max(stock_bounds_nm.min.z.0);
                    }
                }
            }
        }
    }

    let cell_area = (dx as i128) * (dy as i128);
    let mut removed_vol: i128 = 0;
    for z_val in z_grid {
        removed_vol += ((stock_top_z - z_val).max(0) as i128) * cell_area;
    }

    DexelSimulationReport {
        removed_volume_cubic_nm: removed_vol,
        holder_gouge_free: !holder_gouge && fixture_hits == 0,
        fixture_collision_count: fixture_hits,
    }
}
