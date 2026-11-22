// Module: engine/crates/signal-git-sync/src/lib.rs — BYO-Git Multi-Provider Sync, Padme SIV Vault & 2PC Super-Commit
// Invariants: Blinds ciphertext lengths via Padme (<= 12% overhead, F-117); commits <project>-meta CAS FIRST (I-12, F-119).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/13-vcs-git-server-and-storage-analysis.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod campus_edge_sync;
pub mod history_scrubber;
pub mod padme_vault;
pub mod portfolio_dag;
pub mod provider_lfs;
pub mod super_commit;

pub use campus_edge_sync::{QueuedOfflineSuperCommit, sync_campus_edge_store_forward};
pub use history_scrubber::{CommitGraphSnapshot, scrub_history_timeline};
pub use padme_vault::{
    SealedPadmeBlob, VaultMerkleManifest, blind_vault_entity_path, compute_vault_merkle_root,
    enforce_must_encrypt_taint, open_padme_vault_blob, padme_padded_len, seal_padme_vault_blob,
    sign_vault_merkle_manifest, verify_vault_merkle_manifest,
};
pub use portfolio_dag::{PortfolioProjectNode, analyze_portfolio_blast_radius};
pub use provider_lfs::{
    BloblessClonePlan, LFS_STREAM_CHUNK_BYTES, LfsChunkFrame, MAX_LAZY_REPO_CREATIONS_PER_MIN,
    chunk_lfs_stream_64kb, plan_blobless_clone, reassemble_lfs_chunks,
    reconcile_external_push_3way,
};
pub use super_commit::{
    ContinuityWalRecord, CryptoShredReceipt, SimulatedRemoteRepo, SuperCommitCoordinator,
    seal_byok_continuity_wal, shred_continuity_wal_with_receipt, verify_super_commit_fencing,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GitSyncError {
    #[error("stale fencing epoch {provided} < active {active} on Super-Commit")]
    StaleFencingEpoch { provided: u64, active: u64 },
    #[error("invalid BYO-Git repository binding: {0}")]
    InvalidRepositoryBinding(String),
    #[error("MUST_ENCRYPT taint forbids PLAIN_CANONICAL vault mode")]
    MustEncryptTaintViolation,
    #[error("Padme ENCRYPTED_VAULT integrity or rollback check failed: {0}")]
    VaultIntegrityFailure(String),
    #[error("<project>-meta CAS conflict: expected `{expected_oid}`, found `{actual_oid}`")]
    MetaRepoCasConflict {
        expected_oid: String,
        actual_oid: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::{HybridSignature, SecretBox};
    use signal_proto::v1::{
        BoundRepositoryRef, ExecuteSuperCommitRequest, FencingAuthority, GitNamespaceKind,
        GitProviderKind, ProjectVaultMode, ReconcileExternalPushRequest,
        ScrubHistoryTimelineRequest, SubprojectDomain,
    };
    use std::collections::BTreeMap;

    #[test]
    fn padme_siv_vault_blinds_paths_and_blocks_single_blob_rollback_swap() {
        for len in [1_u64, 8, 9, 65, 1025, 37_120, 1_048_577] {
            let padded = padme_padded_len(len);
            assert!(padded >= len);
            if len >= 64 {
                assert!(padded * 100 <= len * 113);
            }
        }
        assert_eq!(
            enforce_must_encrypt_taint(ProjectVaultMode::PlainCanonical, true),
            Err(GitSyncError::MustEncryptTaintViolation)
        );

        let dek = SecretBox::new([0x42_u8; 32]);
        let b1 = seal_padme_vault_blob(&dek, 1, "schematic/secret_asic.sch", b"netlist_v1")
            .expect("seal1");
        let b2 =
            seal_padme_vault_blob(&dek, 1, "pcb/secret_board.pcb", b"layout_v1").expect("seal2");
        assert!(b1.meta.blinded_blob_path.starts_with("blobs/"));
        assert!(!b1.meta.blinded_blob_path.contains("secret_asic"));
        assert_eq!(
            open_padme_vault_blob(&dek, &b1).expect("open"),
            b"netlist_v1"
        );

        let s_ed = SecretBox::new([0x11_u8; 32]);
        let s_pq = SecretBox::new([0x22_u8; 32]);
        let (vk_ed, vk_pq) = HybridSignature::derive_verifier_pair(&s_ed, &s_pq);
        let manifest =
            sign_vault_merkle_manifest(&[b1.meta.clone(), b2.meta.clone()], &s_ed, &s_pq);
        assert!(
            verify_vault_merkle_manifest(&manifest, &[b2.meta.clone(), b1.meta], &vk_ed, &vk_pq)
                .is_ok()
        );

        // Single-blob rollback swap -> detected and rejected!
        let old_b1 = seal_padme_vault_blob(&dek, 1, "schematic/secret_asic.sch", b"netlist_v0_old")
            .expect("old");
        assert!(
            verify_vault_merkle_manifest(&manifest, &[old_b1.meta, b2.meta], &vk_ed, &vk_pq)
                .is_err()
        );
    }

    #[test]
    fn two_phase_super_commit_cas_crash_recovery_lfs_rebase_and_scrubber_work() {
        let mut coord = SuperCommitCoordinator::new(
            100,
            SecretBox::new([0x31_u8; 32]),
            SecretBox::new([0x32_u8; 32]),
        );
        coord.register_repo("gh_meta_1", true, "aaaa0000");
        coord.register_repo("gh_sch_2", false, "bbbb0000");

        let repos = vec![
            BoundRepositoryRef {
                provider: GitProviderKind::Github as i32,
                namespace_kind: GitNamespaceKind::OrgNamespace as i32,
                provider_repo_id: "gh_meta_1".to_string(),
                display_full_name: "acme/apollo-meta".to_string(),
                domain: SubprojectDomain::Meta as i32,
                vault_mode: ProjectVaultMode::EncryptedVault as i32,
            },
            BoundRepositoryRef {
                provider: GitProviderKind::Github as i32,
                namespace_kind: GitNamespaceKind::OrgNamespace as i32,
                provider_repo_id: "gh_sch_2".to_string(),
                display_full_name: "acme/apollo-schematic".to_string(),
                domain: SubprojectDomain::Schematic as i32,
                vault_mode: ProjectVaultMode::EncryptedVault as i32,
            },
        ];
        let plan = plan_blobless_clone(&repos[0]).expect("clone plan");
        assert!(plan.git_argv.iter().any(|a| a == "--filter=blob:none"));

        let req = ExecuteSuperCommitRequest {
            fencing: Some(FencingAuthority {
                org_id: "org_1".to_string(),
                project_id: "proj_1".to_string(),
                fencing_epoch: 100,
                ..Default::default()
            }),
            idempotency_key: "idem_1".to_string(),
            commit_message: "feat: initial super-commit".to_string(),
            author_pseudo_id: "usr_1".to_string(),
            target_repos: repos,
            vault_blobs: Vec::new(),
        };

        // CAS mismatch on <project>-meta aborts cleanly without advancing member branch!
        assert!(
            coord
                .execute_super_commit(&req, Some("wrong_oid"), 1_000)
                .is_err()
        );
        assert_eq!(
            coord.repo_state("gh_sch_2").expect("sch").main_head_oid,
            "bbbb0000"
        );

        // Matching <project>-meta CAS commits atomically across both repos!
        let ok = coord
            .execute_super_commit(&req, Some("aaaa0000"), 1_000)
            .expect("2pc ok");
        assert_eq!(ok.member_commits.len(), 2);
        assert_ne!(
            coord.repo_state("gh_sch_2").expect("sch").main_head_oid,
            "bbbb0000"
        );

        // 64KB LFS chunking + 3-way external push rebase + history scrubber + BYOK WAL shred
        let data = vec![0x5A_u8; 150_000];
        let chunks = chunk_lfs_stream_64kb(&data);
        assert_eq!(chunks.len(), 3);
        assert_eq!(reassemble_lfs_chunks(&chunks).expect("lfs"), data);

        let base = BTreeMap::from([("R1".into(), "10k".into())]);
        let local = BTreeMap::from([("R1".into(), "10k".into()), ("C1".into(), "100n".into())]);
        let ext = BTreeMap::from([("R1".into(), "22k".into())]);
        let (reb, merged) = reconcile_external_push_3way(
            &ReconcileExternalPushRequest {
                fencing: req.fencing.clone(),
                provider_repo_id: "gh_sch_2".into(),
                external_head_commit_sha: "ext123".into(),
            },
            100,
            &base,
            &local,
            &ext,
        )
        .expect("3way");
        assert!(reb.auto_rebased_cleanly);
        assert_eq!(merged.get("R1").map(String::as_str), Some("22k"));
        assert_eq!(merged.get("C1").map(String::as_str), Some("100n"));

        let timeline = scrub_history_timeline(
            &ScrubHistoryTimelineRequest {
                project_id: "proj_1".into(),
                max_commits: 10,
            },
            &[
                CommitGraphSnapshot {
                    super_commit_id: "c1".into(),
                    summary: "init".into(),
                    committed_at_utc_ms: 1,
                    entity_hashes: base,
                },
                CommitGraphSnapshot {
                    super_commit_id: "c2".into(),
                    summary: "add C1".into(),
                    committed_at_utc_ms: 2,
                    entity_hashes: merged,
                },
            ],
        );
        assert_eq!(timeline.entries[0].super_commit_id, "c2");
        assert_eq!(timeline.entries[0].changed_entity_count, 2);

        let wal = seal_byok_continuity_wal("wal_1", &SecretBox::new([0x77_u8; 32]), b"ops", 500);
        let receipt = shred_continuity_wal_with_receipt(
            wal,
            &SecretBox::new([0x11_u8; 32]),
            &SecretBox::new([0x22_u8; 32]),
            600,
        );
        assert_eq!(receipt.wal_id, "wal_1");
    }
}
