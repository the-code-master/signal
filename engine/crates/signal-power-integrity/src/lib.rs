// Module: engine/crates/signal-power-integrity/src/lib.rs — PDN, 3D Geodesic Creepage, IR-Drop, BMS, FOC & Thermal
// Invariants: #![forbid(unsafe_code)]; exact 3D surface creepage across slots/ribs & conservative electrothermal solvers.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod bms_foc_thermal;
pub mod hotloop_bode_seq;
pub mod pdn_creepage_ir;

pub use bms_foc_thermal::{
    BmsPackStepResult, FocSvpwmState, HeatsinkThermalSpec, simulate_bms_randles_pack_step,
    solve_foc_park_clarke_svpwm, solve_heatsink_airflow_tj_mk,
};
pub use hotloop_bode_seq::{
    HotLoopMillerAudit, RailRampSpec, RailSequencingAudit, SicGanHotLoopInput,
    Type3CompensatorSpec, audit_sic_gan_hot_loop, synthesize_type3_k_factor_compensator,
    verify_power_rail_sequencing,
};
pub use pdn_creepage_ir::{
    DecapModel, InsulatingBarrierFeature, analyze_pdn_and_creepage_rpc,
    compute_3d_surface_geodesic_creepage_nm, solve_electrothermal_dc_ir_drop,
    synthesize_pdn_decoupling_bank, verify_3d_geodesic_creepage_rpc,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PowerIntegrityError {
    #[error("missing fencing authority on power-integrity / HV creepage verification")]
    MissingFencing,
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_geom::Point2Nm;
    use signal_proto::v1::AnalyzePdnAndCreepageRequest;

    #[test]
    fn pdn_decap_placer_3d_creepage_rib_and_electrothermal_ir_drop_verified() {
        let resp = analyze_pdn_and_creepage_rpc(&AnalyzePdnAndCreepageRequest {
            branch_id: "main".into(),
            power_net: "VDD_CORE".into(),
            high_voltage_net: "HV_BUS_800V".into(),
            target_impedance_uohm: 15_000, // 15 mOhm target
            required_creepage_nm: 8_000_000,
        });
        assert!(resp.pdn_below_target_mask);
        assert!(resp.geodesic_3d_creepage_nm >= 9_500_000);
        assert!(resp.max_busbar_ir_drop_uv > 0);

        // Narrow slot (< 1.0 mm) is ignored by IEC 60664-1, while 3 mm tall rib adds 6 mm!
        let plain = compute_3d_surface_geodesic_creepage_nm(
            Point2Nm::new(0, 0),
            Point2Nm::new(4_000_000, 0),
            &[InsulatingBarrierFeature {
                width_nm: 500_000,
                height_or_depth_nm: -2_000_000,
            }],
            1_000_000,
        );
        assert_eq!(plain, 4_000_000);
    }

    #[test]
    fn sic_gan_miller_type3_compensator_bms_foc_svpwm_and_sequencing_verified() {
        let hl = audit_sic_gan_hot_loop(&SicGanHotLoopInput {
            loop_area_sq_mm: 4.0,
            plane_spacing_mm: 0.15,
            di_dt_a_per_ns: 2.5,
            dv_dt_v_per_ns: 50.0,
            c_gd_pf: 15.0,
            r_gate_off_ohm: 1.0,
            vgs_off_bias_v: -3.0,
            vth_min_v: 1.6,
        });
        assert!(hl.shoot_through_safe);

        let comp = synthesize_type3_k_factor_compensator(50_000.0, 70.0, 0.25, 10_000.0);
        assert!(comp.k_factor > 1.0 && comp.f_zero_hz < 50_000.0 && comp.f_pole_hz > 50_000.0);

        let bms = simulate_bms_randles_pack_step(16, 2, 0.80, 20.0, 1.0, 298.15);
        assert!(bms.pack_terminal_voltage_v > 55.0 && !bms.arrhenius_thermal_runaway_warning);

        let foc =
            solve_foc_park_clarke_svpwm([0.0, 8.66025, -8.66025], 0.0, [0.0, 12.0], 48.0, 4, 0.05);
        assert!(foc.i_q > 9.5 && foc.electromagnetic_torque_nm > 2.5);
        assert!(foc.pwm_duties_abc.iter().all(|&d| (0.0..=1.0).contains(&d)));

        let base_sink = HeatsinkThermalSpec {
            r_theta_jc_k_per_w: 0.4,
            tim_thickness_mm: 0.1,
            tim_k_w_per_mk: 3.5,
            contact_area_sq_mm: 400.0,
            heatsink_natural_r_k_per_w: 2.2,
            airflow_cfm: 0.0,
        };
        let tj_calm = solve_heatsink_airflow_tj_mk(25.0, 298_150, &base_sink);
        let tj_fan = solve_heatsink_airflow_tj_mk(
            25.0,
            298_150,
            &HeatsinkThermalSpec {
                airflow_cfm: 20.0,
                ..base_sink
            },
        );
        assert!(tj_fan < tj_calm);

        let seq = verify_power_rail_sequencing(
            &[
                RailRampSpec {
                    rail_name: "VDD_CORE".into(),
                    start_delay_ms: 0.0,
                    ramp_duration_ms: 1.0,
                    nominal_v: 0.85,
                    bulk_cap_uf: 100.0,
                },
                RailRampSpec {
                    rail_name: "VDD_IO".into(),
                    start_delay_ms: 2.0,
                    ramp_duration_ms: 1.5,
                    nominal_v: 3.3,
                    bulk_cap_uf: 47.0,
                },
            ],
            0.6,
        );
        assert!(seq.monotonic_order_valid && !seq.esd_backpower_risk_detected);
    }
}
