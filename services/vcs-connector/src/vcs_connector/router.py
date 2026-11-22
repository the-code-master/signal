# Module: services/vcs-connector/src/vcs_connector/router.py — Fail-Closed URL-Scoped FastAPI Router
# Invariants: Explicit Depends(require_repo_access(...)) + uniform 404 anti-enumeration (F-084).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access, require_step_up_2fa
from vcs_connector.schemas import VcsConnectionStatus
from vcs_connector.service import execute_vcs_connector

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.put("/api/v1/orgs/{org_slug}/vcs/connection", response_model=VcsConnectionStatus)
async def handle_vcs_connector(
    org_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
) -> VcsConnectionStatus:
    return await execute_vcs_connector(principal.org_slug)
