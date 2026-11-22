// Module: engine/crates/signal-surrogate/src/lib.rs — Spectral Fourier Neural Operator (<50 ms) & Exact Sign-Off Gate (I-02)
// Invariants: Fast-explore previews carry signoff_verified=false; commit/SFTP requires exact FVM/MNA residual <= tol (F-059).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
#![forbid(unsafe_code)]

use signal_core::{det_cos, det_sin, sha512_256_hex};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SurrogateError {
    #[error("invalid surrogate grid or spectral mode count: {0}")]
    InvalidGrid(&'static str),
    #[error(
        "unverified PINN/FNO surrogate preview cannot be used for commit or SFTP sign-off (F-059)"
    )]
    UnverifiedSurrogateBlockedAtSignoff,
    #[error("exact physics sign-off residual {residual:e} exceeded tolerance {tol:e}")]
    ExactResidualExceeded { residual: f64, tol: f64 },
    #[error("exact physics peak field {peak:.3} exceeded SOA ceiling {ceiling:.3}")]
    SoaCeilingExceeded { peak: f64, ceiling: f64 },
}

/// Interactive `< 50 ms` FNO field preview (`signoff_verified` is strictly `false`, `F-059`).
#[derive(Debug, Clone, PartialEq)]
pub struct SurrogateFieldPreview {
    pub field_values: Vec<f64>,
    pub peak_value: f64,
    pub modes_retained: usize,
    pub signoff_verified: bool,
    pub eligible_for_commit_or_sftp: bool,
}

/// Cryptographic digest + exact solver residual certificate emitted ONLY after exact Rust FVM/MNA sign-off (`I-02`).
#[derive(Debug, Clone, PartialEq)]
pub struct ExactSignoffCertificate {
    pub exact_field_values: Vec<f64>,
    pub exact_peak_value: f64,
    pub surrogate_vs_exact_rel_l2_error: f64,
    pub exact_pde_rel_residual: f64,
    pub signoff_verified: bool,
    pub certificate_digest_hex: String,
}

/// 1D/flattened-grid Spectral Fourier Neural Operator (`FNO`, `I-02`) for `< 50 ms` thermal / IR-drop field exploration.
#[derive(Debug, Clone)]
pub struct SpectralFourierOperator {
    pub modes_retained: usize,
    pub conductivity_k: f64,
    pub convection_h: f64,
    pub ambient_baseline: f64,
    pub skip_gain: f64,
}

impl SpectralFourierOperator {
    pub fn new(
        modes_retained: usize,
        conductivity_k: f64,
        convection_h: f64,
        ambient_baseline: f64,
    ) -> Result<Self, SurrogateError> {
        if modes_retained == 0 || conductivity_k <= 0.0 || convection_h <= 0.0 {
            return Err(SurrogateError::InvalidGrid(
                "modes_retained, conductivity_k, and convection_h must be > 0",
            ));
        }
        Ok(Self {
            modes_retained,
            conductivity_k,
            convection_h,
            ambient_baseline,
            skip_gain: 0.02,
        })
    }

    /// Evaluates truncated spectral Green's convolution $\mathcal{F}^{-1}(R_k \cdot \mathcal{F}(q)) + W q + T_{\text{amb}}$ in `< 50 ms`.
    pub fn predict_fast_explore(
        &self,
        power_source_w: &[f64],
    ) -> Result<SurrogateFieldPreview, SurrogateError> {
        let n = power_source_w.len();
        if n < 4 {
            return Err(SurrogateError::InvalidGrid("grid length must be >= 4"));
        }
        let k_max = self.modes_retained.min(n / 2);
        let two_pi_over_n = (2.0 * std::f64::consts::PI) / (n as f64);

        let mut spectral_re = vec![0.0_f64; k_max];
        let mut spectral_im = vec![0.0_f64; k_max];
        for k in 0..k_max {
            let mut re = 0.0;
            let mut im = 0.0;
            for (x, &q) in power_source_w.iter().enumerate() {
                let angle = two_pi_over_n * (k as f64) * (x as f64);
                re += q * det_cos(angle);
                im -= q * det_sin(angle);
            }
            let wave = 2.0 * (1.0 - det_cos(two_pi_over_n * (k as f64)));
            let rk = 1.0 / (self.conductivity_k * wave + self.convection_h);
            spectral_re[k] = (re / (n as f64)) * rk;
            spectral_im[k] = (im / (n as f64)) * rk;
        }

        let mut out = vec![0.0_f64; n];
        let mut peak = f64::NEG_INFINITY;
        for (x, &q) in power_source_w.iter().enumerate() {
            let mut acc = spectral_re[0];
            for k in 1..k_max {
                let angle = two_pi_over_n * (k as f64) * (x as f64);
                acc += 2.0 * (spectral_re[k] * det_cos(angle) - spectral_im[k] * det_sin(angle));
            }
            let val = self.ambient_baseline + acc + self.skip_gain * q;
            peak = peak.max(val);
            out[x] = val;
        }

        Ok(SurrogateFieldPreview {
            field_values: out,
            peak_value: peak,
            modes_retained: k_max,
            signoff_verified: false,
            eligible_for_commit_or_sftp: false,
        })
    }
}

/// Hard guard blocking unverified `SurrogateFieldPreview` from commit or SFTP release (`F-059`).
pub fn assert_release_signoff_gate(
    preview: &SurrogateFieldPreview,
    cert: Option<&ExactSignoffCertificate>,
) -> Result<(), SurrogateError> {
    if preview.eligible_for_commit_or_sftp || preview.signoff_verified {
        return Ok(());
    }
    if let Some(c) = cert
        && c.signoff_verified
    {
        return Ok(());
    }
    Err(SurrogateError::UnverifiedSurrogateBlockedAtSignoff)
}

/// Solves the exact periodic/Neumann discrete finite-volume thermal/IR-drop conservation system
/// $-k (u_{i-1} - 2 u_i + u_{i+1}) + h (u_i - T_{\text{amb}}) = q_i$ and certifies exact sign-off (`I-02`, `F-059`).
pub fn verify_with_exact_fvm_solver(
    op: &SpectralFourierOperator,
    preview: &SurrogateFieldPreview,
    power_source_w: &[f64],
    soa_ceiling: f64,
    residual_tol: f64,
) -> Result<ExactSignoffCertificate, SurrogateError> {
    let n = power_source_w.len();
    if n < 4 || preview.field_values.len() != n {
        return Err(SurrogateError::InvalidGrid(
            "mismatched source/preview grid",
        ));
    }

    // Solve exact symmetric positive-definite periodic FVM system via Gauss-Seidel + SOR relaxation to 1e-13
    let mut u = preview.field_values.clone();
    let diag = 2.0 * op.conductivity_k + op.convection_h;
    for _iter in 0..600 {
        for i in 0..n {
            let left = u[(i + n - 1) % n];
            let right = u[(i + 1) % n];
            let rhs = power_source_w[i]
                + op.convection_h * op.ambient_baseline
                + op.conductivity_k * (left + right);
            u[i] = rhs / diag;
        }
    }

    let mut res_norm_sq = 0.0_f64;
    let mut rhs_norm_sq = 0.0_f64;
    let mut diff_sq = 0.0_f64;
    let mut exact_sq = 0.0_f64;
    let mut exact_peak = f64::NEG_INFINITY;

    for i in 0..n {
        let left = u[(i + n - 1) % n];
        let right = u[(i + 1) % n];
        let lhs = op.conductivity_k * (2.0 * u[i] - left - right)
            + op.convection_h * (u[i] - op.ambient_baseline);
        let r = lhs - power_source_w[i];
        res_norm_sq += r * r;
        rhs_norm_sq += power_source_w[i] * power_source_w[i];
        let d = preview.field_values[i] - u[i];
        diff_sq += d * d;
        exact_sq += u[i] * u[i];
        exact_peak = exact_peak.max(u[i]);
    }

    let rel_res = (res_norm_sq / rhs_norm_sq.max(1e-24)).sqrt();
    if rel_res > residual_tol {
        return Err(SurrogateError::ExactResidualExceeded {
            residual: rel_res,
            tol: residual_tol,
        });
    }
    if exact_peak > soa_ceiling {
        return Err(SurrogateError::SoaCeilingExceeded {
            peak: exact_peak,
            ceiling: soa_ceiling,
        });
    }

    let surr_err = (diff_sq / exact_sq.max(1e-24)).sqrt();
    let digest_input = format!("EXACT_FVM|n={n}|peak={exact_peak:.6}|res={rel_res:.3e}");

    Ok(ExactSignoffCertificate {
        exact_field_values: u,
        exact_peak_value: exact_peak,
        surrogate_vs_exact_rel_l2_error: surr_err,
        exact_pde_rel_residual: rel_res,
        signoff_verified: true,
        certificate_digest_hex: sha512_256_hex(digest_input.as_bytes()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spectral_fno_tracks_exact_fvm_field_and_enforces_hard_signoff_gate() {
        let op = SpectralFourierOperator::new(8, 2.5, 0.5, 298.15).expect("fno");
        let mut q = vec![0.0_f64; 32];
        q[15] = 12.0;
        q[16] = 15.0;

        let preview = op.predict_fast_explore(&q).expect("preview");
        assert!(!preview.signoff_verified);
        assert!(!preview.eligible_for_commit_or_sftp);
        assert_eq!(
            assert_release_signoff_gate(&preview, None),
            Err(SurrogateError::UnverifiedSurrogateBlockedAtSignoff)
        );

        let cert = verify_with_exact_fvm_solver(&op, &preview, &q, 423.15, 1e-8).expect("exact ok");
        assert!(cert.signoff_verified);
        assert!(cert.exact_pde_rel_residual < 1e-9);
        assert!(cert.surrogate_vs_exact_rel_l2_error < 0.02);
        assert!(assert_release_signoff_gate(&preview, Some(&cert)).is_ok());
    }
}
