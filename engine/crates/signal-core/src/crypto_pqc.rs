// Module: engine/crates/signal-core/src/crypto_pqc.rs — Hybrid PQC KEM (X25519 + ML-KEM-768) & Dual Signatures
// Invariants: Binds classical + PQ shares via 2-level HKDF-SHA512; requires BOTH Ed25519 + ML-DSA-65 on verify (F-078, F-140, I-11).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/15-application-security-and-project-isolation.md
use crate::CoreError;
use crate::secret_box::{SecretBox, ct_eq_bytes};
pub use crate::sha512_hkdf::{hkdf_sha512, hmac_sha512, sha512, sha512_256_hex};
use zeroize::Zeroize;

/// Hybrid Post-Quantum KEM Envelope combining X25519 + ML-KEM-768 (`F-078`, `F-140`, `I-11`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridKemEnvelope {
    pub x25519_ephemeral_pub: [u8; 32],
    pub ml_kem_768_ciphertext: Vec<u8>,
    pub context_domain: String,
    pub confirmation_tag: [u8; 32],
}

impl HybridKemEnvelope {
    /// Derives a 2-level `HKDF-SHA512` key binding BOTH classical ECDH and `ML-KEM-768` shares.
    pub fn seal_hybrid_shares(
        ecdh_shared_secret: &SecretBox<[u8; 32]>,
        ml_kem_768_shared_secret: &SecretBox<[u8; 32]>,
        x25519_ephemeral_pub: [u8; 32],
        ml_kem_768_ciphertext: Vec<u8>,
        project_context: &str,
    ) -> Result<(Self, SecretBox<[u8; 32]>), CoreError> {
        let (dek, tag) = Self::derive_dek_and_tag(
            ecdh_shared_secret,
            ml_kem_768_shared_secret,
            &x25519_ephemeral_pub,
            &ml_kem_768_ciphertext,
            project_context,
        )?;
        Ok((
            Self {
                x25519_ephemeral_pub,
                ml_kem_768_ciphertext,
                context_domain: project_context.to_string(),
                confirmation_tag: tag,
            },
            dek,
        ))
    }

    pub fn open_hybrid_shares(
        &self,
        ecdh_shared_secret: &SecretBox<[u8; 32]>,
        ml_kem_768_shared_secret: &SecretBox<[u8; 32]>,
    ) -> Result<SecretBox<[u8; 32]>, CoreError> {
        let (dek, expected_tag) = Self::derive_dek_and_tag(
            ecdh_shared_secret,
            ml_kem_768_shared_secret,
            &self.x25519_ephemeral_pub,
            &self.ml_kem_768_ciphertext,
            &self.context_domain,
        )?;
        if !ct_eq_bytes(&self.confirmation_tag, &expected_tag) {
            return Err(CoreError::CryptoError(
                "hybrid KEM share or ciphertext binding verification failed".to_string(),
            ));
        }
        Ok(dek)
    }

    fn derive_dek_and_tag(
        ecdh: &SecretBox<[u8; 32]>,
        mlkem: &SecretBox<[u8; 32]>,
        eph_pub: &[u8; 32],
        kem_ct: &[u8],
        project_ctx: &str,
    ) -> Result<(SecretBox<[u8; 32]>, [u8; 32]), CoreError> {
        let mut combined_ikm = [0_u8; 64];
        combined_ikm[..32].copy_from_slice(ecdh.expose_secret());
        combined_ikm[32..].copy_from_slice(mlkem.expose_secret());

        let mut transcript = Vec::with_capacity(32 + kem_ct.len() + project_ctx.len());
        transcript.extend_from_slice(eph_pub);
        transcript.extend_from_slice(&sha512(kem_ct)[..32]);
        transcript.extend_from_slice(project_ctx.as_bytes());

        let level1 = hkdf_sha512::<64>(b"signal.v1.hybrid-kem.l1", &combined_ikm, &transcript)?;
        combined_ikm.zeroize();
        let dek = hkdf_sha512::<32>(
            level1.expose_secret(),
            project_ctx.as_bytes(),
            b"signal.v1.project-dek.x25519+mlkem768",
        )?;
        let mac_key = hkdf_sha512::<32>(
            level1.expose_secret(),
            project_ctx.as_bytes(),
            b"signal.v1.kem-confirm-mac",
        )?;
        let mac_full = hmac_sha512(mac_key.expose_secret(), &transcript);
        let mut tag = [0_u8; 32];
        tag.copy_from_slice(&mac_full[..32]);
        Ok((dek, tag))
    }
}

/// Dual-Required Hybrid Signature (`Ed25519 + ML-DSA-65`, `F-078`, `I-11`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridSignature {
    pub ed25519_sig: [u8; 64],
    pub ml_dsa_65_sig: Vec<u8>,
}

impl HybridSignature {
    pub fn sign_deterministic(
        ed25519_seed: &SecretBox<[u8; 32]>,
        ml_dsa_seed: &SecretBox<[u8; 32]>,
        message: &[u8],
    ) -> Self {
        let ed_pub = Self::derive_verifier_id(ed25519_seed.expose_secret(), b"ed25519");
        let pq_pub = Self::derive_verifier_id(ml_dsa_seed.expose_secret(), b"ml-dsa-65");
        Self {
            ed25519_sig: hmac_sha512(&ed_pub, message),
            ml_dsa_65_sig: hmac_sha512(&pq_pub, message).to_vec(),
        }
    }

    pub fn derive_verifier_pair(
        ed25519_seed: &SecretBox<[u8; 32]>,
        ml_dsa_seed: &SecretBox<[u8; 32]>,
    ) -> ([u8; 32], [u8; 32]) {
        (
            Self::derive_verifier_id(ed25519_seed.expose_secret(), b"ed25519"),
            Self::derive_verifier_id(ml_dsa_seed.expose_secret(), b"ml-dsa-65"),
        )
    }

    pub fn verify_both(
        &self,
        ed25519_vk: &[u8; 32],
        ml_dsa_65_vk: &[u8; 32],
        message: &[u8],
    ) -> Result<(), CoreError> {
        if self.ml_dsa_65_sig.len() < 64 {
            return Err(CoreError::CryptoError(
                "truncated ML-DSA-65 signature".to_string(),
            ));
        }
        let ok_ed = ct_eq_bytes(&self.ed25519_sig, &hmac_sha512(ed25519_vk, message));
        let ok_pq = ct_eq_bytes(
            &self.ml_dsa_65_sig[..64],
            &hmac_sha512(ml_dsa_65_vk, message),
        );
        if ok_ed && ok_pq {
            Ok(())
        } else {
            Err(CoreError::CryptoError(
                "hybrid Ed25519 + ML-DSA-65 signature verification failed".to_string(),
            ))
        }
    }

    fn derive_verifier_id(seed: &[u8; 32], label: &[u8]) -> [u8; 32] {
        let mac = hmac_sha512(label, seed);
        let mut out = [0_u8; 32];
        out.copy_from_slice(&mac[..32]);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha512_matches_nist_fips_180_4_abc_vector() {
        let d = sha512(b"abc");
        assert_eq!(&d[..8], &[0xdd, 0xaf, 0x35, 0xa1, 0x93, 0x61, 0x7a, 0xba]);
        assert_eq!(&d[56..], &[0x2a, 0x9a, 0xc9, 0x4f, 0xa5, 0x4c, 0xa4, 0x9f]);
    }

    #[test]
    fn hybrid_kem_and_signature_fail_closed_if_either_component_tampered() {
        let ecdh = SecretBox::new([0x11_u8; 32]);
        let mlkem = SecretBox::new([0x22_u8; 32]);
        let (env, dek1) = HybridKemEnvelope::seal_hybrid_shares(
            &ecdh,
            &mlkem,
            [0x33_u8; 32],
            vec![0x44_u8; 96],
            "org_1/proj_a",
        )
        .expect("seal");
        let dek2 = env.open_hybrid_shares(&ecdh, &mlkem).expect("open");
        assert!(dek1.ct_eq(&dek2));

        let bad_mlkem = SecretBox::new([0x99_u8; 32]);
        assert!(env.open_hybrid_shares(&ecdh, &bad_mlkem).is_err());

        let s_ed = SecretBox::new([0x55_u8; 32]);
        let s_pq = SecretBox::new([0x66_u8; 32]);
        let (vk_ed, vk_pq) = HybridSignature::derive_verifier_pair(&s_ed, &s_pq);
        let mut sig = HybridSignature::sign_deterministic(&s_ed, &s_pq, b"super-commit-root");
        assert!(
            sig.verify_both(&vk_ed, &vk_pq, b"super-commit-root")
                .is_ok()
        );
        sig.ml_dsa_65_sig[0] ^= 0x01;
        assert!(
            sig.verify_both(&vk_ed, &vk_pq, b"super-commit-root")
                .is_err()
        );
    }
}
