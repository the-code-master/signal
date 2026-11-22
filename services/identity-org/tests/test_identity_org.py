# Module: services/identity-org/tests/test_identity_org.py — PKCE, Break-Glass, Contractor Sweep & Part 11 Tests
# Invariants: Verifies S256 PKCE + __Host- cookie, 2-of-3 quorum + 24h cooling, expires_at sweep, and ML-DSA-65 sign-off.
# Spec: architecture/08-identity-enterprise-and-project-model.md, architecture/15-*.md
from __future__ import annotations

import asyncio

import httpx
from api_gateway.main import create_app
from identity_org.service import compute_pkce_s256_challenge, verify_pkce_s256


def test_pkce_s256_verifier_and_host_cookie_upgrade() -> None:
    verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk_signal2026"
    challenge = compute_pkce_s256_challenge(verifier)
    assert verify_pkce_s256(verifier, challenge) is True
    assert verify_pkce_s256("short-verifier", challenge) is False

    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            ok_res = await client.post(
                "/api/v1/auth/sso/pkce/callback",
                headers={"Idempotency-Key": "10000000-0000-4000-8000-000000000010"},
                json={
                    "org_slug": "signal-design-inc",
                    "sso_subject": "saml:okta:alice",
                    "provider": "GITHUB",
                    "provider_user_id": "gh_uid_991827",
                    "code_verifier": verifier,
                    "expected_code_challenge": challenge,
                    "state_nonce": "nonce-abc-123456",
                    "expected_state_nonce": "nonce-abc-123456",
                },
            )
            assert ok_res.status_code == 200
            body = ok_res.json()
            assert body["session_cookie_name"] == "__Host-signal_session"
            assert "SameSite=Strict" in body["cookie_attributes"]
            set_cookie = ok_res.headers.get("set-cookie", "")
            assert "__Host-signal_session=" in set_cookie
            assert "HttpOnly" in set_cookie

            bad_res = await client.post(
                "/api/v1/auth/sso/pkce/callback",
                headers={"Idempotency-Key": "10000000-0000-4000-8000-000000000011"},
                json={
                    "org_slug": "signal-design-inc",
                    "sso_subject": "saml:okta:alice",
                    "provider": "GITHUB",
                    "provider_user_id": "gh_uid_991827",
                    "code_verifier": verifier,
                    "expected_code_challenge": "wrong_challenge",
                    "state_nonce": "nonce-abc-123456",
                    "expected_state_nonce": "nonce-abc-123456",
                },
            )
            assert bad_res.status_code == 400

    asyncio.run(_run())


def test_break_glass_2_of_3_quorum_24h_cooling_and_uniform_404() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            # Non-admin contractor receives uniform 404 (never 403):
            denied = await client.post(
                "/api/v1/orgs/signal-design-inc/break-glass/initiate",
                headers={
                    "X-Signal-Guest-User": "local-pcb-contractor-0002",
                    "X-Signal-StepUp-Proof": "webauthn_passkey_assertion_valid_001",
                    "Idempotency-Key": "10000000-0000-4000-8000-000000000001",
                },
                json={"reason": "IdP outage emergency recovery", "initiated_at_utc_ms": "1800000000000"},
            )
            assert denied.status_code == 404

            init_res = await client.post(
                "/api/v1/orgs/signal-design-inc/break-glass/initiate",
                headers={
                    "X-Signal-StepUp-Proof": "webauthn_passkey_assertion_valid_001",
                    "Idempotency-Key": "10000000-0000-4000-8000-000000000002",
                },
                json={"reason": "IdP outage emergency recovery", "initiated_at_utc_ms": "1800000000000"},
            )
            assert init_res.status_code == 200
            bg = init_res.json()
            assert bg["status"] == "PENDING_QUORUM"
            assert bg["cooling_ends_at_utc_ms"] == "1800086400000"
            req_id = bg["request_id"]

            # Duplicate approval by same admin rejected (409):
            dup_res = await client.post(
                f"/api/v1/orgs/signal-design-inc/break-glass/{req_id}/approve",
                headers={
                    "X-Signal-StepUp-Proof": "webauthn_passkey_assertion_valid_002",
                    "Idempotency-Key": "10000000-0000-4000-8000-000000000003",
                },
                json={"approver_user_id": "local-guest-0001", "approved_at_utc_ms": "1800000050000"},
            )
            assert dup_res.status_code == 409

            # Second distinct super-admin within 24h -> COOLING_24H:
            cool_res = await client.post(
                f"/api/v1/orgs/signal-design-inc/break-glass/{req_id}/approve",
                headers={
                    "X-Signal-StepUp-Proof": "webauthn_passkey_assertion_valid_003",
                    "Idempotency-Key": "10000000-0000-4000-8000-000000000004",
                },
                json={"approver_user_id": "super-admin-0002", "approved_at_utc_ms": "1800001000000"},
            )
            assert cool_res.status_code == 200
            assert cool_res.json()["quorum_met"] is True
            assert cool_res.json()["status"] == "COOLING_24H"

            # Third super-admin after 24h cooling window -> ACTIVE_EMERGENCY:
            active_res = await client.post(
                f"/api/v1/orgs/signal-design-inc/break-glass/{req_id}/approve",
                headers={
                    "X-Signal-StepUp-Proof": "webauthn_passkey_assertion_valid_004",
                    "Idempotency-Key": "10000000-0000-4000-8000-000000000005",
                },
                json={"approver_user_id": "super-admin-0003", "approved_at_utc_ms": "1800090000000"},
            )
            assert active_res.status_code == 200
            assert active_res.json()["status"] == "ACTIVE_EMERGENCY"

    asyncio.run(_run())


def test_contractor_expiry_sweep_and_part11_ml_dsa_signature() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            sweep = await client.post(
                "/api/v1/orgs/signal-design-inc/memberships/sweep-expired",
                headers={"Idempotency-Key": "10000000-0000-4000-8000-000000000006"},
                json={"evaluation_utc_ms": "1800000000000"},
            )
            assert sweep.status_code == 200
            sbody = sweep.json()
            assert "temp-emc-consultant-0003" in sbody["revoked_member_ids"]
            assert "local-guest-0001" in sbody["active_member_ids"]

            att = await client.post(
                "/api/v1/orgs/signal-design-inc/attestations/sign",
                headers={
                    "X-Signal-StepUp-Proof": "webauthn_passkey_part11_signoff_001",
                    "Idempotency-Key": "10000000-0000-4000-8000-000000000007",
                },
                json={
                    "standard": "ISO_26262",
                    "meaning": "APPROVED",
                    "artifact_id": "fmeda-aurora-asil-d-v1",
                    "artifact_blake3_hex": "a" * 64,
                    "ml_dsa_65_signature_hex": "b" * 96,
                    "ed25519_signature_hex": "c" * 64,
                    "signed_at_utc_ms": "1800000005000",
                },
            )
            assert att.status_code == 200
            abody = att.json()
            assert abody["dual_pqc_verified"] is True
            assert len(abody["worm_attestation_digest"]) == 64

    asyncio.run(_run())
