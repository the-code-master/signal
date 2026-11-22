# Module: services/parts-registry/src/parts_registry/router.py — Fail-Closed URL-Scoped FastAPI Router
# Invariants: Explicit Depends(require_repo_access(...)) + uniform 404 anti-enumeration (F-084).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from parts_registry.schemas import PartsSearchResponse
from parts_registry.service import execute_parts_registry
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.get("/api/v1/orgs/{org_slug}/parts", response_model=PartsSearchResponse)
async def handle_parts_registry(
    org_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> PartsSearchResponse:
    return await execute_parts_registry(principal.org_slug)
