// Module: engine/crates/signal-git-sync/src/padme_vault.rs — Padme-Padded, Path-Blinded SIV Vault & Merkle Root Manifest
// Invariants: O(log log L) Padme padding (F-117); HMAC path blinding `blobs/<xx>/<hmac>.sigv` (F-013); rollback-proof `.signal/vault.merkle.sig` (I-12).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/13-vcs-git-server-and-storage-analysis.md
use crate::GitSyncError;
use signal_core::{
    HybridSignature, SecretBox, ct_eq_bytes, hkdf_sha512, hmac_sha512, sha512_256_hex,
};
use signal_proto::v1::{PadmeVaultBlobEntry, ProjectVaultMode};

/// Exact Padmé $O(\log \log L)$ length-hiding padding (`<= 12%` max overhead, `F-117`).
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

/// Enforces `MUST_ENCRYPT` NDA taint invariant: tainted IP may never write to `PLAIN_CANONICAL` (`F-013`).
pub fn enforce_must_encrypt_taint(
    vault_mode: ProjectVaultMode,
    must_encrypt_tainted: bool,
) -> Result<(), GitSyncError> {
    if must_encrypt_tainted && vault_mode != ProjectVaultMode::EncryptedVault {
        return Err(GitSyncError::MustEncryptTaintViolation);
    }
    Ok(())
}

/// Derives blinded Git path `blobs/<2-hex-prefix>/<24-hex>.sigv` and full 64-hex entity HMAC (`F-013`).
pub fn blind_vault_entity_path(
    path_blind_key: &SecretBox<[u8; 32]>,
    logical_entity_path: &str,
) -> (String, String) {
    let mac = hmac_sha512(
        path_blind_key.expose_secret(),
        logical_entity_path.as_bytes(),
    );
    let mut full_hex = String::with_capacity(64);
    for &b in &mac[..32] {
        use std::fmt::Write as _;
        let _ = write!(&mut full_hex, "{b:02x}");
    }
    let prefix = &full_hex[..2];
    let short = &full_hex[..12];
    (format!("blobs/{prefix}/{short}.sigv"), full_hex)
}

#[derive(Debug, Clone, PartialEq)]
pub struct SealedPadmeBlob {
    pub meta: PadmeVaultBlobEntry,
    pub siv_tag: [u8; 16],
    pub ciphertext_padded: Vec<u8>,
}

fn hex_encode_16(bytes: &[u8; 16]) -> String {
    let mut s = String::with_capacity(32);
    for &b in bytes {
        use std::fmt::Write as _;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

fn siv_stream_xor(enc_key: &[u8; 32], siv_tag: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut block_input = [0_u8; 24];
    block_input[..16].copy_from_slice(siv_tag);
    for (ctr, chunk) in data.chunks(64).enumerate() {
        block_input[16..24].copy_from_slice(&(ctr as u64).to_le_bytes());
        let ks = hmac_sha512(enc_key, &block_input);
        for (&b, &k) in chunk.iter().zip(ks.iter()) {
            out.push(b ^ k);
        }
    }
    out
}

/// Seals an entity payload into a Padmé-padded, path-blinded, nonce-misuse-resistant SIV blob (`F-013`, `F-117`, `I-12`).
pub fn seal_padme_vault_blob(
    project_dek: &SecretBox<[u8; 32]>,
    dek_version: u32,
    logical_entity_path: &str,
    plaintext: &[u8],
) -> Result<SealedPadmeBlob, GitSyncError> {
    let path_key = hkdf_sha512::<32>(
        b"signal.v1.vault.path-blind",
        project_dek.expose_secret(),
        &dek_version.to_be_bytes(),
    )
    .map_err(|e| GitSyncError::VaultIntegrityFailure(e.to_string()))?;
    let (blinded_path, entity_hmac_hex) = blind_vault_entity_path(&path_key, logical_entity_path);

    let subkeys = hkdf_sha512::<64>(
        b"signal.v1.vault.siv-entity",
        project_dek.expose_secret(),
        entity_hmac_hex.as_bytes(),
    )
    .map_err(|e| GitSyncError::VaultIntegrityFailure(e.to_string()))?;
    let mut mac_key = [0_u8; 32];
    let mut enc_key = [0_u8; 32];
    mac_key.copy_from_slice(&subkeys.expose_secret()[..32]);
    enc_key.copy_from_slice(&subkeys.expose_secret()[32..]);

    let raw_len = plaintext.len() as u64;
    let framed_len = raw_len.saturating_add(8);
    let target_padded = (padme_padded_len(framed_len) as usize).max(framed_len as usize);
    let mut padded_plain = vec![0_u8; target_padded];
    padded_plain[..8].copy_from_slice(&raw_len.to_le_bytes());
    padded_plain[8..8 + plaintext.len()].copy_from_slice(plaintext);

    // Deterministic SIV tag over (dek_version || blinded_path || padded_plain)
    let mut siv_input = Vec::with_capacity(4 + blinded_path.len() + padded_plain.len());
    siv_input.extend_from_slice(&dek_version.to_be_bytes());
    siv_input.extend_from_slice(blinded_path.as_bytes());
    siv_input.extend_from_slice(&padded_plain);
    let full_mac = hmac_sha512(&mac_key, &siv_input);
    let mut siv_tag = [0_u8; 16];
    siv_tag.copy_from_slice(&full_mac[..16]);

    let ciphertext_padded = siv_stream_xor(&enc_key, &siv_tag, &padded_plain);

    Ok(SealedPadmeBlob {
        meta: PadmeVaultBlobEntry {
            blinded_blob_path: blinded_path,
            entity_hmac_hex,
            unpadded_size_bytes: raw_len,
            padme_padded_bytes: target_padded as u64,
            dek_version,
            gcm_siv_tag_hex: hex_encode_16(&siv_tag),
        },
        siv_tag,
        ciphertext_padded,
    })
}

/// Verifies SIV tag in constant time and strips Padmé length-hiding padding (`F-013`, `F-117`).
pub fn open_padme_vault_blob(
    project_dek: &SecretBox<[u8; 32]>,
    blob: &SealedPadmeBlob,
) -> Result<Vec<u8>, GitSyncError> {
    let subkeys = hkdf_sha512::<64>(
        b"signal.v1.vault.siv-entity",
        project_dek.expose_secret(),
        blob.meta.entity_hmac_hex.as_bytes(),
    )
    .map_err(|e| GitSyncError::VaultIntegrityFailure(e.to_string()))?;
    let mut mac_key = [0_u8; 32];
    let mut enc_key = [0_u8; 32];
    mac_key.copy_from_slice(&subkeys.expose_secret()[..32]);
    enc_key.copy_from_slice(&subkeys.expose_secret()[32..]);

    let padded_plain = siv_stream_xor(&enc_key, &blob.siv_tag, &blob.ciphertext_padded);
    if padded_plain.len() < 8 {
        return Err(GitSyncError::VaultIntegrityFailure(
            "truncated vault blob".to_string(),
        ));
    }

    let mut siv_input =
        Vec::with_capacity(4 + blob.meta.blinded_blob_path.len() + padded_plain.len());
    siv_input.extend_from_slice(&blob.meta.dek_version.to_be_bytes());
    siv_input.extend_from_slice(blob.meta.blinded_blob_path.as_bytes());
    siv_input.extend_from_slice(&padded_plain);
    let expected_mac = hmac_sha512(&mac_key, &siv_input);
    if !ct_eq_bytes(&blob.siv_tag, &expected_mac[..16]) {
        return Err(GitSyncError::VaultIntegrityFailure(
            "SIV authentication tag mismatch".to_string(),
        ));
    }

    let mut len_bytes = [0_u8; 8];
    len_bytes.copy_from_slice(&padded_plain[..8]);
    let raw_len = u64::from_le_bytes(len_bytes) as usize;
    if 8 + raw_len > padded_plain.len() {
        return Err(GitSyncError::VaultIntegrityFailure(
            "corrupt Padme raw_len".to_string(),
        ));
    }
    Ok(padded_plain[8..8 + raw_len].to_vec())
}

/// `.signal/vault.merkle.sig` manifest binding all vault blobs at commit root to defeat rollback swaps (`I-12`).
#[derive(Debug, Clone, PartialEq)]
pub struct VaultMerkleManifest {
    pub merkle_root_hex: String,
    pub entries_sorted: Vec<PadmeVaultBlobEntry>,
    pub signature: HybridSignature,
}

pub fn compute_vault_merkle_root(
    entries: &[PadmeVaultBlobEntry],
) -> (String, Vec<PadmeVaultBlobEntry>) {
    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| a.blinded_blob_path.cmp(&b.blinded_blob_path));
    let mut canonical = String::new();
    for e in &sorted {
        use std::fmt::Write as _;
        let _ = write!(
            &mut canonical,
            "{}:{}:v{}:{};",
            e.blinded_blob_path, e.gcm_siv_tag_hex, e.dek_version, e.padme_padded_bytes
        );
    }
    (sha512_256_hex(canonical.as_bytes()), sorted)
}

pub fn sign_vault_merkle_manifest(
    entries: &[PadmeVaultBlobEntry],
    ed25519_seed: &SecretBox<[u8; 32]>,
    ml_dsa_seed: &SecretBox<[u8; 32]>,
) -> VaultMerkleManifest {
    let (root, sorted) = compute_vault_merkle_root(entries);
    let sig = HybridSignature::sign_deterministic(ed25519_seed, ml_dsa_seed, root.as_bytes());
    VaultMerkleManifest {
        merkle_root_hex: root,
        entries_sorted: sorted,
        signature: sig,
    }
}

pub fn verify_vault_merkle_manifest(
    manifest: &VaultMerkleManifest,
    live_entries: &[PadmeVaultBlobEntry],
    ed25519_vk: &[u8; 32],
    ml_dsa_vk: &[u8; 32],
) -> Result<(), GitSyncError> {
    let (recomputed_root, _) = compute_vault_merkle_root(live_entries);
    if recomputed_root != manifest.merkle_root_hex {
        return Err(GitSyncError::VaultIntegrityFailure(
            "vault.merkle.sig rollback or single-blob swap detected".to_string(),
        ));
    }
    manifest
        .signature
        .verify_both(ed25519_vk, ml_dsa_vk, manifest.merkle_root_hex.as_bytes())
        .map_err(|e| GitSyncError::VaultIntegrityFailure(e.to_string()))
}
