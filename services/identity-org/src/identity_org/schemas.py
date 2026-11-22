# Module: services/identity-org/src/identity_org/schemas.py — SSO+PKCE, Break-Glass, Contractor & Part 11 Schemas
# Invariants: StrictSchema (extra="forbid", frozen=True); Int64Str on 64-bit timestamps; SecretStr on PKCE verifiers.
# Spec: architecture/08-identity-enterprise-and-project-model.md, architecture/15-*.md
from __future__ import annotations

from typing import Literal

from pydantic import SecretStr

from services.common.schemas import Int64Str, StrictSchema

GitAuthProvider = Literal["GITHUB", "GITLAB", "BITBUCKET"]
BreakGlassStatus = Literal["PENDING_QUORUM", "COOLING_24H", "ACTIVE_EMERGENCY"]
ComplianceStandard = Literal["CFR_PART_11", "ISO_26262", "DO_254"]
AttestationMeaning = Literal["REVIEWED", "APPROVED", "RELEASED"]


class AccessReviewEntry(StrictSchema):
    entries: tuple[str, ...]
    posture: str


class PkceCallbackRequest(StrictSchema):
    org_slug: str
    sso_subject: str
    provider: GitAuthProvider
    provider_user_id: str
    code_verifier: SecretStr
    expected_code_challenge: str
    state_nonce: str
    expected_state_nonce: str


class PkceCallbackResponse(StrictSchema):
    org_slug: str
    sso_subject: str
    provider: GitAuthProvider
    provider_user_id: str
    actor_hmac: str
    session_cookie_name: str
    cookie_attributes: tuple[str, ...]


class BreakGlassInitiateRequest(StrictSchema):
    reason: str
    initiated_at_utc_ms: Int64Str


class BreakGlassApproveRequest(StrictSchema):
    approver_user_id: str
    approved_at_utc_ms: Int64Str


class BreakGlassQuorumRecord(StrictSchema):
    request_id: str
    org_slug: str
    reason: str
    initiated_at_utc_ms: Int64Str
    cooling_ends_at_utc_ms: Int64Str
    approver_actor_hmacs: tuple[str, ...]
    quorum_met: bool
    status: BreakGlassStatus


class ContractorMembershipRecord(StrictSchema):
    provider_user_id: str
    org_role: str
    authorized_provider_repo_ids: tuple[str, ...]
    expires_at_utc_ms: Int64Str
    revoked: bool = False


class ContractorSweepRequest(StrictSchema):
    evaluation_utc_ms: Int64Str


class ContractorSweepResponse(StrictSchema):
    org_slug: str
    evaluation_utc_ms: Int64Str
    active_member_ids: tuple[str, ...]
    revoked_member_ids: tuple[str, ...]
    revoked_count: int


class ElectronicSignatureRequest(StrictSchema):
    standard: ComplianceStandard
    meaning: AttestationMeaning
    artifact_id: str
    artifact_blake3_hex: str
    ml_dsa_65_signature_hex: str
    ed25519_signature_hex: str
    signed_at_utc_ms: Int64Str


class ElectronicSignatureReceipt(StrictSchema):
    attestation_id: str
    org_slug: str
    actor_hmac: str
    standard: ComplianceStandard
    meaning: AttestationMeaning
    artifact_id: str
    signed_at_utc_ms: Int64Str
    dual_pqc_verified: bool
    worm_attestation_digest: str
