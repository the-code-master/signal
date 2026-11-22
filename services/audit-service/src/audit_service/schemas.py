# Module: services/audit-service/src/audit_service/schemas.py — Pseudonymous Tier-0 Audit Schemas
# Invariants: Rejects raw email/@ PII in actor_hmac so WORM Merkle chains stay GDPR-erasure-safe (F-100).
# Spec: architecture/07-security-encryption-dlp-and-sftp.md
from __future__ import annotations

from pydantic import field_validator

from services.common.schemas import Int64Str, StrictSchema


class Tier0AuditEvent(StrictSchema):
    org_id: str
    actor_hmac: str
    action_kind: str
    target_resource_id: str
    occurred_at_ms: Int64Str

    @field_validator("actor_hmac")
    @classmethod
    def _forbid_raw_pii(cls, v: str) -> str:
        if "@" in v or not v.startswith("actor_hmac_"):
            raise ValueError("GDPR violation (F-100): WORM audit events require pseudonymous actor_hmac_*")
        return v


class QuorumCommitReceipt(StrictSchema):
    outbox_seq: Int64Str
    quorum_acks: int
    entry_merkle_hash: str
