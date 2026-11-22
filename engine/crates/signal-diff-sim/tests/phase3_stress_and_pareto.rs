// Module: engine/crates/signal-diff-sim/tests/phase3_stress_and_pareto.rs — FEAT-102 Stress Ray & FEAT-103 NSGA-III Tests
// Invariants: Verifies exact earliest SOA/timing breakdown culprit & mutually non-dominated 5-objective Pareto frontier.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_diff_sim::{
    ComponentStressEnvelope, explore_pareto_frontier, find_stress_breaking_point, pareto_dominates,
};
use signal_proto::v1::{ExploreParetoFrontierRequest, FindStressBreakingPointRequest};

#[test]
fn ray_marching_pinpoints_earliest_soa_breaking_component() {
    let comps = vec![
        ComponentStressEnvelope {
            entity_id: "U_REG_ROBUST".into(),
            nominal_voltage_uv: 12_000_000,
            voltage_gain_from_supply: 1.0,
            abs_max_voltage_uv: 36_000_000,
            nominal_power_mw: 150.0,
            r_theta_ja_mk_per_w: 35_000.0,
            max_tj_mkelvin: 423_150,
            timing_slack_at_25c_ps: 500.0,
            timing_derate_ps_per_kelvin: 1.0,
        },
        ComponentStressEnvelope {
            entity_id: "C_IN_TANTALUM_FRAGILE".into(),
            nominal_voltage_uv: 12_000_000,
            voltage_gain_from_supply: 1.0,
            abs_max_voltage_uv: 16_000_000, // Fails at +4V surge!
            nominal_power_mw: 5.0,
            r_theta_ja_mk_per_w: 80_000.0,
            max_tj_mkelvin: 398_150,
            timing_slack_at_25c_ps: 1_000.0,
            timing_derate_ps_per_kelvin: 0.1,
        },
    ];

    let res = find_stress_breaking_point(
        &FindStressBreakingPointRequest {
            branch_id: "main".into(),
            max_ambient_temp_mkelvin: 358_150,
            max_supply_surge_uv: 8_000_000, // +8V surge => C_IN fails at alpha = 0.5!
        },
        &comps,
    );
    assert_eq!(res.first_failing_entity_id, "C_IN_TANTALUM_FRAGILE");
    assert_eq!(res.failure_mechanism, "SOA_OVERVOLTAGE_BREAKDOWN");
    assert!((res.safety_headroom_pct - (-50.0)).abs() < 1.0);
}

#[test]
fn nsga3_evolves_strictly_non_dominated_five_objective_pareto_frontier() {
    let resp = explore_pareto_frontier(&ExploreParetoFrontierRequest {
        branch_id: "buck-v2".into(),
        population_size: 24,
        generations: 8,
    });
    assert!(resp.frontier.len() >= 4);
    for (i, a) in resp.frontier.iter().enumerate() {
        for (j, b) in resp.frontier.iter().enumerate() {
            if i != j {
                assert!(
                    !pareto_dominates(a, b),
                    "frontier member {i} unexpectedly dominated {j}"
                );
            }
        }
    }
}
