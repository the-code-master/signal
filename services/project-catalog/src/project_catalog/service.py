# Module: services/project-catalog/src/project_catalog/service.py — Pure Async Domain Orchestration
# Invariants: Zero raw SQL, zero physics/DRC kernels.
# Spec: docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from project_catalog.repository import fetch_project_catalog_record
from project_catalog.schemas import ProjectCatalogResponse


async def execute_project_catalog(org_slug: str) -> ProjectCatalogResponse:
    return await fetch_project_catalog_record(org_slug)
