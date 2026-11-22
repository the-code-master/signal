// Module: engine/crates/signal-ic/src/digital_sta_eco_ucie.rs — Floorplan/NoC, DME CTS, STA/Metal-ECO, UCIe Halo & Litho/CMP
// Invariants: Signed *.halo.gds.sig strips core IP deeper than 5 um (F-125); post-mask ECO rewires M1..M4 spare cells (F-134).
// Spec: architecture/03-rust-eda-engine.md, architecture/15-application-security-and-project-isolation.md
use signal_core::{HybridSignature, SecretBox, sha512_256_hex};
use signal_geom::{BBox2Nm, Point2Nm};
use signal_proto::v1::{HybridPqcSignature, SignedChipletHaloManifest};

/// Computes XY Dimension-Order Routing (DOR) NoC hop count and wirelength for a macro floorplan (`FEAT-045`).
pub fn route_xy_dor_noc_hops(
    src_tile: (u16, u16),
    dst_tile: (u16, u16),
    cycle_per_hop_fs: i64,
) -> (u32, i64) {
    let dx = u32::from(src_tile.0.abs_diff(dst_tile.0));
    let dy = u32::from(src_tile.1.abs_diff(dst_tile.1));
    let hops = dx + dy;
    // Router pipeline = 2 cycles per hop + 1 injection cycle
    let latency_fs = i64::from(hops * 2 + 1).saturating_mul(cycle_per_hop_fs.max(1));
    (hops, latency_fs)
}

/// Deferred-Merge Embedding (DME) exact zero-skew balance point between two clock sinks (`FEAT-046`).
pub fn compute_dme_zero_skew_tap_nm(
    sink_a_nm: Point2Nm,
    delay_a_fs: i64,
    sink_b_nm: Point2Nm,
    delay_b_fs: i64,
    fs_per_um: f64,
) -> Point2Nm {
    let manhattan_nm =
        (sink_a_nm.x.0 - sink_b_nm.x.0).abs() + (sink_a_nm.y.0 - sink_b_nm.y.0).abs();
    if manhattan_nm == 0 {
        return sink_a_nm;
    }
    let skew_fs = (delay_b_fs - delay_a_fs) as f64;
    let wire_delay_fs = ((manhattan_nm as f64) * 1e-3) * fs_per_um.max(0.1);
    let ratio = ((wire_delay_fs + skew_fs) / (2.0 * wire_delay_fs)).clamp(0.0, 1.0);
    Point2Nm::new(
        sink_a_nm.x.0 + (((sink_b_nm.x.0 - sink_a_nm.x.0) as f64) * ratio).round() as i64,
        sink_a_nm.y.0 + (((sink_b_nm.y.0 - sink_a_nm.y.0) as f64) * ratio).round() as i64,
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaTimingPath {
    pub path_id: String,
    pub arrival_delay_fs: i64,
    pub setup_time_fs: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaAndMetalEcoReport {
    pub worst_negative_slack_fs: i64,
    pub total_negative_slack_fs: i64,
    pub spare_cells_rewired: u32,
}

/// Evaluates topological STA (`WNS`/`TNS`) and applies Post-Mask Spare-Cell Metal-Only IC ECO (`FEAT-047`, `F-134`).
pub fn evaluate_sta_and_post_mask_metal_eco(
    clock_period_fs: i64,
    paths: &[StaTimingPath],
    available_spare_cells: u32,
    run_metal_eco: bool,
) -> StaAndMetalEcoReport {
    let mut wns_fs = i64::MAX;
    let mut tns_fs = 0_i64;
    let mut spare_used = 0_u32;

    for p in paths {
        let mut slack = clock_period_fs - p.arrival_delay_fs - p.setup_time_fs;
        if slack < 0 && run_metal_eco && spare_used < available_spare_cells {
            // Rewiring M1..M4 into pre-placed spare buffer/NOR/NAND reduces RC + logic delay by ~95 ps (95_000 fs)
            let cells_needed = (((-slack + 94_999) / 95_000) as u32)
                .min(available_spare_cells.saturating_sub(spare_used))
                .max(1);
            spare_used += cells_needed;
            slack += i64::from(cells_needed) * 95_000;
        }
        wns_fs = wns_fs.min(slack);
        if slack < 0 {
            tns_fs = tns_fs.saturating_add(slack);
        }
    }
    if wns_fs == i64::MAX {
        wns_fs = clock_period_fs;
    }

    StaAndMetalEcoReport {
        worst_negative_slack_fs: wns_fs,
        total_negative_slack_fs: tns_fs,
        spare_cells_rewired: spare_used,
    }
}

/// Generates a signed `5 µm` (`5_000_000 nm`) Boundary-Halo Chiplet Abstract stripping interior core polygons (`FEAT-048`, `F-125`).
pub fn build_signed_chiplet_halo_abstract(
    chiplet_id: &str,
    die_outline_nm: BBox2Nm,
    halo_ring_width_nm: i64,
    polygons_nm: &[BBox2Nm],
    ucie_bump_count: u32,
    broker_signing_seed: &[u8; 32],
) -> (SignedChipletHaloManifest, usize) {
    let ring = halo_ring_width_nm.max(5_000_000); // >= 5 um perimeter ring
    let core_keepout = BBox2Nm {
        min: Point2Nm::new(
            die_outline_nm.min.x.0.saturating_add(ring),
            die_outline_nm.min.y.0.saturating_add(ring),
        ),
        max: Point2Nm::new(
            die_outline_nm.max.x.0.saturating_sub(ring),
            die_outline_nm.max.y.0.saturating_sub(ring),
        ),
    };

    let mut halo_payload = Vec::new();
    halo_payload.extend_from_slice(chiplet_id.as_bytes());
    let mut retained = 0_usize;
    for poly in polygons_nm {
        let strictly_interior = poly.min.x.0 >= core_keepout.min.x.0
            && poly.max.x.0 <= core_keepout.max.x.0
            && poly.min.y.0 >= core_keepout.min.y.0
            && poly.max.y.0 <= core_keepout.max.y.0;
        if !strictly_interior {
            retained += 1;
            halo_payload.extend_from_slice(&poly.min.x.0.to_le_bytes());
            halo_payload.extend_from_slice(&poly.min.y.0.to_le_bytes());
            halo_payload.extend_from_slice(&poly.max.x.0.to_le_bytes());
            halo_payload.extend_from_slice(&poly.max.y.0.to_le_bytes());
        }
    }

    let digest = sha512_256_hex(&halo_payload);
    let ed_seed = SecretBox::new(*broker_signing_seed);
    let mut pq_raw = *broker_signing_seed;
    pq_raw[0] ^= 0xA5;
    let pq_seed = SecretBox::new(pq_raw);
    let sig = HybridSignature::sign_deterministic(&ed_seed, &pq_seed, digest.as_bytes());

    (
        SignedChipletHaloManifest {
            chiplet_id: chiplet_id.to_string(),
            halo_ring_width_nm: ring,
            ucie_bump_count,
            halo_gds_blake3: digest,
            broker_signature: Some(HybridPqcSignature {
                ed25519_sig: sig.ed25519_sig.to_vec(),
                ml_dsa_65_sig: sig.ml_dsa_65_sig,
                key_id: "signal-federation-broker-halo-v1".to_string(),
                signed_at_utc_ms: 1_795_325_000_000,
            }),
        },
        retained,
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LithoCmpReport {
    pub rayleigh_half_pitch_nm: f64,
    pub aerial_image_contrast: f64,
    pub dummy_fill_tiles_inserted: u32,
}

/// Evaluates Hopkins optical aerial-image contrast & inserts CMP metal-density dummy fill tiles (`FEAT-049`).
pub fn evaluate_hopkins_litho_and_cmp_fill(
    wavelength_nm: f64,
    numerical_aperture: f64,
    target_half_pitch_nm: f64,
    window_metal_densities_pct: &[f64],
) -> LithoCmpReport {
    let k1 = 0.28;
    let res_limit = k1 * wavelength_nm / numerical_aperture.max(0.1);
    let pitch_ratio = (target_half_pitch_nm / res_limit.max(1.0)).clamp(0.4, 3.0);
    let contrast = (1.0 - (-1.4 * pitch_ratio).exp()).clamp(0.05, 0.99);

    let mut dummy_tiles = 0_u32;
    for &d in window_metal_densities_pct {
        if d < 25.0 {
            dummy_tiles += (((25.0 - d) * 4.0).ceil() as u32).max(1);
        }
    }
    LithoCmpReport {
        rayleigh_half_pitch_nm: res_limit,
        aerial_image_contrast: contrast,
        dummy_fill_tiles_inserted: dummy_tiles,
    }
}
