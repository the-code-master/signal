# Module: ai/rag-service/src/rag_service/schemas.py — 3-Tier Sharded pgvector, Tri-Modal RAG & Mirrored BOM Schemas
# Invariants: <=500 partitions/cluster (F-086); k=256 chaff-padded external BOM batches (I-09, F-146); StrictSchema.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/18-universal-component-and-datasheet-repository.md
from __future__ import annotations

from typing import Literal

from pydantic import Field

from services.common.schemas import Int64Str, StrictSchema

RagShardingTier = Literal[
    "TIER1_GLOBAL",
    "TIER2_ORG_HASH64",
    "TIER3_SOVEREIGN_DEDICATED",
]


class PgVectorShardPlan(StrictSchema):
    tier: RagShardingTier
    target_table: str
    partition_bucket_id: str
    cluster_partition_count: int = Field(le=500)
    rls_role: Literal["signal_app_nobypassrls"] = "signal_app_nobypassrls"
    sql_session_preamble: tuple[str, ...]


class RagChunkDocument(StrictSchema):
    chunk_id: str
    org_id: str | None = None
    mpn: str
    package_family: str
    max_vin_uv: Int64Str
    max_temp_mk: Int64Str
    embedding: tuple[float, ...]
    text_body: str
    page: int = Field(ge=1)
    bbox_norm: tuple[float, float, float, float]
    linked_errata_ids: tuple[str, ...] = ()


class RagSearchQuery(StrictSchema):
    org_id: str
    query_text: str
    query_embedding: tuple[float, ...]
    required_min_vin_uv: Int64Str = 0
    required_min_temp_mk: Int64Str = 0
    preferred_package: str = ""
    top_k: int = Field(default=5, ge=1, le=50)


class RankedRagHit(StrictSchema):
    chunk_id: str
    mpn: str
    parametric_score: float
    dense_cosine: float
    bm25_lexical: float
    fused_score: float
    page: int
    bbox_norm: tuple[float, float, float, float]
    linked_errata_ids: tuple[str, ...]
    snippet: str


class MirroredDistributorOffer(StrictSchema):
    mpn: str
    distributor: Literal["DIGIKEY", "MOUSER", "ARROW", "LCSC", "AVNET"]
    stock_qty: int
    unit_price_micro_usd: Int64Str
    lead_time_days: int
    embodied_carbon_mg_co2e: Int64Str
    served_from_local_mirror: bool = True


class PaddedExternalBatchQuery(StrictSchema):
    batch_size: Literal[256] = 256
    shuffled_mpns: tuple[str, ...]
    real_mpn_count: int
    chaff_mpn_count: int


class BomLineItemSpec(StrictSchema):
    refdes: str
    mpn: str
    component_kind: Literal["RESISTOR", "CAPACITOR", "INDUCTOR", "IC"]
    package_code: str
    nominal_value_si: float
    min_voltage_rating_v: float
    critical_loop_sensitivity: float = 0.25


class BomConsolidationReport(StrictSchema):
    unique_line_items_before: int
    unique_line_items_after: int
    smt_feeder_slots_saved: int
    adjoint_zero_shift_verified: bool
    replacement_map: tuple[tuple[str, str], ...]
