# Module: services/project-catalog/src/project_catalog/service.py — Multi-Type Project CRUD & SPDX Contamination Gate
# Invariants: Blocks strong copyleft (CERN-OHL-S-2.0, GPL-3.0-only, AGPL-3.0-only) on proprietary ENCRYPTED_VAULTs (F-099).
# Spec: architecture/08-identity-enterprise-and-project-model.md, architecture/16-*.md
from __future__ import annotations

import uuid

from fastapi import HTTPException

from project_catalog.repository import (
    fetch_project_catalog_record,
    get_project_by_slug,
    get_template_by_id,
    list_templates,
    save_project_record,
)
from project_catalog.schemas import (
    CreateMultiTypeProjectRequest,
    InstantiateTemplateRequest,
    ProjectCatalogResponse,
    ProjectTemplateDescriptor,
    SpdxCompatibilityRequest,
    SpdxCompatibilityResponse,
)
from services.common.errors import raise_not_found

STRONG_COPYLEFT_SPDX: frozenset[str] = frozenset(
    {"CERN-OHL-S-2.0", "GPL-3.0-only", "GPL-3.0-or-later", "AGPL-3.0-only"}
)
PERMISSIVE_OR_WEAK_SPDX: frozenset[str] = frozenset(
    {"Apache-2.0", "MIT", "BSD-3-Clause", "CERN-OHL-P-2.0", "CERN-OHL-W-2.0", "LicenseRef-Proprietary-NDA"}
)


def evaluate_spdx_compatibility(req: SpdxCompatibilityRequest) -> SpdxCompatibilityResponse:
    dep = req.imported_dependency_spdx.strip()
    proj = req.project_spdx.strip()
    is_proprietary = req.project_vault_mode == "ENCRYPTED_VAULT" or proj.startswith("LicenseRef-Proprietary")
    if is_proprietary and dep in STRONG_COPYLEFT_SPDX:
        return SpdxCompatibilityResponse(
            compatible=False,
            contamination_blocked=True,
            reason=(
                f"SPDX gate blocked reciprocal copyleft `{dep}` from contaminating proprietary "
                f"`{req.project_vault_mode}` project (F-099)"
            ),
        )
    if dep not in STRONG_COPYLEFT_SPDX and dep not in PERMISSIVE_OR_WEAK_SPDX:
        return SpdxCompatibilityResponse(
            compatible=False,
            contamination_blocked=True,
            reason=f"Unverified SPDX identifier `{dep}` requires legal review before import (F-099)",
        )
    return SpdxCompatibilityResponse(
        compatible=True,
        contamination_blocked=False,
        reason=f"SPDX `{dep}` is compatible with `{proj}` ({req.project_vault_mode})",
    )


async def execute_project_catalog(org_slug: str) -> ProjectCatalogResponse:
    return await fetch_project_catalog_record(org_slug)


async def fetch_single_project(org_slug: str, proj_slug: str) -> ProjectCatalogResponse:
    found = await get_project_by_slug(org_slug, proj_slug)
    if found is None:
        raise_not_found()
    return found


async def create_multitype_project(
    org_slug: str,
    req: CreateMultiTypeProjectRequest,
) -> ProjectCatalogResponse:
    if not req.enabled_domains:
        raise HTTPException(status_code=400, detail="At least one SubprojectDomain is required")
    gate = evaluate_spdx_compatibility(
        SpdxCompatibilityRequest(
            project_vault_mode=req.encryption_mode,
            project_spdx="LicenseRef-Proprietary-NDA"
            if req.encryption_mode == "ENCRYPTED_VAULT" and req.spdx_license in STRONG_COPYLEFT_SPDX
            else req.spdx_license,
            imported_dependency_spdx=req.spdx_license,
        )
    )
    if not gate.compatible:
        raise HTTPException(status_code=409, detail=gate.reason)
    record = ProjectCatalogResponse(
        project_id=str(uuid.uuid4()),
        org_slug=org_slug,
        slug=req.slug.strip().lower(),
        meta_provider_repo_id=f"gh_meta_{req.slug.strip().lower()}",
        fencing_epoch=1,
        encryption_mode=req.encryption_mode,
        enabled_domains=req.enabled_domains,
        spdx_license=req.spdx_license,
    )
    return await save_project_record(record)


async def list_project_templates() -> tuple[ProjectTemplateDescriptor, ...]:
    return await list_templates()


async def instantiate_project_from_template(
    org_slug: str,
    req: InstantiateTemplateRequest,
) -> ProjectCatalogResponse:
    tpl = await get_template_by_id(req.template_id)
    if tpl is None:
        raise_not_found()
    gate = evaluate_spdx_compatibility(
        SpdxCompatibilityRequest(
            project_vault_mode=req.target_encryption_mode,
            project_spdx=req.target_project_spdx,
            imported_dependency_spdx=tpl.spdx_license,
        )
    )
    if not gate.compatible:
        raise HTTPException(status_code=409, detail=gate.reason)
    record = ProjectCatalogResponse(
        project_id=str(uuid.uuid4()),
        org_slug=org_slug,
        slug=req.new_project_slug.strip().lower(),
        meta_provider_repo_id=f"gh_meta_{req.new_project_slug.strip().lower()}",
        fencing_epoch=1,
        encryption_mode=req.target_encryption_mode,
        enabled_domains=tpl.default_domains,
        spdx_license=req.target_project_spdx,
    )
    return await save_project_record(record)
