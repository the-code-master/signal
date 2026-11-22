# Module: services/vcs-connector/src/vcs_connector/service.py — Pure Async Domain Orchestration
# Invariants: Zero raw SQL, zero physics/DRC kernels.
# Spec: docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from vcs_connector.repository import fetch_vcs_connector_record
from vcs_connector.schemas import VcsConnectionStatus


async def execute_vcs_connector(org_slug: str) -> VcsConnectionStatus:
    return await fetch_vcs_connector_record(org_slug)
