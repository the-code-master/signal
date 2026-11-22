# Module: services/project-catalog/src/project_catalog/schemas.py — Immutable Pydantic v2 DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); 64-bit string parity (F-144).
# Spec: architecture/10-api-and-data-contracts.md
from __future__ import annotations

from services.common.schemas import Int64Str, StrictSchema


class ProjectCatalogResponse(StrictSchema):
    project_id: str
    org_slug: str
    slug: str
    meta_provider_repo_id: str
    fencing_epoch: Int64Str
    encryption_mode: str
