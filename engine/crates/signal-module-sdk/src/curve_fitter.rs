// Module: engine/crates/signal-module-sdk/src/curve_fitter.rs — Levenberg-Marquardt Datasheet Curve Fitter (FEAT-073)
// Invariants: Cross-ISA deterministic math (det_exp/det_ln/det_tanh); enforces passivity & physical parameter bounds.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/18-universal-component-and-datasheet-repository.md

use crate::SdkError;
use signal_core::{det_exp, det_ln, det_tanh};
use signal_proto::v1::{FitCompactModelCurveRequest, FitCompactModelCurveResponse};

const VT_300K_V: f64 = 0.025_851_99;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactCurveFamily {
    ShockleyDiodeIv,
    MlccDcBiasCv,
    FosterThermalZth,
}

impl CompactCurveFamily {
    pub fn parse(raw: &str) -> Result<Self, SdkError> {
        let up = raw.trim().to_ascii_uppercase();
        if up.contains("DIODE") || up.contains("SHOCKLEY") {
            Ok(Self::ShockleyDiodeIv)
        } else if up.contains("MLCC") || up.contains("CAP") || up.contains("CV") {
            Ok(Self::MlccDcBiasCv)
        } else if up.contains("FOSTER") || up.contains("THERM") || up.contains("ZTH") {
            Ok(Self::FosterThermalZth)
        } else {
            Err(SdkError::CurveFitError(format!(
                "unsupported compact model family `{raw}`"
            )))
        }
    }
}

fn eval_model(family: CompactCurveFamily, p: &[f64], x: f64) -> f64 {
    match family {
        // Fit in log-current space for multi-decade dynamic range: y_hat = ln(I(V)) where V_j = x - I*Rs approx
        CompactCurveFamily::ShockleyDiodeIv => {
            let is_a = p[0].max(1e-20);
            let n = p[1].clamp(0.7, 4.0);
            let rs = p[2].max(0.0);
            let i_ideal = is_a * (det_exp((x / (n * VT_300K_V)).clamp(-40.0, 45.0)) - 1.0);
            let v_eff = (x - i_ideal.min(10.0) * rs).max(0.0);
            is_a * (det_exp((v_eff / (n * VT_300K_V)).clamp(-40.0, 45.0)) - 1.0).max(1e-22)
        }
        // C(V) = C_min + (C_0 - C_min) * (1 - tanh(V / V_sat)^2)
        CompactCurveFamily::MlccDcBiasCv => {
            let c0 = p[0].max(1e-15);
            let vsat = p[1].max(0.1);
            let cmin = p[2].clamp(1e-16, c0);
            let th = det_tanh(x.abs() / vsat);
            cmin + (c0 - cmin) * (1.0 - th * th)
        }
        // Z_th(t) = R_th * (1 - exp(-t / tau))
        CompactCurveFamily::FosterThermalZth => {
            let r_th = p[0].max(1e-6);
            let tau = p[1].max(1e-9);
            r_th * (1.0 - det_exp((-x.max(0.0) / tau).clamp(-60.0, 0.0)))
        }
    }
}

fn project_parameters(family: CompactCurveFamily, p: &mut [f64], enforce_passivity: bool) {
    if !enforce_passivity {
        return;
    }
    match family {
        CompactCurveFamily::ShockleyDiodeIv => {
            p[0] = p[0].clamp(1e-18, 1e-1);
            p[1] = p[1].clamp(0.85, 3.5);
            p[2] = p[2].clamp(0.0, 50.0);
        }
        CompactCurveFamily::MlccDcBiasCv => {
            p[0] = p[0].max(1e-14);
            p[1] = p[1].clamp(0.2, 1_000.0);
            p[2] = p[2].clamp(1e-15, p[0]);
        }
        CompactCurveFamily::FosterThermalZth => {
            p[0] = p[0].max(1e-5);
            p[1] = p[1].max(1e-7);
        }
    }
}

fn solve_small_linear_system(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for k in 0..n {
        let mut piv = k;
        for i in (k + 1)..n {
            if a[i][k].abs() > a[piv][k].abs() {
                piv = i;
            }
        }
        if a[piv][k].abs() < 1e-24 {
            return None;
        }
        a.swap(k, piv);
        b.swap(k, piv);
        let diag = a[k][k];
        for i in (k + 1)..n {
            let factor = a[i][k] / diag;
            b[i] -= factor * b[k];
            let row_k: Vec<f64> = a[k][k..n].to_vec();
            for (offset, &rk_val) in row_k.iter().enumerate() {
                a[i][k + offset] -= factor * rk_val;
            }
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = b[i];
        for j in (i + 1)..n {
            sum -= a[i][j] * x[j];
        }
        x[i] = sum / a[i][i];
    }
    Some(x)
}

fn initial_guess(family: CompactCurveFamily, xs: &[f64], ys: &[f64]) -> Vec<f64> {
    match family {
        CompactCurveFamily::ShockleyDiodeIv => {
            let y0 = ys[0].max(1e-15);
            let y1 = ys[ys.len() / 2].max(1e-14);
            let dx = (xs[ys.len() / 2] - xs[0]).max(0.05);
            let slope = ((det_ln(y1) - det_ln(y0)) / dx).max(5.0);
            let n_est = (1.0 / (slope * VT_300K_V)).clamp(1.0, 2.5);
            let is_est =
                (y0 / det_exp((xs[0] / (n_est * VT_300K_V)).clamp(0.1, 35.0))).clamp(1e-16, 1e-5);
            vec![is_est, n_est, 0.05]
        }
        CompactCurveFamily::MlccDcBiasCv => {
            let c_max = ys.iter().copied().fold(1e-12_f64, f64::max);
            let c_min = ys.iter().copied().fold(c_max, f64::min).min(c_max * 0.8);
            let x_span = xs.last().copied().unwrap_or(10.0).abs().max(1.0) * 0.5;
            vec![c_max, x_span, c_min]
        }
        CompactCurveFamily::FosterThermalZth => {
            let r_max = ys.iter().copied().fold(1e-3_f64, f64::max);
            let t_mid = xs[xs.len() / 2].max(1e-4);
            vec![r_max, t_mid]
        }
    }
}

fn normalized_residuals(family: CompactCurveFamily, p: &[f64], xs: &[f64], ys: &[f64]) -> Vec<f64> {
    xs.iter()
        .zip(ys.iter())
        .map(|(&x, &y)| {
            let pred = eval_model(family, p, x);
            if family == CompactCurveFamily::ShockleyDiodeIv {
                det_ln(pred.max(1e-22)) - det_ln(y.max(1e-22))
            } else {
                (pred - y) / y.abs().max(1e-12)
            }
        })
        .collect()
}

/// Fits compact semiconductor/passive/thermal parameters from datasheet `(stimulus_x, measured_y)` curves
/// via damped **Levenberg-Marquardt** (`FEAT-073`).
pub fn fit_compact_model_curve(
    req: &FitCompactModelCurveRequest,
) -> Result<FitCompactModelCurveResponse, SdkError> {
    if req.stimulus_x.len() < 3 || req.stimulus_x.len() != req.measured_y.len() {
        return Err(SdkError::CurveFitError(
            "stimulus_x and measured_y must have equal length >= 3".into(),
        ));
    }
    let family = CompactCurveFamily::parse(&req.model_family)?;
    let mut p = initial_guess(family, &req.stimulus_x, &req.measured_y);
    project_parameters(family, &mut p, req.enforce_passivity_monotonicity);

    let m = p.len();
    let mut lambda = 1e-2_f64;
    let mut res = normalized_residuals(family, &p, &req.stimulus_x, &req.measured_y);
    let mut cost: f64 = res.iter().map(|r| r * r).sum();

    for _iter in 0..80 {
        let mut jtj = vec![vec![0.0_f64; m]; m];
        let mut jtr = vec![0.0_f64; m];

        for j in 0..m {
            let step = (p[j].abs() * 1e-5).max(1e-17);
            let mut p_plus = p.clone();
            p_plus[j] += step;
            let res_plus = normalized_residuals(family, &p_plus, &req.stimulus_x, &req.measured_y);
            for (i, &r_i) in res.iter().enumerate() {
                let dr_dp = (res_plus[i] - r_i) / step;
                jtr[j] += dr_dp * r_i;
            }
            for k in 0..=j {
                let step_k = (p[k].abs() * 1e-5).max(1e-17);
                let mut p_pk = p.clone();
                p_pk[k] += step_k;
                let res_pk = if k == j {
                    res_plus.clone()
                } else {
                    normalized_residuals(family, &p_pk, &req.stimulus_x, &req.measured_y)
                };
                let mut acc = 0.0;
                for i in 0..res.len() {
                    let dj = (res_plus[i] - res[i]) / step;
                    let dk = (res_pk[i] - res[i]) / step_k;
                    acc += dj * dk;
                }
                jtj[j][k] = acc;
                jtj[k][j] = acc;
            }
        }

        let mut lhs = jtj;
        for (j, row) in lhs.iter_mut().enumerate() {
            row[j] += lambda * (row[j].max(1e-12));
        }
        let neg_jtr: Vec<f64> = jtr.iter().map(|v| -v).collect();
        let Some(delta) = solve_small_linear_system(lhs, neg_jtr) else {
            lambda *= 10.0;
            continue;
        };

        let mut cand: Vec<f64> = p.iter().zip(delta.iter()).map(|(a, d)| a + d).collect();
        project_parameters(family, &mut cand, req.enforce_passivity_monotonicity);
        let cand_res = normalized_residuals(family, &cand, &req.stimulus_x, &req.measured_y);
        let cand_cost: f64 = cand_res.iter().map(|r| r * r).sum();

        if cand_cost < cost {
            let rel_imp = (cost - cand_cost) / cost.max(1e-18);
            p = cand;
            res = cand_res;
            cost = cand_cost;
            lambda = (lambda * 0.3).max(1e-9);
            if rel_imp < 1e-9 {
                break;
            }
        } else {
            lambda = (lambda * 5.0).min(1e8);
        }
    }

    let rms_rel = (req
        .stimulus_x
        .iter()
        .zip(req.measured_y.iter())
        .map(|(&x, &y)| {
            let pred = eval_model(family, &p, x);
            let e = (pred - y) / y.abs().max(1e-15);
            e * e
        })
        .sum::<f64>()
        / (req.stimulus_x.len() as f64))
        .sqrt();

    let fitted_parameters = if family == CompactCurveFamily::FosterThermalZth {
        let r_th = p[0];
        let c_th = p[1] / r_th.max(1e-12);
        vec![r_th, c_th]
    } else {
        p
    };

    Ok(FitCompactModelCurveResponse {
        converged: rms_rel < 0.15,
        rms_relative_error: rms_rel,
        fitted_parameters,
    })
}
