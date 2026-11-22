// Module: engine/crates/signal-ic/src/mpw_shuttle.rs — Open-Silicon TinyTapeout / ChipIgnite MPW Shuttle Precheck (FEAT-115)
// Invariants: Enforces Sky130 / GF180MCU / IHP SG13G2 tile dimensions, pin-ring harness pitch, metal density & antenna rules.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use crate::IcError;
use signal_core::sha512_256_hex;
use signal_proto::v1::{
    OpenSiliconPdkKind, ValidateMpwShuttleTileRequest, ValidateMpwShuttleTileResponse,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShuttleSubmissionMetrics {
    pub drc_violations: u32,
    pub lvs_mismatches: u32,
    pub max_gate_antenna_ratio: f64,
    pub metal_density_pct: f64,
    pub io_pin_pitch_nm: i64,
}

fn pdk_slot_limits_and_antenna(
    pdk: OpenSiliconPdkKind,
    slot_preset: &str,
) -> Result<(i64, i64, i64, f64), IcError> {
    let is_2x2 = slot_preset.trim().eq_ignore_ascii_case("2x2");
    let is_chipignite = slot_preset.trim().eq_ignore_ascii_case("CHIPIGNITE");
    match pdk {
        OpenSiliconPdkKind::SkywaterSky130 => {
            let (w_nm, h_nm) = if is_chipignite {
                (2_920_000, 3_520_000)
            } else if is_2x2 {
                (330_000, 220_000)
            } else {
                (160_000, 100_000)
            };
            Ok((w_nm, h_nm, 3_400, 400.0))
        }
        OpenSiliconPdkKind::Gf180mcu => {
            let (w_nm, h_nm) = if is_2x2 {
                (500_000, 500_000)
            } else {
                (250_000, 250_000)
            };
            Ok((w_nm, h_nm, 5_600, 350.0))
        }
        OpenSiliconPdkKind::IhpSg13g2 => {
            let (w_nm, h_nm) = if is_2x2 {
                (400_000, 300_000)
            } else {
                (200_000, 150_000)
            };
            Ok((w_nm, h_nm, 2_800, 300.0))
        }
        OpenSiliconPdkKind::Unspecified => Err(IcError::UnsupportedOpenSiliconPdk),
    }
}

/// Validates an Open-Silicon Multi-Project Wafer (`TinyTapeout`/`ChipIgnite`) tile against Sky130 / GF180MCU / IHP SG13G2 rules (`FEAT-115`).
pub fn validate_mpw_shuttle_tile(
    req: &ValidateMpwShuttleTileRequest,
    metrics: ShuttleSubmissionMetrics,
) -> Result<ValidateMpwShuttleTileResponse, IcError> {
    let pdk = OpenSiliconPdkKind::try_from(req.pdk).unwrap_or(OpenSiliconPdkKind::Unspecified);
    let (max_w_nm, max_h_nm, harness_pitch_nm, max_antenna) =
        pdk_slot_limits_and_antenna(pdk, &req.tile_slot_preset)?;

    let fits_slot = req.die_width_nm > 0
        && req.die_height_nm > 0
        && req.die_width_nm <= max_w_nm
        && req.die_height_nm <= max_h_nm;
    let density_ok = (30.0..=70.0).contains(&metrics.metal_density_pct);
    let antenna_ok =
        metrics.max_gate_antenna_ratio >= 0.0 && metrics.max_gate_antenna_ratio <= max_antenna;
    let harness_aligned =
        metrics.io_pin_pitch_nm > 0 && metrics.io_pin_pitch_nm % harness_pitch_nm == 0;
    let precheck_clean = fits_slot
        && density_ok
        && antenna_ok
        && harness_aligned
        && metrics.drc_violations == 0
        && metrics.lvs_mismatches == 0;

    let mut payload = Vec::with_capacity(96);
    payload.extend_from_slice(req.project_id.as_bytes());
    payload.extend_from_slice(&req.pdk.to_le_bytes());
    payload.extend_from_slice(&req.die_width_nm.to_le_bytes());
    payload.extend_from_slice(&req.die_height_nm.to_le_bytes());
    payload.extend_from_slice(&metrics.metal_density_pct.to_bits().to_le_bytes());
    let digest = sha512_256_hex(&payload);

    Ok(ValidateMpwShuttleTileResponse {
        precheck_drc_lvs_antenna_clean: precheck_clean,
        metal_density_pct: metrics.metal_density_pct,
        pin_ring_harness_aligned: harness_aligned,
        shuttle_submission_digest: digest,
    })
}
