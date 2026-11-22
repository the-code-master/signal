# Module: services/plm-ops/src/plm_ops/schemas.py — Native PLM Multi-BOM, ECO/MCN/PCN, Deviation, 8D & CTB DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); Int64Str on 64-bit SI/timestamps/quantities; SecretStr on secrets.
# Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
from __future__ import annotations

from typing import Literal

from pydantic import Field, SecretStr

from services.common.schemas import Int64Str, StrictSchema

BomViewKind = Literal["EBOM", "MBOM", "SBOM"]
ChangeOrderKind = Literal["ECR", "ECO", "ECN"]
ChangeLifecycleState = Literal["DRAFT", "CCB_REVIEW", "APPROVED", "RELEASED"]
DispositionAction = Literal["USE_AS_IS", "REWORK", "SCRAP", "RETROFIT_FIELD", "NONE"]
EightDStep = Literal[
    "D1_TEAM",
    "D2_PROBLEM",
    "D3_CONTAINMENT",
    "D4_ROOT_CAUSE",
    "D5_CORRECTIVE_ACTION",
    "D6_VALIDATION",
    "D7_PREVENTION",
    "D8_CLOSURE",
]
ErpSystemTarget = Literal[
    "SAP_S4HANA",
    "ORACLE_NETSUITE",
    "ARENA_PLM",
    "PROPEL_PLM",
    "KATANA_MRP",
    "FACTORY_MES",
]


class PlmChangeNoticeSummary(StrictSchema):
    change_id: str
    project_slug: str
    kind: str
    status: str
    ctb_readiness_bps: int


class BomLineDto(StrictSchema):
    line_id: str
    upid: str
    orderable_mpn: str
    quantity_per_unit: int = Field(ge=1)
    on_hand_inventory_qty: int = Field(default=5000, ge=0)
    lead_time_days: int = Field(default=14, ge=0)
    unit_cost_micros_usd: Int64Str
    embodied_co2e_mg: Int64Str
    rohs_compliant: bool = True
    reach_svhc_free: bool = True
    pfas_free: bool = True
    workstation_step_id: str = "SMT-LINE-01"


class MultiViewBomRollup(StrictSchema):
    view: BomViewKind
    lines: tuple[BomLineDto, ...]
    total_unit_cost_micros_usd: Int64Str
    total_embodied_co2e_mg: Int64Str
    dpp_compliant: bool


class EcoCreateRequest(StrictSchema):
    kind: ChangeOrderKind = "ECR"
    title: str = Field(min_length=4, max_length=160)
    wip_disposition: DispositionAction = "REWORK"
    in_stock_disposition: DispositionAction = "USE_AS_IS"
    field_disposition: DispositionAction = "NONE"
    affected_upids: tuple[str, ...] = ()


class EcoTransitionRequest(StrictSchema):
    target_state: ChangeLifecycleState
    ccb_approver_count: int = Field(default=0, ge=0, le=12)
    ml_dsa_65_signature: str = ""


class EcoRecordView(StrictSchema):
    change_id: str
    kind: ChangeOrderKind
    title: str
    state: ChangeLifecycleState
    wip_disposition: DispositionAction
    in_stock_disposition: DispositionAction
    field_disposition: DispositionAction
    affected_upids: tuple[str, ...]
    ops_repo_commit_sha: str


class McnCreateRequest(StrictSchema):
    workstation_id: str = Field(min_length=2, max_length=64)
    updated_torque_milli_nm: Int64Str
    cnc_or_slicer_recipe_hash: str = Field(min_length=8, max_length=96)
    ml_dsa_65_signature: str = Field(pattern=r"^ml-dsa-65:[A-Za-z0-9_-]{8,128}$")


class McnRecordView(StrictSchema):
    mcn_id: str
    workstation_id: str
    updated_torque_milli_nm: Int64Str
    cnc_or_slicer_recipe_hash: str
    ops_repo_commit_sha: str


class PcnIngestRequest(StrictSchema):
    vendor_notice_code: str = Field(min_length=3, max_length=64)
    affected_mpn: str = Field(min_length=3, max_length=64)
    last_time_buy_utc_ms: Int64Str
    is_end_of_life_pdn: bool = True


class PcnImpactView(StrictSchema):
    pcn_id: str
    affected_mpn: str
    matched_in_project_bom: bool
    last_time_buy_utc_ms: Int64Str
    spawned_ecr_id: str | None


class DeviationCreateRequest(StrictSchema):
    reason: str = Field(min_length=5, max_length=200)
    authorized_qty_limit: Int64Str
    consumed_qty: Int64Str = 0
    expires_at_utc_ms: Int64Str
    ccb_ml_dsa_65_signature: str = Field(pattern=r"^ml-dsa-65:[A-Za-z0-9_-]{8,128}$")


class DeviationWaiverView(StrictSchema):
    deviation_id: str
    reason: str
    authorized_qty_limit: Int64Str
    consumed_qty: Int64Str
    expires_at_utc_ms: Int64Str
    remaining_qty: Int64Str


class DeviationGateCheckResult(StrictSchema):
    release_permitted: bool
    blocking_deviation_ids: tuple[str, ...]
    evaluated_deviation_count: int


class NcrCapaCreateRequest(StrictSchema):
    serial_number: str = Field(min_length=3, max_length=64)
    current_8d_step: EightDStep
    linked_pcb_or_mcad_entity_id: str = Field(min_length=2, max_length=64)
    root_cause_summary: str = Field(min_length=4, max_length=240)
    capa_corrective_action: str = Field(min_length=4, max_length=240)


class NcrCapaView(StrictSchema):
    ncr_id: str
    serial_number: str
    current_8d_step: EightDStep
    linked_pcb_or_mcad_entity_id: str
    root_cause_summary: str
    capa_corrective_action: str


class CtbShortageWaterfallRow(StrictSchema):
    orderable_mpn: str
    required_for_lot: int
    on_hand_qty: int
    deficit_qty: int


class CtbEvaluationRequest(StrictSchema):
    target_lot_size: int = Field(ge=1, le=1_000_000)
    evaluation_utc_ms: Int64Str


class CtbReportView(StrictSchema):
    target_lot_size: int
    max_buildable_units: int
    clear_to_build: bool
    gating_shortage_mpns: tuple[str, ...]
    shortage_waterfall: tuple[CtbShortageWaterfallRow, ...]
    earliest_clear_date_utc_ms: Int64Str


class ErpSyncRequest(StrictSchema):
    target_system: ErpSystemTarget
    webhook_secret: SecretStr
    change_or_bom_ref: str = Field(min_length=3, max_length=64)


class ErpSyncEnvelope(StrictSchema):
    sync_event_id: str
    target_system: ErpSystemTarget
    change_or_bom_ref: str
    payload_sha256: str
    hmac_sha256_signature: str
