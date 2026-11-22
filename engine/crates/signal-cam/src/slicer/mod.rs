// Module: engine/crates/signal-cam/src/slicer/mod.rs — Direct-B-Rep Additive 3D Print Slicer & Gyroid Exports
// Invariants: Slices analytic B-Rep surfaces directly without STL chordal faceting loss (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
pub mod direct_brep_gyroid;

pub use direct_brep_gyroid::{
    AnalyticBRepPrimitive, AnalyticLayerContour, SlicedLayerPlan, emit_additive_machine_bytes,
    nest_parts_on_build_plate_2d, sample_stress_guided_gyroid_infill, slice_analytic_brep_at_z,
    slice_brep_for_3d_print,
};
