# Module: ai/rag-service/src/rag_service/router.py — Fail-Closed FastAPI Router for 3-Tier RAG & Mirrored BOM
# Invariants: Explicit Depends(require_org_member) + startup assert_all_routes_guarded; tenant-isolated RAG (F-086).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from fastapi import APIRouter, Depends, FastAPI

from rag_service.bom_consolidator import consolidate_bom_feeder_slots
from rag_service.pgvector_router import TriModalHybridIndex, plan_three_tier_pgvector_route
from rag_service.schemas import (
    BomConsolidationReport,
    BomLineItemSpec,
    PgVectorShardPlan,
    RagSearchQuery,
    RagShardingTier,
    RankedRagHit,
    StrictSchema,
)
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import assert_all_routes_guarded, require_org_member

router = APIRouter()
_DEFAULT_INDEX = TriModalHybridIndex()


class ShardPlanApiRequest(StrictSchema):
    tier: RagShardingTier = "TIER2_ORG_HASH64"
    active_cluster_partitions: int = 65


class ConsolidateBomApiRequest(StrictSchema):
    items: tuple[BomLineItemSpec, ...]
    max_allowed_metric_shift_pct: float = 1.5


@router.post(
    "/api/v1/orgs/{org_slug}/ai/rag/shard-plan",
    response_model=PgVectorShardPlan,
)
async def handle_shard_plan(
    org_slug: str,
    payload: ShardPlanApiRequest,
    principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> PgVectorShardPlan:
    _ = org_slug
    return plan_three_tier_pgvector_route(
        org_id=principal.org_id,
        org_slug=principal.org_slug,
        tier=payload.tier,
        active_cluster_partitions=payload.active_cluster_partitions,
    )


@router.post(
    "/api/v1/orgs/{org_slug}/ai/rag/search",
    response_model=tuple[RankedRagHit, ...],
)
async def handle_rag_search(
    org_slug: str,
    payload: RagSearchQuery,
    principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> tuple[RankedRagHit, ...]:
    _ = org_slug
    enforced_query = RagSearchQuery(
        org_id=principal.org_id,
        query_text=payload.query_text,
        query_embedding=payload.query_embedding,
        required_min_vin_uv=payload.required_min_vin_uv,
        required_min_temp_mk=payload.required_min_temp_mk,
        preferred_package=payload.preferred_package,
        top_k=payload.top_k,
    )
    return _DEFAULT_INDEX.search(enforced_query)


@router.post(
    "/api/v1/orgs/{org_slug}/ai/rag/consolidate-bom",
    response_model=BomConsolidationReport,
)
async def handle_consolidate_bom(
    org_slug: str,
    payload: ConsolidateBomApiRequest,
    _principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> BomConsolidationReport:
    _ = org_slug
    return consolidate_bom_feeder_slots(payload.items, payload.max_allowed_metric_shift_pct)


def create_app() -> FastAPI:
    app = FastAPI(title="Signal 3-Tier Sharded RAG & Mirrored BOM Service")
    app.include_router(router)
    assert_all_routes_guarded(app)
    return app
