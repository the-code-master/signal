# Module: services/parts-registry/src/parts_registry/router.py — 3-Tier Universal Parts, Org AVL & Provenance Router
# Invariants: Explicit Depends(require_*) + Step-Up 2FA on AVL writes + uniform 404 anti-enumeration (18-*, F-084).
# Spec: architecture/18-universal-component-and-datasheet-repository.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from parts_registry.schemas import (
    DistributorProvenanceRequest,
    DistributorProvenanceResponse,
    LockfileDriftRequest,
    LockfileDriftResponse,
    OrgAvlOverlayRecord,
    OrgAvlOverlayUpsertRequest,
    PartsSearchResponse,
    ResolvedUniversalPartResponse,
    ResolveThreeTierPartRequest,
)
from parts_registry.service import (
    execute_parts_registry,
    get_all_org_avl_overlays,
    inspect_parts_lockfile_drift,
    resolve_three_tier_universal_part,
    save_org_avl_overlay,
    verify_distributor_provenance,
)
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_org_admin, require_org_member, require_repo_access, require_step_up_2fa

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.get("/api/v1/orgs/{org_slug}/parts", response_model=PartsSearchResponse)
async def handle_parts_registry(
    org_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> PartsSearchResponse:
    _ = org_slug
    return await execute_parts_registry(principal.org_slug)


@router.get("/api/v1/orgs/{org_slug}/parts/avl-overlays", response_model=tuple[OrgAvlOverlayRecord, ...])
async def handle_list_org_avl_overlays(
    org_slug: str,
    principal: PrincipalContext = Depends(require_org_member),
) -> tuple[OrgAvlOverlayRecord, ...]:
    _ = org_slug
    return await get_all_org_avl_overlays(principal.org_slug)


@router.put("/api/v1/orgs/{org_slug}/parts/avl-overlays", response_model=OrgAvlOverlayRecord)
async def handle_upsert_org_avl_overlay(
    org_slug: str,
    payload: OrgAvlOverlayUpsertRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
    _idem: str = Depends(require_idempotency_key),
) -> OrgAvlOverlayRecord:
    _ = org_slug
    return await save_org_avl_overlay(principal.org_slug, payload)


@router.post("/api/v1/orgs/{org_slug}/parts/resolve", response_model=ResolvedUniversalPartResponse)
async def handle_resolve_universal_part(
    org_slug: str,
    payload: ResolveThreeTierPartRequest,
    principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> ResolvedUniversalPartResponse:
    _ = org_slug
    return await resolve_three_tier_universal_part(principal.org_slug, payload)


@router.post("/api/v1/orgs/{org_slug}/parts/lockfile/check-drift", response_model=LockfileDriftResponse)
async def handle_lockfile_drift(
    org_slug: str,
    payload: LockfileDriftRequest,
    principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> LockfileDriftResponse:
    _ = org_slug
    return await inspect_parts_lockfile_drift(principal.org_slug, payload)


@router.post("/api/v1/orgs/{org_slug}/parts/provenance/verify", response_model=DistributorProvenanceResponse)
async def handle_verify_provenance(
    org_slug: str,
    payload: DistributorProvenanceRequest,
    _principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> DistributorProvenanceResponse:
    _ = org_slug
    return verify_distributor_provenance(payload)
