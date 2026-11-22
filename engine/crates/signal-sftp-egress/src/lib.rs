// Module: engine/crates/signal-sftp-egress/src/lib.rs — Write-Only SFTP Pre-Flight Gate
// Invariants: Fail-closed if Tier-0 3x Quorum Audit seq == 0, Step-Up 2FA approval missing, or host key unpinned (F-080, F-147).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md
#![forbid(unsafe_code)]

use signal_proto::v1::PushSftpReleaseRequest;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SftpGateError {
    #[error("Step-Up 2FA signoff approval missing")]
    MissingStepUpApproval,
    #[error("Tier-0 3x Quorum Audit outbox sequence not committed (fail-closed F-147)")]
    MissingQuorumAuditCommit,
    #[error("pinned Ed25519 SFTP host key missing")]
    MissingPinnedHostKey,
}

pub fn verify_sftp_release_preflight(req: &PushSftpReleaseRequest) -> Result<(), SftpGateError> {
    if req.approvals.is_empty()
        || req
            .approvals
            .iter()
            .any(|a| a.burned_totp_or_webauthn_nonce.trim().is_empty())
    {
        return Err(SftpGateError::MissingStepUpApproval);
    }
    if req.quorum_audit_outbox_seq == 0 {
        return Err(SftpGateError::MissingQuorumAuditCommit);
    }
    if req.pinned_host_key_ed25519.trim().is_empty() {
        return Err(SftpGateError::MissingPinnedHostKey);
    }
    Ok(())
}
