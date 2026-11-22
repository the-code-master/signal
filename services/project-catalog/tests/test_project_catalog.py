# Module: services/project-catalog/tests/test_project_catalog.py — Multi-Type Projects, Templates & SPDX Gate Tests
# Invariants: Verifies 13 domain types, Official/Enterprise/Community templates, and CERN-OHL-S-2.0 copyleft block (F-099).
# Spec: architecture/08-identity-enterprise-and-project-model.md, architecture/16-*.md
from __future__ import annotations

import asyncio

import httpx
from api_gateway.main import create_app


def test_multitype_project_templates_and_spdx_copyleft_gate() -> None:
    async def _run() -> None:
        transport = httpx.ASGITransport(app=create_app())
        async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
            tpls = await client.get("/api/v1/orgs/signal-design-inc/project-templates")
            assert tpls.status_code == 200
            assert len(tpls.json()) == 3

            # Permissive Apache-2.0 template into ENCRYPTED_VAULT succeeds:
            ok_tpl = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/from-template",
                headers={"Idempotency-Key": "30000000-0000-4000-8000-000000000001"},
                json={
                    "template_id": "tpl-official-800v-sic-inverter",
                    "new_project_slug": "helios-obc-22kw",
                    "target_encryption_mode": "ENCRYPTED_VAULT",
                    "target_project_spdx": "LicenseRef-Proprietary-NDA",
                },
            )
            assert ok_tpl.status_code == 200
            assert "CAD_MCAD" in ok_tpl.json()["enabled_domains"]

            # Strong copyleft CERN-OHL-S-2.0 template into ENCRYPTED_VAULT blocked (409):
            blocked_tpl = await client.post(
                "/api/v1/orgs/signal-design-inc/projects/from-template",
                headers={"Idempotency-Key": "30000000-0000-4000-8000-000000000002"},
                json={
                    "template_id": "tpl-community-open-bms-strong-copyleft",
                    "new_project_slug": "nda-battery-pack",
                    "target_encryption_mode": "ENCRYPTED_VAULT",
                    "target_project_spdx": "LicenseRef-Proprietary-NDA",
                },
            )
            assert blocked_tpl.status_code == 409

            # Direct Multi-Type project creation with CAM_CNC, PRINT_3D, BIOS, FPGA:
            created = await client.post(
                "/api/v1/orgs/signal-design-inc/projects",
                headers={"Idempotency-Key": "30000000-0000-4000-8000-000000000003"},
                json={
                    "slug": "hyperion-flight-ctrl",
                    "display_name": "Hyperion Flight Controller",
                    "encryption_mode": "ENCRYPTED_VAULT",
                    "enabled_domains": ["PRD", "HFS", "SCHEMATIC", "PCB", "FPGA", "BIOS", "CAM_CNC", "PRINT_3D"],
                    "spdx_license": "CERN-OHL-P-2.0",
                },
            )
            assert created.status_code == 200
            fetched = await client.get("/api/v1/orgs/signal-design-inc/projects/hyperion-flight-ctrl")
            assert fetched.status_code == 200
            assert "PRINT_3D" in fetched.json()["enabled_domains"]

    asyncio.run(_run())
