// Module: engine/crates/signal-em3d/src/lib.rs — 3D Full-Wave PEEC/FDTD, Passive Vector Fitting, 224G PAM4 COM & TDR (I-04)
// Invariants: #![forbid(unsafe_code)]; enforces S-parameter passivity sigma_max(S(jw)) <= 1.0 (F-032).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod sparam_com;
pub mod via_return_rf;

pub use sparam_com::{
    Pam4SerdesComResult, TwoPortSPoint, enforce_passivity_singular_values,
    evaluate_pam4_serdes_com, extract_lossy_channel_sparams, run_serdes_224g_em3d_rpc,
};
pub use via_return_rf::{
    EmFieldSample, RfWilkinsonAndPatchSpec, TdrSpatialStep,
    compute_split_plane_detour_inductance_nh, compute_via_stub_notch_and_backdrill,
    peel_tdr_impedance_profile, sample_em3d_field_frame, synthesize_rf_microstrip_wilkinson_patch,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Em3dError {
    #[error("missing fencing authority on 3D EM / 224G SerDes extraction")]
    MissingFencing,
    #[error("invalid 3D EM frequency sweep or SerDes baud rate: {0}")]
    InvalidFrequencySweep(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_geom::Point2Nm;
    use signal_proto::v1::{FencingAuthority, RunSerdes224gEm3dRequest};

    #[test]
    fn enforces_schur_passivity_and_computes_224g_pam4_com() {
        let mut pts = vec![TwoPortSPoint {
            freq_hz: 56.0e9,
            s11_re: 0.4,
            s11_im: 0.0,
            s21_re: 0.85, // |s11 + s21| = 1.25 > 1.0 active violation!
            s21_im: 0.0,
        }];
        let clamped = enforce_passivity_singular_values(&mut pts);
        assert_eq!(clamped, 1);
        assert!(pts[0].max_singular_value() <= 1.0);

        let resp = run_serdes_224g_em3d_rpc(&RunSerdes224gEm3dRequest {
            fencing: Some(FencingAuthority {
                org_id: "o1".into(),
                project_id: "p1".into(),
                fencing_epoch: 1,
                lease_holder_pod_id: "pod-1".into(),
                issued_at_utc_ms: 1,
                expires_at_utc_ms: 999,
            }),
            branch_id: "main".into(),
            channel_id: "QSFP_DD_TX0".into(),
            baud_rate_hz: 112_000_000_000, // 112 GBaud PAM4 = 224 Gbps!
        })
        .expect("224g com");
        assert!(resp.vector_fit_passive_causal);
        assert!(resp.channel_operating_margin_db > 3.0);
    }

    #[test]
    fn verifies_via_backdrill_return_detour_rf_wilkinson_and_tdr_layer_peeling() {
        let (f_notch, drill_nm) = compute_via_stub_notch_and_backdrill(2_000_000, 3.8, 28.0e9);
        assert!(f_notch < 25.0e9 && drill_nm > 1_000_000);

        let dl_nh = compute_split_plane_detour_inductance_nh(12_000_000, 150_000, 100_000);
        assert!(dl_nh > 5.0);

        // 0.30 mm trace over 0.16 mm FR4 (er=4.3) ~= 50 Ohm microstrip
        let rf = synthesize_rf_microstrip_wilkinson_patch(300_000, 160_000, 4.3, 2.45e9);
        assert!((rf.microstrip_z0_ohm - 50.0).abs() < 4.0);
        assert!((rf.wilkinson_isolation_r_ohm - 2.0 * rf.microstrip_z0_ohm).abs() < 1e-6);

        let fields = sample_em3d_field_frame(
            Point2Nm::new(0, 0),
            Point2Nm::new(10_000_000, 0),
            5_000_000,
            0.25,
            16,
        );
        assert_eq!(fields.len(), 16);

        // 50 Ohm -> 75 Ohm step has Gamma = (75 - 50)/(75 + 50) = +0.20
        let peeled = peel_tdr_impedance_profile(
            50.0,
            &[0.20],
            Point2Nm::new(0, 0),
            Point2Nm::new(10_000_000, 0),
        );
        assert_eq!(peeled[0].impedance_mohm, 75_000);
    }
}
