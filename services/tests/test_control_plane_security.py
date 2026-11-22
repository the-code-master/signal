# Module: services/tests/test_control_plane_security.py — Control-Plane Auth, Uniform-404 & BigInt Tests
# Invariants: Verifies startup fail-closed route gate, contractor repo scoping, and >2^53-1 Int64Str parity.
# Spec: architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

import asyncio
import json
from pathlib import Path

import httpx
import pytest
from api_gateway.main import create_app
from fastapi import FastAPI

from services.common.schemas import FencingAuthoritySchema
from services.common.security import assert_all_routes_guarded

REPO_ROOT = Path(__file__).resolve().parents[2]


def test_startup_gate_rejects_unguarded_route() -> None:
    bad_app = FastAPI()

    @bad_app.get("/api/v1/orgs/{org_slug}/secret-designs")
    async def _forgot_auth(org_slug: str) -> dict[str, str]:
        return {"org": org_slug}

    with pytest.raises(RuntimeError, match="lacks Depends"):
        assert_all_routes_guarded(bad_app)


def test_startup_gate_rejects_forbidden_dlp_download_route() -> None:
    dlp_app = FastAPI()

    @dlp_app.get("/api/v1/orgs/{org_slug}/download")
    async def _bad_download(org_slug: str) -> dict[str, str]:
        return {"org": org_slug}

    with pytest.raises(RuntimeError, match="Zero-Leak DLP gate"):
        assert_all_routes_guarded(dlp_app)


def test_int64str_roundtrips_golden_fencing_fixture_beyond_2_pow_53() -> None:
    fx = (REPO_ROOT / "contracts" / "fixtures" / "fencing_authority.fixture.json").read_text(encoding="utf-8")
    parsed = FencingAuthoritySchema.model_validate_json(fx)
    assert parsed.fencing_epoch == 9_007_199_254_740_993
    dumped = json.loads(parsed.model_dump_json())
    assert dumped["fencing_epoch"] == "9007199254740993"


def test_uniform_404_anti_enumeration_and_contractor_repo_scoping() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            ok_releases = await client.get("/api/v1/platform/releases")
            assert ok_releases.status_code == 200
            assert ok_releases.json()[0]["version"] == "0.1.0"

            wrong_org = await client.get("/api/v1/orgs/other-tenant/projects")
            assert wrong_org.status_code == 404
            assert wrong_org.json()["code"] == "NOT_FOUND"

            contractor_headers = {"X-Signal-Guest-User": "local-pcb-contractor-0002"}
            denied_plm = await client.get(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/change-notices",
                headers=contractor_headers,
            )
            assert denied_plm.status_code == 404

            ok_session = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sessions",
                headers={
                    "X-Signal-Guest-User": "local-pcb-contractor-0002",
                    "Idempotency-Key": "7f9c2b10-6e44-4a80-9b11-001122334455",
                },
            )
            assert ok_session.status_code == 200
            assert ok_session.json()["active_fencing_epoch"] == "9007199254740993"

    asyncio.run(_run())
