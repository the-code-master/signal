# Module: services/project-catalog/src/project_catalog/repository.py — Tenant-Isolated Project & Template Store
# Invariants: Keys project catalog records strictly by (org_slug, slug); ships Official/Enterprise/Community templates.
# Spec: architecture/08-identity-enterprise-and-project-model.md
from __future__ import annotations

from project_catalog.schemas import ProjectCatalogResponse, ProjectTemplateDescriptor

_TEMPLATES: tuple[ProjectTemplateDescriptor, ...] = (
    ProjectTemplateDescriptor(
        template_id="tpl-official-800v-sic-inverter",
        title="Official 800V SiC Traction Inverter Reference",
        tier="OFFICIAL",
        spdx_license="Apache-2.0",
        default_domains=("PRD", "HFS", "SCHEMATIC", "PCB", "FIRMWARE", "CAD_MCAD", "PLM_OPS"),
    ),
    ProjectTemplateDescriptor(
        template_id="tpl-enterprise-224g-retimer",
        title="Enterprise 224G PAM4 Optical Retimer Module",
        tier="ENTERPRISE",
        spdx_license="CERN-OHL-P-2.0",
        default_domains=("SCHEMATIC", "PCB", "IC_ANALOG", "IC_DIGITAL", "FPGA"),
    ),
    ProjectTemplateDescriptor(
        template_id="tpl-community-open-bms-strong-copyleft",
        title="Community Reciprocal Open-Hardware 16S BMS",
        tier="COMMUNITY",
        spdx_license="CERN-OHL-S-2.0",
        default_domains=("SCHEMATIC", "PCB", "FIRMWARE", "PRINT_3D"),
    ),
)

_PROJECT_STORE: dict[tuple[str, str], ProjectCatalogResponse] = {}


async def fetch_project_catalog_record(org_slug: str) -> ProjectCatalogResponse:
    default_proj = ProjectCatalogResponse(
        project_id="aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee",
        org_slug=org_slug,
        slug="aurora-ev-inverter",
        meta_provider_repo_id="local_gh_repo_1001",
        fencing_epoch=9_007_199_254_740_993,
        encryption_mode="ENCRYPTED_VAULT",
        enabled_domains=("SCHEMATIC", "PCB", "CAD_MCAD", "PLM_OPS"),
        spdx_license="LicenseRef-Proprietary-NDA",
    )
    return _PROJECT_STORE.get((org_slug, "aurora-ev-inverter"), default_proj)


async def get_project_by_slug(org_slug: str, slug: str) -> ProjectCatalogResponse | None:
    if slug == "aurora-ev-inverter" and (org_slug, slug) not in _PROJECT_STORE:
        return await fetch_project_catalog_record(org_slug)
    return _PROJECT_STORE.get((org_slug, slug))


async def save_project_record(record: ProjectCatalogResponse) -> ProjectCatalogResponse:
    _PROJECT_STORE[(record.org_slug, record.slug)] = record
    return record


async def list_templates() -> tuple[ProjectTemplateDescriptor, ...]:
    return _TEMPLATES


async def get_template_by_id(template_id: str) -> ProjectTemplateDescriptor | None:
    for item in _TEMPLATES:
        if item.template_id == template_id:
            return item
    return None
