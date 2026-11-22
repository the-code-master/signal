# Module: services/session-broker/src/session_broker/router.py — Sticky Session Lease & Paired-Region DR Router
# Invariants: Enforces Depends(require_*) + Idempotency-Key + uniform 404 (F-084, F-102, F-142).
# Spec: architecture/04-kubernetes-runtime-and-collaboration.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_org_admin, require_repo_access
from session_broker.schemas import (
    PairedRegionDrPolicyRequest,
    PairedRegionDrPolicyResponse,
    SessionLeaseResponse,
)
from session_broker.service import broker_project_session, configure_paired_region_dr_policy

router = APIRouter()
_META_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/sessions",
    response_model=SessionLeaseResponse,
)
async def create_project_session(
    org_slug: str,
    proj_slug: str,
    principal: PrincipalContext = Depends(_META_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> SessionLeaseResponse:
    _ = org_slug
    return await broker_project_session(proj_slug, principal.actor_hmac)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/sessions/dr-policy",
    response_model=PairedRegionDrPolicyResponse,
)
async def handle_dr_policy(
    org_slug: str,
    proj_slug: str,
    payload: PairedRegionDrPolicyRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _idem: str = Depends(require_idempotency_key),
) -> PairedRegionDrPolicyResponse:
    _ = org_slug
    return await configure_paired_region_dr_policy(principal.org_slug, proj_slug, payload)
