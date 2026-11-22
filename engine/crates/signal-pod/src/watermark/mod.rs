// Module: engine/crates/signal-pod/src/watermark/mod.rs — Tardos Collusion-Resistant & Constellation Watermarks (F-080, F-128)
// Invariants: Tardos log-likelihood catches colluding leakers; HMAC constellation shifts survive AI diffusion & vector tracing.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md

use signal_core::{det_sin, hmac_sha512};
use signal_proto::v1::ConstellationWatermarkMetadata;

#[derive(Debug, Clone, PartialEq)]
pub struct TardosSessionWatermark {
    pub bias_probabilities: Vec<f64>,
    pub user_codeword_bits: Vec<u8>,
}

/// Generates arcsine-biased column probabilities $p_i \in [\kappa, 1-\kappa]$ (`epoch_seed`)
/// and a collaborator-specific Tardos codeword (`session_id`, `F-080`).
pub fn generate_tardos_watermark(
    epoch_seed: &[u8; 32],
    session_id: &str,
    code_len_bits: usize,
) -> TardosSessionWatermark {
    let m = code_len_bits.clamp(32, 2048);
    let kappa = 0.02_f64;
    let mut probs = Vec::with_capacity(m);
    let mut bits = Vec::with_capacity(m);

    for i in 0..m {
        let col_mac = hmac_sha512(epoch_seed, format!("tardos-col:{i}").as_bytes());
        let u = (u32::from_be_bytes([col_mac[0], col_mac[1], col_mac[2], col_mac[3]]) as f64)
            / (u32::MAX as f64);
        let t = kappa + (1.0 - 2.0 * kappa) * u;
        let s = det_sin(0.5 * std::f64::consts::PI * t);
        let p_i = (s * s).clamp(kappa, 1.0 - kappa);
        probs.push(p_i);

        let usr_mac = hmac_sha512(
            epoch_seed,
            format!("tardos-usr:{session_id}:{i}").as_bytes(),
        );
        let r = (u32::from_be_bytes([usr_mac[0], usr_mac[1], usr_mac[2], usr_mac[3]]) as f64)
            / (u32::MAX as f64);
        bits.push(u8::from(r < p_i));
    }

    TardosSessionWatermark {
        bias_probabilities: probs,
        user_codeword_bits: bits,
    }
}

/// Embeds spread-spectrum $\pm 2$ blue-channel steganographic chips into an `RGBA8` viewport tile (`F-080`).
pub fn embed_tardos_rgba8_tile(rgba: &mut [u8], codeword_bits: &[u8]) {
    if codeword_bits.is_empty() {
        return;
    }
    for (px_idx, px) in rgba.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        let bit = codeword_bits[px_idx % codeword_bits.len()];
        let delta: i16 = if bit == 1 { 2 } else { -2 };
        px[2] = (i16::from(px[2]) + delta).clamp(0, 255) as u8;
    }
}

/// Computes Tardos accusation correlation score against a recovered pirated `RGBA8` tile (`F-080`).
pub fn score_tardos_accusation(
    pirated_rgba: &[u8],
    reference_clean_rgba: &[u8],
    candidate: &TardosSessionWatermark,
) -> f64 {
    let m = candidate.user_codeword_bits.len();
    if m == 0 {
        return 0.0;
    }
    let mut extracted_votes = vec![0_i32; m];
    for (px_idx, (pir, cln)) in pirated_rgba
        .as_chunks::<4>()
        .0
        .iter()
        .zip(reference_clean_rgba.as_chunks::<4>().0.iter())
        .enumerate()
    {
        let diff = i16::from(pir[2]) - i16::from(cln[2]);
        if diff > 0 {
            extracted_votes[px_idx % m] += 1;
        } else if diff < 0 {
            extracted_votes[px_idx % m] -= 1;
        }
    }

    let mut score = 0.0_f64;
    for i in 0..m {
        if extracted_votes[i] > 0 {
            let p = candidate.bias_probabilities[i];
            if candidate.user_codeword_bits[i] == 1 {
                score += ((1.0 - p) / p).sqrt();
            } else {
                score -= (p / (1.0 - p)).sqrt();
            }
        }
    }
    score
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstellationAnchorShift {
    pub anchor_id: String,
    pub silkscreen_kerning_px: i8,
    pub chamfer_phase_offset_nm: i64,
}

/// Computes AI-resilient topological & geometric constellation micro-shifts (`F-128`)
/// keyed by `HMAC(session_seed, tile_id || anchor_id)`.
pub fn build_constellation_watermark(
    session_seed: &[u8; 32],
    watermark_epoch_id: &str,
    tile_id: &str,
    anchor_ids: &[String],
    tardos_bits: u32,
) -> (
    ConstellationWatermarkMetadata,
    Vec<ConstellationAnchorShift>,
) {
    let mut shifts = Vec::with_capacity(anchor_ids.len());
    for aid in anchor_ids {
        let mac = hmac_sha512(session_seed, format!("constel:{tile_id}:{aid}").as_bytes());
        let k_px = ((mac[0] % 5) as i8) - 2; // -2..=+2 px silkscreen kerning delta
        let ch_nm = (i64::from(i16::from_be_bytes([mac[1], mac[2]])) % 150) * 1_000;
        shifts.push(ConstellationAnchorShift {
            anchor_id: aid.clone(),
            silkscreen_kerning_px: if k_px == 0 { 1 } else { k_px },
            chamfer_phase_offset_nm: ch_nm,
        });
    }

    let fp_mac = hmac_sha512(session_seed, format!("fp:{watermark_epoch_id}").as_bytes());
    let mut fp_hex = String::with_capacity(32);
    for &b in &fp_mac[..16] {
        use std::fmt::Write as _;
        let _ = write!(&mut fp_hex, "{b:02x}");
    }

    let meta = ConstellationWatermarkMetadata {
        watermark_epoch_id: watermark_epoch_id.to_string(),
        tardos_code_length_bits: tardos_bits,
        constellation_anchor_count: shifts.len() as u32,
        session_fingerprint_hmac: fp_hex,
    };
    (meta, shifts)
}
