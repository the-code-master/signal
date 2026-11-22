# Module: services/plm-ops/src/plm_ops/service.py — Pure Async Domain Orchestration
# Invariants: Zero raw SQL, zero physics/DRC kernels.
# Spec: docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from plm_ops.repository import fetch_plm_ops_record
from plm_ops.schemas import PlmChangeNoticeSummary


async def execute_plm_ops(org_slug: str) -> PlmChangeNoticeSummary:
    return await fetch_plm_ops_record(org_slug)
