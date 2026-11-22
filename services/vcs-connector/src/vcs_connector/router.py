# Module: services/vcs-connector/src/vcs_connector/router.py — BYO-Git Onboarding, Lazy Sub-Repo Pacer & Roster Sync Router
# Invariants: Explicit Depends(require_*) + Step-Up 2FA on VCS credential writes + Idempotency-Key + uniform 404.
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_org_admin, require_repo_access, require_step_up_2fa
from vcs_connector.schemas import (
    BootstrapMetaRepoRequest,
    BoundSubprojectRepo,
    LazyMaterializeSubrepoRequest,
    LazyMaterializeSubrepoResponse,
    RosterSyncRequest,
    RosterSyncResponse,
    VcsConnectionStatus,
    VcsOnboardingRequest,
)
from vcs_connector.service import (
    bootstrap_project_meta_repo,
    execute_vcs_connector,
    materialize_lazy_subrepo,
    sync_repo_roster_conditional_etag,
)

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.put("/api/v1/orgs/{org_slug}/vcs/connection", response_model=VcsConnectionStatus)
async def handle_vcs_connector(
    org_slug: str,
    payload: VcsOnboardingRequest | None = None,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
) -> VcsConnectionStatus:
    _ = org_slug
    return await execute_vcs_connector(principal.org_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/vcs/projects/{proj_slug}/bootstrap-meta",
    response_model=BoundSubprojectRepo,
)
async def handle_bootstrap_meta_repo(
    org_slug: str,
    proj_slug: str,
    payload: BootstrapMetaRepoRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _idem: str = Depends(require_idempotency_key),
) -> BoundSubprojectRepo:
    _ = org_slug
    return await bootstrap_project_meta_repo(principal.org_slug, proj_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/vcs/projects/{proj_slug}/subrepos/materialize",
    response_model=LazyMaterializeSubrepoResponse,
)
async def handle_lazy_materialize_subrepo(
    org_slug: str,
    proj_slug: str,
    payload: LazyMaterializeSubrepoRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> LazyMaterializeSubrepoResponse:
    _ = org_slug
    return await materialize_lazy_subrepo(principal.org_slug, proj_slug, payload)


@router.post("/api/v1/orgs/{org_slug}/vcs/roster/sync", response_model=RosterSyncResponse)
async def handle_roster_sync(
    org_slug: str,
    payload: RosterSyncRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _idem: str = Depends(require_idempotency_key),
) -> RosterSyncResponse:
    _ = org_slug
    return await sync_repo_roster_conditional_etag(principal.org_slug, payload)
