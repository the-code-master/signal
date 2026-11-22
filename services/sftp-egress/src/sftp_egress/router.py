# Module: services/sftp-egress/src/sftp_egress/router.py — Fail-Closed URL-Scoped SFTP Egress FastAPI Router
# Invariants: Explicit Depends(require_repo_access(...)) + uniform 404 + zero /export or /download paths (F-084).
# Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access, require_step_up_2fa
from sftp_egress.schemas import (
    SftpEndpointCreateRequest,
    SftpEndpointView,
    SftpReleaseQueued,
    SftpReleaseRequest,
)
from sftp_egress.service import (
    execute_sftp_egress,
    list_sftp_endpoints,
    register_sftp_endpoint,
)

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1005")


@router.get(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/sftp-endpoints",
    response_model=list[SftpEndpointView],
)
async def handle_list_sftp_endpoints(
    org_slug: str,
    proj_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> list[SftpEndpointView]:
    _ = (org_slug, proj_slug)
    return await list_sftp_endpoints(principal.org_slug)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/sftp-endpoints",
    response_model=SftpEndpointView,
)
async def handle_register_sftp_endpoint(
    org_slug: str,
    proj_slug: str,
    payload: SftpEndpointCreateRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> SftpEndpointView:
    _ = (org_slug, proj_slug)
    return await register_sftp_endpoint(principal.org_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/sftp-releases",
    response_model=SftpReleaseQueued,
)
async def handle_sftp_egress(
    org_slug: str,
    proj_slug: str,
    payload: SftpReleaseRequest | None = None,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
) -> SftpReleaseQueued:
    _ = org_slug
    return await execute_sftp_egress(principal, proj_slug, payload)
