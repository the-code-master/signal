# Module: services/identity-org/src/identity_org/router.py — Fail-Closed URL-Scoped FastAPI Router
# Invariants: Explicit Depends(require_repo_access(...)) + uniform 404 anti-enumeration (F-084).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from identity_org.schemas import AccessReviewEntry
from identity_org.service import execute_identity_org
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.get("/api/v1/orgs/{org_slug}/access-review", response_model=AccessReviewEntry)
async def handle_identity_org(
    org_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> AccessReviewEntry:
    return await execute_identity_org(principal.org_slug)
