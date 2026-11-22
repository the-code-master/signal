# Module: ai/rag-service/tests/test_rag_service.py — Unit Tests for 3-Tier pgvector RAG & Mirrored BOM Rescue
# Invariants: Verifies <=500 partition cap, relaxed_order HNSW, tenant isolation, k=256 chaff batching, and FEAT-108.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/16-*
from __future__ import annotations

import pytest
from rag_service import (
    CHAFF_PADDED_BATCH_SIZE,
    BomLineItemSpec,
    MirroredDistributorCatalog,
    MirroredDistributorOffer,
    PartitionCeilingExceededError,
    RagChunkDocument,
    RagSearchQuery,
    TriModalHybridIndex,
    build_k256_chaff_padded_batch,
    compute_org_hash64_bucket,
    consolidate_bom_feeder_slots,
    create_app,
    plan_three_tier_pgvector_route,
)


def test_three_tier_pgvector_sharding_enforces_500_partition_cap_and_hnsw_scan() -> None:
    bucket_idx, bucket_name = compute_org_hash64_bucket("org-alpha-01")
    assert 0 <= bucket_idx < 64
    assert bucket_name == f"org_hash_{bucket_idx:02d}"

    t2 = plan_three_tier_pgvector_route(
        org_id="org-alpha-01",
        org_slug="alpha",
        tier="TIER2_ORG_HASH64",
        active_cluster_partitions=65,
    )
    assert t2.target_table == f"datasheet_chunks_{bucket_name}"
    assert "SET LOCAL hnsw.iterative_scan = 'relaxed_order';" in t2.sql_session_preamble
    assert t2.rls_role == "signal_app_nobypassrls"

    t3 = plan_three_tier_pgvector_route(
        org_id="org-defense",
        org_slug="defense-aero",
        tier="TIER3_SOVEREIGN_DEDICATED",
        active_cluster_partitions=1,
    )
    assert t3.target_table.startswith("sovereign_defense_aero.")

    with pytest.raises(PartitionCeilingExceededError):
        plan_three_tier_pgvector_route(
            org_id="org-overflow",
            org_slug="overflow",
            tier="TIER2_ORG_HASH64",
            active_cluster_partitions=501,
        )


def test_trimodal_hybrid_reranker_enforces_tenant_isolation_and_ranks_parametric_fit() -> None:
    idx = TriModalHybridIndex()
    idx.upsert_chunk(
        RagChunkDocument(
            chunk_id="chk-buck-36v",
            org_id=None,
            mpn="LM5164QDDARQ1",
            package_family="SO PowerPAD-8",
            max_vin_uv=100_000_000,
            max_temp_mk=423_150,
            embedding=(0.9, 0.1, 0.0),
            text_body="Automotive 100V synchronous buck converter low IQ ultra-low standby.",
            page=1,
            bbox_norm=(0.05, 0.1, 0.9, 0.3),
            linked_errata_ids=("ERR-LM5164-01",),
        )
    )
    idx.upsert_chunk(
        RagChunkDocument(
            chunk_id="chk-buck-5v",
            org_id=None,
            mpn="TPS62840DLCR",
            package_family="VSON-8",
            max_vin_uv=6_500_000,
            max_temp_mk=398_150,
            embedding=(0.88, 0.12, 0.0),
            text_body="6.5V step-down synchronous buck converter.",
            page=2,
            bbox_norm=(0.1, 0.2, 0.8, 0.4),
        )
    )
    idx.upsert_chunk(
        RagChunkDocument(
            chunk_id="chk-nda-other-org",
            org_id="org-rival",
            mpn="RIVAL-SECRET-ASIC",
            package_family="FCBGA",
            max_vin_uv=100_000_000,
            max_temp_mk=423_150,
            embedding=(1.0, 0.0, 0.0),
            text_body="Automotive 100V synchronous buck converter secret NDA.",
            page=1,
            bbox_norm=(0.0, 0.0, 1.0, 1.0),
        )
    )

    hits = idx.search(
        RagSearchQuery(
            org_id="org-caller",
            query_text="automotive synchronous buck converter",
            query_embedding=(0.92, 0.08, 0.0),
            required_min_vin_uv=36_000_000,
            required_min_temp_mk=398_150,
        )
    )
    assert len(hits) == 2  # org-rival NDA chunk strictly invisible!
    assert hits[0].mpn == "LM5164QDDARQ1"
    assert hits[0].fused_score > hits[1].fused_score


def test_mirrored_distributor_catalog_k256_chaff_batch_and_bom_consolidator() -> None:
    cat = MirroredDistributorCatalog()
    cat.ingest_mirror_snapshot(
        [
            MirroredDistributorOffer(
                mpn="TPS62840DLCR",
                distributor="DIGIKEY",
                stock_qty=14_500,
                unit_price_micro_usd=890_000,
                lead_time_days=3,
                embodied_carbon_mg_co2e=42_000,
            )
        ]
    )

    # 100% mirror hit => zero external batch emitted!
    offers_hit, ext_none = cat.lookup_bom_with_privacy_guard(["TPS62840DLCR"])
    assert len(offers_hit) == 1
    assert ext_none is None

    # Cache miss => padded to exact k=256 shuffled batch!
    offers_mix, ext_batch = cat.lookup_bom_with_privacy_guard(
        ["TPS62840DLCR", "NEW_UNMIRRORED_LDO_01", "NEW_UNMIRRORED_LDO_02"],
        category_hint="POWER",
    )
    assert len(offers_mix) == 1
    assert ext_batch is not None
    assert len(ext_batch.shuffled_mpns) == CHAFF_PADDED_BATCH_SIZE
    assert ext_batch.real_mpn_count == 2
    assert ext_batch.chaff_mpn_count == 254
    assert "NEW_UNMIRRORED_LDO_01" in ext_batch.shuffled_mpns
    assert ext_batch.shuffled_mpns[0] != "NEW_UNMIRRORED_LDO_01"  # shuffled, not prefix-leaked

    direct = build_k256_chaff_padded_batch(["RC0402-10K"])
    assert len(set(direct.shuffled_mpns)) == 256

    # FEAT-108 greedy set-cover BOM consolidation
    bom = [
        BomLineItemSpec(
            refdes="C1",
            mpn="CAP-0402-100N-16V",
            component_kind="CAPACITOR",
            package_code="0402",
            nominal_value_si=100e-9,
            min_voltage_rating_v=16.0,
            critical_loop_sensitivity=0.05,
        ),
        BomLineItemSpec(
            refdes="C2",
            mpn="CAP-0402-100N-25V",
            component_kind="CAPACITOR",
            package_code="0402",
            nominal_value_si=100e-9,
            min_voltage_rating_v=25.0,
            critical_loop_sensitivity=0.05,
        ),
        BomLineItemSpec(
            refdes="R1",
            mpn="RES-0402-10K0",
            component_kind="RESISTOR",
            package_code="0402",
            nominal_value_si=10_000.0,
            min_voltage_rating_v=50.0,
            critical_loop_sensitivity=0.02,
        ),
        BomLineItemSpec(
            refdes="R2",
            mpn="RES-0402-10K2",
            component_kind="RESISTOR",
            package_code="0402",
            nominal_value_si=10_200.0,
            min_voltage_rating_v=50.0,
            critical_loop_sensitivity=0.02,
        ),
    ]
    consolidated = consolidate_bom_feeder_slots(bom, max_allowed_metric_shift_pct=1.0)
    assert consolidated.unique_line_items_before == 4
    assert consolidated.unique_line_items_after == 2
    assert consolidated.smt_feeder_slots_saved == 2
    assert consolidated.adjoint_zero_shift_verified is True

    app = create_app()
    assert len(app.routes) >= 3
