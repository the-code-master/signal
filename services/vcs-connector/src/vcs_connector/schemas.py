# Module: services/vcs-connector/src/vcs_connector/schemas.py — Immutable Pydantic v2 DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); 64-bit string parity (F-144).
# Spec: architecture/10-api-and-data-contracts.md
from __future__ import annotations

from services.common.schemas import StrictSchema


class VcsConnectionStatus(StrictSchema):
    provider: str
    org_slug: str
    vault_mode: str
