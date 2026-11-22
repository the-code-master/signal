# Module: services/parts-registry/src/parts_registry/service.py — Pure Async Domain Orchestration
# Invariants: Zero raw SQL, zero physics/DRC kernels.
# Spec: docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from parts_registry.repository import fetch_parts_registry_record
from parts_registry.schemas import PartsSearchResponse


async def execute_parts_registry(org_slug: str) -> PartsSearchResponse:
    return await fetch_parts_registry_record(org_slug)
