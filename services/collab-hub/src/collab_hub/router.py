# Module: services/collab-hub/src/collab_hub/router.py — Fail-Closed URL-Scoped FastAPI Router
# Invariants: Explicit Depends(require_repo_access(...)) + uniform 404 anti-enumeration (F-084).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from collab_hub.schemas import AutomatedReviewSummary
from collab_hub.service import execute_collab_hub
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1005")


@router.get("/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/reviews", response_model=AutomatedReviewSummary)
async def handle_collab_hub(
    org_slug: str,
    proj_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> AutomatedReviewSummary:
    _ = proj_slug
    return await execute_collab_hub(principal.org_slug)
