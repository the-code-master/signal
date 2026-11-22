// Module: engine/crates/signal-core/src/sha512_hkdf.rs — Pure-Rust FIPS 180-4 SHA-512, HMAC-SHA512 & RFC 5869 HKDF
// Invariants: Zero unsafe code; zeroizes intermediate PRK/key pads before return (F-078, F-088).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/15-application-security-and-project-isolation.md
use crate::CoreError;
use crate::secret_box::SecretBox;
use zeroize::Zeroize;

const SHA512_H0: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

const SHA512_K: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

fn compress_sha512(state: &mut [u64; 8], block: &[u8; 128]) {
    let mut w = [0_u64; 80];
    for (i, word) in w.iter_mut().take(16).enumerate() {
        let mut b = [0_u8; 8];
        b.copy_from_slice(&block[i * 8..(i + 1) * 8]);
        *word = u64::from_be_bytes(b);
    }
    for i in 16..80 {
        let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
        let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for i in 0..80 {
        let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
        let ch = (e & f) ^ ((!e) & g);
        let t1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(SHA512_K[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(s0.wrapping_add(maj));
    }
    for (dst, add) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *dst = dst.wrapping_add(add);
    }
}

pub fn sha512(data: &[u8]) -> [u8; 64] {
    let mut state = SHA512_H0;
    let (blocks, rem) = data.as_chunks::<128>();
    for blk in blocks {
        compress_sha512(&mut state, blk);
    }
    let mut pad = [0_u8; 256];
    pad[..rem.len()].copy_from_slice(rem);
    pad[rem.len()] = 0x80;
    let total_blocks = if rem.len() < 112 { 1 } else { 2 };
    let bit_len = (data.len() as u128) * 8;
    pad[total_blocks * 128 - 16..total_blocks * 128].copy_from_slice(&bit_len.to_be_bytes());
    for b_idx in 0..total_blocks {
        let mut blk = [0_u8; 128];
        blk.copy_from_slice(&pad[b_idx * 128..(b_idx + 1) * 128]);
        compress_sha512(&mut state, &blk);
    }
    let mut out = [0_u8; 64];
    for (i, word) in state.iter().enumerate() {
        out[i * 8..(i + 1) * 8].copy_from_slice(&word.to_be_bytes());
    }
    out
}

pub fn sha512_256_hex(data: &[u8]) -> String {
    let digest = sha512(data);
    let mut s = String::with_capacity(64);
    for &b in &digest[..32] {
        use std::fmt::Write as _;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

pub fn hmac_sha512(key: &[u8], msg: &[u8]) -> [u8; 64] {
    let mut k_pad = [0_u8; 128];
    if key.len() > 128 {
        let hashed = sha512(key);
        k_pad[..64].copy_from_slice(&hashed);
    } else {
        k_pad[..key.len()].copy_from_slice(key);
    }
    let mut inner = Vec::with_capacity(128 + msg.len());
    for &b in &k_pad {
        inner.push(b ^ 0x36);
    }
    inner.extend_from_slice(msg);
    let inner_hash = sha512(&inner);
    inner.zeroize();

    let mut outer = [0_u8; 192];
    for (i, &b) in k_pad.iter().enumerate() {
        outer[i] = b ^ 0x5c;
    }
    outer[128..].copy_from_slice(&inner_hash);
    k_pad.zeroize();
    let res = sha512(&outer);
    outer.zeroize();
    res
}

pub fn hkdf_sha512<const N: usize>(
    salt: &[u8],
    ikm: &[u8],
    info: &[u8],
) -> Result<SecretBox<[u8; N]>, CoreError> {
    if N == 0 || N > 255 * 64 {
        return Err(CoreError::CryptoError(
            "invalid HKDF output length".to_string(),
        ));
    }
    let effective_salt = if salt.is_empty() {
        &[0_u8; 64][..]
    } else {
        salt
    };
    let mut prk = hmac_sha512(effective_salt, ikm);
    let mut okm = [0_u8; N];
    let mut prev = [0_u8; 64];
    let mut prev_len = 0_usize;
    let mut generated = 0_usize;
    let mut counter = 1_u8;

    while generated < N {
        let mut buf = Vec::with_capacity(prev_len + info.len() + 1);
        buf.extend_from_slice(&prev[..prev_len]);
        buf.extend_from_slice(info);
        buf.push(counter);
        prev = hmac_sha512(&prk, &buf);
        prev_len = 64;
        let take = (N - generated).min(64);
        okm[generated..generated + take].copy_from_slice(&prev[..take]);
        generated += take;
        counter = counter.wrapping_add(1);
    }
    prk.zeroize();
    prev.zeroize();
    Ok(SecretBox::new(okm))
}
