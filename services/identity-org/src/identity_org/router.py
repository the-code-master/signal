# Module: services/identity-org/src/identity_org/router.py — Identity, PKCE, Break-Glass & Part 11 Router
# Invariants: Explicit Depends(require_*) guards on all non-public routes + uniform 404 anti-enumeration (F-084).
# Spec: architecture/08-identity-enterprise-and-project-model.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from fastapi import APIRouter, Depends, Response

from identity_org.schemas import (
    AccessReviewEntry,
    BreakGlassApproveRequest,
    BreakGlassInitiateRequest,
    BreakGlassQuorumRecord,
    ContractorSweepRequest,
    ContractorSweepResponse,
    ElectronicSignatureReceipt,
    ElectronicSignatureRequest,
    PkceCallbackRequest,
    PkceCallbackResponse,
)
from identity_org.service import (
    approve_break_glass,
    attest_electronic_signature,
    complete_dual_bound_pkce_callback,
    execute_identity_org,
    initiate_break_glass,
    sweep_expired_contractors,
)
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_org_admin, require_org_member, require_repo_access, require_step_up_2fa

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.get("/api/v1/orgs/{org_slug}/access-review", response_model=AccessReviewEntry)
async def handle_identity_org(
    org_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> AccessReviewEntry:
    _ = org_slug
    return await execute_identity_org(principal.org_slug)


@router.post("/api/v1/auth/sso/pkce/callback", response_model=PkceCallbackResponse)
async def handle_pkce_callback(
    payload: PkceCallbackRequest,
    response: Response,
    _idem: str = Depends(require_idempotency_key),
) -> PkceCallbackResponse:
    result = await complete_dual_bound_pkce_callback(payload)
    response.set_cookie(
        key=result.session_cookie_name,
        value=result.provider_user_id,
        httponly=True,
        secure=True,
        samesite="strict",
        path="/",
    )
    return result


@router.post("/api/v1/orgs/{org_slug}/break-glass/initiate", response_model=BreakGlassQuorumRecord)
async def handle_break_glass_initiate(
    org_slug: str,
    payload: BreakGlassInitiateRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
    _idem: str = Depends(require_idempotency_key),
) -> BreakGlassQuorumRecord:
    _ = org_slug
    return await initiate_break_glass(principal.org_slug, principal.actor_hmac, payload)


@router.post("/api/v1/orgs/{org_slug}/break-glass/{request_id}/approve", response_model=BreakGlassQuorumRecord)
async def handle_break_glass_approve(
    org_slug: str,
    request_id: str,
    payload: BreakGlassApproveRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
    _idem: str = Depends(require_idempotency_key),
) -> BreakGlassQuorumRecord:
    _ = org_slug
    return await approve_break_glass(principal.org_slug, request_id, payload)


@router.post("/api/v1/orgs/{org_slug}/memberships/sweep-expired", response_model=ContractorSweepResponse)
async def handle_sweep_expired_contractors(
    org_slug: str,
    payload: ContractorSweepRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _idem: str = Depends(require_idempotency_key),
) -> ContractorSweepResponse:
    _ = org_slug
    return await sweep_expired_contractors(principal.org_slug, payload)


@router.post("/api/v1/orgs/{org_slug}/attestations/sign", response_model=ElectronicSignatureReceipt)
async def handle_electronic_signature(
    org_slug: str,
    payload: ElectronicSignatureRequest,
    principal: PrincipalContext = Depends(require_org_member),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
    _idem: str = Depends(require_idempotency_key),
) -> ElectronicSignatureReceipt:
    _ = org_slug
    return await attest_electronic_signature(principal.org_slug, principal.actor_hmac, payload)
