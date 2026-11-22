# Module: services/sftp-egress/src/sftp_egress/router.py — Fail-Closed URL-Scoped FastAPI Router
# Invariants: Explicit Depends(require_repo_access(...)) + uniform 404 anti-enumeration (F-084).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access, require_step_up_2fa
from sftp_egress.schemas import SftpReleaseQueued
from sftp_egress.service import execute_sftp_egress

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1003")


@router.post("/api/v1/orgs/{org_slug}/projects/{proj_slug}/sftp-releases", response_model=SftpReleaseQueued)
async def handle_sftp_egress(
    org_slug: str,
    proj_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
) -> SftpReleaseQueued:
    _ = proj_slug
    return await execute_sftp_egress(principal.org_slug)
