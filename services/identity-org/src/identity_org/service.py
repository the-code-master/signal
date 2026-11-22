# Module: services/identity-org/src/identity_org/service.py — Pure Async Domain Orchestration
# Invariants: Zero raw SQL, zero physics/DRC kernels.
# Spec: docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from identity_org.repository import fetch_identity_org_record
from identity_org.schemas import AccessReviewEntry


async def execute_identity_org(org_slug: str) -> AccessReviewEntry:
    return await fetch_identity_org_record(org_slug)
