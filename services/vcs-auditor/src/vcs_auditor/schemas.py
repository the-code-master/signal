# Module: services/vcs-auditor/src/vcs_auditor/schemas.py — Immutable Pydantic v2 DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); 64-bit string parity (F-144).
# Spec: architecture/10-api-and-data-contracts.md
from __future__ import annotations

from services.common.schemas import StrictSchema


class VcsPostureReport(StrictSchema):
    org_slug: str
    score: int
    padme_vault_enforced: bool
