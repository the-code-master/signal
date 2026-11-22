// Module: engine/crates/signal-pod/tests/ga_148_flaws_and_120_features.rs — End-to-End GA Security, Physics & Feature Suite
// Invariants: Verifies zombie fencing, SSRF IMDS block, <150ms migration, Padme rollback, H100/B200 CC, WASM policy, MPW & Campus Edge.
// Spec: architecture/15-application-security-and-project-isolation.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use signal_core::{HybridSignature, SecretBox, det_cos, det_sin, sha512_256_hex};
use signal_egress_proxy::SocketPinnedEgressGate;
use signal_git_sync::{
    PortfolioProjectNode, QueuedOfflineSuperCommit, analyze_portfolio_blast_radius,
    seal_padme_vault_blob, sign_vault_merkle_manifest, sync_campus_edge_store_forward,
    verify_vault_merkle_manifest,
};
use signal_ic::{ShuttleSubmissionMetrics, validate_mpw_shuttle_tile};
use signal_operator::{
    DEFAULT_MAX_CUTOVER_FREEZE_PS, HardwareGpuSku, OperatorError, ThreePhaseMigrationController,
    validate_confidential_gpu_schedule,
};
use signal_pod::wasm_policy_runner::{
    DeclarativePolicyRule, PodPolicyError, PolicyTargetDesignSnapshot, evaluate_wasm_policy_pack,
};
use signal_pod::watermark::{
    build_constellation_watermark, embed_tardos_rgba8_tile, generate_tardos_watermark,
    score_tardos_accusation,
};
use signal_proto::v1::{
    AnalyzePortfolioBlastRadiusRequest, ConfidentialGpuMode, EvaluateWasmPolicyPackRequest,
    FencingAuthority, GradeAcademyLabRequest, HybridPqcSignature, OpenSiliconPdkKind,
    ProjectVaultMode, QueryEnterpriseFinOpsResponse, SignedWasmPolicyManifest,
    SyncCampusEdgeStoreForwardRequest, ThreePhaseLiveMigrateRequest, ValidateMpwShuttleTileRequest,
    VerifyEgressFencingTokenRequest,
};
use signal_sim::{
    StudentCircuitSubmission, derive_seeded_target_divider_ratio, grade_academy_circuit_lab,
};

#[test]
fn ga_suite_verifies_ssrf_fencing_migration_padme_gpu_cc_and_core_math() {
    // 1. F-089 + F-119: Socket-pinned IMDSv2 / IPv4-mapped IPv6 block + Zombie pod stale fencing rejection
    let mut gate = SocketPinnedEgressGate::default();
    let f1 = FencingAuthority {
        org_id: "org-ga".into(),
        project_id: "proj-ga".into(),
        fencing_epoch: 50,
        lease_holder_pod_id: "pod-a".into(),
        issued_at_utc_ms: 1_000,
        expires_at_utc_ms: 90_000,
    };
    let imds_block = gate.verify_outbound_connect(
        &VerifyEgressFencingTokenRequest {
            fencing: Some(f1.clone()),
            target_host: "github.com".into(),
            resolved_socket_ip: "::ffff:169.254.169.254".into(),
            provider_repo_id: "gh-1".into(),
        },
        5_000,
    );
    assert!(!imds_block.allowed);

    gate.advance_project_epoch("proj-ga", 51);
    let zombie = gate.verify_outbound_connect(
        &VerifyEgressFencingTokenRequest {
            fencing: Some(f1),
            target_host: "github.com".into(),
            resolved_socket_ip: "140.82.112.4".into(),
            provider_repo_id: "gh-1".into(),
        },
        5_000,
    );
    assert!(!zombie.allowed);
    assert_eq!(zombie.denial_reason_code, "STALE_FENCING_EPOCH_412");

    // 2. F-118 + F-123: Hopper H100 MIG+CC rejected; Blackwell B200 TEE-I/O MIG accepted; < 150ms live migration
    assert_eq!(
        validate_confidential_gpu_schedule(
            HardwareGpuSku::NvidiaH100Hopper,
            ConfidentialGpuMode::H100FullCcExclusive,
            true,
        ),
        Err(OperatorError::IllegalHopperMigConfidentialCombo)
    );
    assert!(
        validate_confidential_gpu_schedule(
            HardwareGpuSku::NvidiaB200Blackwell,
            ConfidentialGpuMode::B200TeeIoConfidentialMig,
            true,
        )
        .is_ok()
    );

    let mut mig = ThreePhaseMigrationController::new("proj-ga", "node-1", 51, 32_768, 40.0);
    mig.run_phase1_precopy_round(0.05);
    let cutover = mig
        .complete_cutover(&ThreePhaseLiveMigrateRequest {
            next_fencing: Some(FencingAuthority {
                org_id: "org-ga".into(),
                project_id: "proj-ga".into(),
                fencing_epoch: 52,
                lease_holder_pod_id: "pod-b".into(),
                issued_at_utc_ms: 6_000,
                expires_at_utc_ms: 95_000,
            }),
            destination_node_id: "node-2".into(),
            max_cutover_freeze_ps: DEFAULT_MAX_CUTOVER_FREEZE_PS,
        })
        .expect("sub-150ms cutover");
    assert!(cutover.observed_cutover_freeze_ps <= DEFAULT_MAX_CUTOVER_FREEZE_PS);

    // 3. F-117 + F-033 + F-080/F-128: Padme rollback block, 0.5-ULP trig identity, and Tardos/Constellation watermark
    let dek = SecretBox::new([0x5A; 32]);
    let cur_blob = seal_padme_vault_blob(&dek, 2, "pcb/top.pcb", b"rev2").expect("seal");
    let old_blob = seal_padme_vault_blob(&dek, 1, "pcb/top.pcb", b"rev1").expect("old");
    let s_ed = SecretBox::new([0x11; 32]);
    let s_pq = SecretBox::new([0x22; 32]);
    let (vk_ed, vk_pq) = HybridSignature::derive_verifier_pair(&s_ed, &s_pq);
    let merkle = sign_vault_merkle_manifest(std::slice::from_ref(&cur_blob.meta), &s_ed, &s_pq);
    assert!(verify_vault_merkle_manifest(&merkle, &[old_blob.meta], &vk_ed, &vk_pq).is_err());

    for k in 0..16 {
        let th = (k as f64) * 0.37;
        let pyth = det_sin(th).powi(2) + det_cos(th).powi(2);
        assert!((pyth - 1.0).abs() < 1e-14);
    }

    let wm_leaker = generate_tardos_watermark(&[0x77; 32], "sess-leaker", 128);
    let wm_innocent = generate_tardos_watermark(&[0x77; 32], "sess-clean", 128);
    let clean_rgba = vec![128_u8; 4 * 512];
    let mut leaked_rgba = clean_rgba.clone();
    embed_tardos_rgba8_tile(&mut leaked_rgba, &wm_leaker.user_codeword_bits);
    assert!(
        score_tardos_accusation(&leaked_rgba, &clean_rgba, &wm_leaker)
            > score_tardos_accusation(&leaked_rgba, &clean_rgba, &wm_innocent) + 5.0
    );
    let (_, shifts) = build_constellation_watermark(
        &[0x77; 32],
        "ep-1",
        "tile-0",
        &["U1".into(), "J2".into()],
        128,
    );
    assert_eq!(shifts.len(), 2);
}

#[test]
fn ga_suite_verifies_wasm_policy_portfolio_mpw_academy_campus_edge_and_phase8_fixture() {
    // 1. FEAT-117: Signed WASM Policy-as-Code execution + fuel exhaustion guard
    let wasm_bytes = b"\0asm\x01\0\0\0signal-stackup-policy";
    let wasm_hash = sha512_256_hex(wasm_bytes);
    let s_ed = SecretBox::new([0x31; 32]);
    let s_pq = SecretBox::new([0x32; 32]);
    let (vk_ed, vk_pq) = HybridSignature::derive_verifier_pair(&s_ed, &s_pq);

    let mut signable = Vec::new();
    signable.extend_from_slice(b"pol_mil_aero_v1");
    signable.extend_from_slice(b"1.0.0");
    signable.extend_from_slice(wasm_hash.as_bytes());
    let sig = HybridSignature::sign_deterministic(&s_ed, &s_pq, &signable);

    let manifest = SignedWasmPolicyManifest {
        policy_pack_id: "pol_mil_aero_v1".into(),
        version: "1.0.0".into(),
        fuel_budget_instructions: 10_000,
        max_linear_memory_kib: 4096,
        wasm_module_blake3: wasm_hash,
        org_admin_signature: Some(HybridPqcSignature {
            ed25519_sig: sig.ed25519_sig.to_vec(),
            ml_dsa_65_sig: sig.ml_dsa_65_sig,
            key_id: "org-admin-01".into(),
            signed_at_utc_ms: 1_795_327_000_000,
        }),
    };
    let rules = [
        DeclarativePolicyRule::MinPcbLayerCount(6),
        DeclarativePolicyRule::RequirePowerNetPrefix("PWR_".into()),
        DeclarativePolicyRule::RequireEncryptedVaultMode,
    ];
    let snap = PolicyTargetDesignSnapshot {
        pcb_layer_count: 8,
        power_net_names: vec!["PWR_3V3".into(), "PWR_0V8".into()],
        min_observed_hv_clearance_nm: 2_500_000,
        vault_mode: ProjectVaultMode::EncryptedVault,
        peak_junction_temp_mkelvin: 358_000,
        requested_linear_memory_kib: 1024,
    };
    let pol_ok = evaluate_wasm_policy_pack(
        &EvaluateWasmPolicyPackRequest {
            fencing: None,
            branch_id: "main".into(),
            manifest: Some(manifest.clone()),
        },
        wasm_bytes,
        (&vk_ed, &vk_pq),
        &rules,
        &snap,
    )
    .expect("wasm policy ok");
    assert!(pol_ok.policy_compliant);

    let mut low_fuel = manifest;
    low_fuel.fuel_budget_instructions = 300;
    assert_eq!(
        evaluate_wasm_policy_pack(
            &EvaluateWasmPolicyPackRequest {
                fencing: None,
                branch_id: "main".into(),
                manifest: Some(low_fuel),
            },
            wasm_bytes,
            (&vk_ed, &vk_pq),
            &rules,
            &snap,
        ),
        Err(PodPolicyError::FuelExhausted { limit: 300 })
    );

    // 2. FEAT-118 + FEAT-120: Portfolio Blast-Radius DAG + Campus Edge Store-and-Forward Sync
    let blast = analyze_portfolio_blast_radius(
        &AnalyzePortfolioBlastRadiusRequest {
            org_id: "org-1".into(),
            affected_mpn_or_ip_id: "STM32H753VIT6".into(),
        },
        &[
            PortfolioProjectNode {
                org_id: "org-1".into(),
                project_id: "core-som".into(),
                annual_revenue_micros_usd: 2_000_000_000_000,
                locked_mpns_and_ips: vec!["STM32H753VIT6".into()],
                upstream_project_dependencies: vec![],
            },
            PortfolioProjectNode {
                org_id: "org-1".into(),
                project_id: "flight-ctrl".into(),
                annual_revenue_micros_usd: 5_500_000_000_000,
                locked_mpns_and_ips: vec![],
                upstream_project_dependencies: vec!["core-som".into()],
            },
        ],
    );
    assert_eq!(blast.impacted_project_ids, vec!["core-som", "flight-ctrl"]);
    assert_eq!(blast.transitive_depth, 2);
    assert_eq!(
        blast.estimated_revenue_at_risk_micros_usd,
        7_500_000_000_000
    );

    let sync = sync_campus_edge_store_forward(
        &SyncCampusEdgeStoreForwardRequest {
            campus_appliance_id: "campus-nairobi-01".into(),
            queued_super_commit_count: 2,
            offline_duration_ms: 14_400_000,
        },
        &[
            QueuedOfflineSuperCommit {
                super_commit_id: "sc-1".into(),
                queued_at_utc_ms: 1_000,
                raw_blob_payloads: vec![vec![0xAB; 4096]],
            },
            QueuedOfflineSuperCommit {
                super_commit_id: "sc-2".into(),
                queued_at_utc_ms: 2_000,
                raw_blob_payloads: vec![vec![0xAB; 4096], vec![0xCD; 2048]],
            },
        ],
        true,
    );
    assert!(sync.upstream_reconciled);
    assert_eq!(sync.synced_super_commit_count, 2);

    // 3. FEAT-115 + FEAT-112: Sky130 MPW Shuttle + LTI 1.3 Lab Auto-Grader + Phase 8 BigInt Fixture
    let mpw = validate_mpw_shuttle_tile(
        &ValidateMpwShuttleTileRequest {
            project_id: "tt09-alu".into(),
            pdk: OpenSiliconPdkKind::SkywaterSky130 as i32,
            tile_slot_preset: "1x1".into(),
            die_width_nm: 160_000,
            die_height_nm: 100_000,
        },
        ShuttleSubmissionMetrics {
            drc_violations: 0,
            lvs_mismatches: 0,
            max_gate_antenna_ratio: 180.0,
            metal_density_pct: 48.5,
            io_pin_pitch_nm: 6_800,
        },
    )
    .expect("sky130 mpw");
    assert!(mpw.precheck_drc_lvs_antenna_clean && mpw.pin_ring_harness_aligned);

    let ratio = derive_seeded_target_divider_ratio(42, "stu_1");
    let lab = grade_academy_circuit_lab(
        &GradeAcademyLabRequest {
            lab_assignment_id: "LAB-1".into(),
            student_pseudo_id: "stu_1".into(),
            submission_branch_id: "b1".into(),
            randomized_rubric_seed: 42,
        },
        StudentCircuitSubmission {
            r_top_ohm: 10_000.0 * (1.0 - ratio),
            r_bot_ohm: 10_000.0 * ratio,
            filter_cap_f: 22e-9,
        },
        &s_ed,
        &s_pq,
        1_795_327_100_000,
    );
    assert_eq!(lab.passed_fault_rubrics, 5);

    let raw_fix = include_str!("../../../../contracts/fixtures/academy_finops_policy.fixture.json");
    let root: serde_json::Value = serde_json::from_str(raw_fix).expect("fixture json");
    let parsed_pol: SignedWasmPolicyManifest =
        serde_json::from_value(root["wasm_policy_manifest"].clone()).expect("pol deser");
    let parsed_fin: QueryEnterpriseFinOpsResponse =
        serde_json::from_value(root["finops_summary"].clone()).expect("finops deser");
    assert_eq!(
        parsed_pol.fuel_budget_instructions,
        9_007_199_254_740_993_u64
    );
    assert_eq!(
        parsed_fin.accrued_spend_micros_usd,
        9_007_199_254_740_997_i64
    );
}
