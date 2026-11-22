# Module: services/collab-hub/src/collab_hub/service.py — Pure Async Domain Orchestration
# Invariants: Zero raw SQL, zero physics/DRC kernels.
# Spec: docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from collab_hub.repository import fetch_collab_hub_record
from collab_hub.schemas import AutomatedReviewSummary


async def execute_collab_hub(org_slug: str) -> AutomatedReviewSummary:
    return await fetch_collab_hub_record(org_slug)
