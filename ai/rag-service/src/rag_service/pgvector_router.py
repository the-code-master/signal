# Module: ai/rag-service/src/rag_service/pgvector_router.py — 3-Tier Sharded pgvector Router & Tri-Modal Reranker
# Invariants: Enforces <=500 partitions/cluster, 64 org_hash buckets, NOBYPASSRLS, and 0.45/0.35/0.20 fusion (F-086, F-130).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

import hashlib
import math
import re

from rag_service.schemas import (
    PgVectorShardPlan,
    RagChunkDocument,
    RagSearchQuery,
    RagShardingTier,
    RankedRagHit,
)

MAX_PARTITIONS_PER_PG_CLUSTER = 500
TIER2_HASH_BUCKET_COUNT = 64
_WORD_RE = re.compile(r"[a-z0-9_]+")


class PartitionCeilingExceededError(RuntimeError):
    """Raised when a PostgreSQL pgvector cluster would exceed the 500-partition catalog ceiling (F-086)."""


def compute_org_hash64_bucket(org_id: str) -> tuple[int, str]:
    """Maps an org_id deterministically onto fixed partition `org_hash_00`..`org_hash_63` (F-086, F-130)."""
    clean = org_id.strip()
    if not clean:
        raise ValueError("org_id must be non-empty")
    digest = hashlib.sha256(clean.encode("utf-8")).digest()
    bucket = int.from_bytes(digest[:4], "big") % TIER2_HASH_BUCKET_COUNT
    return bucket, f"org_hash_{bucket:02d}"


def plan_three_tier_pgvector_route(
    *,
    org_id: str,
    org_slug: str,
    tier: RagShardingTier,
    active_cluster_partitions: int = 65,
) -> PgVectorShardPlan:
    """Plans 3-Tier pgvector query routing with hard <=500 partition cap and relaxed_order iterative HNSW scan."""
    if active_cluster_partitions > MAX_PARTITIONS_PER_PG_CLUSTER:
        raise PartitionCeilingExceededError(
            f"pgvector cluster partition count {active_cluster_partitions} exceeds ceiling {MAX_PARTITIONS_PER_PG_CLUSTER}"
        )
    safe_org = re.sub(r"[^a-zA-Z0-9_-]", "", org_id)
    safe_slug = re.sub(r"[^a-z0-9_]", "_", org_slug.lower()).strip("_") or "tenant"

    if tier == "TIER1_GLOBAL":
        return PgVectorShardPlan(
            tier=tier,
            target_table="datasheet_chunks_global",
            partition_bucket_id="global_shared",
            cluster_partition_count=active_cluster_partitions,
            sql_session_preamble=(
                "SET LOCAL ROLE signal_app_nobypassrls;",
                "SET LOCAL hnsw.iterative_scan = 'relaxed_order';",
            ),
        )
    if tier == "TIER2_ORG_HASH64":
        _, bucket_name = compute_org_hash64_bucket(safe_org)
        return PgVectorShardPlan(
            tier=tier,
            target_table=f"datasheet_chunks_{bucket_name}",
            partition_bucket_id=bucket_name,
            cluster_partition_count=active_cluster_partitions,
            sql_session_preamble=(
                "SET LOCAL ROLE signal_app_nobypassrls;",
                f"SET LOCAL app.current_org_id = '{safe_org}';",
                "SET LOCAL hnsw.iterative_scan = 'relaxed_order';",
            ),
        )
    return PgVectorShardPlan(
        tier="TIER3_SOVEREIGN_DEDICATED",
        target_table=f"sovereign_{safe_slug}.datasheet_chunks_isolated",
        partition_bucket_id=f"sovereign_{safe_slug}",
        cluster_partition_count=1,
        sql_session_preamble=(
            "SET LOCAL ROLE signal_app_nobypassrls;",
            f"SET LOCAL app.current_org_id = '{safe_org}';",
            "SET LOCAL hnsw.iterative_scan = 'strict_order';",
        ),
    )


def _tokenize(text: str) -> list[str]:
    return _WORD_RE.findall(text.lower())


def _cosine_similarity(a: tuple[float, ...], b: tuple[float, ...]) -> float:
    if not a or len(a) != len(b):
        return 0.0
    dot = sum(x * y for x, y in zip(a, b, strict=True))
    na = math.sqrt(sum(x * x for x in a))
    nb = math.sqrt(sum(y * y for y in b))
    if na <= 1e-12 or nb <= 1e-12:
        return 0.0
    return max(0.0, min(1.0, (dot / (na * nb) + 1.0) * 0.5 if dot < 0 else dot / (na * nb)))


class TriModalHybridIndex:
    """Tri-modal `0.45 * parametric + 0.35 * dense_cosine + 0.20 * bm25_lexical` retriever with tenant isolation."""

    def __init__(self) -> None:
        self._docs: list[RagChunkDocument] = []

    def upsert_chunk(self, doc: RagChunkDocument) -> None:
        self._docs = [d for d in self._docs if d.chunk_id != doc.chunk_id]
        self._docs.append(doc)

    def search(self, query: RagSearchQuery) -> tuple[RankedRagHit, ...]:
        # Strict NOBYPASSRLS tenant filter: global chunks (org_id is None) + caller's exact org_id ONLY
        visible = [d for d in self._docs if d.org_id is None or d.org_id == query.org_id]
        if not visible:
            return ()

        q_tokens = _tokenize(query.query_text)
        doc_tokens = [_tokenize(f"{d.mpn} {d.package_family} {d.text_body}") for d in visible]
        avg_dl = max(1.0, sum(len(toks) for toks in doc_tokens) / len(doc_tokens))
        n_docs = len(visible)

        df: dict[str, int] = {}
        for term in set(q_tokens):
            df[term] = sum(1 for toks in doc_tokens if term in toks)

        raw_bm25: list[float] = []
        k1, b = 1.5, 0.75
        for toks in doc_tokens:
            dl = len(toks)
            score = 0.0
            for term in q_tokens:
                tf = toks.count(term)
                if tf == 0:
                    continue
                n_t = df.get(term, 0)
                idf = math.log(1.0 + (n_docs - n_t + 0.5) / (n_t + 0.5))
                denom = tf + k1 * (1.0 - b + b * (dl / avg_dl))
                score += idf * (tf * (k1 + 1.0)) / max(1e-9, denom)
            raw_bm25.append(score)

        max_bm25 = max(raw_bm25, default=0.0)
        hits: list[RankedRagHit] = []
        for doc, bm25_val in zip(visible, raw_bm25, strict=True):
            # Parametric electrical score
            vin_ok = 1.0 if doc.max_vin_uv >= query.required_min_vin_uv else 0.15
            temp_ok = 1.0 if doc.max_temp_mk >= query.required_min_temp_mk else 0.20
            pkg_bonus = (
                1.0
                if not query.preferred_package or query.preferred_package.lower() in doc.package_family.lower()
                else 0.65
            )
            param_score = round(vin_ok * temp_ok * pkg_bonus, 6)
            dense_score = round(_cosine_similarity(query.query_embedding, doc.embedding), 6)
            bm25_norm = round(bm25_val / max_bm25, 6) if max_bm25 > 1e-9 else 0.0

            fused = round(0.45 * param_score + 0.35 * dense_score + 0.20 * bm25_norm, 6)
            hits.append(
                RankedRagHit(
                    chunk_id=doc.chunk_id,
                    mpn=doc.mpn,
                    parametric_score=param_score,
                    dense_cosine=dense_score,
                    bm25_lexical=bm25_norm,
                    fused_score=fused,
                    page=doc.page,
                    bbox_norm=doc.bbox_norm,
                    linked_errata_ids=doc.linked_errata_ids,
                    snippet=doc.text_body[:180],
                )
            )

        hits.sort(key=lambda h: (h.fused_score, h.chunk_id), reverse=True)
        return tuple(hits[: query.top_k])
