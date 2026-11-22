# Module: services/project-catalog/src/project_catalog/router.py — Multi-Type Project Catalog, Templates & SPDX Router
# Invariants: Explicit Depends(require_*) + Idempotency-Key on mutations + uniform 404 anti-enumeration (F-084).
# Spec: architecture/08-identity-enterprise-and-project-model.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from project_catalog.schemas import (
    CreateMultiTypeProjectRequest,
    InstantiateTemplateRequest,
    ProjectCatalogResponse,
    ProjectTemplateDescriptor,
    SpdxCompatibilityRequest,
    SpdxCompatibilityResponse,
)
from project_catalog.service import (
    create_multitype_project,
    evaluate_spdx_compatibility,
    execute_project_catalog,
    fetch_single_project,
    instantiate_project_from_template,
    list_project_templates,
)
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_org_admin, require_org_member, require_repo_access

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1001")


@router.get("/api/v1/orgs/{org_slug}/projects", response_model=ProjectCatalogResponse)
async def handle_project_catalog(
    org_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> ProjectCatalogResponse:
    _ = org_slug
    return await execute_project_catalog(principal.org_slug)


@router.post("/api/v1/orgs/{org_slug}/projects", response_model=ProjectCatalogResponse)
async def handle_create_multitype_project(
    org_slug: str,
    payload: CreateMultiTypeProjectRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _idem: str = Depends(require_idempotency_key),
) -> ProjectCatalogResponse:
    _ = org_slug
    return await create_multitype_project(principal.org_slug, payload)


@router.get("/api/v1/orgs/{org_slug}/project-templates", response_model=tuple[ProjectTemplateDescriptor, ...])
async def handle_list_templates(
    org_slug: str,
    _principal: PrincipalContext = Depends(require_org_member),
) -> tuple[ProjectTemplateDescriptor, ...]:
    _ = org_slug
    return await list_project_templates()


@router.post("/api/v1/orgs/{org_slug}/projects/from-template", response_model=ProjectCatalogResponse)
async def handle_instantiate_template(
    org_slug: str,
    payload: InstantiateTemplateRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _idem: str = Depends(require_idempotency_key),
) -> ProjectCatalogResponse:
    _ = org_slug
    return await instantiate_project_from_template(principal.org_slug, payload)


@router.post("/api/v1/orgs/{org_slug}/projects/spdx-check", response_model=SpdxCompatibilityResponse)
async def handle_spdx_check(
    org_slug: str,
    payload: SpdxCompatibilityRequest,
    _principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> SpdxCompatibilityResponse:
    _ = org_slug
    return evaluate_spdx_compatibility(payload)


@router.get("/api/v1/orgs/{org_slug}/projects/{proj_slug}", response_model=ProjectCatalogResponse)
async def handle_get_project(
    org_slug: str,
    proj_slug: str,
    principal: PrincipalContext = Depends(require_org_member),
) -> ProjectCatalogResponse:
    _ = org_slug
    return await fetch_single_project(principal.org_slug, proj_slug)
