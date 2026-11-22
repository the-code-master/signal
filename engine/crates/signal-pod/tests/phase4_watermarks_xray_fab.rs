// Module: engine/crates/signal-pod/tests/phase4_watermarks_xray_fab.rs — Tardos/Constellation, X-Ray & Fab Writer Tests
// Invariants: Verifies Tardos leaker accusation > innocent score, FEAT-096 O(1) X-Ray mask, and RAM-only Gerber/IPC-2581 writers.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md

use signal_geom::Point2Nm;
use signal_pod::fab_writers::{
    FabDrillHit, FabTrackPrimitive, write_excellon_drill_in_memory, write_gerber_x3_in_memory,
    write_ipc2581_revc_xml_in_memory, write_odb_plusplus_manifest_in_memory,
};
use signal_pod::render_mask::GpuLayerAndXrayUniform;
use signal_pod::watermark::{
    build_constellation_watermark, embed_tardos_rgba8_tile, generate_tardos_watermark,
    score_tardos_accusation,
};

#[test]
fn tardos_spread_spectrum_identifies_leaking_session_and_constellation_shifts_are_deterministic() {
    let seed = [0x91_u8; 32];
    let leaker = generate_tardos_watermark(&seed, "sess-mallory", 256);
    let innocent = generate_tardos_watermark(&seed, "sess-alice", 256);

    let clean = vec![128_u8; 256 * 4 * 4];
    let mut pirated = clean.clone();
    embed_tardos_rgba8_tile(&mut pirated, &leaker.user_codeword_bits);

    let s_mallory = score_tardos_accusation(&pirated, &clean, &leaker);
    let s_alice = score_tardos_accusation(&pirated, &clean, &innocent);
    assert!(
        s_mallory > s_alice + 25.0,
        "Mallory score {s_mallory:.2} must decisively exceed Alice {s_alice:.2}"
    );

    let (meta, shifts) = build_constellation_watermark(
        &seed,
        "wm-epoch-4",
        "tile-0-0",
        &["REF_U1".to_string(), "VIA_C12".to_string()],
        256,
    );
    assert_eq!(meta.constellation_anchor_count, 2);
    assert_eq!(shifts.len(), 2);
    assert_ne!(shifts[0].silkscreen_kerning_px, 0);
}

#[test]
fn xray_layer_mask_and_in_memory_gerber_x3_excellon_odb_ipc2581_writers_verified() {
    let mask = GpuLayerAndXrayUniform::ALL_LAYERS_DEFAULT
        .toggle_layer(3)
        .with_xray_net(Some(42));
    assert_eq!(mask.evaluate_primitive_alpha_u8(3, 42), 0); // Layer 3 toggled off
    assert_eq!(mask.evaluate_primitive_alpha_u8(0, 42), 255); // Target X-Ray net full bright
    assert_eq!(mask.evaluate_primitive_alpha_u8(0, 99), 36); // Other nets dimmed

    let tracks = [FabTrackPrimitive {
        start_nm: Point2Nm::new(0, 0),
        end_nm: Point2Nm::new(5_000_000, 2_000_000),
        width_nm: 150_000,
    }];
    let drills = [FabDrillHit {
        pos_nm: Point2Nm::new(2_500_000, 1_000_000),
        diameter_nm: 250_000,
    }];

    let gbr = write_gerber_x3_in_memory("Copper,L1,Top", "Single", &tracks);
    assert!(String::from_utf8_lossy(&gbr).contains("%TF.FileFunction,Copper,L1,Top*%"));

    let drl = write_excellon_drill_in_memory(&drills);
    assert!(String::from_utf8_lossy(&drl).contains("METRIC,TZ"));

    let odb = write_odb_plusplus_manifest_in_memory("BOARD_A", &["TOP", "GND"], &tracks);
    assert!(String::from_utf8_lossy(&odb).contains("ODB++_ARCHIVE_V8"));

    let ipc = write_ipc2581_revc_xml_in_memory("PROJ_A", &tracks, &drills);
    assert!(String::from_utf8_lossy(&ipc).contains("<IPC-2581 revision=\"C\""));
}
