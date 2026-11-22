# Module: services/vcs-auditor/src/vcs_auditor/router.py — Fail-Closed URL-Scoped FastAPI Router
# Invariants: Explicit Depends(require_repo_access(...)) + uniform 404 anti-enumeration (F-084).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access
from vcs_auditor.schemas import VcsPostureReport
from vcs_auditor.service import execute_vcs_auditor

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.get("/api/v1/orgs/{org_slug}/vcs/posture", response_model=VcsPostureReport)
async def handle_vcs_auditor(
    org_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> VcsPostureReport:
    return await execute_vcs_auditor(principal.org_slug)
