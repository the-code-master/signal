# Module: services/api-gateway/tests/test_releases_and_skew.py — N-2 Version Skew, What's New & Campus Edge Tests
# Invariants: Verifies N-2 minor version skew rules, What's New epoch feed, Campus Edge sync & uniform 404 auth gate.
# Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/20-*.md
from __future__ import annotations

import asyncio
import uuid

import httpx
from api_gateway.main import create_app


def _headers(user: str = "local-guest-0001", web_ver: str | None = None) -> dict[str, str]:
    hdrs = {
        "X-Signal-Guest-User": user,
        "Idempotency-Key": str(uuid.uuid4()),
    }
    if web_ver is not None:
        hdrs["X-Signal-Web-Version"] = web_ver
    return hdrs


def test_version_skew_whats_new_and_campus_edge_sync() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            anon = await client.get("/api/v1/platform/whats-new", headers=_headers("anonymous"))
            assert anon.status_code == 404

            cur = await client.get(
                "/api/v1/platform/version-skew?simulated_gateway_version=0.1.0",
                headers=_headers(web_ver="0.1.0"),
            )
            assert cur.status_code == 200
            assert cur.json()["skew_status"] == "CURRENT"
            assert cur.json()["wire_compatible"] is True
            assert cur.json()["reload_web_chunks_recommended"] is False

            soft = await client.get(
                "/api/v1/platform/version-skew?simulated_gateway_version=0.3.0",
                headers=_headers(web_ver="0.1.0"),
            )
            assert soft.status_code == 200
            assert soft.json()["skew_status"] == "SOFT_REFRESH_AVAILABLE"
            assert soft.json()["wire_compatible"] is True
            assert soft.json()["reload_web_chunks_recommended"] is True

            hard = await client.get(
                "/api/v1/platform/version-skew?simulated_gateway_version=0.4.0",
                headers=_headers(web_ver="0.1.0"),
            )
            assert hard.status_code == 200
            assert hard.json()["skew_status"] == "HARD_RELOAD_REQUIRED"
            assert hard.json()["wire_compatible"] is False

            whats_new = await client.get("/api/v1/platform/whats-new", headers=_headers())
            assert whats_new.status_code == 200
            wn_json = whats_new.json()
            assert wn_json["physics_determinism_epoch"] >= 1
            assert len(wn_json["releases"]) >= 1

            campus = await client.post(
                "/api/v1/platform/campus-edge/sync",
                headers=_headers(),
                json={
                    "campus_appliance_id": "campus-edge-k3s-iit-01",
                    "queued_super_commit_count": 4,
                    "offline_duration_ms": 7_200_000,
                },
            )
            assert campus.status_code == 200
            cbody = campus.json()
            assert cbody["upstream_reconciled"] is True
            assert cbody["synced_super_commit_count"] == 4
            assert cbody["compressed_delta_bytes"] == str(4 * 40_960)
            assert "SKYWATER_SKY130" in cbody["cached_open_pdks"]

    asyncio.run(_run())
