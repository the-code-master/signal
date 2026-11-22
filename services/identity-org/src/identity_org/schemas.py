# Module: services/identity-org/src/identity_org/schemas.py — Immutable Pydantic v2 DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); 64-bit string parity (F-144).
# Spec: architecture/10-api-and-data-contracts.md
from __future__ import annotations

from services.common.schemas import StrictSchema


class AccessReviewEntry(StrictSchema):
    entries: tuple[str, ...]
    posture: str
