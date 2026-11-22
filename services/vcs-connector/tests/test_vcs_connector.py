# Module: services/vcs-connector/tests/test_vcs_connector.py — Multi-Provider Onboarding, <=40/min Pacer & ETag Tests
# Invariants: Verifies custom PEM CA validation, <project>-meta prerequisite, 40/min token-bucket cap, and ETag cache hit.
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md
from __future__ import annotations

import asyncio

import httpx
from api_gateway.main import create_app


def test_vcs_onboarding_pem_ca_lazy_pacer_and_etag_sync() -> None:
    pem = "-----BEGIN CERTIFICATE-----\nMIIB+TCCAWOgAwIBAgIJALtestCA==\n-----END CERTIFICATE-----"

    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            onb = await client.put(
                "/api/v1/orgs/signal-design-inc/vcs/connection",
                headers={
                    "Idempotency-Key": "20000000-0000-4000-8000-000000000001",
                    "X-Signal-StepUp-Proof": "webauthn_vcs_onboard_proof_0001",
                },
                json={
                    "provider": "GITLAB",
                    "namespace_kind": "ORG_NAMESPACE",
                    "namespace_handle": "signal-design-ee",
                    "vault_mode": "ENCRYPTED_VAULT",
                    "custom_ca_pem": pem,
                },
            )
            assert onb.status_code == 200
            assert onb.json()["provider"] == "GITLAB"
            assert onb.json()["custom_ca_sha256"].startswith("sha256:")

            # Lazy subrepo before <project>-meta fails closed with 409:
            premature = await client.post(
                "/api/v1/orgs/signal-design-inc/vcs/projects/ traction-gen2/subrepos/materialize".replace(" ", ""),
                headers={"Idempotency-Key": "20000000-0000-4000-8000-000000000002"},
                json={"subproject_type": "PCB", "now_utc_ms": "1810000000000"},
            )
            assert premature.status_code == 409

            # Bootstrap <project>-meta first (consumes token #1):
            meta = await client.post(
                "/api/v1/orgs/signal-design-inc/vcs/projects/traction-gen2/bootstrap-meta",
                headers={"Idempotency-Key": "20000000-0000-4000-8000-000000000003"},
                json={"now_utc_ms": "1810000000000"},
            )
            assert meta.status_code == 200
            assert meta.json()["repo_slug"] == "traction-gen2-meta"

            # Materialize 39 subrepos up to 40/min cap, then 41st queues!
            for idx in range(39):
                res = await client.post(
                    "/api/v1/orgs/signal-design-inc/vcs/projects/traction-gen2/subrepos/materialize",
                    headers={"Idempotency-Key": f"20000000-0000-4000-8000-0000000001{idx:02d}"},
                    json={"subproject_type": f"MOD_{idx}", "now_utc_ms": str(1810000000100 + idx)},
                )
                assert res.status_code == 200
                assert res.json()["status"] == "MATERIALIZED"

            capped = await client.post(
                "/api/v1/orgs/signal-design-inc/vcs/projects/traction-gen2/subrepos/materialize",
                headers={"Idempotency-Key": "20000000-0000-4000-8000-000000000299"},
                json={"subproject_type": "OVER_BURST", "now_utc_ms": "1810000010000"},
            )
            assert capped.status_code == 200
            assert capped.json()["status"] == "QUEUED_RATE_LIMITED"
            assert capped.json()["next_slot_utc_ms"] == "1810000060000"

            # Conditional ETag roster sync:
            sync1 = await client.post(
                "/api/v1/orgs/signal-design-inc/vcs/roster/sync",
                headers={"Idempotency-Key": "20000000-0000-4000-8000-000000000301"},
                json={
                    "provider_repo_id": "gl_repo_id_signal-design-inc_2001",
                    "upstream_etag": 'W/"etag-roster-v1"',
                    "collaborator_user_ids": ["local-guest-0001", "local-pcb-contractor-0002"],
                },
            )
            assert sync1.status_code == 200
            assert sync1.json()["not_modified"] is False

            sync2 = await client.post(
                "/api/v1/orgs/signal-design-inc/vcs/roster/sync",
                headers={"Idempotency-Key": "20000000-0000-4000-8000-000000000302"},
                json={
                    "provider_repo_id": "gl_repo_id_signal-design-inc_2001",
                    "if_none_match_etag": 'W/"etag-roster-v1"',
                    "upstream_etag": 'W/"etag-roster-v1"',
                    "collaborator_user_ids": [],
                },
            )
            assert sync2.status_code == 200
            assert sync2.json()["not_modified"] is True
            assert len(sync2.json()["synced_user_ids"]) == 2

    asyncio.run(_run())
