// Module: engine/crates/signal-ic/src/cryo_cmos/mod.rs — 4K..20mK Cryo-CMOS Log-Scaled Fermi-Dirac Continuation Solver (F-148)
// Invariants: Prevents exp(-qV/kT) NaN/Inf overflow down to 20 mK via band-tail T_eff & log-sum-exp Fermi-Dirac integral (F-148).
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use crate::IcError;
use crate::photonics_cpo::simulate_cpo_mzm_link_baseband;
use signal_core::core_math::{det_exp, det_ln};
use signal_proto::v1::{RunPhotonicsAndCryoSimRequest, RunPhotonicsAndCryoSimResponse};

const BOLTZMANN_EV_PER_K: f64 = 8.617_333_262e-5;
pub const BAND_TAIL_FLOOR_TEMP_MKELVIN: i64 = 12_000; // 12 K silicon band-tail disorder saturation

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CryoMosOperatingState {
    pub effective_temp_mkelvin: i64,
    pub subthreshold_swing_mv_per_dec: f64,
    pub drain_current_na: f64,
    pub continuation_steps: u32,
}

/// Computes effective band-tail regularized carrier temperature $T_{\text{eff}}(T) = \sqrt{T^2 + T_0^2}$ in mK (`F-148`).
pub fn compute_cryo_effective_temp_mkelvin(stage_temp_mkelvin: i64) -> Result<i64, IcError> {
    if !(10..=500_000).contains(&stage_temp_mkelvin) {
        return Err(IcError::InvalidCryoTemperature(stage_temp_mkelvin));
    }
    let t = stage_temp_mkelvin as f64;
    let t0 = BAND_TAIL_FLOOR_TEMP_MKELVIN as f64;
    Ok(t.hypot(t0).round() as i64)
}

/// Overflow-proof 0th-order Fermi-Dirac integral $\mathcal{F}_0(\eta) = \ln(1 + e^\eta) = \max(\eta, 0) + \ln(1 + e^{-|\eta|})$ (`F-148`).
pub fn log_fermi_dirac_f0(eta: f64) -> f64 {
    let abs_eta = eta.abs();
    let tail = if abs_eta > 45.0 {
        0.0
    } else {
        det_ln(1.0 + det_exp(-abs_eta))
    };
    eta.max(0.0) + tail
}

/// Solves a Cryo-CMOS gate bias point from `300 K` down to `stage_temp_mkelvin` (`4.2 K` / `20 mK`) via smooth continuation (`F-148`).
pub fn solve_cryo_cmos_fermi_continuation(
    stage_temp_mkelvin: i64,
    vgs_v: f64,
    vds_v: f64,
    vth_300k_v: f64,
) -> Result<CryoMosOperatingState, IcError> {
    let mut current_t_mk = 300_000_i64;
    let mut steps = 0_u32;

    let (last_teff_mk, last_id_na) = loop {
        steps += 1;
        let teff_mk = compute_cryo_effective_temp_mkelvin(current_t_mk)?;
        let t_eff_k = (teff_mk as f64) * 1e-3;
        let vt_v = BOLTZMANN_EV_PER_K * t_eff_k;

        // Cryogenic V_th shift (~ +0.85 mV/K cooling shift below 300 K) + mobility boost
        let delta_t_k = (300.0 - ((current_t_mk as f64) * 1e-3)).max(0.0);
        let vth_cryo = vth_300k_v + 0.000_85 * delta_t_k;
        let mobility_boost = 1.0 + 2.8 * (delta_t_k / 296.0);

        let eta_src = (vgs_v - vth_cryo) / (1.15 * vt_v);
        let eta_drn = (vgs_v - vth_cryo - vds_v.max(0.0)) / (1.15 * vt_v);
        let f_src = log_fermi_dirac_f0(eta_src);
        let f_drn = log_fermi_dirac_f0(eta_drn);

        // EKV-like charge-conserving Fermi-Dirac drain current free of exp() overflow
        let i_spec_na = 850.0 * mobility_boost * vt_v * vt_v * 1e6;
        let id_na = i_spec_na * (f_src * f_src - f_drn * f_drn).max(0.0);
        if !id_na.is_finite() {
            return Err(IcError::CryoContinuationDiverged);
        }

        if current_t_mk <= stage_temp_mkelvin {
            break (teff_mk, id_na);
        }
        current_t_mk = ((current_t_mk as f64 * 0.45).round() as i64).max(stage_temp_mkelvin);
    };

    let ss_mv_dec =
        (BOLTZMANN_EV_PER_K * ((last_teff_mk as f64) * 1e-3) * std::f64::consts::LN_10 * 1.15)
            * 1_000.0;
    Ok(CryoMosOperatingState {
        effective_temp_mkelvin: last_teff_mk,
        subthreshold_swing_mv_per_dec: ss_mv_dec,
        drain_current_na: last_id_na,
        continuation_steps: steps,
    })
}

/// Combined RPC entrypoint for Baseband CPO Photonics + 4 K Cryo-CMOS simulation (`RunPhotonicsAndCryoSim`, `F-148`).
pub fn run_photonics_and_cryo_sim(
    req: &RunPhotonicsAndCryoSimRequest,
) -> Result<RunPhotonicsAndCryoSimResponse, IcError> {
    let cpo = simulate_cpo_mzm_link_baseband(
        req.optical_carrier_wavelength_pm,
        req.step_size_ps,
        1.8,
        1.75,
        1.4,
        0.92,
    )?;
    let cryo = solve_cryo_cmos_fermi_continuation(req.stage_temperature_mkelvin, 0.65, 0.70, 0.32)?;

    Ok(RunPhotonicsAndCryoSimResponse {
        optical_extinction_ratio_db: cpo.extinction_ratio_db,
        cryo_effective_temp_mkelvin: cryo.effective_temp_mkelvin,
        fermi_dirac_continuation_converged: cryo.drain_current_na.is_finite()
            && cryo.continuation_steps >= 1,
    })
}
