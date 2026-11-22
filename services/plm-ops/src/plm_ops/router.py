# Module: services/plm-ops/src/plm_ops/router.py — Fail-Closed URL-Scoped FastAPI Router
# Invariants: Explicit Depends(require_repo_access(...)) + uniform 404 anti-enumeration (F-084).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from plm_ops.schemas import PlmChangeNoticeSummary
from plm_ops.service import execute_plm_ops
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1005")


@router.get("/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/change-notices", response_model=PlmChangeNoticeSummary)
async def handle_plm_ops(
    org_slug: str,
    proj_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> PlmChangeNoticeSummary:
    _ = proj_slug
    return await execute_plm_ops(principal.org_slug)
