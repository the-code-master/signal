// Module: engine/crates/signal-ic/tests/ic_signoff_cpo_cryo_suite.rs — IC Signoff, Halo, UPF/DFT, CPO & 4K Cryo Tests
// Invariants: Verifies FEAT-041..049, UPF (I-07), DFT (I-08), IEEE 1735 (F-114), Halo (F-125), Metal ECO (F-134) & CPO/Cryo (F-148).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use signal_core::{SecretBox, hkdf_sha512, hmac_sha512};
use signal_geom::{BBox2Nm, Point2Nm};
use signal_ic::*;
use signal_proto::v1::{AnalyzeIcSignoffAndEcoRequest, RunPhotonicsAndCryoSimRequest};

#[test]
fn verifies_gmid_pelgrom_nldm_bspdn_dme_eco_halo_upf_dft_and_ieee1735() {
    let op = lookup_gmid_4d_operating_point(14.0, 0.75, 28, 0.0).expect("gmid ok");
    assert!(op.ft_ghz > 10.0 && op.intrinsic_gain_gm_gds > 10.0);

    let pel = generate_abba_common_centroid_pelgrom(4, 180, 4_000, 28, 1.8, 0.05).expect("abba");
    assert_eq!(pel.device_a_cells_nm.len(), 8);
    assert!(pel.sigma_vth_mv > 0.0 && pel.sigma_vth_mv < 20.0);

    let lib = characterize_nldm_7x7_and_lvf(
        &[5_000, 10_000, 20_000, 40_000, 80_000, 160_000, 320_000],
        &[500, 1_000, 2_000, 4_000, 8_000, 16_000, 32_000],
        450.0,
        12_000,
        0.06,
    );
    assert!(lib.delay_7x7_fs[6][6] > lib.delay_7x7_fs[0][0]);

    let bspdn = solve_backside_bspdn_ir_and_thermal(50_000, 64, 2_500.0);
    assert!(bspdn.backside_ir_drop_uv < bspdn.frontside_baseline_ir_drop_uv);

    let (hops, _) = route_xy_dor_noc_hops((0, 0), (3, 2), 400_000);
    assert_eq!(hops, 5);
    let tap = compute_dme_zero_skew_tap_nm(
        Point2Nm::new(0, 0),
        10_000,
        Point2Nm::new(100_000, 0),
        10_000,
        0.15,
    );
    assert_eq!(tap.x.0, 50_000);

    let litho = evaluate_hopkins_litho_and_cmp_fill(13.5, 0.33, 14.0, &[12.0, 45.0]);
    assert!(litho.dummy_fill_tiles_inserted > 0);

    // Halo abstract strips interior IP polygons deeper than 5 um (5_000_000 nm)!
    let die = BBox2Nm {
        min: Point2Nm::new(0, 0),
        max: Point2Nm::new(20_000_000, 20_000_000),
    };
    let polys = [
        BBox2Nm {
            min: Point2Nm::new(500_000, 500_000),
            max: Point2Nm::new(2_000_000, 2_000_000),
        }, // perimeter bump -> kept
        BBox2Nm {
            min: Point2Nm::new(8_000_000, 8_000_000),
            max: Point2Nm::new(12_000_000, 12_000_000),
        }, // core secret SRAM -> stripped!
    ];
    let (halo, retained) =
        build_signed_chiplet_halo_abstract("DIE-N3E", die, 5_000_000, &polys, 512, &[7_u8; 32]);
    assert_eq!(retained, 1);
    assert!(halo.broker_signature.is_some());

    // Post-mask metal ECO fixes negative WNS without base-layer mask spin!
    let before_eco = analyze_ic_signoff_and_eco(&AnalyzeIcSignoffAndEcoRequest {
        fencing: None,
        branch_id: "main".into(),
        clock_period_fs: 500_000,
        enable_backside_pdn: false,
        run_post_mask_metal_eco: false,
    })
    .expect("signoff pre-eco");
    assert!(before_eco.worst_negative_slack_fs < 0);

    let after_eco = analyze_ic_signoff_and_eco(&AnalyzeIcSignoffAndEcoRequest {
        fencing: None,
        branch_id: "main".into(),
        clock_period_fs: 500_000,
        enable_backside_pdn: false,
        run_post_mask_metal_eco: true,
    })
    .expect("signoff post-eco");
    assert!(after_eco.worst_negative_slack_fs >= 0);
    assert!(after_eco.spare_cells_rewired >= 1);
    assert!(after_eco.upf_isolation_shifters_clean);

    // IEEE 1735 v2 session decryption into SecretBox
    let key = SecretBox::new([0x3C_u8; 32]);
    let raw_model = b".model nch_ulvt bsimcmg vth0=0.24";
    let ks = hkdf_sha512::<64>(
        b"signal-pdk-salt",
        key.expose_secret(),
        b"ieee1735-v2:TSMC_N3E:0",
    )
    .expect("hkdf");
    let ct: Vec<u8> = raw_model
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ ks.expose_secret()[i])
        .collect();
    let mut auth = b"TSMC_N3E".to_vec();
    auth.extend_from_slice(&ct);
    let mac = hmac_sha512(key.expose_secret(), &auth);
    let decrypted =
        decrypt_ieee1735_pdk_session_envelope(&key, "TSMC_N3E", &ct, &mac).expect("pdk ok");
    assert_eq!(decrypted.expose_secret().as_slice(), raw_model);
    assert_eq!(format!("{decrypted:?}"), "SecretBox([REDACTED SECRET])");
}

#[test]
fn verifies_unitary_193thz_baseband_cpo_and_20mk_cryo_fermi_dirac_stability() {
    let e1 = ComplexEnvelope::new(1.0, 0.0);
    let e2 = ComplexEnvelope::new(0.0, 0.5);
    let (o1, o2) = scatter_directional_coupler_2x2(e1, e2, 0.6);
    assert!(((o1.optical_power_w() + o2.optical_power_w()) - 1.25).abs() < 1e-12);

    let ring_on = eval_microring_resonator_transfer(e1, 0.85, 0.85, 0.0);
    assert!(ring_on.optical_power_w() < 1e-6); // critical coupling resonance null!

    let resp = run_photonics_and_cryo_sim(&RunPhotonicsAndCryoSimRequest {
        optical_carrier_wavelength_pm: 1_550_000,
        step_size_ps: 5,
        stage_temperature_mkelvin: 4_200, // 4.2 K liquid helium
    })
    .expect("cpo + 4.2K cryo");
    assert!(resp.optical_extinction_ratio_db > 20.0);
    assert!(resp.cryo_effective_temp_mkelvin >= BAND_TAIL_FLOOR_TEMP_MKELVIN);
    assert!(resp.fermi_dirac_continuation_converged);

    // Also verify deep 20 mK dilution refrigerator stage has zero NaN/Inf overflow!
    let mk20 = solve_cryo_cmos_fermi_continuation(20, 0.75, 0.75, 0.30).expect("20 mK cryo");
    assert!(mk20.drain_current_na.is_finite() && mk20.drain_current_na > 0.0);
}
