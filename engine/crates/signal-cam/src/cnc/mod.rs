// Module: engine/crates/signal-cam/src/cnc/mod.rs — Subtractive CNC Toolpath, Dexel Gouge & Post-Processor Exports
// Invariants: Pure-Rust exact nanometer CNC CAM with zero browser download paths (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
pub mod post_processors;
pub mod toolpath_dexel;

pub use post_processors::{
    PostProcessorDialect, emit_post_processed_gcode_bytes, generate_cnc_toolpath,
};
pub use toolpath_dexel::{
    CncToolAssemblySpec, CncWaypoint5Axis, DexelSimulationReport, compute_5axis_tool_vector,
    compute_scallop_stepover_nm, optimize_feedrate_nm_per_min, plan_adaptive_trochoidal_pocket,
    plan_pcb_isolation_or_rest_passes, simulate_dexel_material_removal,
};
