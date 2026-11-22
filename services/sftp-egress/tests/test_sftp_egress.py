# Module: services/sftp-egress/tests/test_sftp_egress.py — Step-Up 2FA, TOTP Burn-List & Quorum SFTP Tests
# Invariants: Verifies <=60s WebAuthn, 401 TOTP_NONCE_ALREADY_BURNED (F-076), 503 quorum loss (F-147) & Fab/Lab/MCAD.
# Spec: architecture/07-security-encryption-dlp-and-sftp.md
from __future__ import annotations

import asyncio
import time
import uuid

import httpx
from api_gateway.main import create_app
from sftp_egress.service import compute_rfc6238_totp

SHA_FIXTURE = "a" * 64


def _headers(user: str = "local-guest-0001") -> dict[str, str]:
    return {
        "X-Signal-Guest-User": user,
        "Idempotency-Key": str(uuid.uuid4()),
        "X-Signal-StepUp-Proof": f"stepup-proof-{uuid.uuid4().hex}",
    }


def test_totp_step_up_succeeds_once_and_rejects_same_window_replay_f076() -> None:
    async def _run() -> None:
        now_sec = int(time.time())
        totp_code = compute_rfc6238_totp(now_sec)
        body = {
            "endpoint_id": "ep-fab-ttm-01",
            "bundle_kind": "FAB_GERBER_ODB_IPC2581",
            "artifact_slug": "aurora_rev_a.odb.tgz",
            "artifact_sha256": SHA_FIXTURE,
            "drc_erc_lvs_clean": True,
            "step_up": {
                "modality": "RFC6238_TOTP",
                "issued_at_epoch_sec": str(now_sec),
                "assertion_or_totp_code": totp_code,
                "challenge_nonce": "nonce-totp-window-01",
            },
        }
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            first = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sftp-releases",
                headers=_headers(),
                json=body,
            )
            assert first.status_code == 200
            data = first.json()
            assert data["quorum_audit_committed"] is True
            assert data["temp_upload_name"].endswith(".part")
            assert data["ml_dsa_65_receipt_sig"].startswith("ml-dsa-65:")

            replay = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sftp-releases",
                headers=_headers(),
                json=body,
            )
            assert replay.status_code == 401
            assert replay.json()["detail"] == "TOTP_NONCE_ALREADY_BURNED"

    asyncio.run(_run())


def test_webauthn_expiry_quorum_fail_closed_and_lab_mcad_bundles() -> None:
    async def _run() -> None:
        now_sec = int(time.time())
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            endpoints = await client.get(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sftp-endpoints",
                headers={"X-Signal-Guest-User": "local-guest-0001"},
            )
            assert endpoints.status_code == 200
            assert len(endpoints.json()) >= 3

            stale_webauthn = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sftp-releases",
                headers=_headers(),
                json={
                    "endpoint_id": "ep-lab-tuv-02",
                    "bundle_kind": "REGULATORY_LAB_EMC_PPAP",
                    "artifact_slug": "cispr25_asil_d_ppap.tgz",
                    "artifact_sha256": SHA_FIXTURE,
                    "drc_erc_lvs_clean": True,
                    "step_up": {
                        "modality": "WEBAUTHN_FIDO2",
                        "issued_at_epoch_sec": str(now_sec - 95),
                        "assertion_or_totp_code": "webauthn.assertion:fido2-passkey-sig-001",
                        "challenge_nonce": f"chal-{uuid.uuid4().hex[:10]}",
                    },
                },
            )
            assert stale_webauthn.status_code == 401
            assert stale_webauthn.json()["detail"] == "STEP_UP_ASSERTION_EXPIRED"

            degraded_quorum = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sftp-releases",
                headers=_headers(),
                json={
                    "endpoint_id": "ep-mcad-tier1-03",
                    "bundle_kind": "PARTNER_MCAD_STEP",
                    "artifact_slug": "aurora_housing.step",
                    "artifact_sha256": SHA_FIXTURE,
                    "drc_erc_lvs_clean": True,
                    "simulated_audit_quorum_acks": 1,
                    "step_up": {
                        "modality": "WEBAUTHN_FIDO2",
                        "issued_at_epoch_sec": str(now_sec),
                        "assertion_or_totp_code": "webauthn.assertion:fido2-passkey-sig-002",
                        "challenge_nonce": f"chal-{uuid.uuid4().hex[:10]}",
                    },
                },
            )
            assert degraded_quorum.status_code == 503

            ok_mcad = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sftp-releases",
                headers=_headers(),
                json={
                    "endpoint_id": "ep-mcad-tier1-03",
                    "bundle_kind": "PARTNER_MCAD_STEP",
                    "artifact_slug": "aurora_housing.step",
                    "artifact_sha256": SHA_FIXTURE,
                    "drc_erc_lvs_clean": False,
                    "active_deviation_approved": True,
                    "step_up": {
                        "modality": "WEBAUTHN_FIDO2",
                        "issued_at_epoch_sec": str(now_sec),
                        "assertion_or_totp_code": "webauthn.assertion:fido2-passkey-sig-003",
                        "challenge_nonce": f"chal-{uuid.uuid4().hex[:10]}",
                    },
                },
            )
            assert ok_mcad.status_code == 200
            assert ok_mcad.json()["target_kind"] == "PARTNER_MCAD_STEP"

            contractor_denied = await client.get(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/sftp-endpoints",
                headers={"X-Signal-Guest-User": "local-pcb-contractor-0002"},
            )
            assert contractor_denied.status_code == 404

    asyncio.run(_run())
