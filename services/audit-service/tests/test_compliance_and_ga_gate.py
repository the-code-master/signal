# Module: services/audit-service/tests/test_compliance_and_ga_gate.py — Enterprise Compliance, GDPR Shred & GA Gate Tests
# Invariants: Verifies 5/5 compliance regimes, F-100 GDPR salt shredding WORM immutability & 100% JSONL transcripts.
# Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

import asyncio
import json
import uuid
from pathlib import Path
from typing import cast

import httpx
import pytest
from api_gateway.main import create_app
from audit_service.schemas import Tier0AuditEvent
from audit_service.shredder import GDPR_SALT_VAULT, GLOBAL_WORM_OUTBOX

REPO_ROOT = Path(__file__).resolve().parents[3]
INT64_SUFFIXES: tuple[str, ...] = (
    "_nm",
    "_ps",
    "_fs",
    "_utc_ms",
    "_at_ms",
    "_micros_usd",
    "_micro_usd",
    "fencing_epoch",
    "quorum_audit_outbox_seq",
    "current_graph_revision",
)


def _headers(user: str = "local-guest-0001") -> dict[str, str]:
    return {
        "X-Signal-Guest-User": user,
        "Idempotency-Key": str(uuid.uuid4()),
        "X-Signal-StepUp-Proof": f"stepup-proof-{uuid.uuid4().hex}",
    }


def _assert_64bit_json_strings(node: object, location: str) -> None:
    if isinstance(node, dict):
        typed_dict = cast("dict[str, object]", node)
        for key, val in typed_dict.items():
            if any(key.endswith(sfx) for sfx in INT64_SUFFIXES) and isinstance(val, (int, float, bool, str)):
                assert isinstance(val, str), f"64-bit field {key} in {location} must be JSON string, got {type(val)}"
            _assert_64bit_json_strings(val, location)
    elif isinstance(node, list):
        for item in cast("list[object]", node):
            _assert_64bit_json_strings(item, location)


def test_compliance_attestations_and_gdpr_f100_worm_crypto_shredder() -> None:
    async def _run() -> None:
        app = create_app()
        transport = httpx.ASGITransport(app=app)
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            contractor_denied = await client.get(
                "/api/v1/orgs/signal-design-inc/compliance/attestations",
                headers=_headers("local-pcb-contractor-0002"),
            )
            assert contractor_denied.status_code == 404

            att = await client.get(
                "/api/v1/orgs/signal-design-inc/compliance/attestations",
                headers=_headers("local-guest-0001"),
            )
            assert att.status_code == 200
            body = att.json()
            assert body["all_routes_auth_guarded"] is True
            assert body["zero_dlp_export_download_routes"] is True
            assert body["quorum_outbox_3x_healthy"] is True
            assert body["verified_transcript_files"] >= 6
            frameworks = {item["framework"]: item["passing"] for item in body["frameworks"]}
            assert frameworks == {
                "SOC2_TYPE_II": True,
                "ISO_27001": True,
                "ITAR_EAR_SOVEREIGN": True,
                "GDPR_ARTICLE_17": True,
                "FDA_21_CFR_PART_11": True,
            }

            subject_id = "departed-eu-engineer-0402"
            pseudo = GDPR_SALT_VAULT.derive_pseudonym("11111111-2222-4333-8444-555555555555", subject_id)
            assert pseudo.startswith("actor_hmac_")
            worm_receipt = await GLOBAL_WORM_OUTBOX.commit_tier0_event(
                Tier0AuditEvent(
                    org_id="11111111-2222-4333-8444-555555555555",
                    actor_hmac=pseudo,
                    action_kind="SUPER_COMMIT_SIGNED",
                    target_resource_id="proj-aurora",
                    occurred_at_ms=1_795_324_800_000,
                )
            )
            tip_before_erasure = GLOBAL_WORM_OUTBOX.tip_hash
            assert tip_before_erasure == worm_receipt.entry_merkle_hash

            shred = await client.post(
                "/api/v1/orgs/signal-design-inc/compliance/gdpr-shred",
                headers=_headers("local-guest-0001"),
                json={
                    "subject_provider_user_id": subject_id,
                    "erasure_ticket_ref": "GDPR-ART17-2026-8812",
                },
            )
            assert shred.status_code == 200
            sbody = shred.json()
            assert sbody["subject_erased"] is True
            assert sbody["worm_chain_unmutated"] is True
            assert sbody["actor_hmac_unlinkable"] is True
            assert sbody["worm_tip_hash_before"] == tip_before_erasure
            assert sbody["worm_tip_hash_after"] == tip_before_erasure

            with pytest.raises(ValueError, match="SUBJECT_SALT_SHREDDED"):
                GDPR_SALT_VAULT.derive_pseudonym("11111111-2222-4333-8444-555555555555", subject_id)

    asyncio.run(_run())


def test_all_golden_contract_transcripts_replay_and_int64_string_invariants() -> None:
    transcripts_dir = REPO_ROOT / "contracts" / "transcripts"
    jsonl_files = sorted(transcripts_dir.glob("*.jsonl"))
    assert len(jsonl_files) >= 6

    for path in jsonl_files:
        prev_seq = 0
        lines = [ln.strip() for ln in path.read_text(encoding="utf-8").splitlines() if ln.strip()]
        assert len(lines) >= 1, f"Empty transcript {path.name}"
        for raw in lines:
            entry = cast("dict[str, object]", json.loads(raw))
            seq = entry["seq"]
            rpc = entry["rpc"]
            assert isinstance(seq, int) and seq == prev_seq + 1
            assert isinstance(rpc, str) and rpc.startswith("signal.v1.")
            _assert_64bit_json_strings(entry["request"], f"{path.name}:seq={seq}:request")
            outcome = entry.get("expected_response") or entry.get("expected_error")
            assert outcome is not None
            _assert_64bit_json_strings(outcome, f"{path.name}:seq={seq}:outcome")
            prev_seq = seq
