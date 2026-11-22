# Module: services/parts-registry/tests/test_parts_registry.py — 3-Tier Parts, NDA Org AVL, Lockfile & Provenance Tests
# Invariants: Verifies zero Tier-2 NDA leakage to Tier-1 Global catalog, lockfile drift check, and FEAT-077 broker block.
# Spec: architecture/18-universal-component-and-datasheet-repository.md
from __future__ import annotations

import asyncio

import httpx
from api_gateway.main import create_app
from parts_registry.repository import global_catalog_contains


def test_three_tier_parts_org_avl_isolation_lockfile_and_provenance() -> None:
    nda_urn = "urn:signal:part:org-nda:CUSTOM_ASIC_7NM_REV_B"

    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            # Upsert private NDA Org AVL part -> stays strictly in <org>/signal-org-parts, never in Global catalog:
            avl_res = await client.put(
                "/api/v1/orgs/signal-design-inc/parts/avl-overlays",
                headers={
                    "Idempotency-Key": "40000000-0000-4000-8000-000000000001",
                    "X-Signal-StepUp-Proof": "webauthn_librarian_avl_signoff_01",
                },
                json={
                    "part_urn": nda_urn,
                    "mpn": "AURORA-ASIC-7NM-B0",
                    "internal_ipn": "IPN-900-0042",
                    "avl_status": "PREFERRED",
                    "org_private_nda_taint": True,
                },
            )
            assert avl_res.status_code == 200
            assert avl_res.json()["vault_repo"] == "signal-design-inc/signal-org-parts"
            assert global_catalog_contains(nda_urn) is False

            # Resolve Tier-2 Org AVL part vs Tier-3 Project Lockfile pin:
            res_t2 = await client.post(
                "/api/v1/orgs/signal-design-inc/parts/resolve",
                headers={"Idempotency-Key": "40000000-0000-4000-8000-000000000002"},
                json={"part_urn": nda_urn},
            )
            assert res_t2.status_code == 200
            assert res_t2.json()["resolved_tier"] == "TIER_2_ORG_AVL_VAULT"
            assert res_t2.json()["org_private_nda_taint"] is True

            res_t3 = await client.post(
                "/api/v1/orgs/signal-design-inc/parts/resolve",
                headers={"Idempotency-Key": "40000000-0000-4000-8000-000000000003"},
                json={
                    "part_urn": "urn:signal:part:wolfspeed:C3M0016120K",
                    "pinned_blake3": "b3_rev_c3m0016120k_v1_frozen",
                },
            )
            assert res_t3.status_code == 200
            assert res_t3.json()["resolved_tier"] == "TIER_3_PROJECT_LOCKFILE"
            assert res_t3.json()["revision_blake3"] == "b3_rev_c3m0016120k_v1_frozen"

            # Check parts.lock.json drift:
            drift = await client.post(
                "/api/v1/orgs/signal-design-inc/parts/lockfile/check-drift",
                headers={"Idempotency-Key": "40000000-0000-4000-8000-000000000004"},
                json={
                    "project_slug": "aurora-ev-inverter",
                    "locked_at_utc_ms": "1800000000000",
                    "entries": [
                        {
                            "part_urn": "urn:signal:part:wolfspeed:C3M0016120K",
                            "pinned_revision_blake3": "b3_rev_c3m0016120k_v1_frozen",
                            "pinned_padstack_blake3": "b3_pad_to247_4l_kelvin",
                        }
                    ],
                },
            )
            assert drift.status_code == 200
            assert drift.json()["drifted_count"] == 1
            assert drift.json()["items"][0]["board_protected_by_lock"] is True

            # FEAT-077 Authorized Franchise vs Grey-Market Broker provenance:
            ok_prov = await client.post(
                "/api/v1/orgs/signal-design-inc/parts/provenance/verify",
                headers={"Idempotency-Key": "40000000-0000-4000-8000-000000000005"},
                json={
                    "orderable_mpn": "C3M0016120K",
                    "distributor_code": "DIGIKEY",
                    "lot_trace_signature": "lot_sig_dk_wolfspeed_2026_99812",
                },
            )
            assert ok_prov.status_code == 200
            assert ok_prov.json()["authorized_franchise_verified"] is True
            assert ok_prov.json()["procurement_blocked"] is False

            broker_prov = await client.post(
                "/api/v1/orgs/signal-design-inc/parts/provenance/verify",
                headers={"Idempotency-Key": "40000000-0000-4000-8000-000000000006"},
                json={
                    "orderable_mpn": "C3M0016120K",
                    "distributor_code": "SHENZHEN_SPOT_BROKER",
                    "lot_trace_signature": "lot_sig_spot_unverified_0001",
                },
            )
            assert broker_prov.status_code == 200
            assert broker_prov.json()["counterfeit_risk_tier"] == "GREY_MARKET_BROKER_BLOCKED"
            assert broker_prov.json()["procurement_blocked"] is True

    asyncio.run(_run())
