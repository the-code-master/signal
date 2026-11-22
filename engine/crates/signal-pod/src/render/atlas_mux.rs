// Module: engine/crates/signal-pod/src/render/atlas_mux.rs — Multi-Pane Viewport Atlas, Capability Negotiator & Monitor Mux
// Invariants: Single server-side atlas across all panes (F-067); <= 1 MB dirty tiles; 80 ms crisp settling frame (F-066, F-126, FEAT-092).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::render::qoi_tile::{RenderCodecError, SETTLING_FRAME_DELAY_MS, encode_zstd_qoi_rgba8};
use signal_core::sha512_256_hex;
use signal_proto::v1::{
    DirtyTileFrame, HudOverlayPrimitive, MotionStreamProfile, NegotiateDualPlaneRequest,
    NegotiateDualPlaneResponse, TileEncodingFormat,
};

const MAX_SECONDARY_MONITORS: u32 = 8;

/// Negotiates Dual-Plane motion codec (`HEVC 4:4:4` -> `AV1 Dual-Luma` -> `Zstd-QOI` tiles) & transport (`F-126`).
pub fn negotiate_dual_plane(req: &NegotiateDualPlaneRequest) -> NegotiateDualPlaneResponse {
    let profile = if req.supports_hevc_444_hw {
        MotionStreamProfile::Hevc444Hw
    } else if req.supports_av1_hw {
        MotionStreamProfile::Av1DualLumaHw
    } else {
        MotionStreamProfile::TilesOnly
    };
    let transport = if req.supports_webtransport_h3 {
        "WEBTRANSPORT_H3"
    } else {
        "WSS_H2_FALLBACK"
    };
    NegotiateDualPlaneResponse {
        selected_motion_profile: profile as i32,
        selected_tile_format: TileEncodingFormat::ZstdQoiRgba8 as i32,
        settling_delay_ms: SETTLING_FRAME_DELAY_MS as u32,
        negotiated_transport: transport.to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasPaneRect {
    pub monitor_pane_index: u32,
    pub atlas_x_px: u32,
    pub atlas_y_px: u32,
    pub width_px: u32,
    pub height_px: u32,
}

/// Secondary monitor viewport multiplexer packing primary + pop-out viewports into one GPU/CPU atlas (`F-067`, `FEAT-092`).
#[derive(Debug)]
pub struct MultiPaneViewportAtlas {
    atlas_width_px: u32,
    atlas_height_px: u32,
    tile_size_px: u32,
    panes: Vec<AtlasPaneRect>,
    current_rgba: Vec<u8>,
    previous_rgba: Vec<u8>,
    frame_seq: u64,
    last_motion_ms: u64,
    watermark_key: Vec<u8>,
}

impl MultiPaneViewportAtlas {
    pub fn new(
        pane_width_px: u32,
        pane_height_px: u32,
        secondary_monitors: u32,
        tile_size_px: u32,
        watermark_key: Vec<u8>,
    ) -> Result<Self, RenderCodecError> {
        let clamped_sec = secondary_monitors.min(MAX_SECONDARY_MONITORS);
        let total_panes = 1 + clamped_sec;
        let atlas_width_px = pane_width_px.saturating_mul(total_panes);
        let atlas_height_px = pane_height_px;
        let total_px = (atlas_width_px as usize).saturating_mul(atlas_height_px as usize);
        if pane_width_px == 0 || pane_height_px == 0 || tile_size_px == 0 || total_px > 8192 * 4096
        {
            return Err(RenderCodecError::DimensionsExceeded(
                atlas_width_px,
                atlas_height_px,
            ));
        }

        let mut panes = Vec::with_capacity(total_panes as usize);
        for idx in 0..total_panes {
            panes.push(AtlasPaneRect {
                monitor_pane_index: idx,
                atlas_x_px: idx * pane_width_px,
                atlas_y_px: 0,
                width_px: pane_width_px,
                height_px: pane_height_px,
            });
        }

        Ok(Self {
            atlas_width_px,
            atlas_height_px,
            tile_size_px,
            panes,
            current_rgba: vec![0_u8; total_px * 4],
            previous_rgba: vec![255_u8; total_px * 4],
            frame_seq: 0,
            last_motion_ms: 0,
            watermark_key,
        })
    }

    pub fn panes(&self) -> &[AtlasPaneRect] {
        &self.panes
    }

    pub const fn dimensions_px(&self) -> (u32, u32) {
        (self.atlas_width_px, self.atlas_height_px)
    }

    pub fn record_motion_at(&mut self, timestamp_ms: u64) {
        self.last_motion_ms = timestamp_ms;
    }

    /// Blits RGBA8 rectangle into the pane corresponding to `monitor_pane_index`.
    pub fn blit_pane_rect(
        &mut self,
        monitor_pane_index: u32,
        local_x: u32,
        local_y: u32,
        w: u32,
        h: u32,
        rgba: &[u8],
    ) -> Result<(), RenderCodecError> {
        let Some(pane) = self
            .panes
            .iter()
            .find(|p| p.monitor_pane_index == monitor_pane_index)
            .copied()
        else {
            return Err(RenderCodecError::MalformedFrame(
                "invalid monitor_pane_index",
            ));
        };
        if local_x + w > pane.width_px
            || local_y + h > pane.height_px
            || rgba.len() != (w as usize) * (h as usize) * 4
        {
            return Err(RenderCodecError::BufferMismatch {
                width: w,
                height: h,
                actual: rgba.len(),
            });
        }
        for row in 0..h {
            let dst_y = pane.atlas_y_px + local_y + row;
            let dst_x = pane.atlas_x_px + local_x;
            let dst_off = ((dst_y * self.atlas_width_px + dst_x) as usize) * 4;
            let src_off = ((row * w) as usize) * 4;
            let row_bytes = (w as usize) * 4;
            self.current_rgba[dst_off..dst_off + row_bytes]
                .copy_from_slice(&rgba[src_off..src_off + row_bytes]);
        }
        Ok(())
    }

    /// Extracts and encodes only modified tiles across all multiplexed monitor panes (`F-066`, `FEAT-092`).
    pub fn flush_dirty_tiles(
        &mut self,
        now_ms: u64,
        hud_primitives: &[HudOverlayPrimitive],
    ) -> Result<Vec<DirtyTileFrame>, RenderCodecError> {
        let is_settling = now_ms.saturating_sub(self.last_motion_ms) >= SETTLING_FRAME_DELAY_MS;
        let mut frames = Vec::new();

        let panes = self.panes.clone();
        for pane in panes {
            let mut ty = 0;
            while ty < pane.height_px {
                let th = self.tile_size_px.min(pane.height_px - ty);
                let mut tx = 0;
                while tx < pane.width_px {
                    let tw = self.tile_size_px.min(pane.width_px - tx);
                    let ax = pane.atlas_x_px + tx;
                    let ay = pane.atlas_y_px + ty;

                    if self.is_tile_dirty(ax, ay, tw, th) {
                        let tile_rgba = self.copy_tile_and_commit(ax, ay, tw, th);
                        let compressed = encode_zstd_qoi_rgba8(&tile_rgba, tw, th)?;
                        self.frame_seq += 1;
                        let mut wm_input = self.watermark_key.clone();
                        wm_input.extend_from_slice(&self.frame_seq.to_be_bytes());
                        frames.push(DirtyTileFrame {
                            frame_seq: self.frame_seq,
                            format: TileEncodingFormat::ZstdQoiRgba8 as i32,
                            tile_x_px: tx,
                            tile_y_px: ty,
                            tile_width_px: tw,
                            tile_height_px: th,
                            compressed_rgba_bytes: compressed,
                            is_settling_lossless_frame: is_settling,
                            watermark_epoch_hmac: sha512_256_hex(&wm_input),
                            hud_primitives: if pane.monitor_pane_index == 0 && frames.is_empty() {
                                hud_primitives.to_vec()
                            } else {
                                Vec::new()
                            },
                            monitor_pane_index: pane.monitor_pane_index,
                        });
                    }
                    tx += self.tile_size_px;
                }
                ty += self.tile_size_px;
            }
        }
        Ok(frames)
    }

    fn is_tile_dirty(&self, ax: u32, ay: u32, tw: u32, th: u32) -> bool {
        let row_bytes = (tw as usize) * 4;
        for r in 0..th {
            let off = (((ay + r) * self.atlas_width_px + ax) as usize) * 4;
            if self.current_rgba[off..off + row_bytes] != self.previous_rgba[off..off + row_bytes] {
                return true;
            }
        }
        false
    }

    fn copy_tile_and_commit(&mut self, ax: u32, ay: u32, tw: u32, th: u32) -> Vec<u8> {
        let row_bytes = (tw as usize) * 4;
        let mut tile = vec![0_u8; row_bytes * (th as usize)];
        for r in 0..th {
            let off = (((ay + r) * self.atlas_width_px + ax) as usize) * 4;
            let dst = (r as usize) * row_bytes;
            tile[dst..dst + row_bytes].copy_from_slice(&self.current_rgba[off..off + row_bytes]);
            self.previous_rgba[off..off + row_bytes]
                .copy_from_slice(&self.current_rgba[off..off + row_bytes]);
        }
        tile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negotiates_dual_plane_and_multiplexes_secondary_monitor_dirty_tiles() {
        let resp = negotiate_dual_plane(&NegotiateDualPlaneRequest {
            collaborator_session_id: "sess_1".to_string(),
            supports_hevc_444_hw: false,
            supports_av1_hw: true,
            supports_webtransport_h3: true,
            secondary_monitor_count: 1,
            device_pixel_ratio_x100: 200,
        });
        assert_eq!(
            resp.selected_motion_profile,
            MotionStreamProfile::Av1DualLumaHw as i32
        );
        assert_eq!(resp.settling_delay_ms, 80);

        let mut atlas =
            MultiPaneViewportAtlas::new(128, 128, 1, 64, b"wm_secret".to_vec()).expect("atlas");
        assert_eq!(atlas.panes().len(), 2);
        let initial = atlas.flush_dirty_tiles(100, &[]).expect("initial flush");
        assert_eq!(initial.len(), 8); // 4 tiles on primary + 4 tiles on secondary
        assert!(initial[0].is_settling_lossless_frame);

        let patch = vec![42_u8; 16 * 16 * 4];
        atlas.record_motion_at(200);
        atlas
            .blit_pane_rect(1, 70, 10, 16, 16, &patch)
            .expect("blit");
        let delta = atlas.flush_dirty_tiles(220, &[]).expect("delta flush");
        assert_eq!(delta.len(), 1);
        assert_eq!(delta[0].monitor_pane_index, 1);
        assert!(!delta[0].is_settling_lossless_frame);
    }
}
