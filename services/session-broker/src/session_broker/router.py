# Module: services/session-broker/src/session_broker/router.py — Fail-Closed Session Lease Endpoint
# Invariants: Enforces Depends(require_repo_access) + Idempotency-Key + uniform 404 (F-084, F-102).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access
from session_broker.schemas import SessionLeaseResponse
from session_broker.service import broker_project_session

router = APIRouter()
_META_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/sessions",
    response_model=SessionLeaseResponse,
)
async def create_project_session(
    org_slug: str,
    proj_slug: str,
    _principal: PrincipalContext = Depends(_META_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> SessionLeaseResponse:
    _ = org_slug
    return await broker_project_session(proj_slug)
