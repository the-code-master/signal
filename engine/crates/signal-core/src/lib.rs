// Module: engine/crates/signal-core/src/lib.rs — SI Units, SecretBox<T> & Release Dev-Auth Hard Gate
// Invariants: Redacts SecretBox Debug/Display (F-088); disambiguates m (1e-3) vs M/Meg (1e6) (F-075); blocks release dev-guest-auth (F-095).
// Spec: architecture/03-rust-eda-engine.md, architecture/15-application-security-and-project-isolation.md
#![forbid(unsafe_code)]

#[cfg(all(feature = "dev-guest-auth", not(debug_assertions)))]
compile_error!(
    "FATAL SECURITY VIOLATION (F-095): `dev-guest-auth` is strictly forbidden in release builds!"
);

use std::fmt;
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("unrecognized or ambiguous SI prefix `{0}` (use `m` for milli or `M`/`Meg` for Mega)")]
    InvalidSiPrefix(String),
    #[error("64-bit SI value overflow during scaling")]
    SiOverflow,
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecretBox<T: Zeroize> {
    inner: T,
}

impl<T: Zeroize> SecretBox<T> {
    pub const fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn expose_secret(&self) -> &T {
        &self.inner
    }
}

impl<T: Zeroize> fmt::Debug for SecretBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretBox([REDACTED])")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nanometers(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Picoseconds(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Microvolts(pub i64);

pub fn si_prefix_exponent_milli_steps(prefix: &str) -> Result<i32, CoreError> {
    match prefix {
        "f" => Ok(-15),
        "p" => Ok(-12),
        "n" => Ok(-9),
        "u" | "µ" => Ok(-6),
        "m" => Ok(-3),
        "" => Ok(0),
        "k" => Ok(3),
        "M" | "Meg" | "MEG" => Ok(6),
        "G" => Ok(9),
        "T" => Ok(12),
        other => Err(CoreError::InvalidSiPrefix(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::{CoreError, SecretBox, si_prefix_exponent_milli_steps};

    #[test]
    fn secret_box_never_leaks_plaintext_in_debug() {
        let dek = SecretBox::new(vec![0xAA_u8, 0xBB, 0xCC]);
        assert_eq!(format!("{dek:?}"), "SecretBox([REDACTED])");
        assert_eq!(dek.expose_secret().len(), 3);
    }

    #[test]
    fn disambiguates_milli_vs_mega_and_rejects_ambiguous_meg_case() {
        assert_eq!(si_prefix_exponent_milli_steps("m"), Ok(-3));
        assert_eq!(si_prefix_exponent_milli_steps("M"), Ok(6));
        assert_eq!(si_prefix_exponent_milli_steps("Meg"), Ok(6));
        assert!(matches!(
            si_prefix_exponent_milli_steps("meg"),
            Err(CoreError::InvalidSiPrefix(_))
        ));
    }
}
