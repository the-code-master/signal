# Module: services/project-catalog/src/project_catalog/router.py — Fail-Closed URL-Scoped FastAPI Router
# Invariants: Explicit Depends(require_repo_access(...)) + uniform 404 anti-enumeration (F-084).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from project_catalog.schemas import ProjectCatalogResponse
from project_catalog.service import execute_project_catalog
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.get("/api/v1/orgs/{org_slug}/projects", response_model=ProjectCatalogResponse)
async def handle_project_catalog(
    org_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> ProjectCatalogResponse:
    return await execute_project_catalog(principal.org_slug)
