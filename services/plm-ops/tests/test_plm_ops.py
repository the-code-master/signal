# Module: services/plm-ops/tests/test_plm_ops.py — Multi-BOM, ECR->ECO->ECN, PCN->ECR, CTB & Hard Deviation SFTP Tests
# Invariants: Verifies 21 CFR Part 11 ML-DSA-65 sign-off, PCN auto-ECR, CTB waterfall & 409 Deviation SFTP block.
# Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
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


def test_multi_bom_ecr_eco_ecn_pcn_mcn_ncr_ctb_and_erp_sync() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            ebom = await client.get(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/boms/EBOM",
                headers=_headers(),
            )
            assert ebom.status_code == 200
            assert ebom.json()["dpp_compliant"] is True
            assert int(ebom.json()["total_unit_cost_micros_usd"]) > 100_000_000

            ecr_resp = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/ecos",
                headers=_headers(),
                json={
                    "kind": "ECR",
                    "title": "Qualify second-source 1200V SiC MOSFET die",
                    "wip_disposition": "REWORK",
                    "in_stock_disposition": "USE_AS_IS",
                    "field_disposition": "NONE",
                    "affected_upids": ["urn:signal:part:wolfspeed:C3M0016120K"],
                },
            )
            assert ecr_resp.status_code == 200
            cid = ecr_resp.json()["change_id"]

            to_ccb = await client.post(
                f"/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/ecos/{cid}/transitions",
                headers=_headers(),
                json={"target_state": "CCB_REVIEW"},
            )
            assert to_ccb.status_code == 200
            assert to_ccb.json()["state"] == "CCB_REVIEW"

            missing_sig = await client.post(
                f"/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/ecos/{cid}/transitions",
                headers=_headers(),
                json={"target_state": "APPROVED", "ccb_approver_count": 2, "ml_dsa_65_signature": ""},
            )
            assert missing_sig.status_code == 401

            to_eco = await client.post(
                f"/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/ecos/{cid}/transitions",
                headers=_headers(),
                json={
                    "target_state": "APPROVED",
                    "ccb_approver_count": 3,
                    "ml_dsa_65_signature": "ml-dsa-65:part11-ccb-quorum-sig-001",
                },
            )
            assert to_eco.status_code == 200
            assert to_eco.json()["kind"] == "ECO"

            to_ecn = await client.post(
                f"/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/ecos/{cid}/transitions",
                headers=_headers(),
                json={
                    "target_state": "RELEASED",
                    "ccb_approver_count": 3,
                    "ml_dsa_65_signature": "ml-dsa-65:part11-release-sig-002",
                },
            )
            assert to_ecn.status_code == 200
            assert to_ecn.json()["kind"] == "ECN"
            assert to_ecn.json()["state"] == "RELEASED"

            pcn = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/pcns",
                headers=_headers(),
                json={
                    "vendor_notice_code": "TI-PCN-202611",
                    "affected_mpn": "UCC21750QDWR",
                    "last_time_buy_utc_ms": "1820000000000",
                    "is_end_of_life_pdn": True,
                },
            )
            assert pcn.status_code == 200
            assert pcn.json()["matched_in_project_bom"] is True
            assert pcn.json()["spawned_ecr_id"].startswith("ECR-")

            mcn = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/mcns",
                headers=_headers(),
                json={
                    "workstation_id": "CELL-MECH-FINAL",
                    "updated_torque_milli_nm": "2800",
                    "cnc_or_slicer_recipe_hash": "b3:coldplate-5axis-rev2",
                    "ml_dsa_65_signature": "ml-dsa-65:mfg-eng-sig-101",
                },
            )
            assert mcn.status_code == 200
            assert mcn.json()["updated_torque_milli_nm"] == "2800"

            ncr = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/ncrs",
                headers=_headers(),
                json={
                    "serial_number": "SN-AUR-000419",
                    "current_8d_step": "D4_ROOT_CAUSE",
                    "linked_pcb_or_mcad_entity_id": "F2_GASKET_GROOVE",
                    "root_cause_summary": "O-ring compression 4% below target at corner boss",
                    "capa_corrective_action": "Deepen groove counterbore by 120_000 nm in ECN",
                },
            )
            assert ncr.status_code == 200

            ctb = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/ctb-evaluations",
                headers=_headers(),
                json={"target_lot_size": 500, "evaluation_utc_ms": "1795324800000"},
            )
            assert ctb.status_code == 200
            assert ctb.json()["max_buildable_units"] == 400
            assert ctb.json()["clear_to_build"] is False
            assert "UCC21750QDWR" in ctb.json()["gating_shortage_mpns"]

            erp = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-ev-inverter/plm/erp-sync",
                headers=_headers(),
                json={
                    "target_system": "SAP_S4HANA",
                    "webhook_secret": "sap-outbound-hmac-secret",
                    "change_or_bom_ref": cid,
                },
            )
            assert erp.status_code == 200
            assert erp.json()["hmac_sha256_signature"].startswith("sha256=")

    asyncio.run(_run())


def test_hard_deviation_waiver_blocks_over_limit_sftp_release_with_409() -> None:
    async def _run() -> None:
        now_sec = int(time.time())
        future_ms = (now_sec + 3600) * 1000
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            dev = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-lot-waiver/plm/deviations",
                headers=_headers(),
                json={
                    "reason": "Alternate NTC thermistor reel for DVT pilot build",
                    "authorized_qty_limit": "50",
                    "consumed_qty": "40",
                    "expires_at_utc_ms": str(future_ms),
                    "ccb_ml_dsa_65_signature": "ml-dsa-65:ccb-waiver-approval-004",
                },
            )
            assert dev.status_code == 200
            assert dev.json()["remaining_qty"] == "10"

            within_cap = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-lot-waiver/sftp-releases",
                headers=_headers(),
                json={
                    "endpoint_id": "ep-fab-ttm-01",
                    "bundle_kind": "FAB_GERBER_ODB_IPC2581",
                    "artifact_slug": "aurora_dvt_lot.odb.tgz",
                    "artifact_sha256": "b" * 64,
                    "drc_erc_lvs_clean": False,
                    "active_deviation_approved": True,
                    "requested_release_qty": "10",
                    "step_up": {
                        "modality": "WEBAUTHN_FIDO2",
                        "issued_at_epoch_sec": str(now_sec),
                        "assertion_or_totp_code": "webauthn.assertion:fido2-passkey-waiver-ok",
                        "challenge_nonce": f"chal-{uuid.uuid4().hex[:10]}",
                    },
                },
            )
            assert within_cap.status_code == 200

            over_cap = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/aurora-lot-waiver/sftp-releases",
                headers=_headers(),
                json={
                    "endpoint_id": "ep-fab-ttm-01",
                    "bundle_kind": "FAB_GERBER_ODB_IPC2581",
                    "artifact_slug": "aurora_dvt_lot_extra.odb.tgz",
                    "artifact_sha256": "c" * 64,
                    "drc_erc_lvs_clean": False,
                    "active_deviation_approved": True,
                    "requested_release_qty": "5",
                    "step_up": {
                        "modality": "WEBAUTHN_FIDO2",
                        "issued_at_epoch_sec": str(now_sec),
                        "assertion_or_totp_code": "webauthn.assertion:fido2-passkey-waiver-over",
                        "challenge_nonce": f"chal-{uuid.uuid4().hex[:10]}",
                    },
                },
            )
            assert over_cap.status_code == 409
            assert over_cap.json()["detail"] == "DEVIATION_WAIVER_EXCEEDED"

    asyncio.run(_run())
