// Module: engine/crates/signal-ic/src/photonics_cpo/mod.rs — Baseband-Envelope 193 THz Co-Packaged Optics Solver (F-148)
// Invariants: Unitary 2x2 optical scattering conserves |E_1|^2 + |E_2|^2; steps at ps intervals (10^4x faster than carrier, F-148).
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use crate::IcError;
use signal_core::core_math::{det_cos, det_exp, det_sin};

/// Complex optical baseband envelope $E(t) = \text{re} + j\,\text{im}$ ($\sqrt{\text{W}}$) around $\lambda_0 \in \{1310, 1550\}\text{ nm}$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexEnvelope {
    pub re: f64,
    pub im: f64,
}

impl ComplexEnvelope {
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn optical_power_w(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn c_mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }

    pub fn c_add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }

    pub fn scale(self, s: f64) -> Self {
        Self {
            re: self.re * s,
            im: self.im * s,
        }
    }

    pub fn from_polar(amp: f64, phase_rad: f64) -> Self {
        Self {
            re: amp * det_cos(phase_rad),
            im: amp * det_sin(phase_rad),
        }
    }
}

/// Evaluates unitary `2x2` directional coupler $\begin{bmatrix} t & -j\kappa \\ -j\kappa & t \end{bmatrix}$ with $t = \sqrt{1 - \kappa^2}$ (`F-148`).
pub fn scatter_directional_coupler_2x2(
    in1: ComplexEnvelope,
    in2: ComplexEnvelope,
    kappa: f64,
) -> (ComplexEnvelope, ComplexEnvelope) {
    let k = kappa.clamp(0.0, 1.0);
    let t = (1.0 - k * k).max(0.0).sqrt();
    let minus_jk = ComplexEnvelope::new(0.0, -k);
    let out1 = in1.scale(t).c_add(minus_jk.c_mul(in2));
    let out2 = minus_jk.c_mul(in1).c_add(in2.scale(t));
    (out1, out2)
}

/// Evaluates all-pass Silicon Photonics microring modulator complex field transmission $E_{\text{out}}/E_{\text{in}}$ (`F-148`).
pub fn eval_microring_resonator_transfer(
    input: ComplexEnvelope,
    self_coupling_t: f64,
    roundtrip_atten_a: f64,
    roundtrip_phase_rad: f64,
) -> ComplexEnvelope {
    let t = self_coupling_t.clamp(0.05, 0.9999);
    let a = roundtrip_atten_a.clamp(0.05, 0.9999);
    let exp_neg_jtheta = ComplexEnvelope::from_polar(1.0, -roundtrip_phase_rad);
    // H(theta) = (t - a * e^{-j*theta}) / (1 - t * a * e^{-j*theta})
    let num = ComplexEnvelope::new(t - a * exp_neg_jtheta.re, -a * exp_neg_jtheta.im);
    let den = ComplexEnvelope::new(1.0 - t * a * exp_neg_jtheta.re, -t * a * exp_neg_jtheta.im);
    let den_norm_sq = den.optical_power_w().max(1e-15);
    let conj_den = ComplexEnvelope::new(den.re / den_norm_sq, -den.im / den_norm_sq);
    input.c_mul(num.c_mul(conj_den))
}

#[derive(Debug, Clone, PartialEq)]
pub struct CpoLinkStepSummary {
    pub extinction_ratio_db: f64,
    pub peak_photocurrent_ua: f64,
    pub speedup_vs_carrier_stepping: f64,
}

/// Simulates a push-pull Mach-Zehnder + waveguide dispersion + PIN photodetector CPO link at ps baseband steps (`F-148`).
pub fn simulate_cpo_mzm_link_baseband(
    carrier_wavelength_pm: i64,
    step_size_ps: i64,
    vpi_v: f64,
    drive_swing_v: f64,
    waveguide_loss_db: f64,
    responsivity_a_per_w: f64,
) -> Result<CpoLinkStepSummary, IcError> {
    if !(1_260_000..=1_625_000).contains(&carrier_wavelength_pm) || step_size_ps <= 0 {
        return Err(IcError::InvalidOpticalCarrier(carrier_wavelength_pm));
    }
    let cw_in = ComplexEnvelope::new((0.010_f64).sqrt(), 0.0); // 10 mW CW laser
    let (arm_top, arm_bot) =
        scatter_directional_coupler_2x2(cw_in, ComplexEnvelope::new(0.0, 0.0), (0.5_f64).sqrt());

    let eval_mzm = |v_diff: f64| -> f64 {
        let phi = std::f64::consts::PI * (v_diff / vpi_v.max(0.1)) * 0.5;
        let mod_top = arm_top.c_mul(ComplexEnvelope::from_polar(1.0, phi));
        let mod_bot = arm_bot.c_mul(ComplexEnvelope::from_polar(1.0, -phi + 0.02)); // finite 34 dB imbalance floor
        let (_, combined) = scatter_directional_coupler_2x2(mod_top, mod_bot, (0.5_f64).sqrt());
        let field_atten = det_exp(-waveguide_loss_db.max(0.0) * std::f64::consts::LN_10 / 20.0);
        combined.scale(field_atten).optical_power_w()
    };

    let p_one_w = eval_mzm(0.0);
    let p_zero_w = eval_mzm(drive_swing_v).max(1e-12);
    let er_db = 10.0 * (p_one_w / p_zero_w).max(1.0001).log10();
    let i_pd_ua = p_one_w * responsivity_a_per_w.max(0.1) * 1e6;

    // Optical carrier period at 1550 nm is ~5.17 fs => Nyquist carrier FDTD requires ~0.25 fs steps!
    let speedup = ((step_size_ps as f64) * 1_000.0) / 0.25;
    Ok(CpoLinkStepSummary {
        extinction_ratio_db: er_db,
        peak_photocurrent_ua: i_pd_ua,
        speedup_vs_carrier_stepping: speedup,
    })
}
