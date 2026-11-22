# Module: ai/rag-service/src/rag_service/__init__.py — 3-Tier Sharded RAG & Mirrored BOM Public Exports
# Invariants: <=500 pgvector partitions/cluster (F-086); k=256 chaff-padded external BOM batches (I-09, F-146).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md
from __future__ import annotations

from rag_service.bom_consolidator import consolidate_bom_feeder_slots
from rag_service.mirrored_bom import (
    CHAFF_PADDED_BATCH_SIZE,
    MirroredDistributorCatalog,
    build_k256_chaff_padded_batch,
)
from rag_service.pgvector_router import (
    MAX_PARTITIONS_PER_PG_CLUSTER,
    TIER2_HASH_BUCKET_COUNT,
    PartitionCeilingExceededError,
    TriModalHybridIndex,
    compute_org_hash64_bucket,
    plan_three_tier_pgvector_route,
)
from rag_service.router import create_app, router
from rag_service.schemas import (
    BomConsolidationReport,
    BomLineItemSpec,
    MirroredDistributorOffer,
    PaddedExternalBatchQuery,
    PgVectorShardPlan,
    RagChunkDocument,
    RagSearchQuery,
    RagShardingTier,
    RankedRagHit,
)

__all__ = [
    "CHAFF_PADDED_BATCH_SIZE",
    "MAX_PARTITIONS_PER_PG_CLUSTER",
    "TIER2_HASH_BUCKET_COUNT",
    "BomConsolidationReport",
    "BomLineItemSpec",
    "MirroredDistributorCatalog",
    "MirroredDistributorOffer",
    "PaddedExternalBatchQuery",
    "PartitionCeilingExceededError",
    "PgVectorShardPlan",
    "RagChunkDocument",
    "RagSearchQuery",
    "RagShardingTier",
    "RankedRagHit",
    "TriModalHybridIndex",
    "build_k256_chaff_padded_batch",
    "compute_org_hash64_bucket",
    "consolidate_bom_feeder_slots",
    "create_app",
    "plan_three_tier_pgvector_route",
    "router",
]
