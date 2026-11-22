// Module: engine/crates/signal-git-sync/src/lib.rs — Padme Padding & Dual-Fenced Super-Commit Coordinator
// Invariants: Blinds ciphertext lengths via Padme (<= 12% overhead, F-117); enforces fencing epoch before push (F-119).
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md
#![forbid(unsafe_code)]

use signal_core as _;
use signal_proto::v1::FencingAuthority;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GitSyncError {
    #[error("stale fencing epoch {provided} < active {active} on Super-Commit")]
    StaleFencingEpoch { provided: u64, active: u64 },
}

pub const fn padme_padded_len(raw_len: u64) -> u64 {
    if raw_len <= 8 {
        return raw_len;
    }
    let e = 63 - raw_len.leading_zeros();
    let s = (32 - e.leading_zeros()) + 1;
    let last_bits = e.saturating_sub(s);
    let bit_mask = (1_u64 << last_bits) - 1;
    (raw_len + bit_mask) & !bit_mask
}

pub fn verify_super_commit_fencing(
    fencing: &FencingAuthority,
    min_epoch: u64,
) -> Result<u64, GitSyncError> {
    if fencing.fencing_epoch < min_epoch {
        return Err(GitSyncError::StaleFencingEpoch {
            provided: fencing.fencing_epoch,
            active: min_epoch,
        });
    }
    Ok(fencing.fencing_epoch)
}

#[cfg(test)]
mod tests {
    use super::padme_padded_len;

    #[test]
    fn padme_never_shrinks_and_caps_overhead() {
        for len in [1_u64, 8, 9, 65, 1025, 1_048_577] {
            let padded = padme_padded_len(len);
            assert!(padded >= len);
            if len >= 64 {
                assert!(padded * 100 <= len * 113);
            }
        }
    }
}
