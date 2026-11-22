# Module: services/parts-registry/src/parts_registry/schemas.py — Immutable Pydantic v2 DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); 64-bit string parity (F-144).
# Spec: architecture/10-api-and-data-contracts.md
from __future__ import annotations

from services.common.schemas import StrictSchema


class PartsSearchResponse(StrictSchema):
    part_urn: str
    mpn: str
    verification_tier: str
    avl_status: str
