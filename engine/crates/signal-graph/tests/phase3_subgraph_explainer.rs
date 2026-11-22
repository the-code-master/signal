// Module: engine/crates/signal-graph/tests/phase3_subgraph_explainer.rs — Subgraph Stage Explainer Tests (FEAT-101)
// Invariants: Verifies Buck LC, Op-Amp, RC Filter, and Voltage Divider recognition + symbolic transfer functions.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_geom::Point2Nm;
use signal_graph::{StageComponentNode, StagePrimitiveKind, explain_circuit_stages};

#[test]
fn recognizes_buck_stage_rc_filter_and_voltage_divider_with_symbolic_hs() {
    let comps = vec![
        StageComponentNode {
            refdes: "Q1".into(),
            kind: StagePrimitiveKind::MosfetSwitch,
            value_si: 0.01,
            nets: vec!["VIN".into(), "SW_NODE".into()],
            pos_nm: Point2Nm::new(0, 0),
        },
        StageComponentNode {
            refdes: "L1".into(),
            kind: StagePrimitiveKind::Inductor,
            value_si: 2.2e-6,
            nets: vec!["SW_NODE".into(), "VOUT_BUCK".into()],
            pos_nm: Point2Nm::new(5_000_000, 0),
        },
        StageComponentNode {
            refdes: "C1".into(),
            kind: StagePrimitiveKind::Capacitor,
            value_si: 22.0e-6,
            nets: vec!["VOUT_BUCK".into(), "GND".into()],
            pos_nm: Point2Nm::new(10_000_000, 0),
        },
        StageComponentNode {
            refdes: "R_LPF".into(),
            kind: StagePrimitiveKind::Resistor,
            value_si: 1_000.0,
            nets: vec!["SIG_IN".into(), "FILT_MID".into()],
            pos_nm: Point2Nm::new(0, 10_000_000),
        },
        StageComponentNode {
            refdes: "C_LPF".into(),
            kind: StagePrimitiveKind::Capacitor,
            value_si: 100.0e-9,
            nets: vec!["FILT_MID".into(), "GND".into()],
            pos_nm: Point2Nm::new(4_000_000, 10_000_000),
        },
        StageComponentNode {
            refdes: "R_TOP".into(),
            kind: StagePrimitiveKind::Resistor,
            value_si: 30_000.0,
            nets: vec!["VOUT_BUCK".into(), "FB_NODE".into()],
            pos_nm: Point2Nm::new(15_000_000, 0),
        },
        StageComponentNode {
            refdes: "R_BOT".into(),
            kind: StagePrimitiveKind::Resistor,
            value_si: 10_000.0,
            nets: vec!["FB_NODE".into(), "GND".into()],
            pos_nm: Point2Nm::new(15_000_000, -4_000_000),
        },
    ];

    let explained = explain_circuit_stages(&comps);
    assert_eq!(explained.stages.len(), 3);
    assert_eq!(
        explained.stages[0].topology_name,
        "Synchronous Buck Power Stage"
    );
    assert!(explained.stages[0].symbolic_transfer_fn.contains("s^2*L*C"));
    assert!(
        explained
            .stages
            .iter()
            .any(|s| s.topology_name == "First-Order RC Low-Pass Filter")
    );
    assert!(
        explained
            .stages
            .iter()
            .any(|s| s.symbolic_transfer_fn.contains("0.2500"))
    );
}
