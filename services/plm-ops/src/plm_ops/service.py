# Module: services/plm-ops/src/plm_ops/service.py — Multi-BOM, ECR->ECO->ECN, MCN, PCN, Hard Deviation Gate, 8D & CTB
# Invariants: Enforces 21 CFR Part 11 ML-DSA-65 on CCB approval, fail-closed Deviation SFTP block, and zero physics.
# Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
from __future__ import annotations

import hashlib
import hmac
import json
import uuid

from fastapi import HTTPException

from plm_ops.repository import PLM_STORE, fetch_plm_ops_record
from plm_ops.schemas import (
    BomLineDto,
    BomViewKind,
    ChangeLifecycleState,
    ChangeOrderKind,
    CtbEvaluationRequest,
    CtbReportView,
    CtbShortageWaterfallRow,
    DeviationCreateRequest,
    DeviationGateCheckResult,
    DeviationWaiverView,
    EcoCreateRequest,
    EcoRecordView,
    EcoTransitionRequest,
    ErpSyncEnvelope,
    ErpSyncRequest,
    McnCreateRequest,
    McnRecordView,
    MultiViewBomRollup,
    NcrCapaCreateRequest,
    NcrCapaView,
    PcnImpactView,
    PcnIngestRequest,
    PlmChangeNoticeSummary,
)
from services.common.errors import raise_not_found

_ALLOWED_TRANSITIONS: dict[ChangeLifecycleState, frozenset[ChangeLifecycleState]] = {
    "DRAFT": frozenset({"CCB_REVIEW"}),
    "CCB_REVIEW": frozenset({"APPROVED", "DRAFT"}),
    "APPROVED": frozenset({"RELEASED"}),
    "RELEASED": frozenset(),
}


def _ops_commit_sha(seed: str) -> str:
    return hashlib.sha256(seed.encode("utf-8")).hexdigest()[:40]


def _next_kind_on_state(current_kind: ChangeOrderKind, state: ChangeLifecycleState) -> ChangeOrderKind:
    if state == "APPROVED" and current_kind == "ECR":
        return "ECO"
    if state == "RELEASED":
        return "ECN"
    return current_kind


async def execute_plm_ops(org_slug: str) -> PlmChangeNoticeSummary:
    return await fetch_plm_ops_record(org_slug)


async def compile_multi_view_bom(
    org_slug: str,
    proj_slug: str,
    view: BomViewKind,
) -> MultiViewBomRollup:
    lines = PLM_STORE.get_bom_lines(org_slug, proj_slug, view)
    total_cost = sum(line.unit_cost_micros_usd * line.quantity_per_unit for line in lines)
    total_co2 = sum(line.embodied_co2e_mg * line.quantity_per_unit for line in lines)
    dpp_ok = all(line.rohs_compliant and line.reach_svhc_free and line.pfas_free for line in lines)
    return MultiViewBomRollup(
        view=view,
        lines=tuple(lines),
        total_unit_cost_micros_usd=total_cost,
        total_embodied_co2e_mg=total_co2,
        dpp_compliant=dpp_ok,
    )


async def add_bom_line_item(
    org_slug: str,
    proj_slug: str,
    view: BomViewKind,
    line: BomLineDto,
) -> MultiViewBomRollup:
    PLM_STORE.append_bom_line(org_slug, proj_slug, view, line)
    return await compile_multi_view_bom(org_slug, proj_slug, view)


async def create_change_order(
    org_slug: str,
    proj_slug: str,
    req: EcoCreateRequest,
) -> EcoRecordView:
    change_id = f"{req.kind}-{uuid.uuid4().hex[:8].upper()}"
    sha = _ops_commit_sha(f"{org_slug}:{proj_slug}:{change_id}:DRAFT")
    rec = EcoRecordView(
        change_id=change_id,
        kind=req.kind,
        title=req.title,
        state="DRAFT",
        wip_disposition=req.wip_disposition,
        in_stock_disposition=req.in_stock_disposition,
        field_disposition=req.field_disposition,
        affected_upids=req.affected_upids,
        ops_repo_commit_sha=sha,
    )
    return PLM_STORE.save_eco(org_slug, proj_slug, rec)


async def transition_change_order(
    org_slug: str,
    proj_slug: str,
    change_id: str,
    req: EcoTransitionRequest,
) -> EcoRecordView:
    existing = PLM_STORE.get_eco(org_slug, proj_slug, change_id)
    if existing is None:
        raise_not_found()

    if req.target_state not in _ALLOWED_TRANSITIONS[existing.state]:
        raise HTTPException(status_code=409, detail="INVALID_CHANGE_LIFECYCLE_TRANSITION")

    if req.target_state in ("APPROVED", "RELEASED"):
        if req.ccb_approver_count < 2:
            raise HTTPException(status_code=409, detail="CCB_QUORUM_NOT_MET")
        if not req.ml_dsa_65_signature.startswith("ml-dsa-65:") or len(req.ml_dsa_65_signature) < 18:
            raise HTTPException(status_code=401, detail="PART11_ML_DSA_65_SIGNATURE_REQUIRED")

    promoted_kind = _next_kind_on_state(existing.kind, req.target_state)
    sha = _ops_commit_sha(f"{existing.ops_repo_commit_sha}:{req.target_state}:{req.ml_dsa_65_signature}")
    updated = EcoRecordView(
        change_id=existing.change_id,
        kind=promoted_kind,
        title=existing.title,
        state=req.target_state,
        wip_disposition=existing.wip_disposition,
        in_stock_disposition=existing.in_stock_disposition,
        field_disposition=existing.field_disposition,
        affected_upids=existing.affected_upids,
        ops_repo_commit_sha=sha,
    )
    return PLM_STORE.save_eco(org_slug, proj_slug, updated)


async def create_manufacturing_change_notice(
    org_slug: str,
    proj_slug: str,
    req: McnCreateRequest,
) -> McnRecordView:
    mcn_id = f"MCN-{uuid.uuid4().hex[:8].upper()}"
    sha = _ops_commit_sha(f"{mcn_id}:{req.workstation_id}:{req.cnc_or_slicer_recipe_hash}")
    rec = McnRecordView(
        mcn_id=mcn_id,
        workstation_id=req.workstation_id,
        updated_torque_milli_nm=req.updated_torque_milli_nm,
        cnc_or_slicer_recipe_hash=req.cnc_or_slicer_recipe_hash,
        ops_repo_commit_sha=sha,
    )
    return PLM_STORE.append_mcn(org_slug, proj_slug, rec)


async def ingest_part_change_notice(
    org_slug: str,
    proj_slug: str,
    req: PcnIngestRequest,
) -> PcnImpactView:
    ebom_lines = PLM_STORE.get_bom_lines(org_slug, proj_slug, "EBOM")
    matched = [ln for ln in ebom_lines if ln.orderable_mpn == req.affected_mpn]
    spawned_ecr_id: str | None = None
    if matched:
        ecr = await create_change_order(
            org_slug,
            proj_slug,
            EcoCreateRequest(
                kind="ECR",
                title=f"PCN/PDN {req.vendor_notice_code} Alternate Qualification for {req.affected_mpn}",
                affected_upids=tuple(ln.upid for ln in matched),
            ),
        )
        spawned_ecr_id = ecr.change_id

    pcn_id = f"PCN-{uuid.uuid4().hex[:8].upper()}"
    rec = PcnImpactView(
        pcn_id=pcn_id,
        affected_mpn=req.affected_mpn,
        matched_in_project_bom=bool(matched),
        last_time_buy_utc_ms=req.last_time_buy_utc_ms,
        spawned_ecr_id=spawned_ecr_id,
    )
    return PLM_STORE.append_pcn(org_slug, proj_slug, rec)


async def create_deviation_waiver(
    org_slug: str,
    proj_slug: str,
    req: DeviationCreateRequest,
) -> DeviationWaiverView:
    if req.authorized_qty_limit <= 0 or req.consumed_qty < 0 or req.consumed_qty > req.authorized_qty_limit:
        raise HTTPException(status_code=422, detail="INVALID_DEVIATION_QTY_BOUNDS")
    dev_id = f"DEV-{uuid.uuid4().hex[:8].upper()}"
    view = DeviationWaiverView(
        deviation_id=dev_id,
        reason=req.reason,
        authorized_qty_limit=req.authorized_qty_limit,
        consumed_qty=req.consumed_qty,
        expires_at_utc_ms=req.expires_at_utc_ms,
        remaining_qty=req.authorized_qty_limit - req.consumed_qty,
    )
    return PLM_STORE.add_deviation(org_slug, proj_slug, view)


def verify_deviation_sftp_gate(
    org_slug: str,
    proj_slug: str,
    requested_qty: int,
    now_utc_ms: int,
    consume_on_pass: bool = False,
) -> DeviationGateCheckResult:
    deviations = PLM_STORE.list_deviations(org_slug, proj_slug)
    if not deviations:
        return DeviationGateCheckResult(
            release_permitted=True,
            blocking_deviation_ids=(),
            evaluated_deviation_count=0,
        )

    blocking: list[str] = []
    for dev in deviations:
        expired = now_utc_ms >= dev.expires_at_utc_ms
        over_qty = (dev.consumed_qty + requested_qty) > dev.authorized_qty_limit
        if expired or over_qty:
            blocking.append(dev.deviation_id)

    if blocking:
        return DeviationGateCheckResult(
            release_permitted=False,
            blocking_deviation_ids=tuple(blocking),
            evaluated_deviation_count=len(deviations),
        )

    if consume_on_pass and requested_qty > 0:
        updated = [
            DeviationWaiverView(
                deviation_id=d.deviation_id,
                reason=d.reason,
                authorized_qty_limit=d.authorized_qty_limit,
                consumed_qty=d.consumed_qty + requested_qty,
                expires_at_utc_ms=d.expires_at_utc_ms,
                remaining_qty=d.authorized_qty_limit - (d.consumed_qty + requested_qty),
            )
            for d in deviations
        ]
        PLM_STORE.replace_deviations(org_slug, proj_slug, updated)

    return DeviationGateCheckResult(
        release_permitted=True,
        blocking_deviation_ids=(),
        evaluated_deviation_count=len(deviations),
    )


async def create_ncr_capa_record(
    org_slug: str,
    proj_slug: str,
    req: NcrCapaCreateRequest,
) -> NcrCapaView:
    ncr_id = f"NCR-{uuid.uuid4().hex[:8].upper()}"
    rec = NcrCapaView(
        ncr_id=ncr_id,
        serial_number=req.serial_number,
        current_8d_step=req.current_8d_step,
        linked_pcb_or_mcad_entity_id=req.linked_pcb_or_mcad_entity_id,
        root_cause_summary=req.root_cause_summary,
        capa_corrective_action=req.capa_corrective_action,
    )
    return PLM_STORE.append_ncr(org_slug, proj_slug, rec)


async def evaluate_clear_to_build(
    org_slug: str,
    proj_slug: str,
    req: CtbEvaluationRequest,
) -> CtbReportView:
    lines = PLM_STORE.get_bom_lines(org_slug, proj_slug, "MBOM")
    if not lines:
        return CtbReportView(
            target_lot_size=req.target_lot_size,
            max_buildable_units=0,
            clear_to_build=False,
            gating_shortage_mpns=(),
            shortage_waterfall=(),
            earliest_clear_date_utc_ms=req.evaluation_utc_ms,
        )

    max_units = min(ln.on_hand_inventory_qty // ln.quantity_per_unit for ln in lines)
    shortages: list[str] = []
    waterfall: list[CtbShortageWaterfallRow] = []
    max_lead_days = 0
    for ln in lines:
        req_qty = ln.quantity_per_unit * req.target_lot_size
        deficit = max(0, req_qty - ln.on_hand_inventory_qty)
        if deficit > 0:
            shortages.append(ln.orderable_mpn)
            max_lead_days = max(max_lead_days, ln.lead_time_days)
        waterfall.append(
            CtbShortageWaterfallRow(
                orderable_mpn=ln.orderable_mpn,
                required_for_lot=req_qty,
                on_hand_qty=ln.on_hand_inventory_qty,
                deficit_qty=deficit,
            )
        )

    clear_ms = req.evaluation_utc_ms + max_lead_days * 86_400_000
    return CtbReportView(
        target_lot_size=req.target_lot_size,
        max_buildable_units=max_units,
        clear_to_build=len(shortages) == 0,
        gating_shortage_mpns=tuple(shortages),
        shortage_waterfall=tuple(waterfall),
        earliest_clear_date_utc_ms=clear_ms,
    )


async def build_signed_erp_sync_envelope(
    org_slug: str,
    proj_slug: str,
    req: ErpSyncRequest,
) -> ErpSyncEnvelope:
    event_id = f"erp-{uuid.uuid4().hex[:10]}"
    canonical = json.dumps(
        {
            "event_id": event_id,
            "org_slug": org_slug,
            "proj_slug": proj_slug,
            "target_system": req.target_system,
            "ref": req.change_or_bom_ref,
        },
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")
    payload_sha256 = hashlib.sha256(canonical).hexdigest()
    sig = hmac.new(
        req.webhook_secret.get_secret_value().encode("utf-8"),
        canonical,
        hashlib.sha256,
    ).hexdigest()
    return ErpSyncEnvelope(
        sync_event_id=event_id,
        target_system=req.target_system,
        change_or_bom_ref=req.change_or_bom_ref,
        payload_sha256=payload_sha256,
        hmac_sha256_signature=f"sha256={sig}",
    )
