// Module: engine/crates/signal-sftp-egress/tests/sftp_stepup_transcript.rs — Step-Up 2FA SFTP Transcript Replay Test
// Invariants: Replays sftp_stepup_2fa_release.jsonl; verifies SSRF block, atomic .tmp.part->rename, and burned TOTP replay reject.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, contracts/transcripts/sftp_stepup_2fa_release.jsonl

use serde::Deserialize;
use signal_proto::v1::{
    FencingAuthority, PushSftpReleaseRequest, ReleaseBundleKind, ReleaseSignoffApproval,
    StepUpMfaMethod,
};
use signal_sftp_egress::{
    DisposableSftpReleaseExecutor, SftpGateError, WriteOnlySshLockdownPolicy,
};
use std::collections::BTreeSet;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

#[derive(Deserialize)]
struct SftpTranscriptStep {
    seq: u32,
    request: serde_json::Value,
}

fn parse_push_sftp_json(v: &serde_json::Value) -> PushSftpReleaseRequest {
    let fencing: FencingAuthority = serde_json::from_value(v["fencing"].clone()).expect("fencing");
    let bundle_str = v["bundle_kind"].as_str().unwrap_or_default();
    let bundle_kind = ReleaseBundleKind::from_str_name(bundle_str)
        .unwrap_or(ReleaseBundleKind::Unspecified) as i32;
    let approvals = v["approvals"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|a| {
                    let mfa_str = a["mfa_method"].as_str().unwrap_or_default();
                    ReleaseSignoffApproval {
                        actor_pseudo_id: a["actor_pseudo_id"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                        approval_meaning: a["approval_meaning"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                        mfa_method: StepUpMfaMethod::from_str_name(mfa_str)
                            .unwrap_or(StepUpMfaMethod::Unspecified)
                            as i32,
                        burned_totp_or_webauthn_nonce: a["burned_totp_or_webauthn_nonce"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                        signature: None,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    PushSftpReleaseRequest {
        fencing: Some(fencing),
        idempotency_key: v["idempotency_key"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        release_id: v["release_id"].as_str().unwrap_or_default().to_string(),
        super_commit_id: v["super_commit_id"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        bundle_kind,
        sftp_target_id: v["sftp_target_id"].as_str().unwrap_or_default().to_string(),
        pinned_host_key_ed25519: v["pinned_host_key_ed25519"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        release_unit_quantity: v["release_unit_quantity"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        quorum_audit_outbox_seq: v["quorum_audit_outbox_seq"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        approvals,
        watermark_meta: None,
    }
}

#[test]
fn replays_sftp_stepup_2fa_transcript_and_blocks_ssrf_hostkey_or_forwarding() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../contracts/transcripts/sftp_stepup_2fa_release.jsonl");
    let raw = fs::read_to_string(path).expect("read transcript");
    let steps: Vec<SftpTranscriptStep> = raw
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("json"))
        .collect();
    assert_eq!(steps.len(), 3);
    assert_eq!(steps[1].seq, 2);

    let mut bad_policy = WriteOnlySshLockdownPolicy::STRICT_WRITE_ONLY;
    bad_policy.allow_pty_or_exec_shell = true;
    assert_eq!(
        DisposableSftpReleaseExecutor::new(bad_policy, [1; 32], [2; 32]).expect_err("pty"),
        SftpGateError::ForbiddenSshCapabilityEnabled
    );

    let req2 = parse_push_sftp_json(&steps[1].request);
    let req3 = parse_push_sftp_json(&steps[2].request);

    let mut burned = BTreeSet::new();
    let pub_ip = IpAddr::V4(Ipv4Addr::new(198, 51, 100, 42));

    let mut exec_ssrf = DisposableSftpReleaseExecutor::new(
        WriteOnlySshLockdownPolicy::STRICT_WRITE_ONLY,
        [0xAA; 32],
        [0xBB; 32],
    )
    .expect("exec");
    assert!(matches!(
        exec_ssrf.execute_one_shot_push(
            &req2,
            &mut burned,
            IpAddr::V4(Ipv4Addr::new(169, 254, 169, 254)),
            &req2.pinned_host_key_ed25519,
            b"",
            None,
            1_795_324_801_800,
        ),
        Err(SftpGateError::SsrfDestinationBlocked(_))
    ));

    let (resp2, trace2) = exec_ssrf
        .execute_one_shot_push(
            &req2,
            &mut burned,
            pub_ip,
            &req2.pinned_host_key_ed25519,
            b"",
            None,
            1_795_324_801_800,
        )
        .expect("push ok");
    assert!(resp2.atomic_rename_completed && trace2.ssh_key_zeroized && exec_ssrf.is_disposed());
    assert_eq!(
        resp2.remote_verified_sha256,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );

    let mut exec_replay = DisposableSftpReleaseExecutor::new(
        WriteOnlySshLockdownPolicy::STRICT_WRITE_ONLY,
        [0xCC; 32],
        [0xDD; 32],
    )
    .expect("exec2");
    assert_eq!(
        exec_replay
            .execute_one_shot_push(
                &req3,
                &mut burned,
                pub_ip,
                &req3.pinned_host_key_ed25519,
                b"gerber",
                None,
                1_795_324_803_000,
            )
            .expect_err("replayed totp"),
        SftpGateError::TotpNonceAlreadyBurned("totp_window_59844160_code_482910".into())
    );
}
