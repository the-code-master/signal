// Module: engine/crates/signal-aging/src/lib.rs — 15-Year NBTI/HCI/TDDB, Black's Law EM, Cap ESR & Solder Fatigue (I-05)
// Invariants: #![forbid(unsafe_code)]; cross-ISA deterministic math (det_exp/det_ln) across multi-stage mission profiles.
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
#![forbid(unsafe_code)]

use signal_core::{det_exp, det_ln};
use thiserror::Error;

const KB_EV_PER_K: f64 = 8.617_333_262e-5;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AgingError {
    #[error("invalid mission profile duration or temperature: {0}")]
    InvalidMissionProfile(&'static str),
}

fn pow_pos(base: f64, exp: f64) -> f64 {
    det_exp(exp * det_ln(base.max(1e-30)))
}

/// MOSFET NBTI / PBTI threshold voltage drift $\Delta V_{th}$ (`mV`) after `hours` at `temp_kelvin` & gate overdrive `vgs_stress_v` (`I-05`).
pub fn compute_mosfet_nbti_vth_shift_mv(
    vgs_stress_v: f64,
    temp_kelvin: f64,
    stress_hours: f64,
) -> f64 {
    let seconds = (stress_hours.max(1.0e-3)) * 3_600.0;
    let ea_ev = 0.12;
    let arrhenius = det_exp(-ea_ev / (KB_EV_PER_K * temp_kelvin.max(200.0)));
    // Reaction-diffusion t^0.16 power law
    0.85 * pow_pos(vgs_stress_v.abs().max(0.1), 1.8) * arrhenius * pow_pos(seconds, 0.16)
}

/// Black's Law interconnect electromigration Mean-Time-To-Failure (`hours`):
/// $\text{MTTF} = A J^{-2} \exp(E_a / (k_B T))$ (`I-05`).
pub fn compute_blacks_law_em_mttf_hours(
    current_density_a_per_cm2: f64,
    temp_kelvin: f64,
    activation_energy_ev: f64,
) -> f64 {
    let j = current_density_a_per_cm2.abs().max(1.0);
    let a_const = 2.4e11_f64;
    (a_const / (j * j))
        * det_exp(activation_energy_ev.max(0.4) / (KB_EV_PER_K * temp_kelvin.max(200.0)))
}

/// Aluminum electrolytic / polymer capacitor ESR multiplier after `hours` at `temp_kelvin` (Arrhenius `10 K` doubling rule).
pub fn compute_capacitor_esr_drift_multiplier(
    rated_life_hours_at_105c: f64,
    temp_kelvin: f64,
    operating_hours: f64,
) -> f64 {
    let accel = pow_pos(2.0, (temp_kelvin - 378.15) / 10.0);
    let effective_age_ratio =
        (operating_hours.max(0.0) * accel) / rated_life_hours_at_105c.max(100.0);
    1.0 + 1.0 * effective_age_ratio
}

/// Norris-Landzberg SAC305 lead-free solder joint thermal-cycling fatigue life (`cycles_to_failure`, `I-05`).
pub fn compute_norris_landzberg_solder_cycles(
    delta_t_kelvin: f64,
    t_max_kelvin: f64,
    cycles_per_day: f64,
) -> u32 {
    let dt = delta_t_kelvin.max(5.0);
    let f = cycles_per_day.clamp(0.1, 1_000.0);
    let nf = 31_000.0
        * pow_pos(dt, -1.9)
        * pow_pos(f, 1.0 / 3.0)
        * det_exp(1_414.0 / t_max_kelvin.max(250.0));
    (nf.round() as u32).max(1)
}

#[derive(Debug, Clone, PartialEq)]
pub struct MissionAgingReport {
    pub nbti_vth_shift_mv: f64,
    pub interconnect_em_mttf_hours: f64,
    pub cap_esr_end_of_life_ratio: f64,
    pub solder_fatigue_cycles_to_fail: u32,
    pub end_of_life_survival_probability: f64,
}

/// Evaluates a `mission_years` automotive/aerospace/industrial mission profile (`I-05`) and computes system EOL survival probability.
pub fn evaluate_mission_profile_aging(
    mission_years: f64,
    duty_cycle: f64,
    operating_temp_kelvin: f64,
    vgs_stress_v: f64,
    trace_j_a_per_cm2: f64,
    daily_temp_swing_k: f64,
) -> Result<MissionAgingReport, AgingError> {
    if mission_years <= 0.0 || operating_temp_kelvin < 50.0 {
        return Err(AgingError::InvalidMissionProfile(
            "mission_years and operating_temp_kelvin must be positive",
        ));
    }
    let active_hours = mission_years * 8_760.0 * duty_cycle.clamp(0.01, 1.0);
    let vth_mv =
        compute_mosfet_nbti_vth_shift_mv(vgs_stress_v, operating_temp_kelvin, active_hours);
    let em_mttf = compute_blacks_law_em_mttf_hours(trace_j_a_per_cm2, operating_temp_kelvin, 0.70);
    let esr_mult =
        compute_capacitor_esr_drift_multiplier(10_000.0, operating_temp_kelvin, active_hours);
    let solder_nf =
        compute_norris_landzberg_solder_cycles(daily_temp_swing_k, operating_temp_kelvin, 2.0);

    let total_cycles = mission_years * 365.0 * duty_cycle.clamp(0.01, 1.0);
    let hazard_em = (active_hours / em_mttf.max(1.0)).powi(2);
    let hazard_solder = (total_cycles / f64::from(solder_nf)).powi(3) * 0.02;
    let hazard_tddb =
        0.0015 * (mission_years / 10.0) * det_exp((operating_temp_kelvin - 358.15) / 40.0);
    let survival = det_exp(-(hazard_em + hazard_solder + hazard_tddb)).clamp(0.0, 1.0);

    Ok(MissionAgingReport {
        nbti_vth_shift_mv: vth_mv,
        interconnect_em_mttf_hours: em_mttf,
        cap_esr_end_of_life_ratio: esr_mult,
        solder_fatigue_cycles_to_fail: solder_nf,
        end_of_life_survival_probability: survival,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifteen_year_mission_profile_tracks_arrhenius_nbti_em_and_solder_fatigue() {
        let mild = evaluate_mission_profile_aging(15.0, 0.35, 338.15, 1.8, 1.5e5, 30.0)
            .expect("mild aging");
        let harsh = evaluate_mission_profile_aging(15.0, 0.90, 398.15, 3.3, 8.0e5, 85.0)
            .expect("harsh aging");
        assert!(harsh.nbti_vth_shift_mv > mild.nbti_vth_shift_mv);
        assert!(harsh.interconnect_em_mttf_hours < mild.interconnect_em_mttf_hours);
        assert!(harsh.solder_fatigue_cycles_to_fail < mild.solder_fatigue_cycles_to_fail);
        assert!(mild.end_of_life_survival_probability > 0.95);
        assert!(harsh.end_of_life_survival_probability < mild.end_of_life_survival_probability);
    }
}
