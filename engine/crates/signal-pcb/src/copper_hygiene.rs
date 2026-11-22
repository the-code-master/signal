// Module: engine/crates/signal-pcb/src/copper_hygiene.rs — Dead-Copper Island, <90° Acid-Trap & Teardrop Solver (FEAT-014)
// Invariants: Exact i128 dot/cross acute-angle detection; eliminates unconnected islands & synthesizes pad teardrops.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_geom::{BBox2Nm, Point2Nm};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperPourRegion {
    pub region_id: String,
    pub bounds_nm: BBox2Nm,
    pub connected_via_or_pad_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceJunctionCorner {
    pub net_name: String,
    pub p_prev: Point2Nm,
    pub p_vertex: Point2Nm,
    pub p_next: Point2Nm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeardropFilletPolygon {
    pub pad_id: String,
    pub vertices_nm: [Point2Nm; 3],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperHygieneReport {
    pub pruned_dead_island_ids: Vec<String>,
    pub acute_acid_trap_vertices: Vec<Point2Nm>,
    pub generated_teardrops: Vec<TeardropFilletPolygon>,
}

/// Detects acute `< 90°` trace junctions (`u · v > 0` when both vectors emanate from `p_vertex`) using exact `i128` arithmetic.
pub fn is_acute_acid_trap_i128(j: &TraceJunctionCorner) -> bool {
    let ux = i128::from(j.p_prev.x.0) - i128::from(j.p_vertex.x.0);
    let uy = i128::from(j.p_prev.y.0) - i128::from(j.p_vertex.y.0);
    let vx = i128::from(j.p_next.x.0) - i128::from(j.p_vertex.x.0);
    let vy = i128::from(j.p_next.y.0) - i128::from(j.p_vertex.y.0);
    if (ux == 0 && uy == 0) || (vx == 0 && vy == 0) {
        return false;
    }
    let dot = ux * vx + uy * vy;
    let cross = ux * vy - uy * vx;
    // Interior angle < 90 deg iff dot > 0 and vectors are non-collinear (cross != 0)
    dot > 0 && cross != 0
}

/// Synthesizes a tangent teardrop fillet triangle anchoring a trace entry (`trace_entry_nm`) into circular pad `(pad_center_nm, pad_radius_nm)`.
pub fn build_pad_teardrop_fillet(
    pad_id: &str,
    pad_center_nm: Point2Nm,
    pad_radius_nm: i64,
    trace_dir_nm: Point2Nm,
) -> TeardropFilletPolygon {
    let dx = (trace_dir_nm.x.0 - pad_center_nm.x.0) as f64;
    let dy = (trace_dir_nm.y.0 - pad_center_nm.y.0) as f64;
    let len = dx.hypot(dy).max(1.0);
    let ux = dx / len;
    let uy = dy / len;
    let r = pad_radius_nm.max(100_000) as f64;

    let apex = Point2Nm::new(
        pad_center_nm.x.0 + (ux * r * 1.8).round() as i64,
        pad_center_nm.y.0 + (uy * r * 1.8).round() as i64,
    );
    let wing_l = Point2Nm::new(
        pad_center_nm.x.0 + (-uy * r * 0.85).round() as i64,
        pad_center_nm.y.0 + (ux * r * 0.85).round() as i64,
    );
    let wing_r = Point2Nm::new(
        pad_center_nm.x.0 + (uy * r * 0.85).round() as i64,
        pad_center_nm.y.0 + (-ux * r * 0.85).round() as i64,
    );

    TeardropFilletPolygon {
        pad_id: pad_id.to_string(),
        vertices_nm: [apex, wing_l, wing_r],
    }
}

/// Audits copper pour islands, acute `< 90°` acid traps, and generates pad-to-trace teardrops (`FEAT-014`).
pub fn audit_and_heal_copper_hygiene(
    pours: &[CopperPourRegion],
    junctions: &[TraceJunctionCorner],
    pads_for_teardrops: &[(String, Point2Nm, i64, Point2Nm)],
) -> CopperHygieneReport {
    let pruned_dead_island_ids = pours
        .iter()
        .filter(|p| p.connected_via_or_pad_count == 0)
        .map(|p| p.region_id.clone())
        .collect();

    let acute_acid_trap_vertices = junctions
        .iter()
        .filter(|j| is_acute_acid_trap_i128(j))
        .map(|j| j.p_vertex)
        .collect();

    let generated_teardrops = pads_for_teardrops
        .iter()
        .map(|(id, c, r, dir)| build_pad_teardrop_fillet(id, *c, *r, *dir))
        .collect();

    CopperHygieneReport {
        pruned_dead_island_ids,
        acute_acid_trap_vertices,
        generated_teardrops,
    }
}
