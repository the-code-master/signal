// Module: engine/crates/signal-pod/src/render/mod.rs — Dual-Plane Viewport Renderer, QOI/Zstd Codec & Cloud Clipboard
// Invariants: Zero raw geometry egress in Mode A; lossless Zstd-QOI dirty tiles + 6-DOF SpaceMouse camera integration.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/07-security-encryption-dlp-and-sftp.md
pub mod atlas_mux;
pub mod cloud_clipboard;
pub mod qoi_tile;

pub use atlas_mux::{AtlasPaneRect, MultiPaneViewportAtlas, negotiate_dual_plane};
pub use cloud_clipboard::{ClipboardError, CloudClipboardSealer};
pub use qoi_tile::{
    RenderCodecError, SETTLING_FRAME_DELAY_MS, decode_qoi_rgba8, decode_zstd_qoi_rgba8,
    encode_qoi_rgba8, encode_zstd_qoi_rgba8,
};

use signal_geom::Camera6Dof;
use signal_proto::v1::ViewportInputEvent;

/// Applies a streamed `ViewportInputEvent` (including optional 6-DOF SpaceMouse deltas) to a `Camera6Dof`.
pub fn apply_viewport_input_to_camera(camera: &mut Camera6Dof, ev: &ViewportInputEvent) {
    if let Some(sm) = &ev.spacemouse_delta {
        camera.apply_spacemouse_milli(
            sm.tx_milli,
            sm.ty_milli,
            sm.tz_milli,
            sm.rx_milli_rad,
            sm.ry_milli_rad,
            sm.rz_milli_rad,
        );
    }
}
