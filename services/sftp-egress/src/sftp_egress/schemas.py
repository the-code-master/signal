# Module: services/sftp-egress/src/sftp_egress/schemas.py — SFTP Endpoint, Step-Up 2FA & Quorum Release DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); Int64Str on 64-bit timestamps; SecretStr on keys/codes.
# Spec: architecture/07-security-encryption-dlp-and-sftp.md
from __future__ import annotations

from typing import Literal

from pydantic import Field, SecretStr

from services.common.schemas import Int64Str, StrictSchema

SftpTargetKind = Literal[
    "FAB_GERBER_ODB_IPC2581",
    "REGULATORY_LAB_EMC_PPAP",
    "PARTNER_MCAD_STEP",
]
StepUpModality = Literal["WEBAUTHN_FIDO2", "RFC6238_TOTP"]


class SftpEndpointCreateRequest(StrictSchema):
    display_name: str = Field(min_length=3, max_length=96)
    host: str = Field(min_length=3, max_length=255)
    port: int = Field(default=22, ge=1, le=65535)
    remote_drop_dir: str = Field(min_length=1, max_length=255)
    pinned_ed25519_fingerprint: str = Field(pattern=r"^SHA256:[A-Za-z0-9+/=_-]{16,64}$")
    kms_wrapped_ssh_key: SecretStr
    target_kind: SftpTargetKind


class SftpEndpointView(StrictSchema):
    endpoint_id: str
    org_slug: str
    display_name: str
    host: str
    port: int
    remote_drop_dir: str
    pinned_ed25519_fingerprint: str
    target_kind: SftpTargetKind


class FreshStepUpProof(StrictSchema):
    modality: StepUpModality
    issued_at_epoch_sec: Int64Str
    assertion_or_totp_code: SecretStr
    challenge_nonce: str = Field(min_length=8, max_length=128)


class SftpReleaseRequest(StrictSchema):
    endpoint_id: str = Field(min_length=3, max_length=64)
    bundle_kind: SftpTargetKind
    artifact_slug: str = Field(pattern=r"^[a-zA-Z0-9._-]{3,96}$")
    artifact_sha256: str = Field(pattern=r"^[0-9a-f]{64}$")
    drc_erc_lvs_clean: bool = True
    active_deviation_approved: bool = False
    step_up: FreshStepUpProof
    simulated_audit_quorum_acks: int = Field(default=3, ge=0, le=3)


class SftpReleaseQueued(StrictSchema):
    release_id: str
    status: str
    quorum_audit_committed: bool
    target_kind: SftpTargetKind = "FAB_GERBER_ODB_IPC2581"
    temp_upload_name: str = ""
    final_atomic_name: str = ""
    pinned_host_fingerprint: str = ""
    ml_dsa_65_receipt_sig: str = ""
    audit_merkle_hash: str = ""
