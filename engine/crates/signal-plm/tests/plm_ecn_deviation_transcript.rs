// Module: engine/crates/signal-plm/tests/plm_ecn_deviation_transcript.rs — Golden ECN/Deviation Transcript & Multi-BOM/CTB Tests
// Invariants: Replays contracts/transcripts/ecn_deviation_sftp_gate.jsonl and verifies eBOM/mBOM/sBOM + AVL Clear-to-Build.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
use serde_json::Value;
use signal_parts::EnterpriseAvlCatalogEntry;
use signal_plm::{
    BomSourceDomain, InventoryLotPosition, MultiDomainBomCandidate, compile_multiview_bom,
    compute_eco_bom_redline_diff, evaluate_clear_to_build, evaluate_deviation_gates,
    transition_change_order,
};
use signal_proto::v1::{
    BomViewKind, ChangeNoticeKind, CompileMultiViewBomRequest, DeviationWaiverGate,
    EvaluateClearToBuildRequest, HybridPqcSignature, TransitionChangeOrderRequest,
    VerifyDeviationGateRequest,
};

#[test]
fn replays_golden_ecn_deviation_sftp_gate_transcript() {
    let raw = include_str!("../../../../contracts/transcripts/ecn_deviation_sftp_gate.jsonl");
    let active_gate = DeviationWaiverGate {
        deviation_id: "DEV-2026-0042".into(),
        authorized_qty_limit: 500,
        consumed_qty: 100,
        expires_at_utc_ms: 1_800_000_000_000,
        ccb_approval_sig: None,
    };

    for line in raw.lines().filter(|l| !l.trim().is_empty()) {
        let step: Value = serde_json::from_str(line).expect("valid jsonl line");
        let rpc = step["rpc"].as_str().expect("rpc string");
        let req_json = &step["request"];
        let exp_json = &step["expected_response"];

        match rpc {
            "signal.v1.PlmOpsService/TransitionChangeOrder" => {
                let kind_str = req_json["kind"].as_str().expect("kind");
                let kind =
                    ChangeNoticeKind::from_str_name(kind_str).expect("valid ChangeNoticeKind");
                let req = TransitionChangeOrderRequest {
                    org_id: req_json["org_id"].as_str().expect("org").into(),
                    project_id: req_json["project_id"].as_str().expect("proj").into(),
                    change_id: req_json["change_id"].as_str().expect("id").into(),
                    kind: kind as i32,
                    target_state: req_json["target_state"].as_str().expect("state").into(),
                    part11_signature: Some(HybridPqcSignature {
                        ed25519_sig: vec![0x11; 64],
                        ml_dsa_65_sig: vec![0x22; 128],
                        key_id: "ccb-chair-key-01".into(),
                        signed_at_utc_ms: 1_795_324_790_000,
                    }),
                };
                let exp_sha = exp_json["ops_repo_commit_sha"].as_str().expect("sha");
                let resp = transition_change_order(&req, Some(exp_sha)).expect("part11 transition");
                assert_eq!(resp.change_id, exp_json["change_id"].as_str().expect("id"));
                assert_eq!(
                    resp.current_state,
                    exp_json["current_state"].as_str().expect("state")
                );
                assert_eq!(resp.ops_repo_commit_sha, exp_sha);
            }
            "signal.v1.PlmOpsService/VerifyDeviationGate" => {
                let req = VerifyDeviationGateRequest {
                    org_id: req_json["org_id"].as_str().expect("org").into(),
                    project_id: req_json["project_id"].as_str().expect("proj").into(),
                    requested_release_qty: req_json["requested_release_qty"]
                        .as_str()
                        .expect("qty")
                        .parse::<u64>()
                        .expect("u64"),
                    evaluation_time_utc_ms: req_json["evaluation_time_utc_ms"]
                        .as_str()
                        .expect("ms")
                        .parse::<i64>()
                        .expect("i64"),
                };
                let resp = evaluate_deviation_gates(std::slice::from_ref(&active_gate), &req);
                assert_eq!(
                    resp.release_permitted,
                    exp_json["release_permitted"].as_bool().expect("bool")
                );
                let exp_blocking: Vec<String> = exp_json["blocking_deviation_ids"]
                    .as_array()
                    .expect("arr")
                    .iter()
                    .map(|v| v.as_str().expect("str").to_string())
                    .collect();
                assert_eq!(resp.blocking_deviation_ids, exp_blocking);
            }
            other => panic!("unexpected RPC in transcript: {other}"),
        }
    }
}

#[test]
fn compiles_ebom_mbom_sbom_digital_passport_and_solves_avl_ctb() {
    let candidates = vec![
        MultiDomainBomCandidate {
            upid: "urn:signal:part:tps62840:r1".into(),
            orderable_mpn: "TPS62840DLCR".into(),
            domain: BomSourceDomain::EcadPcb,
            base_qty: 2,
            scrap_allowance_pct: 50.0, // mBOM -> 3 units
            unit_cost_micros_usd: 850_000,
            embodied_co2e_mg: 42_000,
            rohs_reach_pfas_compliant: true,
            workstation_step_id: "SMT-LINE-1".into(),
            field_replaceable_spare: false,
            available_stock_units: 600,
        },
        MultiDomainBomCandidate {
            upid: "urn:signal:part:enc-machined-lid:r1".into(),
            orderable_mpn: "ENC-AL6061-LID".into(),
            domain: BomSourceDomain::McadMechanical,
            base_qty: 1,
            scrap_allowance_pct: 0.0,
            unit_cost_micros_usd: 14_500_000,
            embodied_co2e_mg: 1_850_000,
            rohs_reach_pfas_compliant: true,
            workstation_step_id: "FINAL-ASSY-OP20".into(),
            field_replaceable_spare: true,
            available_stock_units: 120,
        },
        MultiDomainBomCandidate {
            upid: "urn:signal:part:solder-sac305:r1".into(),
            orderable_mpn: "SAC305-PASTE-JAR".into(),
            domain: BomSourceDomain::ManufacturingConsumable,
            base_qty: 1,
            scrap_allowance_pct: 0.0,
            unit_cost_micros_usd: 120_000,
            embodied_co2e_mg: 9_000,
            rohs_reach_pfas_compliant: true,
            workstation_step_id: "SMT-STENCIL".into(),
            field_replaceable_spare: false,
            available_stock_units: 1_000,
        },
    ];

    let (ebom, ebom_dpp) = compile_multiview_bom(
        &CompileMultiViewBomRequest {
            org_id: "org-1".into(),
            project_id: "proj-1".into(),
            super_commit_id: "sc-001".into(),
            view: BomViewKind::Ebom as i32,
        },
        &candidates,
    )
    .expect("ebom");
    let (mbom, _) = compile_multiview_bom(
        &CompileMultiViewBomRequest {
            org_id: "org-1".into(),
            project_id: "proj-1".into(),
            super_commit_id: "sc-001".into(),
            view: BomViewKind::Mbom as i32,
        },
        &candidates,
    )
    .expect("mbom");
    let (sbom, _) = compile_multiview_bom(
        &CompileMultiViewBomRequest {
            org_id: "org-1".into(),
            project_id: "proj-1".into(),
            super_commit_id: "sc-001".into(),
            view: BomViewKind::Sbom as i32,
        },
        &candidates,
    )
    .expect("sbom");

    assert_eq!(ebom.lines.len(), 2);
    assert_eq!(mbom.lines.len(), 3);
    assert_eq!(sbom.lines.len(), 1);
    assert!(ebom_dpp.all_rohs_reach_pfas_compliant);

    let diff = compute_eco_bom_redline_diff(&ebom.lines, &mbom.lines);
    assert_eq!(diff.added_upids, vec!["urn:signal:part:solder-sac305:r1"]);
    assert_eq!(
        diff.modified_upids,
        vec![
            "urn:signal:part:enc-machined-lid:r1",
            "urn:signal:part:tps62840:r1"
        ]
    );

    // Verify Clear-to-Build nets primary MPN + approved AVL alternate stock!
    let ctb = evaluate_clear_to_build(
        &EvaluateClearToBuildRequest {
            org_id: "org-1".into(),
            project_id: "proj-1".into(),
            target_lot_size: 100,
        },
        &ebom.lines,
        &[
            InventoryLotPosition {
                mpn: "TPS62840DLCR".into(),
                on_hand_qty: 120, // only 60 units alone
                next_po_receipt_utc_ms: 1_800_000_000_000,
                incoming_po_qty: 500,
            },
            InventoryLotPosition {
                mpn: "TPS62842DLCR".into(),
                on_hand_qty: 100, // +50 units via qualified AVL alternate => 110 total!
                next_po_receipt_utc_ms: 1_800_000_000_000,
                incoming_po_qty: 0,
            },
            InventoryLotPosition {
                mpn: "ENC-AL6061-LID".into(),
                on_hand_qty: 105,
                next_po_receipt_utc_ms: 1_800_000_000_000,
                incoming_po_qty: 0,
            },
        ],
        &[EnterpriseAvlCatalogEntry {
            org_id: "org-1".into(),
            upid: "urn:signal:part:tps62840:r1".into(),
            orderable_mpn: "TPS62840DLCR".into(),
            approved_alternates: vec!["TPS62842DLCR".into()],
        }],
        1_795_000_000_000,
    );
    let report = ctb.report.expect("ctb report");
    assert_eq!(report.max_buildable_units, 105);
    assert!(report.gating_shortage_mpns.is_empty());
}
