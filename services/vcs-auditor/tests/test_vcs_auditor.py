# Module: services/vcs-auditor/tests/test_vcs_auditor.py — Multi-Provider BYO-Git Security Posture Scorer Tests
# Invariants: Verifies 0-100 weighted deductions across public exposure, branch protection, secrets, Padmé & expiry.
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md
from __future__ import annotations

import asyncio

import httpx
from api_gateway.main import create_app


def test_vcs_auditor_evaluates_multi_provider_posture_score() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            clean = await client.post(
                "/api/v1/orgs/signal-design-inc/vcs/posture/evaluate",
                headers={"Idempotency-Key": "21000000-0000-4000-8000-000000000001"},
                json={
                    "repositories": [
                        {
                            "provider_repo_id": "gh_repo_1001",
                            "provider": "GITHUB",
                            "is_public": False,
                            "branch_protection_enabled": True,
                            "secret_scan_clean": True,
                            "padme_merkle_sig_valid": True,
                            "expired_contractor_count": 0,
                        }
                    ]
                },
            )
            assert clean.status_code == 200
            assert clean.json()["score"] == 100

            degraded = await client.post(
                "/api/v1/orgs/signal-design-inc/vcs/posture/evaluate",
                headers={"Idempotency-Key": "21000000-0000-4000-8000-000000000002"},
                json={
                    "repositories": [
                        {
                            "provider_repo_id": "bb_repo_2002",
                            "provider": "BITBUCKET",
                            "is_public": True,
                            "branch_protection_enabled": True,
                            "secret_scan_clean": True,
                            "padme_merkle_sig_valid": False,
                            "expired_contractor_count": 1,
                        }
                    ]
                },
            )
            assert degraded.status_code == 200
            body = degraded.json()
            assert body["score"] == 40
            assert body["padme_vault_enforced"] is False
            assert body["public_exposure_blocked"] is False
            assert len(body["findings"]) == 3

    asyncio.run(_run())
