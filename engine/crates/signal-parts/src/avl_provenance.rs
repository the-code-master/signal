// Module: engine/crates/signal-parts/src/avl_provenance.rs — Enterprise AVL Blast-Radius & Anti-Counterfeit Provenance
// Invariants: Scans cross-project parts.lock.json usage (FEAT-075) & cryptographically verifies franchise lot lineage (FEAT-077).
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_core::hmac_sha512;
use signal_proto::v1::{
    ProjectPartsLockfile, QueryAvlWhereUsedRequest, QueryAvlWhereUsedResponse,
    VerifyDistributorProvenanceRequest, VerifyDistributorProvenanceResponse,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnterpriseAvlCatalogEntry {
    pub org_id: String,
    pub upid: String,
    pub orderable_mpn: String,
    pub approved_alternates: Vec<String>,
}

/// Scans an organization's `ProjectPartsLockfile` index (`FEAT-075`) to compute immediate blast-radius
/// of an EOL/NRND `target_mpn` and recommend a pre-qualified drop-in AVL replacement.
pub fn query_avl_where_used(
    req: &QueryAvlWhereUsedRequest,
    avl_entries: &[EnterpriseAvlCatalogEntry],
    project_locks_by_org: &BTreeMap<(String, String), ProjectPartsLockfile>,
) -> QueryAvlWhereUsedResponse {
    let target = req.target_mpn.trim();
    let matching_avl: Vec<&EnterpriseAvlCatalogEntry> = avl_entries
        .iter()
        .filter(|e| e.org_id == req.org_id && e.orderable_mpn.eq_ignore_ascii_case(target))
        .collect();

    let matching_upids: Vec<&str> = matching_avl.iter().map(|e| e.upid.as_str()).collect();
    let suggested = matching_avl
        .iter()
        .find_map(|e| e.approved_alternates.first().cloned())
        .unwrap_or_default();

    let mut affected = Vec::new();
    for ((org_id, proj_id), lock) in project_locks_by_org {
        if org_id != &req.org_id {
            continue;
        }
        let uses_part = lock.entries.iter().any(|le| {
            matching_upids.contains(&le.upid.as_str())
                || le
                    .upid
                    .to_ascii_uppercase()
                    .contains(&target.to_ascii_uppercase())
        });
        if uses_part {
            affected.push(proj_id.clone());
        }
    }

    QueryAvlWhereUsedResponse {
        affected_project_ids: affected,
        suggested_drop_in_mpn: suggested,
    }
}

fn is_authorized_franchise_code(code: &str) -> bool {
    matches!(
        code.trim().to_ascii_uppercase().as_str(),
        "DIGIKEY" | "MOUSER" | "ARROW" | "AVNET" | "TTI" | "FUTURE" | "TI_DIRECT"
    )
}

/// Generates deterministic authorized-franchise lot-trace HMAC signature (`FEAT-077`).
pub fn sign_authorized_lot_lineage(
    franchise_signing_key: &[u8; 32],
    orderable_mpn: &str,
    distributor_code: &str,
) -> String {
    let msg = format!(
        "SIGNAL-PROVENANCE-V1|{}|{}",
        orderable_mpn.trim().to_ascii_uppercase(),
        distributor_code.trim().to_ascii_uppercase()
    );
    let mac = hmac_sha512(franchise_signing_key, msg.as_bytes());
    let mut hex = String::with_capacity(64);
    for &b in &mac[..32] {
        use std::fmt::Write as _;
        let _ = write!(&mut hex, "{b:02x}");
    }
    hex
}

/// Verifies distributor provenance & classifies counterfeit risk (`LOW_AUTHORIZED`, `ELEVATED_UNVERIFIED_LOT`, `HIGH_GRAY_MARKET_BROKER`, `FEAT-077`).
pub fn verify_distributor_provenance(
    req: &VerifyDistributorProvenanceRequest,
    franchise_signing_key: &[u8; 32],
) -> VerifyDistributorProvenanceResponse {
    if !is_authorized_franchise_code(&req.distributor_code) {
        return VerifyDistributorProvenanceResponse {
            authorized_franchise_verified: false,
            counterfeit_risk_tier: "HIGH_GRAY_MARKET_BROKER".to_string(),
        };
    }
    let expected_sig = sign_authorized_lot_lineage(
        franchise_signing_key,
        &req.orderable_mpn,
        &req.distributor_code,
    );
    let verified = !req.lot_trace_signature.is_empty()
        && req.lot_trace_signature.eq_ignore_ascii_case(&expected_sig);

    VerifyDistributorProvenanceResponse {
        authorized_franchise_verified: verified,
        counterfeit_risk_tier: if verified {
            "LOW_AUTHORIZED_FRANCHISE".to_string()
        } else {
            "ELEVATED_UNVERIFIED_LOT_SIGNATURE".to_string()
        },
    }
}
