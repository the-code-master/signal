// Module: engine/crates/signal-graph/tests/graph_ops_suite.rs — Unit/Integration Tests for CRDT, Undo, CoW & FEAT-001..100
// Invariants: Verifies VCC<->GND & driver-collision rejection, causal undo blocking, Sugiyama, bus slices, swaps & CLI/Query/A11y.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_geom::{BBox2Nm, Point2Nm};
use signal_graph::*;
use signal_proto::v1::{
    ApplyTransactionRequest, AssemblyVariantEntry, CreateSpeculativeBranchRequest,
    FencingAuthority, GraphMutationKind, GraphMutationOp, MergeBranchRequest,
    MultiChannelRepeatRule, Point2dNm, ReannotateDirection, UndoActorLastTransactionRequest,
};
use std::collections::{BTreeMap, BTreeSet};

fn fence(epoch: u64) -> Option<FencingAuthority> {
    Some(FencingAuthority {
        org_id: "org_1".to_string(),
        project_id: "proj_1".to_string(),
        fencing_epoch: epoch,
        ..Default::default()
    })
}

#[test]
fn crdt_rejects_vcc_gnd_short_and_multi_push_pull_driver_collision() {
    let mut eng = DesignGraphEngine::new(10, 0);
    eng.register_pin_role("main", "U1", "Y", ElectricalPinRole::PushPullOutput);
    eng.register_pin_role("main", "U2", "Y", ElectricalPinRole::PushPullOutput);
    eng.register_pin_role("main", "U1", "VDD", ElectricalPinRole::PowerSupplyRail);

    // Short VDD power pin directly to GND -> rejected into TopologicalConflictCard!
    let short_resp = eng
        .apply_transaction(&ApplyTransactionRequest {
            fencing: fence(10),
            actor_pseudo_id: "alice".to_string(),
            expected_base_revision: 0,
            operations: vec![GraphMutationOp {
                op_id: "op_short".to_string(),
                kind: GraphMutationKind::ConnectNet as i32,
                entity_id: "U1.VDD".to_string(),
                net_name: "GND".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .expect("apply");
    assert!(short_resp.has_topological_conflict);
    assert!(short_resp.conflict_card_summary.contains("GND"));

    // Connect U1.Y -> CLK_NET succeeds, then U2.Y -> CLK_NET collides!
    let ok_drv = eng
        .apply_transaction(&ApplyTransactionRequest {
            fencing: fence(10),
            actor_pseudo_id: "alice".to_string(),
            expected_base_revision: 0,
            operations: vec![GraphMutationOp {
                op_id: "op_d1".to_string(),
                kind: GraphMutationKind::ConnectNet as i32,
                entity_id: "U1.Y".to_string(),
                net_name: "CLK_NET".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .expect("d1");
    assert!(!ok_drv.has_topological_conflict);
    assert_eq!(ok_drv.committed_revision, 1);

    let col_drv = eng
        .apply_transaction(&ApplyTransactionRequest {
            fencing: fence(10),
            actor_pseudo_id: "bob".to_string(),
            expected_base_revision: 1,
            operations: vec![GraphMutationOp {
                op_id: "op_d2".to_string(),
                kind: GraphMutationKind::ConnectNet as i32,
                entity_id: "U2.Y".to_string(),
                net_name: "CLK_NET".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .expect("d2");
    assert!(col_drv.has_topological_conflict);
    assert!(matches!(
        eng.last_conflict_card().map(|c| &c.kind),
        Some(TopologicalConflictKind::MultiPushPullDriverCollision { .. })
    ));
}

#[test]
fn causal_selective_undo_blocks_dependent_peer_tx_and_cow_branches_merge() {
    let mut eng = DesignGraphEngine::new(10, 0);
    // Alice places R1
    eng.apply_transaction(&ApplyTransactionRequest {
        fencing: fence(10),
        actor_pseudo_id: "alice".to_string(),
        expected_base_revision: 0,
        operations: vec![GraphMutationOp {
            op_id: "alice_place_r1".to_string(),
            kind: GraphMutationKind::PlaceInstance as i32,
            entity_id: "R1".to_string(),
            position_nm: Some(Point2dNm {
                x_nm: 100,
                y_nm: 200,
            }),
            param_key: "R".to_string(),
            param_si_value: 10_000_000,
            ..Default::default()
        }],
        ..Default::default()
    })
    .expect("alice r1");

    // Bob connects R1.1 -> VCC (causally depends on R1)
    eng.apply_transaction(&ApplyTransactionRequest {
        fencing: fence(10),
        actor_pseudo_id: "bob".to_string(),
        expected_base_revision: 1,
        operations: vec![GraphMutationOp {
            op_id: "bob_wire_r1".to_string(),
            kind: GraphMutationKind::ConnectNet as i32,
            entity_id: "R1.1".to_string(),
            net_name: "VCC".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    })
    .expect("bob r1");

    // Alice tries to undo her R1 placement -> blocked by bob_wire_r1!
    let blocked = eng
        .undo_actor_last_tx(&UndoActorLastTransactionRequest {
            fencing: fence(10),
            actor_pseudo_id: "alice".to_string(),
            branch_id: "main".to_string(),
        })
        .expect("undo alice");
    assert!(!blocked.undone);
    assert_eq!(blocked.blocked_by_dependent_op_id, "bob_wire_r1");

    // Bob undoes his wire -> succeeds! Now Alice can undo too!
    let bob_undo = eng
        .undo_actor_last_tx(&UndoActorLastTransactionRequest {
            fencing: fence(10),
            actor_pseudo_id: "bob".to_string(),
            branch_id: "main".to_string(),
        })
        .expect("undo bob");
    assert!(bob_undo.undone);

    // Speculative CoW branch & merge
    let br = eng
        .create_speculative_branch(&CreateSpeculativeBranchRequest {
            fencing: fence(10),
            parent_branch_id: "main".to_string(),
            new_branch_name: "ai/opt-1".to_string(),
            created_by_agent_id: "agent_1".to_string(),
        })
        .expect("branch");
    let merged = eng
        .merge_branch(&MergeBranchRequest {
            fencing: fence(10),
            source_branch_id: "ai/opt-1".to_string(),
            target_branch_id: "main".to_string(),
            expected_base_graph_merkle_root: br.base_graph_merkle_root,
            attached_proof_bundle_id: "proof_1".to_string(),
        })
        .expect("merge");
    assert!(merged.merged);
    assert!(merged.proof_reverified_after_rebase);
}

#[test]
fn beautify_bus_hierarchy_pinswap_reannotate_cli_query_and_a11y_work() {
    // FEAT-001: Sugiyama crossing reduction on an X-crossed 2-layer graph
    let blocks = vec![
        SchematicBlock {
            entity_id: "A1".to_string(),
            position_nm: Point2Nm::new(0, 0),
        },
        SchematicBlock {
            entity_id: "A2".to_string(),
            position_nm: Point2Nm::new(0, 1000),
        },
        SchematicBlock {
            entity_id: "B1".to_string(),
            position_nm: Point2Nm::new(1000, 0),
        },
        SchematicBlock {
            entity_id: "B2".to_string(),
            position_nm: Point2Nm::new(1000, 1000),
        },
    ];
    let edges = vec![
        ("A1".to_string(), "B2".to_string()),
        ("A2".to_string(), "B1".to_string()),
    ];
    let beau = beautify_schematic_layout(&blocks, &edges, 100_000);
    assert_eq!(beau.crossing_count_before, 1);
    assert_eq!(beau.crossing_count_after, 0);

    // FEAT-002: Bus slice & AXI4_LITE bundle
    let slice = parse_bus_slice("b1", "RAW", "DATA[31:0]").expect("slice");
    assert_eq!(slice.expanded_member_nets.len(), 32);
    assert_eq!(slice.expanded_member_nets[0], "DATA[31]");
    assert_eq!(slice.expanded_member_nets[31], "DATA[0]");
    let axi = expand_protocol_bundle("b2", "M0", "AXI4_LITE").expect("axi");
    assert!(axi.expanded_member_nets.len() > 130);

    // FEAT-005..007: Portals, Repeat(N), Variants
    let portals = build_cross_sheet_portal_index(&[
        SheetNetPort {
            sheet_id: "S1".into(),
            net_name: "I2C_SDA".into(),
            anchor_nm: Point2Nm::new(10, 20),
        },
        SheetNetPort {
            sheet_id: "S2".into(),
            net_name: "I2C_SDA".into(),
            anchor_nm: Point2Nm::new(30, 40),
        },
    ]);
    assert_eq!(portals.len(), 2);

    let mut pnets = BTreeMap::new();
    pnets.insert("1".to_string(), "VCC".to_string());
    pnets.insert("2".to_string(), "OUT".to_string());
    let mut globals = BTreeSet::new();
    globals.insert("VCC".to_string());
    let chans = expand_multi_channel_block(
        &MultiChannelRepeatRule {
            block_id: "AMP".into(),
            repeat_count: 3,
            net_naming_template: "{net}_CH{ch}".into(),
            refdes_offset_rule: "+100".into(),
        },
        &[("R1".to_string(), pnets)],
        &globals,
    )
    .expect("repeat");
    assert_eq!(chans.len(), 3);
    assert_eq!(chans[1].expanded_refdes, "R201");
    assert_eq!(
        chans[1].expanded_pin_nets.get("1").map(String::as_str),
        Some("VCC")
    );
    assert_eq!(
        chans[1].expanded_pin_nets.get("2").map(String::as_str),
        Some("OUT_CH2")
    );

    let vars = resolve_assembly_variant(
        &[("R1".into(), "RC0402_10K".into())],
        &[AssemblyVariantEntry {
            variant_id: "DNP_TEST".into(),
            entity_id: "R1".into(),
            do_not_populate: true,
            alternate_mpn: String::new(),
        }],
        "DNP_TEST",
    );
    assert!(!vars[0].populated);

    // FEAT-008 & FEAT-009: Pin swap & reannotate
    let swaps = propose_optimal_pin_swaps(
        "RN1",
        &[
            SwappablePinAnchor {
                pin_number: 1,
                pin_location_nm: Point2Nm::new(0, 0),
                target_net_anchor_nm: Point2Nm::new(100, 1000),
            },
            SwappablePinAnchor {
                pin_number: 2,
                pin_location_nm: Point2Nm::new(0, 1000),
                target_net_anchor_nm: Point2Nm::new(100, 0),
            },
        ],
    );
    assert_eq!(swaps.len(), 2);
    assert!(swaps[0].wirelength_saved_nm > 0);

    let reann = reannotate_sheet_instances(
        &[
            AnnotatableInstance {
                entity_id: "e2".into(),
                current_refdes: "R9".into(),
                sheet_index: 1,
                position_nm: Point2Nm::new(500, 0),
            },
            AnnotatableInstance {
                entity_id: "e1".into(),
                current_refdes: "R8".into(),
                sheet_index: 1,
                position_nm: Point2Nm::new(100, 0),
            },
        ],
        ReannotateDirection::LeftToRightTopDown,
        100,
        5,
        50,
    );
    assert_eq!(reann.remapped[0].entity_id, "e1");
    assert_eq!(reann.remapped[0].new_refdes, "R1");

    // FEAT-091, FEAT-099, FEAT-100: CLI, Query, A11y & Keyboard A*
    let cli = execute_omni_command_dsl("place R 10k at (100,200)").expect("cli");
    assert!(cli.accepted);
    assert_eq!(cli.generated_ops[0].param_si_value, 10_000_000);

    let mut attrs_bad = BTreeMap::new();
    attrs_bad.insert("voltage_rating".into(), 16.0);
    attrs_bad.insert("net_max_v".into(), 12.0);
    let mut attrs_ok = BTreeMap::new();
    attrs_ok.insert("voltage_rating".into(), 50.0);
    attrs_ok.insert("net_max_v".into(), 12.0);
    let qres = evaluate_selection_query(
        &[
            QueryableEntity {
                entity_id: "C1".into(),
                component_type: "cap".into(),
                numeric_attrs: attrs_bad,
            },
            QueryableEntity {
                entity_id: "C2".into(),
                component_type: "cap".into(),
                numeric_attrs: attrs_ok,
            },
        ],
        "type:cap AND voltage_rating < 2*net_max_v",
    )
    .expect("query");
    assert_eq!(qres.matched_entity_ids, vec!["C1".to_string()]);

    let obstacle = BBox2Nm::from_points(Point2Nm::new(190, -50), Point2Nm::new(210, 50));
    let wire =
        route_keyboard_a11y_wire(Point2Nm::new(0, 0), Point2Nm::new(400, 0), &[obstacle], 100);
    assert!(wire.len() >= 5);
    assert!(wire.iter().all(|&pt| !obstacle.contains_point(pt)));
}
