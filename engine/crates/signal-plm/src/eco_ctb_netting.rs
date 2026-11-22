// Module: engine/crates/signal-plm/src/eco_ctb_netting.rs — Semantic ECO Redline Diff, Part 11 ECN Gate & Clear-to-Build Solver
// Invariants: Requires HybridPqcSignature on RELEASED transitions; nets primary + qualified AVL alternate stock for CTB (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::PlmError;
use signal_core::sha512_256_hex;
use signal_parts::EnterpriseAvlCatalogEntry;
use signal_proto::v1::{
    BomLineItem, ClearToBuildReport, EvaluateClearToBuildRequest, EvaluateClearToBuildResponse,
    TransitionChangeOrderRequest, TransitionChangeOrderResponse,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcoBomRedlineDiff {
    pub added_upids: Vec<String>,
    pub removed_upids: Vec<String>,
    pub modified_upids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryLotPosition {
    pub mpn: String,
    pub on_hand_qty: u32,
    pub next_po_receipt_utc_ms: i64,
    pub incoming_po_qty: u32,
}

/// Computes semantic redline BOM delta between two revisions for an `ECO`/`ECN` review package (`19-*`).
pub fn compute_eco_bom_redline_diff(
    before_lines: &[BomLineItem],
    after_lines: &[BomLineItem],
) -> EcoBomRedlineDiff {
    let before_map: BTreeMap<&str, &BomLineItem> =
        before_lines.iter().map(|l| (l.upid.as_str(), l)).collect();
    let after_map: BTreeMap<&str, &BomLineItem> =
        after_lines.iter().map(|l| (l.upid.as_str(), l)).collect();

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();

    for (&upid, after) in &after_map {
        match before_map.get(upid) {
            None => added.push(upid.to_string()),
            Some(before) => {
                if before.quantity_per_unit != after.quantity_per_unit
                    || before.orderable_mpn != after.orderable_mpn
                    || before.workstation_step_id != after.workstation_step_id
                {
                    modified.push(upid.to_string());
                }
            }
        }
    }
    for &upid in before_map.keys() {
        if !after_map.contains_key(upid) {
            removed.push(upid.to_string());
        }
    }

    EcoBomRedlineDiff {
        added_upids: added,
        removed_upids: removed,
        modified_upids: modified,
    }
}

/// Transitions an `ECR`/`ECO`/`ECN`/`MCN`/`PCN` state while enforcing 21 CFR Part 11 `HybridPqcSignature` on `RELEASED`.
pub fn transition_change_order(
    req: &TransitionChangeOrderRequest,
    ops_commit_override: Option<&str>,
) -> Result<TransitionChangeOrderResponse, PlmError> {
    if req.change_id.trim().is_empty() || req.target_state.trim().is_empty() {
        return Err(PlmError::InvalidChangeTransition(req.change_id.clone()));
    }
    let state_upper = req.target_state.trim().to_ascii_uppercase();
    if matches!(state_upper.as_str(), "RELEASED" | "APPROVED") {
        let sig = req
            .part11_signature
            .as_ref()
            .ok_or_else(|| PlmError::MissingPart11Signature(req.change_id.clone()))?;
        if sig.ed25519_sig.is_empty() || sig.ml_dsa_65_sig.is_empty() {
            return Err(PlmError::MissingPart11Signature(req.change_id.clone()));
        }
    }

    let commit_sha = if let Some(override_sha) = ops_commit_override {
        override_sha.to_string()
    } else {
        let mut payload = Vec::new();
        payload.extend_from_slice(req.org_id.as_bytes());
        payload.extend_from_slice(req.project_id.as_bytes());
        payload.extend_from_slice(req.change_id.as_bytes());
        payload.extend_from_slice(state_upper.as_bytes());
        sha512_256_hex(&payload)[..40].to_string()
    };

    Ok(TransitionChangeOrderResponse {
        change_id: req.change_id.clone(),
        current_state: state_upper,
        ops_repo_commit_sha: commit_sha,
    })
}

/// Solves Clear-to-Build (`CTB`) max buildable lot units and gating shortages by netting primary + qualified AVL alternates.
pub fn evaluate_clear_to_build(
    req: &EvaluateClearToBuildRequest,
    mbom_lines: &[BomLineItem],
    inventory: &[InventoryLotPosition],
    org_avl: &[EnterpriseAvlCatalogEntry],
    now_utc_ms: i64,
) -> EvaluateClearToBuildResponse {
    let inv_by_mpn: BTreeMap<&str, &InventoryLotPosition> =
        inventory.iter().map(|p| (p.mpn.as_str(), p)).collect();
    let avl_by_upid: BTreeMap<&str, &EnterpriseAvlCatalogEntry> = org_avl
        .iter()
        .filter(|e| e.org_id == req.org_id)
        .map(|e| (e.upid.as_str(), e))
        .collect();

    if mbom_lines.is_empty() {
        return EvaluateClearToBuildResponse {
            report: Some(ClearToBuildReport {
                max_buildable_units: 0,
                gating_shortage_mpns: Vec::new(),
                earliest_clear_date_utc_ms: now_utc_ms,
            }),
        };
    }

    let mut max_buildable = u32::MAX;
    let mut shortages = Vec::new();
    let mut earliest_clear_ms = now_utc_ms;

    for line in mbom_lines {
        let qty_per = line.quantity_per_unit.max(1);
        let mut pooled_on_hand = inv_by_mpn
            .get(line.orderable_mpn.as_str())
            .map(|p| p.on_hand_qty)
            .unwrap_or(0);
        let mut line_clear_ms = inv_by_mpn
            .get(line.orderable_mpn.as_str())
            .map(|p| p.next_po_receipt_utc_ms)
            .unwrap_or(now_utc_ms);

        if let Some(avl_entry) = avl_by_upid.get(line.upid.as_str()) {
            for alt_mpn in &avl_entry.approved_alternates {
                if let Some(alt_pos) = inv_by_mpn.get(alt_mpn.as_str()) {
                    pooled_on_hand = pooled_on_hand.saturating_add(alt_pos.on_hand_qty);
                    line_clear_ms = line_clear_ms.max(alt_pos.next_po_receipt_utc_ms);
                }
            }
        }

        let buildable_for_line = pooled_on_hand / qty_per;
        max_buildable = max_buildable.min(buildable_for_line);
        if buildable_for_line < req.target_lot_size {
            shortages.push(line.orderable_mpn.clone());
            earliest_clear_ms = earliest_clear_ms.max(line_clear_ms);
        }
    }

    EvaluateClearToBuildResponse {
        report: Some(ClearToBuildReport {
            max_buildable_units: max_buildable,
            gating_shortage_mpns: shortages,
            earliest_clear_date_utc_ms: earliest_clear_ms,
        }),
    }
}
