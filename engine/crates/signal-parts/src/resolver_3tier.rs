// Module: engine/crates/signal-parts/src/resolver_3tier.rs — 3-Tier Federated Universal Part Resolver (18-*)
// Invariants: Tier 1 Global CAS rejects org_private_nda_taint parts; Tier 3 ProjectPartsLockfile enforces exact BLAKE3 pinning.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/10-api-and-data-contracts.md

use crate::PartsError;
use signal_proto::v1::{
    ProjectPartsLockfile, ResolveUniversalPartRequest, ResolveUniversalPartResponse,
    UniversalComponentRevision,
};
use std::collections::BTreeMap;

/// Returns deterministic 2-hex-char CAS shard prefix (`00..ff`) from a 64-char digest (`18-*`).
pub fn global_cas_shard_prefix(revision_hash_blake3: &str) -> Result<&str, PartsError> {
    if revision_hash_blake3.len() != 64
        || !revision_hash_blake3.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err(PartsError::InvalidBlake3Digest(revision_hash_blake3.len()));
    }
    Ok(&revision_hash_blake3[..2])
}

/// 3-Tier Federated Universal Component Repository (`18-*`):
/// - Tier 1: Immutable Global Content-Addressed Store (`00..ff` sharded, zero NDA parts allowed),
/// - Tier 2: Org-Scoped Encrypted AVL & Custom NDA Part Overlay (`org_id`-isolated),
/// - Tier 3: Project-Scoped `.signal/parts.lock.json` (`ProjectPartsLockfile` exact revision pin).
#[derive(Debug, Default)]
pub struct FederatedPartsRepository {
    tier1_global_cas: BTreeMap<(String, String), UniversalComponentRevision>,
    tier2_org_avl_overlays: BTreeMap<(String, String, String), UniversalComponentRevision>,
    tier3_project_locks: BTreeMap<String, ProjectPartsLockfile>,
}

impl FederatedPartsRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish_tier1_global_part(
        &mut self,
        part: UniversalComponentRevision,
    ) -> Result<String, PartsError> {
        if !part.upid.starts_with("urn:signal:part:") {
            return Err(PartsError::InvalidPartUrn(part.upid));
        }
        if part.org_private_nda_taint {
            return Err(PartsError::NdaPartLeakIntoGlobalCas(part.upid));
        }
        let shard = global_cas_shard_prefix(&part.revision_hash_blake3)?.to_string();
        self.tier1_global_cas
            .insert((part.upid.clone(), part.revision_hash_blake3.clone()), part);
        Ok(shard)
    }

    pub fn publish_tier2_org_overlay_part(
        &mut self,
        org_id: &str,
        part: UniversalComponentRevision,
    ) -> Result<(), PartsError> {
        if org_id.trim().is_empty() {
            return Err(PartsError::PartNotFound("empty org_id".into()));
        }
        if !part.upid.starts_with("urn:signal:part:") {
            return Err(PartsError::InvalidPartUrn(part.upid));
        }
        let _ = global_cas_shard_prefix(&part.revision_hash_blake3)?;
        self.tier2_org_avl_overlays.insert(
            (
                org_id.to_string(),
                part.upid.clone(),
                part.revision_hash_blake3.clone(),
            ),
            part,
        );
        Ok(())
    }

    pub fn register_tier3_project_lockfile(&mut self, lockfile: ProjectPartsLockfile) {
        self.tier3_project_locks
            .insert(lockfile.project_id.clone(), lockfile);
    }

    pub fn project_lockfiles(&self) -> &BTreeMap<String, ProjectPartsLockfile> {
        &self.tier3_project_locks
    }

    /// Resolves `upid` + `pinned_blake3_hash` strictly across Tier 3 lockfile -> Tier 2 Org Overlay -> Tier 1 Global CAS (`18-*`).
    pub fn resolve_universal_part(
        &self,
        req: &ResolveUniversalPartRequest,
        verified_overlap_ratio: f64,
    ) -> Result<ResolveUniversalPartResponse, PartsError> {
        if !req.upid.starts_with("urn:signal:part:") {
            return Err(PartsError::InvalidPartUrn(req.upid.clone()));
        }
        let effective_hash = if !req.pinned_blake3_hash.is_empty() {
            if let Some(lock) = self.tier3_project_locks.get(&req.project_id)
                && let Some(entry) = lock.entries.iter().find(|e| e.upid == req.upid)
                && entry.revision_hash_blake3 != req.pinned_blake3_hash
            {
                return Err(PartsError::PartsLockDigestMismatch {
                    upid: req.upid.clone(),
                    expected: entry.revision_hash_blake3.clone(),
                    provided: req.pinned_blake3_hash.clone(),
                });
            }
            req.pinned_blake3_hash.clone()
        } else if let Some(lock) = self.tier3_project_locks.get(&req.project_id)
            && let Some(entry) = lock.entries.iter().find(|e| e.upid == req.upid)
        {
            entry.revision_hash_blake3.clone()
        } else {
            return Err(PartsError::UnpinnedPartInLockfile(req.upid.clone()));
        };

        let _ = global_cas_shard_prefix(&effective_hash)?;

        if let Some(org_part) = self.tier2_org_avl_overlays.get(&(
            req.org_id.clone(),
            req.upid.clone(),
            effective_hash.clone(),
        )) {
            return Ok(ResolveUniversalPartResponse {
                part: Some(org_part.clone()),
                resolved_from_org_avl_overlay: true,
                step_lead_pad_overlap_ratio: verified_overlap_ratio,
            });
        }

        if let Some(global_part) = self
            .tier1_global_cas
            .get(&(req.upid.clone(), effective_hash.clone()))
        {
            return Ok(ResolveUniversalPartResponse {
                part: Some(global_part.clone()),
                resolved_from_org_avl_overlay: false,
                step_lead_pad_overlap_ratio: verified_overlap_ratio,
            });
        }

        Err(PartsError::PartNotFound(format!(
            "{}@{}",
            req.upid, effective_hash
        )))
    }
}
