# Module: ai/datasheet-ingest/src/datasheet_ingest/schemas.py — Schema-Locked Quarantine AST & Curve Models
# Invariants: StrictSchema extra="forbid", frozen=True; quarantine_verified=True; 64-bit SI Int64Str parity (F-131, F-144).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, contracts/schemas/datasheet-record.schema.json
from __future__ import annotations

from typing import Literal

from pydantic import Field

from services.common.schemas import Int64Str, StrictSchema

ElectricalClassLiteral = Literal[
    "POWER_IN",
    "POWER_OUT",
    "GND",
    "BIDIR",
    "INPUT",
    "OUTPUT",
    "OPEN_DRAIN",
    "DIFF_P",
    "DIFF_N",
    "NC",
]

ConditionMetricLiteral = Literal[
    "C_EFF_AF",
    "L_EFF_NH",
    "R_DS_ON_UOHM",
    "R_THETA_JA_MK_PER_W",
]


class DatasheetCitation(StrictSchema):
    page: int = Field(ge=1)
    bbox_norm: tuple[float, float, float, float]


class DatasheetPinAst(StrictSchema):
    pin_number: str = Field(min_length=1)
    signal_name: str = Field(min_length=1)
    electrical_class: ElectricalClassLiteral
    abs_max_uv: Int64Str
    citation: DatasheetCitation


class ConditionCurvePointAst(StrictSchema):
    metric: ConditionMetricLiteral
    temp_mk: Int64Str
    bias_uv: Int64Str
    freq_hz: Int64Str
    value_scaled: Int64Str


class DatasheetRecordAst(StrictSchema):
    schema_version: Literal["1.0.0"] = "1.0.0"
    mpn: str = Field(min_length=1)
    manufacturer: str = Field(min_length=1)
    doc_blake3: str = Field(pattern=r"^[0-9a-f]{64}$")
    quarantine_verified: Literal[True] = True
    pins: tuple[DatasheetPinAst, ...]
    condition_curves: tuple[ConditionCurvePointAst, ...]


class SiliconErrataBulletin(StrictSchema):
    errata_id: str = Field(min_length=1)
    die_revision: str = Field(min_length=1)
    affected_functional_block: str
    workaround_summary: str


class PcnPdnBulletin(StrictSchema):
    bulletin_id: str = Field(min_length=1)
    notice_kind: Literal["PCN", "PDN", "EOL", "NRND"]
    effective_date_iso: str
    replacement_mpn: str | None = None


class ConditionCurveSummary(StrictSchema):
    mpn: str
    metric_kind: str
    bias_uv_points: tuple[Int64Str, ...]
    temp_mkelvin_points: tuple[Int64Str, ...]
    freq_hz_points: tuple[Int64Str, ...]
    normalized_ratios: tuple[float, ...]
    r_theta_ja_at_100sqmm_mk_per_w: Int64Str
    linked_silicon_errata_ids: tuple[str, ...]


class DualVlmConsensusReport(StrictSchema):
    reconciled_record: DatasheetRecordAst
    needs_human_review: bool
    max_observed_divergence_pct: float
    discrepancy_reasons: tuple[str, ...]
    condition_curve_summary: ConditionCurveSummary
    linked_pcn_ids: tuple[str, ...]


class DatasheetRevisionDiff(StrictSchema):
    mpn: str
    old_doc_blake3: str
    new_doc_blake3: str
    changed_pin_count: int
    tightened_abs_max_count: int
    parameter_delta_summaries: tuple[str, ...]


class DatasheetTestbenchPlan(StrictSchema):
    generated_branch_id: str
    spice_netlist_deck: str
    schematic_component_refdes: tuple[str, ...]
    reference_curve_rms_error_pct: float
    extracted_curves: ConditionCurveSummary


class GlossaryTranslationResult(StrictSchema):
    translated_text: str
    preserved_refdes_and_si_tokens: tuple[str, ...]
    quarantine_warnings: tuple[str, ...]
