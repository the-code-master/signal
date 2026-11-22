// Module: engine/crates/signal-git-sync/src/super_commit.rs — Dual-Fenced 2PC Super-Commit & BYOK Continuity WAL
// Invariants: Prepares `refs/signal/txn/<id>` -> commits `<project>-meta` CAS lockfile FIRST -> advances member branches (F-011, F-119, F-141, I-12).
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md
use crate::GitSyncError;
use crate::padme_vault::compute_vault_merkle_root;
use signal_core::{HybridSignature, SecretBox, hmac_sha512, sha512_256_hex};
use signal_proto::v1::{
    ExecuteSuperCommitRequest, ExecuteSuperCommitResponse, FencingAuthority, HybridPqcSignature,
    RepoCommitEntry,
};
use std::collections::{BTreeMap, HashMap};

pub fn verify_super_commit_fencing(
    fencing: &FencingAuthority,
    min_epoch: u64,
) -> Result<u64, GitSyncError> {
    if fencing.fencing_epoch < min_epoch {
        return Err(GitSyncError::StaleFencingEpoch {
            provided: fencing.fencing_epoch,
            active: min_epoch,
        });
    }
    Ok(fencing.fencing_epoch)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulatedRemoteRepo {
    pub provider_repo_id: String,
    pub is_meta_repo: bool,
    pub main_head_oid: String,
    pub staged_txn_refs: BTreeMap<String, String>,
    pub committed_lock_txn_ids: Vec<String>,
}

/// Dual-Fenced Two-Phase Cross-Repo `Super-Commit` Coordinator (`F-011`, `F-119`, `I-12`).
#[derive(Debug)]
pub struct SuperCommitCoordinator {
    active_epoch: u64,
    repos: BTreeMap<String, SimulatedRemoteRepo>,
    idempotency_cache: HashMap<String, ExecuteSuperCommitResponse>,
    ed25519_seed: SecretBox<[u8; 32]>,
    ml_dsa_seed: SecretBox<[u8; 32]>,
}

impl SuperCommitCoordinator {
    pub fn new(
        initial_epoch: u64,
        ed25519_seed: SecretBox<[u8; 32]>,
        ml_dsa_seed: SecretBox<[u8; 32]>,
    ) -> Self {
        Self {
            active_epoch: initial_epoch,
            repos: BTreeMap::new(),
            idempotency_cache: HashMap::new(),
            ed25519_seed,
            ml_dsa_seed,
        }
    }

    pub fn register_repo(&mut self, provider_repo_id: &str, is_meta_repo: bool, initial_oid: &str) {
        self.repos.insert(
            provider_repo_id.to_string(),
            SimulatedRemoteRepo {
                provider_repo_id: provider_repo_id.to_string(),
                is_meta_repo,
                main_head_oid: initial_oid.to_string(),
                staged_txn_refs: BTreeMap::new(),
                committed_lock_txn_ids: Vec::new(),
            },
        );
    }

    pub fn repo_state(&self, provider_repo_id: &str) -> Option<&SimulatedRemoteRepo> {
        self.repos.get(provider_repo_id)
    }

    /// Executes 2PC `Super-Commit`:
    /// 1) Gate 1/2 fencing check,
    /// 2) Stage `refs/signal/txn/<tx_id>` on all member repos,
    /// 3) CAS-commit `.signal/repos.lock.json` on `<project>-meta` FIRST,
    /// 4) Advance member branch heads and prune staging refs.
    pub fn execute_super_commit(
        &mut self,
        req: &ExecuteSuperCommitRequest,
        expected_meta_oid: Option<&str>,
        now_utc_ms: i64,
    ) -> Result<ExecuteSuperCommitResponse, GitSyncError> {
        let fencing = req
            .fencing
            .as_ref()
            .ok_or_else(|| GitSyncError::InvalidRepositoryBinding("missing fencing".to_string()))?;
        self.active_epoch = verify_super_commit_fencing(fencing, self.active_epoch)?;

        if !req.idempotency_key.is_empty()
            && let Some(cached) = self.idempotency_cache.get(&req.idempotency_key)
        {
            return Ok(cached.clone());
        }
        if req.target_repos.is_empty() {
            return Err(GitSyncError::InvalidRepositoryBinding(
                "Super-Commit requires at least <project>-meta target repo".to_string(),
            ));
        }

        let (merkle_root, _) = compute_vault_merkle_root(&req.vault_blobs);
        let total_padded: u64 = req.vault_blobs.iter().map(|b| b.padme_padded_bytes).sum();
        let dek_ver = req.vault_blobs.first().map_or(1, |b| b.dek_version);
        let txn_id = format!(
            "sc_{}",
            &sha512_256_hex(
                format!(
                    "{}:{}:{}",
                    fencing.project_id, fencing.fencing_epoch, req.commit_message
                )
                .as_bytes()
            )[..16]
        );
        let txn_ref = format!("refs/signal/txn/{txn_id}");

        // Locate <project>-meta repo (first repo ending with `-meta` or first target repo)
        let meta_repo_id = req
            .target_repos
            .iter()
            .find(|r| r.display_full_name.ends_with("-meta"))
            .unwrap_or(&req.target_repos[0])
            .provider_repo_id
            .clone();

        // Phase 1: Stage refs/signal/txn/<txn_id> across all target repos
        let mut prepared_oids: BTreeMap<String, String> = BTreeMap::new();
        for target in &req.target_repos {
            let entry = self
                .repos
                .entry(target.provider_repo_id.clone())
                .or_insert_with(|| SimulatedRemoteRepo {
                    provider_repo_id: target.provider_repo_id.clone(),
                    is_meta_repo: target.provider_repo_id == meta_repo_id,
                    main_head_oid: "0000000000000000000000000000000000000000".to_string(),
                    staged_txn_refs: BTreeMap::new(),
                    committed_lock_txn_ids: Vec::new(),
                });
            let new_oid = sha512_256_hex(
                format!(
                    "{}:{}:{}:{}",
                    entry.main_head_oid, target.provider_repo_id, merkle_root, txn_id
                )
                .as_bytes(),
            )[..40]
                .to_string();
            entry
                .staged_txn_refs
                .insert(txn_ref.clone(), new_oid.clone());
            prepared_oids.insert(target.provider_repo_id.clone(), new_oid);
        }

        // Phase 2A (Commit Point): Strict CAS on <project>-meta `.signal/repos.lock.json` FIRST!
        let meta_state = self
            .repos
            .get_mut(&meta_repo_id)
            .ok_or_else(|| GitSyncError::InvalidRepositoryBinding(meta_repo_id.clone()))?;
        if let Some(exp_oid) = expected_meta_oid
            && meta_state.main_head_oid != exp_oid
        {
            let actual = meta_state.main_head_oid.clone();
            // Abort all prepared txn refs on CAS mismatch
            for repo in self.repos.values_mut() {
                repo.staged_txn_refs.remove(&txn_ref);
            }
            return Err(GitSyncError::MetaRepoCasConflict {
                expected_oid: exp_oid.to_string(),
                actual_oid: actual,
            });
        }
        let meta_new_oid = prepared_oids[&meta_repo_id].clone();
        meta_state.main_head_oid = meta_new_oid.clone();
        meta_state.committed_lock_txn_ids.push(txn_id.clone());
        meta_state.staged_txn_refs.remove(&txn_ref);

        // Phase 2B: Fast-forward remaining member repos and prune staging refs
        let mut member_commits = Vec::with_capacity(req.target_repos.len());
        for target in &req.target_repos {
            let oid = prepared_oids[&target.provider_repo_id].clone();
            if target.provider_repo_id != meta_repo_id
                && let Some(r) = self.repos.get_mut(&target.provider_repo_id)
            {
                r.main_head_oid = oid.clone();
                r.staged_txn_refs.remove(&txn_ref);
            }
            member_commits.push(RepoCommitEntry {
                provider_repo_id: target.provider_repo_id.clone(),
                commit_sha: oid,
                vault_merkle_root: merkle_root.clone(),
                dek_version: dek_ver,
                padme_padded_bytes: total_padded,
            });
        }

        let sig = HybridSignature::sign_deterministic(
            &self.ed25519_seed,
            &self.ml_dsa_seed,
            format!("{txn_id}:{meta_new_oid}:{merkle_root}").as_bytes(),
        );
        let resp = ExecuteSuperCommitResponse {
            super_commit_id: txn_id,
            fencing_epoch: self.active_epoch,
            meta_repo_commit_sha: meta_new_oid,
            member_commits,
            manifest_signature: Some(HybridPqcSignature {
                ed25519_sig: sig.ed25519_sig.to_vec(),
                ml_dsa_65_sig: sig.ml_dsa_65_sig,
                key_id: "key_hybrid_ed25519_mldsa65_v1".to_string(),
                signed_at_utc_ms: now_utc_ms,
            }),
            committed_at_utc_ms: now_utc_ms,
        };
        if !req.idempotency_key.is_empty() {
            self.idempotency_cache
                .insert(req.idempotency_key.clone(), resp.clone());
        }
        Ok(resp)
    }

    /// Crash-recovery reconciliation (`F-011`): rolls forward staged `refs/signal/txn/<txn_id>` if `<project>-meta`
    /// committed `txn_id`, or aborts/prunes staged refs if `<project>-meta` did NOT commit `txn_id`.
    pub fn recover_interrupted_txn(&mut self, meta_repo_id: &str, txn_id: &str) -> bool {
        let txn_ref = format!("refs/signal/txn/{txn_id}");
        let committed_in_meta = self
            .repos
            .get(meta_repo_id)
            .is_some_and(|m| m.committed_lock_txn_ids.iter().any(|id| id == txn_id));

        for repo in self.repos.values_mut() {
            if let Some(staged_oid) = repo.staged_txn_refs.remove(&txn_ref)
                && committed_in_meta
            {
                repo.main_head_oid = staged_oid;
            }
        }
        committed_in_meta
    }
}

/// BYOK-wrapped `<= 72h` emergency Continuity WAL & signed Crypto-Shred Receipt (`F-141`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuityWalRecord {
    pub wal_id: String,
    pub expires_at_utc_ms: i64,
    pub byok_wrapped_payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoShredReceipt {
    pub wal_id: String,
    pub shredded_at_utc_ms: i64,
    pub signature: HybridSignature,
}

pub fn seal_byok_continuity_wal(
    wal_id: &str,
    customer_byok_key: &SecretBox<[u8; 32]>,
    payload: &[u8],
    now_utc_ms: i64,
) -> ContinuityWalRecord {
    const MAX_WAL_TTL_MS: i64 = 72 * 3600 * 1000;
    let ks = hmac_sha512(customer_byok_key.expose_secret(), wal_id.as_bytes());
    let wrapped: Vec<u8> = payload
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ ks[i % 64])
        .collect();
    ContinuityWalRecord {
        wal_id: wal_id.to_string(),
        expires_at_utc_ms: now_utc_ms.saturating_add(MAX_WAL_TTL_MS),
        byok_wrapped_payload: wrapped,
    }
}

pub fn shred_continuity_wal_with_receipt(
    mut wal: ContinuityWalRecord,
    ed25519_seed: &SecretBox<[u8; 32]>,
    ml_dsa_seed: &SecretBox<[u8; 32]>,
    now_utc_ms: i64,
) -> CryptoShredReceipt {
    use zeroize::Zeroize as _;
    wal.byok_wrapped_payload.zeroize();
    let msg = format!("SHREDDED:{}:{}", wal.wal_id, now_utc_ms);
    CryptoShredReceipt {
        wal_id: wal.wal_id,
        shredded_at_utc_ms: now_utc_ms,
        signature: HybridSignature::sign_deterministic(ed25519_seed, ml_dsa_seed, msg.as_bytes()),
    }
}
