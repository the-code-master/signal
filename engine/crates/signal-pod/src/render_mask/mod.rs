// Module: engine/crates/signal-pod/src/render_mask/mod.rs — 64-Layer GPU Bitmask & X-Ray Net Isolation (FEAT-096)
// Invariants: O(1) bitwise layer/net visibility filter without geometry buffer rebuilds during 120 FPS pan/zoom.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/17-extended-feature-specifications-ui-and-rust.md

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpuLayerAndXrayUniform {
    pub visible_layer_mask: u64,
    pub xray_isolated_net_id: Option<u32>,
    pub background_dim_alpha_u8: u8,
}

impl GpuLayerAndXrayUniform {
    pub const ALL_LAYERS_DEFAULT: Self = Self {
        visible_layer_mask: u64::MAX,
        xray_isolated_net_id: None,
        background_dim_alpha_u8: 36,
    };

    pub const fn with_xray_net(mut self, net_id: Option<u32>) -> Self {
        self.xray_isolated_net_id = net_id;
        self
    }

    pub const fn toggle_layer(mut self, layer_idx: u8) -> Self {
        if layer_idx < 64 {
            self.visible_layer_mask ^= 1_u64 << layer_idx;
        }
        self
    }

    /// Evaluates per-primitive alpha (`0..=255`) in `O(1)` for GPU shader uniform parity (`FEAT-096`).
    pub const fn evaluate_primitive_alpha_u8(&self, layer_idx: u8, net_id: u32) -> u8 {
        if layer_idx >= 64 || (self.visible_layer_mask & (1_u64 << layer_idx)) == 0 {
            return 0;
        }
        match self.xray_isolated_net_id {
            Some(target) if target == net_id => 255,
            Some(_) => self.background_dim_alpha_u8,
            None => 255,
        }
    }
}
