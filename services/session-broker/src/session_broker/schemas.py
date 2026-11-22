# Module: services/session-broker/src/session_broker/schemas.py — Session Lease DTOs
# Invariants: Serializes active_fencing_epoch as Int64Str (>2^53-1 safe, F-144).
# Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/10-api-and-data-contracts.md
from __future__ import annotations

from services.common.schemas import Int64Str, StrictSchema


class SessionLeaseResponse(StrictSchema):
    cell_id: str
    runtime_tier: str
    active_fencing_epoch: Int64Str
    engine_version: str
    negotiated_codec: str
