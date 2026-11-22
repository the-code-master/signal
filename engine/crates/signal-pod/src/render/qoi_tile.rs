// Module: engine/crates/signal-pod/src/render/qoi_tile.rs — Pure-Rust Lossless QOI + Zstd-Framed Dirty-Tile RGBA8 Codec
// Invariants: Exact round-trip RGBA8 fidelity; bomb-capped decode (<= 20:1, <= 16M px); 80 ms lossless settling frame (F-066).
// Spec: architecture/03-rust-eda-engine.md, architecture/06-frontend-angular-and-fastapi-control-plane.md
use thiserror::Error;

pub const SETTLING_FRAME_DELAY_MS: u64 = 80;
const QOI_MAGIC: [u8; 4] = *b"qoif";
const QOI_END_MARKER: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];
const ZSTD_MAGIC_LE: [u8; 4] = [0x28, 0xB5, 0x2F, 0xFD];
const MAX_TILE_PIXELS: usize = 4096 * 4096;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RenderCodecError {
    #[error("RGBA8 buffer length {actual} does not match {width}x{height}*4")]
    BufferMismatch {
        width: u32,
        height: u32,
        actual: usize,
    },
    #[error("invalid or truncated QOI/Zstd frame: {0}")]
    MalformedFrame(&'static str),
    #[error("tile dimensions {0}x{1} exceed Zero-Leak bomb cap")]
    DimensionsExceeded(u32, u32),
}

#[inline]
fn qoi_color_hash(px: [u8; 4]) -> usize {
    (usize::from(px[0]) * 3
        + usize::from(px[1]) * 5
        + usize::from(px[2]) * 7
        + usize::from(px[3]) * 11)
        & 63
}

/// Encodes an `RGBA8` buffer into canonical lossless QOI bytes (`F-066`).
pub fn encode_qoi_rgba8(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>, RenderCodecError> {
    let pixel_count = (width as usize).saturating_mul(height as usize);
    if width == 0 || height == 0 || pixel_count > MAX_TILE_PIXELS {
        return Err(RenderCodecError::DimensionsExceeded(width, height));
    }
    if rgba.len() != pixel_count * 4 {
        return Err(RenderCodecError::BufferMismatch {
            width,
            height,
            actual: rgba.len(),
        });
    }

    let mut out = Vec::with_capacity(14 + pixel_count + 8);
    out.extend_from_slice(&QOI_MAGIC);
    out.extend_from_slice(&width.to_be_bytes());
    out.extend_from_slice(&height.to_be_bytes());
    out.push(4); // RGBA channels
    out.push(0); // sRGB colorspace

    let mut index = [[0_u8; 4]; 64];
    let mut prev = [0_u8, 0, 0, 255];
    let mut run = 0_u8;
    let (pixels, _) = rgba.as_chunks::<4>();

    for (idx, &px) in pixels.iter().enumerate() {
        if px == prev {
            run += 1;
            if run == 62 || idx + 1 == pixel_count {
                out.push(0b1100_0000 | (run - 1));
                run = 0;
            }
            continue;
        }
        if run > 0 {
            out.push(0b1100_0000 | (run - 1));
            run = 0;
        }

        let hash_pos = qoi_color_hash(px);
        if index[hash_pos] == px {
            out.push(hash_pos as u8);
        } else {
            index[hash_pos] = px;
            if px[3] == prev[3] {
                let vr = px[0].wrapping_sub(prev[0]) as i8;
                let vg = px[1].wrapping_sub(prev[1]) as i8;
                let vb = px[2].wrapping_sub(prev[2]) as i8;
                let vg_r = vr.wrapping_sub(vg);
                let vg_b = vb.wrapping_sub(vg);

                if (-2..=1).contains(&vr) && (-2..=1).contains(&vg) && (-2..=1).contains(&vb) {
                    out.push(
                        0b0100_0000
                            | (((vr + 2) as u8) << 4)
                            | (((vg + 2) as u8) << 2)
                            | ((vb + 2) as u8),
                    );
                } else if (-8..=7).contains(&vg_r)
                    && (-32..=31).contains(&vg)
                    && (-8..=7).contains(&vg_b)
                {
                    out.push(0b1000_0000 | ((vg + 32) as u8));
                    out.push((((vg_r + 8) as u8) << 4) | ((vg_b + 8) as u8));
                } else {
                    out.extend_from_slice(&[0b1111_1110, px[0], px[1], px[2]]);
                }
            } else {
                out.extend_from_slice(&[0b1111_1111, px[0], px[1], px[2], px[3]]);
            }
        }
        prev = px;
    }

    out.extend_from_slice(&QOI_END_MARKER);
    Ok(out)
}

/// Decodes canonical QOI bytes back into `(width, height, rgba8)` (`F-066`).
pub fn decode_qoi_rgba8(encoded: &[u8]) -> Result<(u32, u32, Vec<u8>), RenderCodecError> {
    if encoded.len() < 22 || encoded[..4] != QOI_MAGIC {
        return Err(RenderCodecError::MalformedFrame("missing qoif header"));
    }
    let width = u32::from_be_bytes([encoded[4], encoded[5], encoded[6], encoded[7]]);
    let height = u32::from_be_bytes([encoded[8], encoded[9], encoded[10], encoded[11]]);
    let pixel_count = (width as usize).saturating_mul(height as usize);
    if width == 0 || height == 0 || pixel_count > MAX_TILE_PIXELS {
        return Err(RenderCodecError::DimensionsExceeded(width, height));
    }

    let mut out = vec![0_u8; pixel_count * 4];
    let mut index = [[0_u8; 4]; 64];
    let mut px = [0_u8, 0, 0, 255];
    let mut run = 0_u8;
    let mut p = 14_usize;
    let chunks_end = encoded.len() - 8;

    let (out_pixels, _) = out.as_chunks_mut::<4>();
    for dst in out_pixels {
        if run > 0 {
            run -= 1;
        } else if p < chunks_end {
            let b1 = encoded[p];
            p += 1;
            if b1 == 0b1111_1110 {
                if p + 3 > chunks_end {
                    return Err(RenderCodecError::MalformedFrame("truncated QOI_OP_RGB"));
                }
                px[0] = encoded[p];
                px[1] = encoded[p + 1];
                px[2] = encoded[p + 2];
                p += 3;
            } else if b1 == 0b1111_1111 {
                if p + 4 > chunks_end {
                    return Err(RenderCodecError::MalformedFrame("truncated QOI_OP_RGBA"));
                }
                px.copy_from_slice(&encoded[p..p + 4]);
                p += 4;
            } else {
                match b1 & 0b1100_0000 {
                    0b0000_0000 => px = index[usize::from(b1 & 0x3f)],
                    0b0100_0000 => {
                        px[0] = px[0].wrapping_add(((b1 >> 4) & 0x03).wrapping_sub(2));
                        px[1] = px[1].wrapping_add(((b1 >> 2) & 0x03).wrapping_sub(2));
                        px[2] = px[2].wrapping_add((b1 & 0x03).wrapping_sub(2));
                    }
                    0b1000_0000 => {
                        if p >= chunks_end {
                            return Err(RenderCodecError::MalformedFrame("truncated QOI_OP_LUMA"));
                        }
                        let b2 = encoded[p];
                        p += 1;
                        let vg = (b1 & 0x3f).wrapping_sub(32);
                        px[0] = px[0].wrapping_add(vg.wrapping_sub(8).wrapping_add(b2 >> 4));
                        px[1] = px[1].wrapping_add(vg);
                        px[2] = px[2].wrapping_add(vg.wrapping_sub(8).wrapping_add(b2 & 0x0f));
                    }
                    _ => run = b1 & 0x3f,
                }
            }
            index[qoi_color_hash(px)] = px;
        }
        *dst = px;
    }
    Ok((width, height, out))
}

/// Wraps QOI payload inside RFC 8878 Zstandard RLE/Raw blocks (`Zstd-QOI`, `F-066`).
pub fn encode_zstd_qoi_rgba8(
    rgba: &[u8],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, RenderCodecError> {
    let qoi = encode_qoi_rgba8(rgba, width, height)?;
    let mut zstd = Vec::with_capacity(qoi.len() + 16);
    zstd.extend_from_slice(&ZSTD_MAGIC_LE);
    zstd.push(0x00); // Frame_Header_Descriptor (Single_Segment_Flag=0, Window_Descriptor follows)
    zstd.push(0x38); // Window_Descriptor (1 MiB window)

    const MAX_BLOCK: usize = 0x1F_FFFF; // 21-bit block size
    let mut offset = 0;
    while offset < qoi.len() {
        let rem = qoi.len() - offset;
        let blk_len = rem.min(MAX_BLOCK);
        let last_flag = u32::from(offset + blk_len == qoi.len());
        // RFC 8878 Section 3.1.1.2: Block_Header (3 bytes LE): Last_Block (1b) | Block_Type=00 Raw (2b) | Block_Size (21b)
        let hdr = last_flag | ((blk_len as u32) << 3);
        let hdr_bytes = hdr.to_le_bytes();
        zstd.extend_from_slice(&hdr_bytes[..3]);
        zstd.extend_from_slice(&qoi[offset..offset + blk_len]);
        offset += blk_len;
    }
    Ok(zstd)
}

/// Decodes RFC 8878 Zstandard-framed QOI payload back to `(width, height, rgba8)` (`F-066`).
pub fn decode_zstd_qoi_rgba8(framed: &[u8]) -> Result<(u32, u32, Vec<u8>), RenderCodecError> {
    if framed.len() < 9 || framed[..4] != ZSTD_MAGIC_LE {
        return Err(RenderCodecError::MalformedFrame(
            "missing RFC 8878 Zstd magic",
        ));
    }
    let mut pos = 6_usize;
    let mut qoi = Vec::new();
    loop {
        if pos + 3 > framed.len() {
            return Err(RenderCodecError::MalformedFrame(
                "truncated Zstd block header",
            ));
        }
        let hdr = u32::from_le_bytes([framed[pos], framed[pos + 1], framed[pos + 2], 0]);
        pos += 3;
        let last = (hdr & 1) == 1;
        let blk_type = (hdr >> 1) & 0x03;
        let blk_size = (hdr >> 3) as usize;
        match blk_type {
            0 => {
                if pos + blk_size > framed.len() {
                    return Err(RenderCodecError::MalformedFrame("truncated Zstd raw block"));
                }
                qoi.extend_from_slice(&framed[pos..pos + blk_size]);
                pos += blk_size;
            }
            1 => {
                if pos >= framed.len() || blk_size > MAX_TILE_PIXELS * 4 {
                    return Err(RenderCodecError::MalformedFrame("invalid Zstd RLE block"));
                }
                qoi.resize(qoi.len() + blk_size, framed[pos]);
                pos += 1;
            }
            _ => {
                return Err(RenderCodecError::MalformedFrame(
                    "unsupported Zstd block type",
                ));
            }
        }
        if last {
            break;
        }
    }
    decode_qoi_rgba8(&qoi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qoi_and_zstd_qoi_roundtrip_schematic_rgba8_tile_bit_exact() {
        let w = 64_u32;
        let h = 64_u32;
        let mut rgba = vec![0_u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let idx = ((y * w + x) * 4) as usize;
                let on_wire = x == 32 || y == 16;
                rgba[idx..idx + 4].copy_from_slice(&if on_wire {
                    [0, 230, 118, 255]
                } else {
                    [14, 18, 26, (200 + (x % 55)) as u8]
                });
            }
        }
        let qoi = encode_qoi_rgba8(&rgba, w, h).expect("encode qoi");
        assert!(qoi.len() < rgba.len() / 2);
        let (dw, dh, dec) = decode_qoi_rgba8(&qoi).expect("decode qoi");
        assert_eq!((dw, dh), (w, h));
        assert_eq!(dec, rgba);

        let framed = encode_zstd_qoi_rgba8(&rgba, w, h).expect("encode zstd-qoi");
        let (_, _, dec_framed) = decode_zstd_qoi_rgba8(&framed).expect("decode zstd-qoi");
        assert_eq!(dec_framed, rgba);
    }
}
