# Module: services/project-catalog/src/project_catalog/schemas.py — Multi-Type Project, Templates & SPDX Gate Schemas
# Invariants: StrictSchema (extra="forbid", frozen=True); Int64Str on 64-bit epochs; 13 canonical subproject domains.
# Spec: architecture/08-identity-enterprise-and-project-model.md, architecture/16-*.md
from __future__ import annotations

from typing import Literal

from services.common.schemas import Int64Str, StrictSchema

SubprojectDomainKind = Literal[
    "PRD",
    "HFS",
    "SCHEMATIC",
    "PCB",
    "IC_ANALOG",
    "IC_DIGITAL",
    "FPGA",
    "FIRMWARE",
    "BIOS",
    "CAD_MCAD",
    "CAM_CNC",
    "PRINT_3D",
    "PLM_OPS",
]
TemplateTierKind = Literal["OFFICIAL", "ENTERPRISE", "COMMUNITY"]
VaultModeKind = Literal["ENCRYPTED_VAULT", "PLAIN_CANONICAL"]


class ProjectCatalogResponse(StrictSchema):
    project_id: str
    org_slug: str
    slug: str
    meta_provider_repo_id: str
    fencing_epoch: Int64Str
    encryption_mode: VaultModeKind = "ENCRYPTED_VAULT"
    enabled_domains: tuple[SubprojectDomainKind, ...] = ("SCHEMATIC", "PCB", "CAD_MCAD", "PLM_OPS")
    spdx_license: str = "LicenseRef-Proprietary-NDA"


class CreateMultiTypeProjectRequest(StrictSchema):
    slug: str
    display_name: str
    encryption_mode: VaultModeKind = "ENCRYPTED_VAULT"
    enabled_domains: tuple[SubprojectDomainKind, ...]
    spdx_license: str = "LicenseRef-Proprietary-NDA"


class ProjectTemplateDescriptor(StrictSchema):
    template_id: str
    title: str
    tier: TemplateTierKind
    spdx_license: str
    default_domains: tuple[SubprojectDomainKind, ...]


class InstantiateTemplateRequest(StrictSchema):
    template_id: str
    new_project_slug: str
    target_encryption_mode: VaultModeKind = "ENCRYPTED_VAULT"
    target_project_spdx: str = "LicenseRef-Proprietary-NDA"


class SpdxCompatibilityRequest(StrictSchema):
    project_vault_mode: VaultModeKind
    project_spdx: str
    imported_dependency_spdx: str


class SpdxCompatibilityResponse(StrictSchema):
    compatible: bool
    contamination_blocked: bool
    reason: str
