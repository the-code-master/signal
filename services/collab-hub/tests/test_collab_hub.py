# Module: services/collab-hub/tests/test_collab_hub.py — Collab Space, Status Tower & Auto-Review Security Tests
# Invariants: Verifies DEK ciphertext, contractor anchor redaction + uniform 404, <=15m SFU TTL, and 2FA+quorum sign-off.
# Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
from __future__ import annotations

import asyncio
import time
import uuid

import httpx
from api_gateway.main import create_app


def _headers(user: str = "local-guest-0001") -> dict[str, str]:
    return {
        "X-Signal-Guest-User": user,
        "Idempotency-Key": str(uuid.uuid4()),
        "X-Signal-StepUp-Proof": f"stepup-proof-{uuid.uuid4().hex}",
    }


def test_five_team_dek_channels_contractor_masking_uniform_404_and_huddle_ticket() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            base = "/api/v1/orgs/signal-design-inc/projects/proj-DekMask01/collab"

            admin_ch = await client.get(f"{base}/channels", headers=_headers("local-guest-0001"))
            assert admin_ch.status_code == 200
            teams = {c["team"] for c in admin_ch.json()}
            assert {
                "DESIGN",
                "DEVELOPMENT_BRINGUP",
                "SOFTWARE_FIRMWARE",
                "MANUFACTURING",
                "OPERATIONS_SUPPLY_CHAIN",
                "CROSS_TEAM_PROGRAM",
            }.issubset(teams)

            contractor_ch = await client.get(f"{base}/channels", headers=_headers("local-pcb-contractor-0002"))
            assert contractor_ch.status_code == 200
            contractor_ids = {c["channel_id"] for c in contractor_ch.json()}
            assert contractor_ids == {"ch-dev-hil", "ch-all-hands"}

            forbidden_ccb = await client.get(
                f"{base}/channels/ch-ccb-board/messages",
                headers=_headers("local-pcb-contractor-0002"),
            )
            assert forbidden_ccb.status_code == 404
            assert forbidden_ccb.json()["code"] == "NOT_FOUND"

            shared_msgs = await client.get(
                f"{base}/channels/ch-all-hands/messages",
                headers=_headers("local-pcb-contractor-0002"),
            )
            assert shared_msgs.status_code == 200
            first_msg = shared_msgs.json()[0]
            assert first_msg["stored_dek_envelope"].startswith("sigv1.collab-dek.")
            assert "@U1_SIC_HS" not in first_msg["stored_dek_envelope"]
            assert first_msg["redacted_anchor_count"] == 1
            anchors = first_msg["entity_anchors"]
            assert anchors[0]["entity_id"] == "@U1_SIC_HS"
            assert anchors[0]["redacted"] is False
            assert anchors[1]["entity_id"] == "[Restricted Entity — Access Required]"
            assert anchors[1]["commit_sha"] == "REDACTED"
            assert anchors[1]["redacted"] is True

            ok_anchor = await client.get(
                f"{base}/anchors/anc-pcb-u1-sic",
                headers=_headers("local-pcb-contractor-0002"),
            )
            assert ok_anchor.status_code == 200
            assert ok_anchor.json()["x_nm"] == "42500000"

            denied_anchor = await client.get(
                f"{base}/anchors/anc-ops-eco-104",
                headers=_headers("local-pcb-contractor-0002"),
            )
            assert denied_anchor.status_code == 404

            post_resp = await client.post(
                f"{base}/channels/ch-dev-hil/messages",
                headers=_headers("local-pcb-contractor-0002"),
                json={
                    "plaintext_body": "Verified gate-drive rise time at @TP_VGS_HS.",
                    "entity_anchors": [
                        {
                            "anchor_id": "anc-pcb-tp-vgs",
                            "domain": "PCB",
                            "entity_id": "@TP_VGS_HS",
                            "commit_sha": "b192e04",
                            "x_nm": 12_800_000,
                            "y_nm": 9_400_000,
                            "z_nm": 0,
                            "required_provider_repo_id": "local_gh_repo_1003",
                        }
                    ],
                },
            )
            assert post_resp.status_code == 200
            assert post_resp.json()["stored_dek_envelope"].startswith("sigv1.collab-dek.")

            now_ms = int(time.time() * 1000)
            huddle = await client.post(
                f"{base}/channels/ch-dev-hil/huddle-tickets",
                headers=_headers("local-pcb-contractor-0002"),
                json={"requested_ttl_ms": 900_000, "follow_presenter_viewport": True},
            )
            assert huddle.status_code == 200
            hbody = huddle.json()
            assert hbody["sfu_turns_endpoint"] == "turns:huddle.signal.local:443"
            assert hbody["geometry_recording_disabled"] is True
            assert int(hbody["expires_at_utc_ms"]) - now_ms <= 905_000

    asyncio.run(_run())


def test_status_tower_dependency_rag_escalation_periodic_reviews_and_signoff_quorum() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            base = "/api/v1/orgs/signal-design-inc/projects/proj-TowerRev02/collab"

            tower0 = await client.get(f"{base}/status-tower?now_utc_ms=1795324800000", headers=_headers())
            assert tower0.status_code == 200
            assert tower0.json()["overall_health"] == "GREEN"

            overdue_dep = await client.post(
                f"{base}/dependencies?now_utc_ms=1795324800000",
                headers=_headers(),
                json={
                    "producer_team": "DESIGN",
                    "consumer_team": "SOFTWARE_FIRMWARE",
                    "deliverable_title": "Late ADC calibration pinout table",
                    "due_at_utc_ms": 1_795_000_000_000,
                },
            )
            assert overdue_dep.status_code == 200
            tbody = overdue_dep.json()
            assert tbody["overall_health"] == "RED"
            fw_card = next(c for c in tbody["team_rag_cards"] if c["team"] == "SOFTWARE_FIRMWARE")
            assert fw_card["auto_health"] == "RED"
            assert fw_card["open_critical_blockers"] == 1

            bad_override = await client.post(
                f"{base}/status-tower/rag-overrides",
                headers=_headers(),
                json={"team": "SOFTWARE_FIRMWARE", "override_health": "AMBER", "override_rationale": "short"},
            )
            assert bad_override.status_code == 422

            ok_override = await client.post(
                f"{base}/status-tower/rag-overrides?now_utc_ms=1795324800000",
                headers=_headers(),
                json={
                    "team": "SOFTWARE_FIRMWARE",
                    "override_health": "AMBER",
                    "override_rationale": "Fallback mock calibration LUT active until rev E netlist lands.",
                },
            )
            assert ok_override.status_code == 200
            assert ok_override.json()["overall_health"] == "AMBER"

            dec = await client.post(
                f"{base}/decisions",
                headers=_headers(),
                json={
                    "record_kind": "EDR",
                    "title": "Pin SiC Gate Resistor to 2.2 Ohm Automotive Pulse Grade",
                    "rationale": "Limits dV/dt < 45 V/ns while keeping switching loss within thermal budget.",
                },
            )
            assert dec.status_code == 200
            assert dec.json()["ml_dsa_65_signature"].startswith("ml-dsa-65:")

            for cadence in ("WEEKLY_ENGINEERING", "MONTHLY_EXECUTIVE", "STAGE_GATE_STATUS"):
                rev_resp = await client.post(
                    f"{base}/reviews/generate",
                    headers=_headers(),
                    json={"cadence": cadence, "stage_gate_name": "DVT", "now_utc_ms": 1_795_324_800_000},
                )
                assert rev_resp.status_code == 200
                rjson = rev_resp.json()
                assert rjson["archived_ops_repo_path"].startswith("reviews/2026/")
                assert rjson["ml_dsa_65_review_signature"].startswith("ml-dsa-65:")
                assert "@U1_SIC_HS" in rjson["gbnf_verified_narrative_md"]
                last_review_id = rjson["review_id"]

            quorum_fail = await client.post(
                f"{base}/reviews/{last_review_id}/sign-off",
                headers=_headers(),
                json={
                    "stepup_2fa_nonce": f"nonce-quorum-fail-{uuid.uuid4().hex}",
                    "ml_dsa_65_approver_signature": "ml-dsa-65:approver99887766554433221100aabbccdd",
                    "simulated_healthy_audit_replicas": 2,
                    "now_utc_ms": 1_795_324_800_000,
                },
            )
            assert quorum_fail.status_code == 503

            shared_nonce = f"nonce-single-use-{uuid.uuid4().hex}"
            sign_ok = await client.post(
                f"{base}/reviews/{last_review_id}/sign-off",
                headers=_headers(),
                json={
                    "stepup_2fa_nonce": shared_nonce,
                    "ml_dsa_65_approver_signature": "ml-dsa-65:approver99887766554433221100aabbccdd",
                    "simulated_healthy_audit_replicas": 3,
                    "now_utc_ms": 1_795_324_800_000,
                },
            )
            assert sign_ok.status_code == 200
            assert sign_ok.json()["gate_approved"] is True
            assert int(sign_ok.json()["quorum_audit_outbox_seq"]) >= 1

            replay_401 = await client.post(
                f"{base}/reviews/{last_review_id}/sign-off",
                headers=_headers(),
                json={
                    "stepup_2fa_nonce": shared_nonce,
                    "ml_dsa_65_approver_signature": "ml-dsa-65:approver99887766554433221100aabbccdd",
                    "simulated_healthy_audit_replicas": 3,
                    "now_utc_ms": 1_795_324_800_000,
                },
            )
            assert replay_401.status_code == 401
            assert replay_401.json()["detail"] == "STEPUP_NONCE_ALREADY_BURNED"

    asyncio.run(_run())
