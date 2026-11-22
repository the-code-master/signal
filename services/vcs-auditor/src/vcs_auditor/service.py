# Module: services/vcs-auditor/src/vcs_auditor/service.py — Pure Async Domain Orchestration
# Invariants: Zero raw SQL, zero physics/DRC kernels.
# Spec: docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from vcs_auditor.repository import fetch_vcs_auditor_record
from vcs_auditor.schemas import VcsPostureReport


async def execute_vcs_auditor(org_slug: str) -> VcsPostureReport:
    return await fetch_vcs_auditor_record(org_slug)
