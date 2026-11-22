# Module: services/plm-ops/src/plm_ops/schemas.py — Immutable Pydantic v2 DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); 64-bit string parity (F-144).
# Spec: architecture/10-api-and-data-contracts.md
from __future__ import annotations

from services.common.schemas import StrictSchema


class PlmChangeNoticeSummary(StrictSchema):
    change_id: str
    project_slug: str
    kind: str
    status: str
    ctb_readiness_bps: int
