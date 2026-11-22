// Module: engine/crates/signal-pcb/src/lib.rs — High-Speed PCB PNS Router, Fiber-Weave, HDI & Signed Fab Profiles
// Invariants: #![forbid(unsafe_code)]; exact nm integer coordinates, signed fab profiles (F-135), minimal ECO (F-134).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod copper_hygiene;
pub mod meander_and_weave;
pub mod pkg_hdi_fab;
pub mod pns_and_bus;

pub use copper_hygiene::{
    CopperHygieneReport, CopperPourRegion, TeardropFilletPolygon, TraceJunctionCorner,
    audit_and_heal_copper_hygiene, build_pad_teardrop_fillet, is_acute_acid_trap_i128,
};
pub use meander_and_weave::{
    TromboneMeanderPlan, analyze_fiber_weave_and_backdrill_rpc, compute_fiber_weave_skew_fs,
    synthesize_trombone_meander,
};
pub use pkg_hdi_fab::{
    HdiMicroviaAudit, SignedFabCapabilityProfile, apply_minimal_perturbation_pcb_eco,
    compute_total_channel_flight_time_fs, validate_ipc2226_hdi_microvia,
};
pub use pns_and_bus::{
    PcbObstacle, PnsRouteOutcome, RoutedTrackSegment, route_pns_bus_trunk, route_pns_single_net,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PcbError {
    #[error("missing fencing authority on mutating PCB routing request")]
    MissingFencing,
    #[error("invalid PCB routing geometry: {0}")]
    InvalidRoutingGeometry(&'static str),
    #[error("unknown PCB fab capability vendor `{0}`")]
    UnknownFabVendor(String),
    #[error("tampered or invalid ML-DSA-65 + Ed25519 signature on fab profile `{0}`")]
    FabProfileSignatureInvalid(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::SecretBox;
    use signal_geom::{BBox2Nm, Point2Nm};
    use signal_proto::v1::{
        AnalyzeFiberWeaveAndBackdrillRequest, FencingAuthority, Point2dNm, RoutePnsBusTrunkRequest,
    };
    use std::collections::BTreeSet;

    #[test]
    fn pns_shoves_movable_traces_and_bus_trunk_routes_16_lanes_zero_drc() {
        let mut obs = vec![PcbObstacle {
            id: "via-movable-1".into(),
            bounds_nm: BBox2Nm::from_points(
                Point2Nm::new(2_000_000, -100_000),
                Point2Nm::new(2_200_000, 100_000),
            ),
            movable: true,
        }];
        let res = route_pns_single_net(
            "DDR_DQ0",
            Point2Nm::new(0, 0),
            Point2Nm::new(5_000_000, 0),
            100_000,
            100_000,
            &mut obs,
        )
        .expect("pns route");
        assert_eq!(res.shoved_obstacle_ids, vec!["via-movable-1".to_string()]);

        let nets: Vec<String> = (0..16).map(|i| format!("LA_{i:02}")).collect();
        let (trunk_resp, segs) = route_pns_bus_trunk(&RoutePnsBusTrunkRequest {
            fencing: Some(FencingAuthority {
                org_id: "o1".into(),
                project_id: "p1".into(),
                fencing_epoch: 1,
                lease_holder_pod_id: "pod-1".into(),
                issued_at_utc_ms: 1,
                expires_at_utc_ms: 999,
            }),
            branch_id: "main".into(),
            member_net_names: nets,
            trunk_waypoints_nm: vec![
                Point2dNm { x_nm: 0, y_nm: 0 },
                Point2dNm {
                    x_nm: 20_000_000,
                    y_nm: 0,
                },
            ],
            trace_width_nm: 90_000,
            spacing_nm: 90_000,
        })
        .expect("bus trunk");
        assert!(trunk_resp.routed_zero_drc);
        assert_eq!(segs.len(), 16);
    }

    #[test]
    fn trombone_meander_fiber_weave_10deg_mitigation_and_copper_hygiene_verified() {
        let m = synthesize_trombone_meander(
            Point2Nm::new(0, 0),
            Point2Nm::new(10_000_000, 0),
            100_000,
            14_000_000,
        )
        .expect("meander");
        assert!(m.lobe_spacing_nm >= 500_000);
        assert!((m.achieved_length_nm - 14_000_000).abs() <= 200_000);

        // 0 deg aligned with 1080 weave has much larger skew than 10 deg zig-zag!
        let skew_0 = compute_fiber_weave_skew_fs(100_000_000, 210_000, 420_000, 3.8, 0.45, 0.0);
        let skew_10 = compute_fiber_weave_skew_fs(100_000_000, 210_000, 420_000, 3.8, 0.45, 10.0);
        assert!(skew_0 > skew_10 * 5);

        let wb = analyze_fiber_weave_and_backdrill_rpc(
            &AnalyzeFiberWeaveAndBackdrillRequest {
                branch_id: "main".into(),
                diff_pair_name: "PCIE6_TX0".into(),
                data_rate_bps: 64_000_000_000,
            },
            2_400_000,
            400_000,
        );
        assert!(wb.recommended_backdrill_depth_nm > 1_500_000);

        let acute = TraceJunctionCorner {
            net_name: "N1".into(),
            p_prev: Point2Nm::new(1_000_000, 0),
            p_vertex: Point2Nm::new(0, 0),
            p_next: Point2Nm::new(1_000_000, 500_000), // ~26.5 deg < 90 deg!
        };
        assert!(is_acute_acid_trap_i128(&acute));
    }

    #[test]
    fn signed_fab_profiles_ipc2226_hdi_and_minimal_eco_preserve_untouched_copper() {
        let seed = SecretBox::new([0x55_u8; 32]);
        for vendor in [
            "JLCPCB",
            "PCBWAY",
            "EUROCIRCUITS",
            "SIERRA_CIRCUITS",
            "TTM_TECH",
        ] {
            let prof = SignedFabCapabilityProfile::builtin_signed(vendor, &seed).expect("vendor");
            assert!(prof.verify_authenticity(&seed).is_ok());
        }
        let hdi = validate_ipc2226_hdi_microvia(75_000, 100_000, 25_000, 3);
        assert!(hdi.ipc2226_aspect_compliant && hdi.barrel_reliability_pass);

        let existing = vec![
            RoutedTrackSegment {
                net_name: "KEEP_NET".into(),
                start_nm: Point2Nm::new(0, 0),
                end_nm: Point2Nm::new(1_000, 0),
                width_nm: 100_000,
            },
            RoutedTrackSegment {
                net_name: "DIRTY_NET".into(),
                start_nm: Point2Nm::new(0, 500),
                end_nm: Point2Nm::new(1_000, 500),
                width_nm: 100_000,
            },
        ];
        let dirty = BTreeSet::from(["DIRTY_NET".to_string()]);
        let (updated, kept) = apply_minimal_perturbation_pcb_eco(&existing, &dirty, Vec::new());
        assert_eq!(kept, 1);
        assert_eq!(updated[0].net_name, "KEEP_NET");
        assert!(compute_total_channel_flight_time_fs(45_000, 50_000_000, 3.7) > 300_000);
    }
}
