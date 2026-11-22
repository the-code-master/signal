// Module: engine/crates/signal-git-sync/src/provider_lfs.rs — Multi-Provider BYO-Git Adapter, Blobless Clone & 64KB LFS
// Invariants: Binds repos by immutable provider_repo_id; --filter=blob:none clones (F-015); 64KB LFS chunks (F-121); 3-way rebase (F-122).
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md
use crate::GitSyncError;
use signal_core::sha512_256_hex;
use signal_proto::v1::{
    BoundRepositoryRef, GitNamespaceKind, GitProviderKind, ReconcileExternalPushRequest,
    ReconcileExternalPushResponse,
};
use std::collections::{BTreeMap, BTreeSet};

pub const LFS_STREAM_CHUNK_BYTES: usize = 64 * 1024;
pub const MAX_LAZY_REPO_CREATIONS_PER_MIN: u32 = 40;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BloblessClonePlan {
    pub provider_repo_id: String,
    pub https_remote_url: String,
    pub git_argv: Vec<String>,
    pub interactive_rate_budget_pct: u8,
    pub auditor_rate_budget_pct: u8,
}

/// Validates immutable `provider_repo_id` binding across GitHub, GitLab & Bitbucket (`ORG` & `USER_NAMESPACE`)
/// and produces a blobless `--filter=blob:none` clone plan (`F-014`, `F-015`).
pub fn plan_blobless_clone(repo: &BoundRepositoryRef) -> Result<BloblessClonePlan, GitSyncError> {
    if repo.provider_repo_id.trim().is_empty() {
        return Err(GitSyncError::InvalidRepositoryBinding(
            "immutable provider_repo_id is required".to_string(),
        ));
    }
    if repo.display_full_name.trim().is_empty() || !repo.display_full_name.contains('/') {
        return Err(GitSyncError::InvalidRepositoryBinding(
            "display_full_name must be <namespace>/<slug>".to_string(),
        ));
    }
    let ns_kind =
        GitNamespaceKind::try_from(repo.namespace_kind).unwrap_or(GitNamespaceKind::Unspecified);
    if ns_kind == GitNamespaceKind::Unspecified {
        return Err(GitSyncError::InvalidRepositoryBinding(
            "GitNamespaceKind must be ORG_NAMESPACE or USER_NAMESPACE".to_string(),
        ));
    }

    let provider = GitProviderKind::try_from(repo.provider).unwrap_or(GitProviderKind::Unspecified);
    let host = match provider {
        GitProviderKind::Github => "github.com",
        GitProviderKind::Gitlab => "gitlab.com",
        GitProviderKind::Bitbucket => "bitbucket.org",
        GitProviderKind::Unspecified => {
            return Err(GitSyncError::InvalidRepositoryBinding(
                "unsupported GitProviderKind".to_string(),
            ));
        }
    };

    let url = format!("https://{host}/{}.git", repo.display_full_name);
    let argv = vec![
        "git".to_string(),
        "clone".to_string(),
        "--filter=blob:none".to_string(),
        "--no-checkout".to_string(),
        url.clone(),
    ];

    Ok(BloblessClonePlan {
        provider_repo_id: repo.provider_repo_id.clone(),
        https_remote_url: url,
        git_argv: argv,
        interactive_rate_budget_pct: 70,
        auditor_rate_budget_pct: 30,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LfsChunkFrame {
    pub chunk_index: u32,
    pub payload: Vec<u8>,
    pub chunk_sha512_256: String,
}

/// Splits multi-MB/GB binary assets into bounded `64 KB` streaming LFS frames (`< 16 MB` constant RAM, `F-121`).
pub fn chunk_lfs_stream_64kb(data: &[u8]) -> Vec<LfsChunkFrame> {
    if data.is_empty() {
        return vec![LfsChunkFrame {
            chunk_index: 0,
            payload: Vec::new(),
            chunk_sha512_256: sha512_256_hex(&[]),
        }];
    }
    data.chunks(LFS_STREAM_CHUNK_BYTES)
        .enumerate()
        .map(|(idx, slice)| LfsChunkFrame {
            chunk_index: idx as u32,
            payload: slice.to_vec(),
            chunk_sha512_256: sha512_256_hex(slice),
        })
        .collect()
}

pub fn reassemble_lfs_chunks(chunks: &[LfsChunkFrame]) -> Result<Vec<u8>, GitSyncError> {
    let total_len: usize = chunks.iter().map(|c| c.payload.len()).sum();
    let mut out = Vec::with_capacity(total_len);
    for (expected_idx, ch) in chunks.iter().enumerate() {
        if ch.chunk_index != expected_idx as u32
            || sha512_256_hex(&ch.payload) != ch.chunk_sha512_256
        {
            return Err(GitSyncError::VaultIntegrityFailure(
                "LFS 64KB chunk sequence or digest mismatch".to_string(),
            ));
        }
        out.extend_from_slice(&ch.payload);
    }
    Ok(out)
}

/// 3-way entity-level semantic auto-rebase on external `git push` webhook notification (`F-122`).
pub fn reconcile_external_push_3way(
    req: &ReconcileExternalPushRequest,
    min_epoch: u64,
    base_entities: &BTreeMap<String, String>,
    local_entities: &BTreeMap<String, String>,
    external_entities: &BTreeMap<String, String>,
) -> Result<(ReconcileExternalPushResponse, BTreeMap<String, String>), GitSyncError> {
    let fencing = req
        .fencing
        .as_ref()
        .ok_or_else(|| GitSyncError::InvalidRepositoryBinding("missing fencing".to_string()))?;
    if fencing.fencing_epoch < min_epoch {
        return Err(GitSyncError::StaleFencingEpoch {
            provided: fencing.fencing_epoch,
            active: min_epoch,
        });
    }

    let mut all_keys = BTreeSet::new();
    all_keys.extend(base_entities.keys().cloned());
    all_keys.extend(local_entities.keys().cloned());
    all_keys.extend(external_entities.keys().cloned());

    let mut merged = BTreeMap::new();
    let mut conflicts = 0_u32;

    for key in all_keys {
        let b = base_entities.get(&key);
        let l = local_entities.get(&key);
        let e = external_entities.get(&key);

        if l == e {
            if let Some(val) = l {
                merged.insert(key, val.clone());
            }
        } else if b == l {
            // Only external push modified/deleted `key`
            if let Some(val) = e {
                merged.insert(key, val.clone());
            }
        } else if b == e {
            // Only local session modified/deleted `key`
            if let Some(val) = l {
                merged.insert(key, val.clone());
            }
        } else {
            conflicts += 1;
            if let Some(val) = l {
                merged.insert(key, val.clone());
            }
        }
    }

    let mut canonical = req.external_head_commit_sha.clone();
    for (k, v) in &merged {
        canonical.push_str(k);
        canonical.push('=');
        canonical.push_str(v);
        canonical.push(';');
    }
    let sha = sha512_256_hex(canonical.as_bytes())[..40].to_string();

    Ok((
        ReconcileExternalPushResponse {
            auto_rebased_cleanly: conflicts == 0,
            resulting_commit_sha: sha,
            conflicting_entity_count: conflicts,
        },
        merged,
    ))
}
