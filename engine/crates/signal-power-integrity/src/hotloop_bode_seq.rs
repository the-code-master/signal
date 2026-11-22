// Module: engine/crates/signal-power-integrity/src/hotloop_bode_seq.rs — SiC/GaN Hot-Loop, Type-III K-Factor & Rail Sequencing
// Invariants: Audits Miller C_gd*dV/dt shoot-through (FEAT-025), exact Venable K-factor poles/zeros (FEAT-026), and sequencing (FEAT-030).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_core::{det_cos, det_sin};

#[derive(Debug, Clone, PartialEq)]
pub struct HotLoopMillerAudit {
    pub loop_inductance_nh: f64,
    pub peak_overshoot_v: f64,
    pub miller_induced_vgs_v: f64,
    pub shoot_through_safe: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SicGanHotLoopInput {
    pub loop_area_sq_mm: f64,
    pub plane_spacing_mm: f64,
    pub di_dt_a_per_ns: f64,
    pub dv_dt_v_per_ns: f64,
    pub c_gd_pf: f64,
    pub r_gate_off_ohm: f64,
    pub vgs_off_bias_v: f64,
    pub vth_min_v: f64,
}

/// Evaluates power-stage commutation hot-loop $L_{\text{loop}} dI/dt$ overshoot & SiC/GaN Miller $C_{gd} dV/dt$ shoot-through (`FEAT-025`).
pub fn audit_sic_gan_hot_loop(inp: &SicGanHotLoopInput) -> HotLoopMillerAudit {
    let mu0 = 4.0e-7 * std::f64::consts::PI;
    // Flux-canceled parallel-plate loop inductance L ~= mu0 * spacing * sqrt(area) / width
    let side_m = (inp.loop_area_sq_mm.max(0.5).sqrt()) * 1.0e-3;
    let h_m = (inp.plane_spacing_mm.max(0.05)) * 1.0e-3;
    let l_loop_nh = (mu0 * h_m * (side_m / side_m.max(1e-4)) * 1.0e9).max(0.25)
        * (inp.loop_area_sq_mm.max(1.0).sqrt());
    let overshoot_v = l_loop_nh * inp.di_dt_a_per_ns.abs();
    // I_miller = C_gd * dV/dt (pF * V/ns = mA = 1e-3 A)
    let i_miller_a = (inp.c_gd_pf * inp.dv_dt_v_per_ns.abs()) * 1.0e-3;
    let induced_vgs = inp.vgs_off_bias_v + i_miller_a * inp.r_gate_off_ohm;

    HotLoopMillerAudit {
        loop_inductance_nh: l_loop_nh,
        peak_overshoot_v: overshoot_v,
        miller_induced_vgs_v: induced_vgs,
        shoot_through_safe: induced_vgs < inp.vth_min_v * 0.85,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type3CompensatorSpec {
    pub k_factor: f64,
    pub f_zero_hz: f64,
    pub f_pole_hz: f64,
    pub r2_ohm: f64,
    pub c1_farads: f64,
    pub c2_farads: f64,
    pub r3_ohm: f64,
    pub c3_farads: f64,
}

/// Synthesizes a Venable **Type-III** analog/PID loop compensator (`FEAT-026`) achieving target crossover `fc_hz`
/// and phase boost `phase_boost_deg` around plant gain `plant_mag_at_fc` with input resistor `r1_ohm`.
pub fn synthesize_type3_k_factor_compensator(
    fc_hz: f64,
    phase_boost_deg: f64,
    plant_mag_at_fc: f64,
    r1_ohm: f64,
) -> Type3CompensatorSpec {
    let boost_clamped = phase_boost_deg.clamp(20.0, 150.0);
    // For Type-III double zero/pole pair: K = (tan(boost/4 + 45 deg))^2
    let angle_rad = (0.25 * boost_clamped + 45.0) * (std::f64::consts::PI / 180.0);
    let tan_a = det_sin(angle_rad) / det_cos(angle_rad).max(1e-6);
    let k = (tan_a * tan_a).max(1.2);
    let sqrt_k = k.sqrt();

    let f_z = fc_hz / sqrt_k;
    let f_p = fc_hz * sqrt_k;
    let g_comp = 1.0 / plant_mag_at_fc.max(1e-6);

    let r2 = g_comp * r1_ohm * (k / (k - 1.0));
    let c2 = 1.0 / (2.0 * std::f64::consts::PI * fc_hz * g_comp * k * r1_ohm);
    let c1 = c2 * (k - 1.0);
    let r3 = r1_ohm / (k - 1.0);
    let c3 = 1.0 / (2.0 * std::f64::consts::PI * fc_hz * sqrt_k * r3);

    Type3CompensatorSpec {
        k_factor: k,
        f_zero_hz: f_z,
        f_pole_hz: f_p,
        r2_ohm: r2,
        c1_farads: c1,
        c2_farads: c2,
        r3_ohm: r3,
        c3_farads: c3,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RailRampSpec {
    pub rail_name: String,
    pub start_delay_ms: f64,
    pub ramp_duration_ms: f64,
    pub nominal_v: f64,
    pub bulk_cap_uf: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RailSequencingAudit {
    pub monotonic_order_valid: bool,
    pub max_inrush_current_a: f64,
    pub esd_backpower_risk_detected: bool,
}

/// Verifies multi-rail ramp ordering, capacitor inrush $I = C \Delta V / \Delta t$, and ESD clamp diode back-powering (`FEAT-030`).
pub fn verify_power_rail_sequencing(
    rails_in_required_order: &[RailRampSpec],
    max_esd_diode_drop_v: f64,
) -> RailSequencingAudit {
    let mut order_ok = true;
    let mut max_inrush_a = 0.0_f64;
    let mut backpower = false;

    for (idx, r) in rails_in_required_order.iter().enumerate() {
        let dt_s = (r.ramp_duration_ms.max(0.05)) * 1.0e-3;
        let inrush = (r.bulk_cap_uf * 1.0e-6) * r.nominal_v / dt_s;
        max_inrush_a = max_inrush_a.max(inrush);
        if idx > 0 {
            let prev = &rails_in_required_order[idx - 1];
            if r.start_delay_ms < prev.start_delay_ms {
                order_ok = false;
            }
            // If I/O rail ramps significantly earlier than core/clamp rail, I/O ESD diode conducts into unpowered rail!
            if prev.rail_name.contains("CORE")
                && r.rail_name.contains("IO")
                && (r.start_delay_ms < prev.start_delay_ms + prev.ramp_duration_ms)
                && (r.nominal_v > max_esd_diode_drop_v)
            {
                backpower = true;
            }
        }
    }

    RailSequencingAudit {
        monotonic_order_valid: order_ok,
        max_inrush_current_a: max_inrush_a,
        esd_backpower_risk_detected: backpower,
    }
}
