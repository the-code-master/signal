// Module: engine/crates/signal-diff-sim/src/saltation.rs — Exact Filippov Sensitivity Saltation Matrix (I-01)
// Invariants: S = I + ((f+ - f-) * n^T) / (n^T * f- + dh/dt); guards grazing trajectories (I-01, F-034, F-113).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use crate::DiffSimError;

const GRAZING_DENOM_FLOOR: f64 = 1.0e-14;

/// Switching hyperplane crossing data at $h(x(t^*), t^*, p) = 0$.
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchingEvent {
    pub t_star: f64,
    pub f_minus: Vec<f64>,
    pub f_plus: Vec<f64>,
    pub normal_grad_x: Vec<f64>,
    pub dh_dt: f64,
    pub dh_dp: Vec<f64>,
}

/// Computes the exact **Filippov Sensitivity Saltation Matrix**:
/// $$S = I + \frac{(f_+ - f_-) n^T}{n^T f_- + \partial h / \partial t}$$
/// at a switching hyperplane $h(x, t) = 0$ (`I-01`, `F-034`).
pub fn compute_saltation_matrix(
    f_minus: &[f64],
    f_plus: &[f64],
    normal_n: &[f64],
    dh_dt: f64,
) -> Result<Vec<Vec<f64>>, DiffSimError> {
    let dim = f_minus.len();
    if dim == 0 || f_plus.len() != dim || normal_n.len() != dim {
        return Err(DiffSimError::DimensionMismatch(
            "saltation vector lengths mismatch",
        ));
    }

    let n_dot_f_minus: f64 = normal_n.iter().zip(f_minus).map(|(ni, fi)| ni * fi).sum();
    let denom = n_dot_f_minus + dh_dt;
    if denom.abs() < GRAZING_DENOM_FLOOR {
        return Err(DiffSimError::GrazingSwitchingSurface {
            transversality_denom: denom,
        });
    }

    let inv_denom = 1.0 / denom;
    let mut s_mat = vec![vec![0.0; dim]; dim];
    for (i, row) in s_mat.iter_mut().enumerate() {
        let df_i = (f_plus[i] - f_minus[i]) * inv_denom;
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = if i == j { 1.0 } else { 0.0 } + df_i * normal_n[j];
        }
    }
    Ok(s_mat)
}

/// Propagates forward tangent sensitivity $s_+ = S s_- + \frac{f_+ - f_-}{n^T f_- + \partial h/\partial t} \frac{\partial h}{\partial p}$.
pub fn apply_forward_saltation_jump(
    event: &SwitchingEvent,
    s_minus: &[f64],
    dh_dp_scalar: f64,
) -> Result<Vec<f64>, DiffSimError> {
    let s_mat = compute_saltation_matrix(
        &event.f_minus,
        &event.f_plus,
        &event.normal_grad_x,
        event.dh_dt,
    )?;
    let n_dot_fm: f64 = event
        .normal_grad_x
        .iter()
        .zip(&event.f_minus)
        .map(|(n, f)| n * f)
        .sum();
    let denom = n_dot_fm + event.dh_dt;
    let dim = s_minus.len();
    let mut s_plus = vec![0.0; dim];
    for i in 0..dim {
        let mut acc = 0.0;
        for j in 0..dim {
            acc += s_mat[i][j] * s_minus[j];
        }
        acc += ((event.f_plus[i] - event.f_minus[i]) / denom) * dh_dp_scalar;
        s_plus[i] = acc;
    }
    Ok(s_plus)
}

/// Propagates reverse-mode adjoint covector $\lambda(t^{*-}) = S^T \lambda(t^{*+})$ across switching event $t^*$ (`I-01`)
/// and returns `(lambda_minus, param_grad_crossing_delta)`.
pub fn apply_adjoint_saltation_jump(
    event: &SwitchingEvent,
    lambda_plus: &[f64],
) -> Result<(Vec<f64>, Vec<f64>), DiffSimError> {
    let s_mat = compute_saltation_matrix(
        &event.f_minus,
        &event.f_plus,
        &event.normal_grad_x,
        event.dh_dt,
    )?;
    let dim = lambda_plus.len();
    if dim != s_mat.len() {
        return Err(DiffSimError::DimensionMismatch(
            "adjoint lambda length mismatch",
        ));
    }

    // lambda_minus = S^T * lambda_plus
    let mut lambda_minus = vec![0.0; dim];
    for j in 0..dim {
        let mut acc = 0.0;
        for i in 0..dim {
            acc += s_mat[i][j] * lambda_plus[i];
        }
        lambda_minus[j] = acc;
    }

    let n_dot_fm: f64 = event
        .normal_grad_x
        .iter()
        .zip(&event.f_minus)
        .map(|(n, f)| n * f)
        .sum();
    let denom = n_dot_fm + event.dh_dt;
    let lam_dot_df: f64 = lambda_plus
        .iter()
        .zip(event.f_plus.iter().zip(&event.f_minus))
        .map(|(l, (fp, fm))| l * (fp - fm))
        .sum();
    let scale = lam_dot_df / denom;
    let dp_jump: Vec<f64> = event.dh_dp.iter().map(|&dh| scale * dh).collect();

    Ok((lambda_minus, dp_jump))
}
