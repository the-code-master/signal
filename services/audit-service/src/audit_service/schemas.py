# Module: services/audit-service/src/audit_service/schemas.py — Pseudonymous Tier-0 Audit & Compliance Schemas
# Invariants: Rejects raw email/@ PII in actor_hmac so WORM Merkle chains stay GDPR-erasure-safe (F-100).
# Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

from typing import Literal

from pydantic import Field, field_validator

from services.common.schemas import Int64Str, StrictSchema

ComplianceFrameworkLiteral = Literal[
    "SOC2_TYPE_II",
    "ISO_27001",
    "ITAR_EAR_SOVEREIGN",
    "GDPR_ARTICLE_17",
    "FDA_21_CFR_PART_11",
]


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


class GdprSaltShredRequest(StrictSchema):
    subject_provider_user_id: str = Field(min_length=3, max_length=128)
    erasure_ticket_ref: str = Field(min_length=4, max_length=128)


class GdprSaltShredReceipt(StrictSchema):
    subject_erased: bool
    erasure_ticket_ref: str
    worm_outbox_seq: Int64Str
    worm_tip_hash_before: str
    worm_tip_hash_after: str
    worm_chain_unmutated: bool
    actor_hmac_unlinkable: bool


class ComplianceEvidenceItem(StrictSchema):
    framework: ComplianceFrameworkLiteral
    passing: bool
    control_summary: str
    evidence_sha256: str
    verified_at_utc_ms: Int64Str


class EnterpriseComplianceAttestationView(StrictSchema):
    org_slug: str
    total_api_routes_verified: int
    all_routes_auth_guarded: bool
    zero_dlp_export_download_routes: bool
    quorum_outbox_3x_healthy: bool
    verified_transcript_files: int
    verified_transcript_steps: int
    frameworks: tuple[ComplianceEvidenceItem, ...]
    ml_dsa_65_attestation_signature: str
