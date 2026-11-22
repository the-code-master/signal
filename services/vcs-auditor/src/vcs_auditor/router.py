# Module: services/vcs-auditor/src/vcs_auditor/router.py — BYO-Git Security Posture Router
# Invariants: Explicit Depends(require_repo_access(...)) + Idempotency-Key on mutations + uniform 404 (F-084).
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_org_admin, require_repo_access
from vcs_auditor.schemas import VcsPostureEvaluateRequest, VcsPostureReport
from vcs_auditor.service import evaluate_and_store_vcs_posture, execute_vcs_auditor

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.get("/api/v1/orgs/{org_slug}/vcs/posture", response_model=VcsPostureReport)
async def handle_vcs_auditor(
    org_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> VcsPostureReport:
    _ = org_slug
    return await execute_vcs_auditor(principal.org_slug)


@router.post("/api/v1/orgs/{org_slug}/vcs/posture/evaluate", response_model=VcsPostureReport)
async def handle_evaluate_vcs_posture(
    org_slug: str,
    payload: VcsPostureEvaluateRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _idem: str = Depends(require_idempotency_key),
) -> VcsPostureReport:
    _ = org_slug
    return await evaluate_and_store_vcs_posture(principal.org_slug, payload)
