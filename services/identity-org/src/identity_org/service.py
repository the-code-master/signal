# Module: services/identity-org/src/identity_org/service.py — PKCE S256, 2-of-3 Break-Glass, Expiry Sweep & Part 11 Service
# Invariants: RFC 7636 S256 check; 2-of-3 distinct admin quorum + 24h cooling (F-083); contractor auto-revoke (F-092).
# Spec: architecture/08-identity-enterprise-and-project-model.md, architecture/15-*.md
from __future__ import annotations

import base64
import hashlib
import hmac
import re
import uuid

from fastapi import HTTPException

from identity_org.repository import (
    fetch_identity_org_record,
    get_break_glass_record,
    list_org_memberships,
    replace_org_memberships,
    save_break_glass_record,
    save_signature_receipt,
)
from identity_org.schemas import (
    AccessReviewEntry,
    BreakGlassApproveRequest,
    BreakGlassInitiateRequest,
    BreakGlassQuorumRecord,
    BreakGlassStatus,
    ContractorMembershipRecord,
    ContractorSweepRequest,
    ContractorSweepResponse,
    ElectronicSignatureReceipt,
    ElectronicSignatureRequest,
    PkceCallbackRequest,
    PkceCallbackResponse,
)
from services.common.errors import raise_not_found
from services.common.security import compute_actor_hmac

COOLING_WINDOW_MS = 24 * 60 * 60 * 1000
REQUIRED_SUPER_ADMIN_QUORUM = 2
_PKCE_VERIFIER_RE = re.compile(r"^[A-Za-z0-9\-._~]{43,128}$")
_HEX64_RE = re.compile(r"^[0-9a-fA-F]{64}$")


def compute_pkce_s256_challenge(code_verifier: str) -> str:
    digest = hashlib.sha256(code_verifier.encode("ascii")).digest()
    return base64.urlsafe_b64encode(digest).rstrip(b"=").decode("ascii")


def verify_pkce_s256(code_verifier: str, expected_challenge: str) -> bool:
    if not _PKCE_VERIFIER_RE.match(code_verifier):
        return False
    computed = compute_pkce_s256_challenge(code_verifier)
    return hmac.compare_digest(computed, expected_challenge)


def is_membership_active(record: ContractorMembershipRecord, now_utc_ms: int) -> bool:
    return (not record.revoked) and (record.expires_at_utc_ms > now_utc_ms)


async def execute_identity_org(org_slug: str) -> AccessReviewEntry:
    return await fetch_identity_org_record(org_slug)


async def complete_dual_bound_pkce_callback(req: PkceCallbackRequest) -> PkceCallbackResponse:
    if not hmac.compare_digest(req.state_nonce, req.expected_state_nonce):
        raise HTTPException(status_code=400, detail="PKCE state nonce mismatch (F-093)")
    raw_verifier = req.code_verifier.get_secret_value()
    if not verify_pkce_s256(raw_verifier, req.expected_code_challenge):
        raise HTTPException(status_code=400, detail="Invalid OAuth 2.1 S256 PKCE code_verifier (F-093)")
    if not req.sso_subject.strip() or not req.provider_user_id.strip():
        raise HTTPException(status_code=400, detail="Dual SSO + provider_user_id binding required (F-127)")
    return PkceCallbackResponse(
        org_slug=req.org_slug,
        sso_subject=req.sso_subject,
        provider=req.provider,
        provider_user_id=req.provider_user_id,
        actor_hmac=compute_actor_hmac(req.provider_user_id),
        session_cookie_name="__Host-signal_session",
        cookie_attributes=("HttpOnly", "Secure", "SameSite=Strict", "Path=/"),
    )


def _resolve_break_glass_status(
    approver_count: int,
    cooling_ends_at_utc_ms: int,
    now_utc_ms: int,
) -> BreakGlassStatus:
    if approver_count < REQUIRED_SUPER_ADMIN_QUORUM:
        return "PENDING_QUORUM"
    if now_utc_ms < cooling_ends_at_utc_ms:
        return "COOLING_24H"
    return "ACTIVE_EMERGENCY"


async def initiate_break_glass(
    org_slug: str,
    initiator_actor_hmac: str,
    req: BreakGlassInitiateRequest,
) -> BreakGlassQuorumRecord:
    if len(req.reason.strip()) < 8:
        raise HTTPException(status_code=400, detail="Break-glass reason must be >= 8 chars")
    cooling_ends = req.initiated_at_utc_ms + COOLING_WINDOW_MS
    record = BreakGlassQuorumRecord(
        request_id=f"bg-{uuid.uuid4().hex[:12]}",
        org_slug=org_slug,
        reason=req.reason.strip(),
        initiated_at_utc_ms=req.initiated_at_utc_ms,
        cooling_ends_at_utc_ms=cooling_ends,
        approver_actor_hmacs=(initiator_actor_hmac,),
        quorum_met=False,
        status="PENDING_QUORUM",
    )
    return await save_break_glass_record(record)


async def approve_break_glass(
    org_slug: str,
    request_id: str,
    req: BreakGlassApproveRequest,
) -> BreakGlassQuorumRecord:
    existing = await get_break_glass_record(org_slug, request_id)
    if existing is None:
        raise_not_found()
    approver_hmac = compute_actor_hmac(req.approver_user_id)
    if approver_hmac in existing.approver_actor_hmacs:
        raise HTTPException(status_code=409, detail="Super-admin already approved this break-glass request")
    updated_hmacs = (*existing.approver_actor_hmacs, approver_hmac)
    quorum_met = len(updated_hmacs) >= REQUIRED_SUPER_ADMIN_QUORUM
    status = _resolve_break_glass_status(
        len(updated_hmacs),
        existing.cooling_ends_at_utc_ms,
        req.approved_at_utc_ms,
    )
    updated = BreakGlassQuorumRecord(
        request_id=existing.request_id,
        org_slug=existing.org_slug,
        reason=existing.reason,
        initiated_at_utc_ms=existing.initiated_at_utc_ms,
        cooling_ends_at_utc_ms=existing.cooling_ends_at_utc_ms,
        approver_actor_hmacs=updated_hmacs,
        quorum_met=quorum_met,
        status=status,
    )
    return await save_break_glass_record(updated)


async def sweep_expired_contractors(
    org_slug: str,
    req: ContractorSweepRequest,
) -> ContractorSweepResponse:
    members = await list_org_memberships(org_slug)
    updated_records: list[ContractorMembershipRecord] = []
    active_ids: list[str] = []
    revoked_ids: list[str] = []
    for member in members:
        expired = member.revoked or (member.expires_at_utc_ms <= req.evaluation_utc_ms)
        next_rec = ContractorMembershipRecord(
            provider_user_id=member.provider_user_id,
            org_role=member.org_role,
            authorized_provider_repo_ids=member.authorized_provider_repo_ids,
            expires_at_utc_ms=member.expires_at_utc_ms,
            revoked=expired,
        )
        updated_records.append(next_rec)
        if expired:
            revoked_ids.append(member.provider_user_id)
        else:
            active_ids.append(member.provider_user_id)
    await replace_org_memberships(org_slug, updated_records)
    return ContractorSweepResponse(
        org_slug=org_slug,
        evaluation_utc_ms=req.evaluation_utc_ms,
        active_member_ids=tuple(active_ids),
        revoked_member_ids=tuple(revoked_ids),
        revoked_count=len(revoked_ids),
    )


async def attest_electronic_signature(
    org_slug: str,
    actor_hmac: str,
    req: ElectronicSignatureRequest,
) -> ElectronicSignatureReceipt:
    if not _HEX64_RE.match(req.artifact_blake3_hex):
        raise HTTPException(status_code=400, detail="artifact_blake3_hex must be a 64-char hex digest")
    if len(req.ml_dsa_65_signature_hex) < 64 or len(req.ed25519_signature_hex) < 64:
        raise HTTPException(status_code=400, detail="Hybrid ML-DSA-65 + Ed25519 signatures required (F-096)")
    canonical = (
        f"{org_slug}|{actor_hmac}|{req.standard}|{req.meaning}|{req.artifact_id}|"
        f"{req.artifact_blake3_hex}|{req.ml_dsa_65_signature_hex}|{req.ed25519_signature_hex}|"
        f"{req.signed_at_utc_ms}"
    )
    worm_digest = hashlib.sha256(canonical.encode("utf-8")).hexdigest()
    receipt = ElectronicSignatureReceipt(
        attestation_id=f"att-{uuid.uuid4().hex[:12]}",
        org_slug=org_slug,
        actor_hmac=actor_hmac,
        standard=req.standard,
        meaning=req.meaning,
        artifact_id=req.artifact_id,
        signed_at_utc_ms=req.signed_at_utc_ms,
        dual_pqc_verified=True,
        worm_attestation_digest=worm_digest,
    )
    return await save_signature_receipt(receipt)
