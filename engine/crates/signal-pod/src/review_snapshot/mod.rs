// Module: engine/crates/signal-pod/src/review_snapshot/mod.rs — Live @entity Anchor Telemetry & Signed Review KPI Exporter (FEAT-109)
// Invariants: Redacts out-of-scope required_provider_repo_id anchors; computes 100% deterministic non-LLM review KPIs (21-*).
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_core::{HybridSignature, SecretBox, sha512_256_hex};
use signal_proto::v1::{
    EntityAnchorRef, ExportEntityReviewSnapshotRequest, ExportEntityReviewSnapshotResponse,
    HybridPqcSignature, Point3dNm, SubprojectDomain,
};
use std::collections::{BTreeMap, BTreeSet};

pub const RESTRICTED_ENTITY_PLACEHOLDER: &str = "[Restricted Entity — Access Required]";

#[derive(Debug, Clone, PartialEq)]
pub struct LiveEntityTelemetryRecord {
    pub canonical_id: String,
    pub domain: SubprojectDomain,
    pub commit_sha: String,
    pub location_nm: Point3dNm,
    pub required_provider_repo_id: String,
    pub verified_readout_summary: String,
    pub drc_erc_lvs_violations: u32,
    pub spice_soa_margin_pct: f64,
    pub sta_slack_fs: i64,
    pub mcad_collisions: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedEntityAnchorView {
    Authorized {
        anchor: EntityAnchorRef,
        verified_readout_summary: String,
    },
    RedactedRestrictedScope {
        masked_label: String,
    },
    NotFoundUniform404,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignedReviewKpiSnapshot {
    pub response: ExportEntityReviewSnapshotResponse,
    pub kpi_payload_digest: String,
    pub pod_signature: HybridPqcSignature,
}

/// Normalizes shorthand `@entity` tokens (`@U12`, `@Net:VOUT`, `@STEP:Housing_Top`, `@Toolpath:OP20`, `@ECN-118`) to canonical IDs.
pub fn normalize_entity_anchor_token(raw_token: &str) -> String {
    let trimmed = raw_token.trim().trim_start_matches('@');
    for prefix in [
        "comp:",
        "net:",
        "Net:",
        "STEP:",
        "cad:",
        "Toolpath:",
        "cam:",
        "pcb:",
        "req:",
        "ecn:",
    ] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return rest.trim().to_string();
        }
    }
    trimmed.to_string()
}

/// Resolves a live `@entity` anchor while enforcing per-repo `required_provider_repo_id` RBAC masking (`21-*`).
pub fn resolve_live_entity_anchor(
    raw_anchor_token: &str,
    registry: &BTreeMap<String, LiveEntityTelemetryRecord>,
    caller_authorized_repo_ids: &BTreeSet<String>,
) -> ResolvedEntityAnchorView {
    let key = normalize_entity_anchor_token(raw_anchor_token);
    let Some(rec) = registry.get(&key) else {
        return ResolvedEntityAnchorView::NotFoundUniform404;
    };
    if !rec.required_provider_repo_id.is_empty()
        && !caller_authorized_repo_ids.contains(&rec.required_provider_repo_id)
    {
        return ResolvedEntityAnchorView::RedactedRestrictedScope {
            masked_label: RESTRICTED_ENTITY_PLACEHOLDER.to_string(),
        };
    }

    ResolvedEntityAnchorView::Authorized {
        anchor: EntityAnchorRef {
            domain: rec.domain as i32,
            entity_id: rec.canonical_id.clone(),
            commit_sha: rec.commit_sha.clone(),
            location_nm: Some(rec.location_nm),
            required_provider_repo_id: rec.required_provider_repo_id.clone(),
        },
        verified_readout_summary: rec.verified_readout_summary.clone(),
    }
}

/// Counts how many `EntityAnchorRef` items in a message must be redacted for a caller's `REPO_ACCESS_ENTRY` set (`21-*`).
pub fn count_redacted_message_anchors(
    anchors: &[EntityAnchorRef],
    caller_authorized_repo_ids: &BTreeSet<String>,
) -> u32 {
    let mut redacted = 0_u32;
    for a in anchors {
        if !a.required_provider_repo_id.is_empty()
            && !caller_authorized_repo_ids.contains(&a.required_provider_repo_id)
        {
            redacted += 1;
        }
    }
    redacted
}

/// Exports 100% deterministic engineering KPIs signed by the isolated `signal-pod` cell (`ExportEntityReviewSnapshot`, `FEAT-109`).
pub fn export_signed_entity_review_snapshot(
    req: &ExportEntityReviewSnapshotRequest,
    registry: &BTreeMap<String, LiveEntityTelemetryRecord>,
    pod_ed25519_seed: &SecretBox<[u8; 32]>,
    pod_mldsa_seed: &SecretBox<[u8; 32]>,
    signed_at_utc_ms: i64,
) -> SignedReviewKpiSnapshot {
    let mut total_violations = 0_u32;
    let mut worst_soa_pct = 100.0_f64;
    let mut worst_slack_fs = i64::MAX;
    let mut total_mcad_collisions = 0_u32;
    let mut matched_any = false;

    let mut visit_record = |rec: &LiveEntityTelemetryRecord| {
        matched_any = true;
        total_violations = total_violations.saturating_add(rec.drc_erc_lvs_violations);
        worst_soa_pct = worst_soa_pct.min(rec.spice_soa_margin_pct);
        worst_slack_fs = worst_slack_fs.min(rec.sta_slack_fs);
        total_mcad_collisions = total_mcad_collisions.saturating_add(rec.mcad_collisions);
    };

    if req.anchor_entity_ids.is_empty() {
        for rec in registry.values() {
            visit_record(rec);
        }
    } else {
        for raw_id in &req.anchor_entity_ids {
            let norm = normalize_entity_anchor_token(raw_id);
            if let Some(rec) = registry.get(&norm) {
                visit_record(rec);
            }
        }
    }

    if !matched_any {
        worst_slack_fs = 0;
    }

    let response = ExportEntityReviewSnapshotResponse {
        drc_erc_lvs_violations: total_violations,
        worst_spice_soa_margin_pct: worst_soa_pct,
        worst_sta_slack_fs: worst_slack_fs,
        mcad_collision_count: total_mcad_collisions,
    };

    let mut transcript = Vec::with_capacity(128);
    transcript.extend_from_slice(req.project_id.as_bytes());
    transcript.extend_from_slice(&response.drc_erc_lvs_violations.to_le_bytes());
    transcript.extend_from_slice(&response.worst_spice_soa_margin_pct.to_bits().to_le_bytes());
    transcript.extend_from_slice(&response.worst_sta_slack_fs.to_le_bytes());
    transcript.extend_from_slice(&response.mcad_collision_count.to_le_bytes());
    let digest = sha512_256_hex(&transcript);
    let sig =
        HybridSignature::sign_deterministic(pod_ed25519_seed, pod_mldsa_seed, digest.as_bytes());

    SignedReviewKpiSnapshot {
        response,
        kpi_payload_digest: digest,
        pod_signature: HybridPqcSignature {
            ed25519_sig: sig.ed25519_sig.to_vec(),
            ml_dsa_65_sig: sig.ml_dsa_65_sig,
            key_id: format!("pod-cell-{}", req.project_id),
            signed_at_utc_ms,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_live_entity_anchors_redacts_unauthorized_repos_and_signs_deterministic_kpis() {
        let mut reg = BTreeMap::new();
        reg.insert(
            "U12".to_string(),
            LiveEntityTelemetryRecord {
                canonical_id: "U12".into(),
                domain: SubprojectDomain::Pcb,
                commit_sha: "34c2651".into(),
                location_nm: Point3dNm {
                    x_nm: 12_500_000,
                    y_nm: 8_400_000,
                    z_nm: 0,
                },
                required_provider_repo_id: "gh_repo_987654322".into(),
                verified_readout_summary: "TPS62840 VOUT=1.80V, Tj=48.2C, SOA_margin=28.4%".into(),
                drc_erc_lvs_violations: 0,
                spice_soa_margin_pct: 28.4,
                sta_slack_fs: 125_000,
                mcad_collisions: 0,
            },
        );
        reg.insert(
            "Housing_Top".to_string(),
            LiveEntityTelemetryRecord {
                canonical_id: "Housing_Top".into(),
                domain: SubprojectDomain::CadMcad,
                commit_sha: "34c2651".into(),
                location_nm: Point3dNm {
                    x_nm: 0,
                    y_nm: 0,
                    z_nm: 14_000_000,
                },
                required_provider_repo_id: "gh_repo_987659999".into(),
                verified_readout_summary: "AL6061 Lid, min_pcb_clearance=2.0mm, IP67=PASS".into(),
                drc_erc_lvs_violations: 0,
                spice_soa_margin_pct: 41.0,
                sta_slack_fs: 210_000,
                mcad_collisions: 0,
            },
        );

        // PCB contractor holds ONLY gh_repo_987654322 -> @U12 resolves, @STEP:Housing_Top is redacted!
        let pcb_contractor_repos: BTreeSet<String> =
            ["gh_repo_987654322".to_string()].into_iter().collect();
        assert!(matches!(
            resolve_live_entity_anchor("@U12", &reg, &pcb_contractor_repos),
            ResolvedEntityAnchorView::Authorized { .. }
        ));
        assert_eq!(
            resolve_live_entity_anchor("@STEP:Housing_Top", &reg, &pcb_contractor_repos),
            ResolvedEntityAnchorView::RedactedRestrictedScope {
                masked_label: RESTRICTED_ENTITY_PLACEHOLDER.to_string(),
            }
        );

        let snap = export_signed_entity_review_snapshot(
            &ExportEntityReviewSnapshotRequest {
                project_id: "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee".into(),
                anchor_entity_ids: vec!["@U12".into(), "@STEP:Housing_Top".into()],
            },
            &reg,
            &SecretBox::new([0x11; 32]),
            &SecretBox::new([0x22; 32]),
            1_795_324_800_000,
        );
        assert_eq!(snap.response.drc_erc_lvs_violations, 0);
        assert!((snap.response.worst_spice_soa_margin_pct - 28.4).abs() < 1e-9);
        assert_eq!(snap.response.worst_sta_slack_fs, 125_000);
        assert_eq!(snap.response.mcad_collision_count, 0);
        assert_eq!(snap.kpi_payload_digest.len(), 64);
        assert_eq!(snap.pod_signature.ed25519_sig.len(), 64);
    }
}
