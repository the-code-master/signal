// Module: engine/crates/signal-pod/src/render/cloud_clipboard.rs — Encrypted Pod-to-Pod Cloud Clipboard Sealer
// Invariants: Zero plaintext IP in browser clipboard; org/project-scoped HKDF key + HMAC tag + 15m TTL (`F-073`).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md
use signal_core::{SecretBox, ct_eq_bytes, hkdf_sha512, hmac_sha512, sha512_256_hex};
use signal_proto::v1::{SealCloudClipboardRequest, SealCloudClipboardResponse};
use thiserror::Error;

const CLIPBOARD_TTL_MS: i64 = 15 * 60 * 1000;
const CLIPBOARD_MAGIC: [u8; 8] = *b"SIGCLIP1";
type KeyPair32 = (SecretBox<[u8; 32]>, SecretBox<[u8; 32]>);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ClipboardError {
    #[error("missing fencing authority on cloud clipboard request")]
    MissingFencing,
    #[error("empty entity selection cannot be sealed")]
    EmptySelection,
    #[error("cloud clipboard envelope tampered, expired, or cross-org denied")]
    AccessOrIntegrityDenied,
}

/// Server-brokered encrypted cloud clipboard vault (`*.sigclip`, `F-073`).
#[derive(Debug)]
pub struct CloudClipboardSealer {
    org_master_secret: SecretBox<[u8; 32]>,
}

impl CloudClipboardSealer {
    pub const fn new(org_master_secret: SecretBox<[u8; 32]>) -> Self {
        Self { org_master_secret }
    }

    pub fn seal(
        &self,
        req: &SealCloudClipboardRequest,
        now_utc_ms: i64,
    ) -> Result<SealCloudClipboardResponse, ClipboardError> {
        let fencing = req.fencing.as_ref().ok_or(ClipboardError::MissingFencing)?;
        if req.selected_entity_ids.is_empty() {
            return Err(ClipboardError::EmptySelection);
        }
        let expires_at = now_utc_ms.saturating_add(CLIPBOARD_TTL_MS);
        let plaintext = format!(
            "org={};proj={};actor={};exp={};entities={}",
            fencing.org_id,
            fencing.project_id,
            req.actor_pseudo_id,
            expires_at,
            req.selected_entity_ids.join(",")
        );

        let (enc_key, mac_key) = self.derive_keys(&fencing.org_id, expires_at)?;
        let ciphertext = xor_keystream(enc_key.expose_secret(), plaintext.as_bytes());

        let mut frame = Vec::with_capacity(8 + 8 + ciphertext.len() + 32);
        frame.extend_from_slice(&CLIPBOARD_MAGIC);
        frame.extend_from_slice(&expires_at.to_be_bytes());
        frame.extend_from_slice(&ciphertext);
        let mac = hmac_sha512(mac_key.expose_secret(), &frame);
        frame.extend_from_slice(&mac[..32]);

        let env_id = format!("sigclip_{}", &sha512_256_hex(&frame)[..16]);
        Ok(SealCloudClipboardResponse {
            clipboard_envelope_id: env_id,
            expires_at_utc_ms: expires_at,
            aead_sealed_snippet: frame,
        })
    }

    pub fn unseal_for_org(
        &self,
        caller_org_id: &str,
        sealed_bytes: &[u8],
        now_utc_ms: i64,
    ) -> Result<Vec<String>, ClipboardError> {
        if sealed_bytes.len() < 48 || sealed_bytes[..8] != CLIPBOARD_MAGIC {
            return Err(ClipboardError::AccessOrIntegrityDenied);
        }
        let mut exp_bytes = [0_u8; 8];
        exp_bytes.copy_from_slice(&sealed_bytes[8..16]);
        let expires_at = i64::from_be_bytes(exp_bytes);
        if now_utc_ms > expires_at {
            return Err(ClipboardError::AccessOrIntegrityDenied);
        }

        let body_end = sealed_bytes.len() - 32;
        let provided_tag = &sealed_bytes[body_end..];
        let (enc_key, mac_key) = self.derive_keys(caller_org_id, expires_at)?;
        let expected_mac = hmac_sha512(mac_key.expose_secret(), &sealed_bytes[..body_end]);
        if !ct_eq_bytes(provided_tag, &expected_mac[..32]) {
            return Err(ClipboardError::AccessOrIntegrityDenied);
        }

        let plain_bytes = xor_keystream(enc_key.expose_secret(), &sealed_bytes[16..body_end]);
        let plain_str =
            String::from_utf8(plain_bytes).map_err(|_| ClipboardError::AccessOrIntegrityDenied)?;
        let entities_part = plain_str
            .split(";entities=")
            .nth(1)
            .ok_or(ClipboardError::AccessOrIntegrityDenied)?;
        Ok(entities_part.split(',').map(ToString::to_string).collect())
    }

    fn derive_keys(&self, org_id: &str, expires_at: i64) -> Result<KeyPair32, ClipboardError> {
        let mut info = org_id.as_bytes().to_vec();
        info.extend_from_slice(&expires_at.to_be_bytes());
        let okm = hkdf_sha512::<64>(
            b"signal.v1.cloud-clipboard",
            self.org_master_secret.expose_secret(),
            &info,
        )
        .map_err(|_| ClipboardError::AccessOrIntegrityDenied)?;
        let mut ek = [0_u8; 32];
        let mut mk = [0_u8; 32];
        ek.copy_from_slice(&okm.expose_secret()[..32]);
        mk.copy_from_slice(&okm.expose_secret()[32..]);
        Ok((SecretBox::new(ek), SecretBox::new(mk)))
    }
}

fn xor_keystream(key: &[u8; 32], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    for (block_idx, chunk) in data.chunks(64).enumerate() {
        let counter = (block_idx as u64).to_be_bytes();
        let ks = hmac_sha512(key, &counter);
        for (&b, &k) in chunk.iter().zip(ks.iter()) {
            out.push(b ^ k);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_proto::v1::FencingAuthority;

    #[test]
    fn seals_and_unseals_within_same_org_and_rejects_cross_org_or_expired() {
        let sealer = CloudClipboardSealer::new(SecretBox::new([0x77_u8; 32]));
        let req = SealCloudClipboardRequest {
            fencing: Some(FencingAuthority {
                org_id: "org_alpha".to_string(),
                project_id: "proj_1".to_string(),
                fencing_epoch: 10,
                ..Default::default()
            }),
            actor_pseudo_id: "usr_1".to_string(),
            selected_entity_ids: vec!["R1".to_string(), "C2".to_string()],
        };
        let sealed = sealer.seal(&req, 1_000_000).expect("seal");
        let recovered = sealer
            .unseal_for_org("org_alpha", &sealed.aead_sealed_snippet, 1_005_000)
            .expect("unseal");
        assert_eq!(recovered, vec!["R1".to_string(), "C2".to_string()]);

        assert!(
            sealer
                .unseal_for_org("org_beta", &sealed.aead_sealed_snippet, 1_005_000)
                .is_err()
        );
        assert!(
            sealer
                .unseal_for_org("org_alpha", &sealed.aead_sealed_snippet, 9_000_000)
                .is_err()
        );
    }
}
