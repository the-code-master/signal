// Module: engine/crates/signal-graph/tests/phase2_collab_breadboard_snippets.rs — Phase 2 Graph Tests
// Invariants: Verifies 60 Hz Follow-Me presence (FEAT-094), Breadboard Twin short detection (FEAT-111), and Golden Snippets (FEAT-079).
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_geom::Point2Nm;
use signal_graph::{
    EphemeralPresenceHub, GoldenSnippetTemplate, SnippetComponentTemplate, SnippetTraceTemplate,
    build_breadboard_strips_from_netlist, stamp_golden_snippet, sync_breadboard_twin,
};
use signal_proto::v1::{
    BreadboardStripMapping, BroadcastFollowPresenceRequest, Point2dNm, StampGoldenSnippetRequest,
    SyncBreadboardTwinRequest,
};
use std::collections::BTreeMap;

#[test]
fn presence_hub_paces_60hz_and_propagates_follow_me_lock() {
    let mut hub = EphemeralPresenceHub::new();
    hub.subscribe_follower("sess-alice", "sess-lead");
    hub.subscribe_follower("sess-bob", "sess-lead");

    let req = BroadcastFollowPresenceRequest {
        project_id: "proj-1".into(),
        presenter_session_id: "sess-lead".into(),
        camera_center_nm: Some(Point2dNm {
            x_nm: 12_500_000,
            y_nm: -4_000_000,
        }),
        zoom_scale: 2.5,
        laser_pointer_nm: Some(Point2dNm {
            x_nm: 13_000_000,
            y_nm: -3_500_000,
        }),
        follow_me_locked: true,
    };
    let resp = hub.broadcast_presence(&req, 100_000);
    assert_eq!(resp.follower_count, 2);
    assert_eq!(hub.active_locked_presenter(), Some("sess-lead"));
    let alice = hub.session_state("sess-alice").expect("alice synced");
    assert_eq!(
        alice.camera_center_nm,
        Point2Nm::new(12_500_000, -4_000_000)
    );
    assert!((alice.zoom_scale - 2.5).abs() < 1e-9);
}

#[test]
fn breadboard_twin_roundtrips_nets_and_detects_strip_or_rail_shorts() {
    let net_counts = BTreeMap::from([
        ("VCC".to_string(), 3_u32),
        ("GND".to_string(), 3_u32),
        ("I2C_SDA".to_string(), 7_u32),
    ]);
    let clean_strips = build_breadboard_strips_from_netlist(&net_counts);
    let ok_req = SyncBreadboardTwinRequest {
        project_id: "proj-bb".into(),
        branch_id: "main".into(),
        strips: clean_strips,
    };
    let (ok_resp, _) = sync_breadboard_twin(&ok_req, 10);
    assert!(!ok_resp.short_circuit_detected);
    assert_eq!(ok_resp.synced_revision, 11);

    let short_req = SyncBreadboardTwinRequest {
        project_id: "proj-bb".into(),
        branch_id: "main".into(),
        strips: vec![
            BreadboardStripMapping {
                strip_id: "T_R05_AE".into(),
                net_name: "VCC".into(),
                occupied_hole_indices: vec![0, 1],
            },
            BreadboardStripMapping {
                strip_id: "T_R05_AE".into(),
                net_name: "GND".into(),
                occupied_hole_indices: vec![4],
            },
        ],
    };
    let (short_resp, _) = sync_breadboard_twin(&short_req, 11);
    assert!(short_resp.short_circuit_detected);
    assert_eq!(short_resp.synced_revision, 11);
}

#[test]
fn stamps_golden_snippet_with_refdes_offset_and_isolated_local_nets() {
    let tpl = GoldenSnippetTemplate {
        snippet_id: "snip-buck-stage".into(),
        components: vec![
            SnippetComponentTemplate {
                refdes_prefix: "U".into(),
                local_index: 1,
                upid: "urn:signal:part:tps62840:rev1".into(),
                relative_pos_nm: Point2Nm::new(0, 0),
                pin_nets: vec![
                    ("VIN".into(), "VCC".into()),
                    ("GND".into(), "GND".into()),
                    ("SW".into(), "SW_NODE".into()),
                ],
            },
            SnippetComponentTemplate {
                refdes_prefix: "L".into(),
                local_index: 1,
                upid: "urn:signal:part:ind-2u2:rev1".into(),
                relative_pos_nm: Point2Nm::new(5_000_000, 0),
                pin_nets: vec![
                    ("1".into(), "SW_NODE".into()),
                    ("2".into(), "VOUT_LOCAL".into()),
                ],
            },
        ],
        pre_routed_traces: vec![SnippetTraceTemplate {
            local_net_name: "SW_NODE".into(),
            start_rel_nm: Point2Nm::new(1_000_000, 0),
            end_rel_nm: Point2Nm::new(4_000_000, 0),
            width_nm: 400_000,
            layer_index: 0,
        }],
    };
    let req = StampGoldenSnippetRequest {
        snippet_id: "snip-buck-stage".into(),
        target_branch_id: "main".into(),
        anchor_nm: Some(Point2dNm {
            x_nm: 20_000_000,
            y_nm: 10_000_000,
        }),
        refdes_base_offset: 200,
    };
    let stamped = stamp_golden_snippet(&tpl, &req, 41).expect("stamp ok");
    assert_eq!(stamped.response.stamped_entity_count, 3);
    assert_eq!(stamped.response.new_revision, 42);
    assert_eq!(stamped.components[0].stamped_refdes, "U201");
    assert_eq!(stamped.components[1].stamped_refdes, "L201");
    assert_eq!(stamped.components[0].resolved_pin_nets[0].1, "VCC");
    assert_eq!(
        stamped.components[0].resolved_pin_nets[2].1,
        "SNIP_200_SW_NODE"
    );
    assert_eq!(stamped.traces[0].resolved_net_name, "SNIP_200_SW_NODE");
    assert_eq!(
        stamped.traces[0].start_world_nm,
        Point2Nm::new(21_000_000, 10_000_000)
    );
}
