// Module: engine/crates/signal-power-integrity/src/bms_foc_thermal.rs — 2-RC BMS Pack, Clarke/Park FOC SVPWM & Heatsink CFD
// Invariants: Exact Clarke/Park dq0 + SVPWM sector duties (FEAT-028), 2-RC Randles BMS + Arrhenius runaway (FEAT-027), and thermal R_th (FEAT-029).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_core::{det_cos, det_exp, det_sin};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BmsPackStepResult {
    pub pack_terminal_voltage_v: f64,
    pub next_soc: f64,
    pub cell_temp_kelvin: f64,
    pub arrhenius_thermal_runaway_warning: bool,
}

/// Simulates an $N_s \times M_p$ 2-RC Randles Li-Ion pack step and checks Arrhenius SEI exothermic self-heating (`FEAT-027`).
pub fn simulate_bms_randles_pack_step(
    series_cells: u32,
    parallel_strings: u32,
    soc: f64,
    pack_discharge_current_a: f64,
    dt_s: f64,
    cell_temp_kelvin: f64,
) -> BmsPackStepResult {
    let ns = f64::from(series_cells.max(1));
    let mp = f64::from(parallel_strings.max(1));
    let i_cell = pack_discharge_current_a / mp;
    let soc_clamped = soc.clamp(0.01, 1.0);
    let v_ocv_cell = 3.0 + 1.15 * soc_clamped - 0.08 * (1.0 - soc_clamped).powi(2);

    let r0 = 0.018;
    let r1 = 0.012 * (1.0 - det_exp(-dt_s / 8.0));
    let r2 = 0.015 * (1.0 - det_exp(-dt_s / 60.0));
    let v_cell = (v_ocv_cell - i_cell * (r0 + r1 + r2)).max(2.5);

    let q_nom_as = 3.5 * 3_600.0;
    let next_soc = (soc_clamped - (i_cell * dt_s) / q_nom_as).clamp(0.0, 1.0);

    // Joule + Arrhenius exothermic reaction onset above ~393 K (120 C)
    let p_joule = i_cell * i_cell * (r0 + r1 + r2);
    let p_arrhenius = 2.5e7 * det_exp(-8_500.0 / cell_temp_kelvin.max(250.0));
    let next_temp_k = cell_temp_kelvin + ((p_joule + p_arrhenius) * dt_s) / 95.0;

    BmsPackStepResult {
        pack_terminal_voltage_v: ns * v_cell,
        next_soc,
        cell_temp_kelvin: next_temp_k,
        arrhenius_thermal_runaway_warning: next_temp_k >= 393.15 || p_arrhenius > 2.0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FocSvpwmState {
    pub i_d: f64,
    pub i_q: f64,
    pub electromagnetic_torque_nm: f64,
    pub pwm_duties_abc: [f64; 3],
}

/// Executes amplitude-invariant Clarke $(a, b, c) \to (\alpha, \beta)$, Park $(\alpha, \beta) \to (d, q)$,
/// PMSM torque $T_e = \frac{3}{2} p \psi_f i_q$, and min-max zero-sequence **SVPWM** duty generation (`FEAT-028`).
pub fn solve_foc_park_clarke_svpwm(
    i_abc: [f64; 3],
    elec_angle_rad: f64,
    v_dq_cmd: [f64; 2],
    v_dc_bus: f64,
    pole_pairs: u32,
    flux_linkage_wb: f64,
) -> FocSvpwmState {
    let i_alpha = (2.0 / 3.0) * (i_abc[0] - 0.5 * i_abc[1] - 0.5 * i_abc[2]);
    let i_beta = (1.0 / 3.0_f64.sqrt()) * (i_abc[1] - i_abc[2]);
    let c = det_cos(elec_angle_rad);
    let s = det_sin(elec_angle_rad);

    let i_d = i_alpha * c + i_beta * s;
    let i_q = -i_alpha * s + i_beta * c;
    let torque = 1.5 * f64::from(pole_pairs.max(1)) * flux_linkage_wb * i_q;

    // Inverse Park (d, q) -> (alpha, beta) -> 3-phase saddle-wave SVPWM
    let v_alpha = v_dq_cmd[0] * c - v_dq_cmd[1] * s;
    let v_beta = v_dq_cmd[0] * s + v_dq_cmd[1] * c;
    let va = v_alpha;
    let vb = -0.5 * v_alpha + (0.5 * 3.0_f64.sqrt()) * v_beta;
    let vc = -0.5 * v_alpha - (0.5 * 3.0_f64.sqrt()) * v_beta;
    let v_min = va.min(vb).min(vc);
    let v_max = va.max(vb).max(vc);
    let v_offset = -0.5 * (v_min + v_max);
    let vdc = v_dc_bus.max(1.0);

    let duties = [
        (0.5 + (va + v_offset) / vdc).clamp(0.0, 1.0),
        (0.5 + (vb + v_offset) / vdc).clamp(0.0, 1.0),
        (0.5 + (vc + v_offset) / vdc).clamp(0.0, 1.0),
    ];

    FocSvpwmState {
        i_d,
        i_q,
        electromagnetic_torque_nm: torque,
        pwm_duties_abc: duties,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeatsinkThermalSpec {
    pub r_theta_jc_k_per_w: f64,
    pub tim_thickness_mm: f64,
    pub tim_k_w_per_mk: f64,
    pub contact_area_sq_mm: f64,
    pub heatsink_natural_r_k_per_w: f64,
    pub airflow_cfm: f64,
}

/// Solves junction temperature (`mK`) across Junction -> Case -> TIM -> Forced-Airflow Heatsink (`FEAT-029`).
pub fn solve_heatsink_airflow_tj_mk(
    power_w: f64,
    ambient_mk: i64,
    spec: &HeatsinkThermalSpec,
) -> i64 {
    let area_m2 = (spec.contact_area_sq_mm.max(1.0)) * 1.0e-6;
    let r_tim =
        ((spec.tim_thickness_mm.max(0.02)) * 1.0e-3) / (spec.tim_k_w_per_mk.max(0.2) * area_m2);
    let forced_factor = 1.0 / (1.0 + 0.35 * spec.airflow_cfm.max(0.0).sqrt());
    let r_sink = spec.heatsink_natural_r_k_per_w.max(0.05) * forced_factor;
    let total_rise_k = power_w.max(0.0) * (spec.r_theta_jc_k_per_w + r_tim + r_sink);
    ambient_mk.saturating_add((total_rise_k * 1_000.0).round() as i64)
}
