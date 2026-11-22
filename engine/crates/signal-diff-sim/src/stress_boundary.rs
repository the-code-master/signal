// Module: engine/crates/signal-diff-sim/src/stress_boundary.rs — Multi-Physics Stress Breaking-Point Ray-Marcher (FEAT-102)
// Invariants: Solves exact earliest stress alpha* along (ambient_temp, supply_surge) ray to SOA/timing failure.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_proto::v1::{FindStressBreakingPointRequest, FindStressBreakingPointResponse};

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentStressEnvelope {
    pub entity_id: String,
    pub nominal_voltage_uv: i64,
    pub voltage_gain_from_supply: f64,
    pub abs_max_voltage_uv: i64,
    pub nominal_power_mw: f64,
    pub r_theta_ja_mk_per_w: f64,
    pub max_tj_mkelvin: i64,
    pub timing_slack_at_25c_ps: f64,
    pub timing_derate_ps_per_kelvin: f64,
}

fn margin_at_alpha(
    comp: &ComponentStressEnvelope,
    alpha: f64,
    base_temp_mk: i64,
    delta_temp_mk: f64,
    delta_supply_uv: f64,
) -> (f64, &'static str) {
    let t_amb_mk = (base_temp_mk as f64) + alpha * delta_temp_mk;
    let v_applied_uv =
        (comp.nominal_voltage_uv as f64) + alpha * delta_supply_uv * comp.voltage_gain_from_supply;
    let v_scale = (v_applied_uv / (comp.nominal_voltage_uv.max(1) as f64)).max(1.0);
    let p_diss_w = (comp.nominal_power_mw * 1e-3) * v_scale * v_scale;
    let tj_mk = t_amb_mk + p_diss_w * comp.r_theta_ja_mk_per_w;

    let v_margin =
        ((comp.abs_max_voltage_uv as f64) - v_applied_uv) / (comp.abs_max_voltage_uv.max(1) as f64);
    let tj_margin = ((comp.max_tj_mkelvin as f64) - tj_mk) / (comp.max_tj_mkelvin.max(1) as f64);
    let temp_rise_k = (tj_mk - 298_150.0).max(0.0) / 1_000.0;
    let timing_slack_ps =
        comp.timing_slack_at_25c_ps - temp_rise_k * comp.timing_derate_ps_per_kelvin;
    let timing_margin = timing_slack_ps / comp.timing_slack_at_25c_ps.max(1.0);

    if v_margin <= tj_margin && v_margin <= timing_margin {
        (v_margin, "SOA_OVERVOLTAGE_BREAKDOWN")
    } else if tj_margin <= timing_margin {
        (tj_margin, "THERMAL_JUNCTION_SOA_EXCEEDED")
    } else {
        (timing_margin, "SETUP_TIMING_SLACK_COLLAPSE")
    }
}

/// Ray-marches + bisects along the worst-case `(T_ambient, V_supply_surge)` envelope (`FEAT-102`)
/// to pinpoint the exact first component (`alpha*`) to cross SOA breakdown or timing collapse.
pub fn find_stress_breaking_point(
    req: &FindStressBreakingPointRequest,
    components: &[ComponentStressEnvelope],
) -> FindStressBreakingPointResponse {
    if components.is_empty() {
        return FindStressBreakingPointResponse {
            first_failing_entity_id: String::new(),
            failure_mechanism: "NONE".to_string(),
            safety_headroom_pct: 100.0,
        };
    }

    let base_temp_mk = 298_150_i64; // 25.0 C
    let delta_temp_mk = ((req.max_ambient_temp_mkelvin - base_temp_mk).max(10_000)) as f64;
    let delta_supply_uv = (req.max_supply_surge_uv.max(500_000)) as f64;

    let mut earliest_alpha = f64::INFINITY;
    let mut culprit_id = components[0].entity_id.clone();
    let mut culprit_mech = "THERMAL_JUNCTION_SOA_EXCEEDED";

    for comp in components {
        let (m0, mech0) = margin_at_alpha(comp, 0.0, base_temp_mk, delta_temp_mk, delta_supply_uv);
        if m0 <= 0.0 {
            return FindStressBreakingPointResponse {
                first_failing_entity_id: comp.entity_id.clone(),
                failure_mechanism: mech0.to_string(),
                safety_headroom_pct: 0.0,
            };
        }

        // Coarse ray march in [0, 4.0], then 32-step exact bisection on first sign change
        let mut hi = 4.0_f64;
        for step in 1..=40 {
            let probe = (step as f64) * 0.1;
            let (mp, _) =
                margin_at_alpha(comp, probe, base_temp_mk, delta_temp_mk, delta_supply_uv);
            if mp <= 0.0 {
                hi = probe;
                break;
            }
        }
        let mut lo = (hi - 0.1).max(0.0);
        let mut mech_star = mech0;
        for _ in 0..32 {
            let mid = 0.5 * (lo + hi);
            let (mm, mech) =
                margin_at_alpha(comp, mid, base_temp_mk, delta_temp_mk, delta_supply_uv);
            mech_star = mech;
            if mm <= 0.0 {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        let alpha_star = 0.5 * (lo + hi);
        if alpha_star < earliest_alpha {
            earliest_alpha = alpha_star;
            culprit_id.clone_from(&comp.entity_id);
            culprit_mech = mech_star;
        }
    }

    // alpha == 1.0 corresponds to 100% of requested surge/ambient envelope
    let headroom_pct = ((earliest_alpha - 1.0) * 100.0).clamp(-100.0, 300.0);
    FindStressBreakingPointResponse {
        first_failing_entity_id: culprit_id,
        failure_mechanism: culprit_mech.to_string(),
        safety_headroom_pct: headroom_pct,
    }
}
