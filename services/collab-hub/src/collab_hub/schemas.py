# Module: services/collab-hub/src/collab_hub/schemas.py — Immutable Pydantic v2 DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); 64-bit string parity (F-144).
# Spec: architecture/10-api-and-data-contracts.md
from __future__ import annotations

from services.common.schemas import Int64Str, StrictSchema


class AutomatedReviewSummary(StrictSchema):
    review_id: str
    cadence: str
    overall_rag: str
    stage_gate: str
    generated_at_ms: Int64Str
