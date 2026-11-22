// Module: engine/crates/signal-graph/tests/phase4_constraint_matrix.rs — FEAT-095 Bulk Constraint Matrix Tests
// Invariants: Verifies symmetric pairwise (ClassA, ClassB) clearance override & base net-class fallback.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_graph::{ConstraintMatrixResolver, NetClassSpec};

#[test]
fn resolves_symmetric_pairwise_hv_clearance_and_net_class_defaults() {
    let mut mtx = ConstraintMatrixResolver::new();
    mtx.upsert_net_class(NetClassSpec {
        class_name: "LV_LOGIC".into(),
        min_width_nm: 90_000,
        base_clearance_nm: 100_000,
        target_diff_z0_mohm: Some(100_000),
        max_pair_skew_fs: Some(500),
    });
    mtx.upsert_net_class(NetClassSpec {
        class_name: "HV_MAINS".into(),
        min_width_nm: 400_000,
        base_clearance_nm: 2_500_000,
        target_diff_z0_mohm: None,
        max_pair_skew_fs: None,
    });
    mtx.set_pairwise_clearance_nm("LV_LOGIC", "HV_MAINS", 5_500_000);

    assert_eq!(
        mtx.resolve_clearance_nm("HV_MAINS", "LV_LOGIC", 100_000),
        5_500_000
    );
    assert_eq!(
        mtx.resolve_clearance_nm("LV_LOGIC", "LV_LOGIC", 100_000),
        100_000
    );
}
