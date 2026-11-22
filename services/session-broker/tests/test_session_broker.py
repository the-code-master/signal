# Module: services/session-broker/tests/test_session_broker.py — Sticky WebTransport Ticket & Paired-Region DR Tests
# Invariants: Verifies signed wtt_v1 sticky tickets and strict rejection of cross-jurisdiction & Sovereign DR (F-142).
# Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/16-*.md
from __future__ import annotations

import asyncio

import httpx
from api_gateway.main import create_app


def test_sticky_cell_ticket_and_in_jurisdiction_paired_dr_policy() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            sess = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sessions",
                headers={"Idempotency-Key": "31000000-0000-4000-8000-000000000001"},
            )
            assert sess.status_code == 200
            sbody = sess.json()
            assert sbody["sticky_webtransport_ticket"].startswith("wtt_v1.")
            assert sbody["active_fencing_epoch"] == "9007199254740993"

            # Allowed US pair (us-east-1 <-> us-west-2) and EU pair (eu-central-1 <-> eu-west-1):
            ok_us = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sessions/dr-policy",
                headers={"Idempotency-Key": "31000000-0000-4000-8000-000000000002"},
                json={
                    "residency_policy": "IN_JURISDICTION_PAIRED_DR",
                    "primary_region": "us-east-1",
                    "dr_standby_region": "us-west-2",
                },
            )
            assert ok_us.status_code == 200
            assert ok_us.json()["jurisdiction_code"] == "US_COMMERCIAL"

            ok_eu = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sessions/dr-policy",
                headers={"Idempotency-Key": "31000000-0000-4000-8000-000000000003"},
                json={
                    "residency_policy": "IN_JURISDICTION_PAIRED_DR",
                    "primary_region": "eu-central-1",
                    "dr_standby_region": "eu-west-1",
                },
            )
            assert ok_eu.status_code == 200
            assert ok_eu.json()["jurisdiction_code"] == "EU_GDPR"

            # Cross-jurisdiction US <-> EU strictly rejected (409):
            cross = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sessions/dr-policy",
                headers={"Idempotency-Key": "31000000-0000-4000-8000-000000000004"},
                json={
                    "residency_policy": "IN_JURISDICTION_PAIRED_DR",
                    "primary_region": "us-east-1",
                    "dr_standby_region": "eu-west-1",
                },
            )
            assert cross.status_code == 409

            # SOVEREIGN_SINGLE_REGION_STRICT with secondary DR region strictly rejected (409):
            sov_bad = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sessions/dr-policy",
                headers={"Idempotency-Key": "31000000-0000-4000-8000-000000000005"},
                json={
                    "residency_policy": "SOVEREIGN_SINGLE_REGION_STRICT",
                    "primary_region": "eu-central-1",
                    "dr_standby_region": "eu-west-1",
                },
            )
            assert sov_bad.status_code == 409

    asyncio.run(_run())
