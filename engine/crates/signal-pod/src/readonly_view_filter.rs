// Module: engine/crates/signal-pod/src/readonly_view_filter.rs — Zero-Leak Partner Read-Only View Sanitizer (FEAT-119)
// Invariants: Strips unit_cost, internal engineering notes, NDA MPNs & unauthorized sub-sheets before guest rendering.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_core::sha512_256_hex;
use signal_proto::v1::{IssuePartnerReadonlyViewRequest, IssuePartnerReadonlyViewResponse};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalSheetEntity {
    pub entity_id: String,
    pub sheet_id: String,
    pub refdes: String,
    pub orderable_mpn: String,
    pub nda_restricted_mpn: bool,
    pub unit_cost_usd_micros: Option<i64>,
    pub internal_engineering_notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SanitizedPartnerEntity {
    pub entity_id: String,
    pub sheet_id: String,
    pub refdes: String,
    pub visible_mpn: String,
    pub unit_cost_usd_micros: Option<i64>,
    pub internal_engineering_notes: Option<String>,
}

/// Filters project entities to `allowed_sheet_ids` and scrubs cost, internal notes & NDA MPNs (`FEAT-119`).
pub fn issue_sanitized_partner_readonly_view(
    req: &IssuePartnerReadonlyViewRequest,
    entities: &[InternalSheetEntity],
    now_utc_ms: i64,
) -> Option<(
    IssuePartnerReadonlyViewResponse,
    Vec<SanitizedPartnerEntity>,
)> {
    if req.expires_at_utc_ms <= now_utc_ms || req.allowed_sheet_ids.is_empty() {
        return None;
    }
    let allowed_sheets: BTreeSet<&str> = req.allowed_sheet_ids.iter().map(String::as_str).collect();

    let sanitized: Vec<SanitizedPartnerEntity> = entities
        .iter()
        .filter(|e| allowed_sheets.contains(e.sheet_id.as_str()))
        .map(|e| SanitizedPartnerEntity {
            entity_id: e.entity_id.clone(),
            sheet_id: e.sheet_id.clone(),
            refdes: e.refdes.clone(),
            visible_mpn: if e.nda_restricted_mpn {
                "[REDACTED-NDA-MPN]".to_string()
            } else {
                e.orderable_mpn.clone()
            },
            unit_cost_usd_micros: None,
            internal_engineering_notes: None,
        })
        .collect();

    let ticket_material = format!(
        "{}:{}:{}:{:?}",
        req.project_id, req.partner_actor_pseudo_id, req.expires_at_utc_ms, req.allowed_sheet_ids
    );
    let ticket_id = format!("ptk_{}", &sha512_256_hex(ticket_material.as_bytes())[..20]);

    Some((
        IssuePartnerReadonlyViewResponse {
            partner_ticket_id: ticket_id,
            internal_cost_and_notes_stripped: true,
        },
        sanitized,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_cost_notes_nda_mpns_and_unlisted_sheets_and_rejects_expired_requests() {
        let ents = vec![
            InternalSheetEntity {
                entity_id: "U1".into(),
                sheet_id: "sheet_io".into(),
                refdes: "U1".into(),
                orderable_mpn: "CUSTOM_ASIC_X1".into(),
                nda_restricted_mpn: true,
                unit_cost_usd_micros: Some(45_000_000),
                internal_engineering_notes: Some("yield issue on rev A0".into()),
            },
            InternalSheetEntity {
                entity_id: "U2".into(),
                sheet_id: "sheet_secret_core".into(),
                refdes: "U2".into(),
                orderable_mpn: "SECRET_PLL".into(),
                nda_restricted_mpn: true,
                unit_cost_usd_micros: Some(12_000_000),
                internal_engineering_notes: Some("proprietary".into()),
            },
        ];

        let req = IssuePartnerReadonlyViewRequest {
            project_id: "proj_1".into(),
            partner_actor_pseudo_id: "guest_cm_1".into(),
            expires_at_utc_ms: 50_000,
            allowed_sheet_ids: vec!["sheet_io".into()],
        };
        let (resp, filtered) =
            issue_sanitized_partner_readonly_view(&req, &ents, 10_000).expect("valid partner view");
        assert!(resp.internal_cost_and_notes_stripped);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].visible_mpn, "[REDACTED-NDA-MPN]");
        assert!(filtered[0].unit_cost_usd_micros.is_none());
        assert!(filtered[0].internal_engineering_notes.is_none());

        // Expired request fails closed
        assert!(issue_sanitized_partner_readonly_view(&req, &ents, 60_000).is_none());
    }
}
