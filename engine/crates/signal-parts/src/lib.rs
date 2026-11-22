// Module: engine/crates/signal-parts/src/lib.rs — 3-Tier Universal Component Pinning & Verification
// Invariants: Rejects unpinned revisions in production builds; validates urn:signal:part:* syntax (18-*).
// Spec: architecture/18-universal-component-and-datasheet-repository.md
#![forbid(unsafe_code)]

use signal_proto::v1::VerificationBadge;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PartsError {
    #[error("invalid universal part URN `{0}`")]
    InvalidPartUrn(String),
    #[error("invalid BLAKE3 digest length ({0} chars; expected 64 hex chars)")]
    InvalidBlake3Digest(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockedPartEntry {
    pub part_urn: String,
    pub revision_blake3: String,
    pub badge: VerificationBadge,
}

impl LockedPartEntry {
    pub fn validate(&self) -> Result<(), PartsError> {
        if !self.part_urn.starts_with("urn:signal:part:") {
            return Err(PartsError::InvalidPartUrn(self.part_urn.clone()));
        }
        if self.revision_blake3.len() != 64
            || !self.revision_blake3.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(PartsError::InvalidBlake3Digest(self.revision_blake3.len()));
        }
        Ok(())
    }
}
