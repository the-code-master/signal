// Module: engine/crates/signal-core/src/secret_box.rs — Zeroize-on-Drop Redacted Secret Wrapper
// Invariants: Redacts Debug and Display (`[REDACTED SECRET]`, F-088); zeroizes backing memory on drop.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/15-application-security-and-project-isolation.md
use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Constant-time slice equality avoiding timing side-channels (`F-088`).
pub fn ct_eq_bytes(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut diff = 0_u8;
    for (&a, &b) in lhs.iter().zip(rhs.iter()) {
        diff |= a ^ b;
    }
    std::hint::black_box(diff) == 0
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

    pub fn expose_secret_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Zeroize + Clone> Clone for SecretBox<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Zeroize + AsRef<[u8]>> SecretBox<T> {
    pub fn ct_eq(&self, other: &Self) -> bool {
        ct_eq_bytes(self.inner.as_ref(), other.inner.as_ref())
    }
}

impl<T: Zeroize> fmt::Debug for SecretBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretBox([REDACTED SECRET])")
    }
}

impl<T: Zeroize> fmt::Display for SecretBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED SECRET]")
    }
}

#[cfg(test)]
mod tests {
    use super::{SecretBox, ct_eq_bytes};
    use zeroize::Zeroize;

    #[test]
    fn redacts_debug_and_display_and_compares_constant_time() {
        let a = SecretBox::new([0x42_u8; 32]);
        let b = SecretBox::new([0x42_u8; 32]);
        let c = SecretBox::new([0x43_u8; 32]);
        assert_eq!(format!("{a:?}"), "SecretBox([REDACTED SECRET])");
        assert_eq!(format!("{a}"), "[REDACTED SECRET]");
        assert!(a.ct_eq(&b));
        assert!(!a.ct_eq(&c));
        assert!(!ct_eq_bytes(&[1, 2], &[1, 2, 3]));
    }

    #[test]
    fn zeroizes_secret_buffer_in_place() {
        let mut raw = [0xAA_u8; 16];
        raw.zeroize();
        assert_eq!(raw, [0_u8; 16]);
    }
}
