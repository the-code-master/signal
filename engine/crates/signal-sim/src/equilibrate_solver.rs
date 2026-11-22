// Module: engine/crates/signal-sim/src/equilibrate_solver.rs — Ruiz Equilibration, Compensated LU & KLU<->cuDSS Dispatcher
// Invariants: Iterative Ruiz row/col inf-norm scaling + Neumaier compensated iterative refinement (F-023, F-116).
// Spec: architecture/03-rust-eda-engine.md
use crate::{GPU_CUDSS_NODE_CROSSOVER, LinearSolverBackend, SimError};

pub const GPU_MONTE_CARLO_BATCH_CROSSOVER: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinearSolverDispatcher {
    pub gpu_available: bool,
}

impl LinearSolverDispatcher {
    pub const fn dispatch(
        &self,
        node_count: usize,
        monte_carlo_batch: usize,
    ) -> LinearSolverBackend {
        if self.gpu_available
            && (node_count >= GPU_CUDSS_NODE_CROSSOVER
                || monte_carlo_batch >= GPU_MONTE_CARLO_BATCH_CROSSOVER)
        {
            LinearSolverBackend::GpuCuDssMixedPrecision
        } else {
            LinearSolverBackend::CpuKluRuizEquilibrated
        }
    }
}

/// Neumaier compensated dot product avoiding catastrophic cancellation on $10^{12}$ conductance spreads.
#[inline]
pub fn neumaier_dot(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0_f64;
    let mut comp = 0.0_f64;
    for (&ai, &bi) in a.iter().zip(b.iter()) {
        let prod = ai * bi;
        let t = sum + prod;
        if sum.abs() >= prod.abs() {
            comp += (sum - t) + prod;
        } else {
            comp += (prod - t) + sum;
        }
        sum = t;
    }
    sum + comp
}

#[derive(Debug, Clone)]
pub struct RuizEquilibratedSystem {
    pub scaled_matrix: Vec<Vec<f64>>,
    pub row_scale: Vec<f64>,
    pub col_scale: Vec<f64>,
}

/// Iterative Ruiz row/column infinity-norm equilibration ($D_r A D_c$) (`F-023`).
pub fn ruiz_equilibrate(matrix: &[Vec<f64>], max_iters: usize) -> RuizEquilibratedSystem {
    let n = matrix.len();
    let mut a = matrix.to_vec();
    let mut d_r = vec![1.0_f64; n];
    let mut d_c = vec![1.0_f64; n];

    for _ in 0..max_iters.max(1) {
        let mut r_norm = vec![0.0_f64; n];
        let mut c_norm = vec![0.0_f64; n];
        for i in 0..n {
            for j in 0..n {
                let v = a[i][j].abs();
                if v > r_norm[i] {
                    r_norm[i] = v;
                }
                if v > c_norm[j] {
                    c_norm[j] = v;
                }
            }
        }
        let mut converged = true;
        for i in 0..n {
            if r_norm[i] > 1.0e-300 && (1.0 - r_norm[i]).abs() > 1.0e-3 {
                converged = false;
            }
            if c_norm[i] > 1.0e-300 && (1.0 - c_norm[i]).abs() > 1.0e-3 {
                converged = false;
            }
        }
        for i in 0..n {
            let sr = if r_norm[i] > 1.0e-300 {
                1.0 / r_norm[i].sqrt()
            } else {
                1.0
            };
            d_r[i] *= sr;
            for cell in a[i].iter_mut().take(n) {
                *cell *= sr;
            }
        }
        for j in 0..n {
            let sc = if c_norm[j] > 1.0e-300 {
                1.0 / c_norm[j].sqrt()
            } else {
                1.0
            };
            d_c[j] *= sc;
            for row in a.iter_mut().take(n) {
                row[j] *= sc;
            }
        }
        if converged {
            break;
        }
    }

    RuizEquilibratedSystem {
        scaled_matrix: a,
        row_scale: d_r,
        col_scale: d_c,
    }
}

fn lu_factor_in_place(a: &mut [Vec<f64>]) -> Result<Vec<usize>, SimError> {
    let n = a.len();
    let mut piv: Vec<usize> = (0..n).collect();
    for k in 0..n {
        let mut max_row = k;
        let mut max_val = a[k][k].abs();
        for (r, row) in a.iter().enumerate().skip(k + 1) {
            if row[k].abs() > max_val {
                max_val = row[k].abs();
                max_row = r;
            }
        }
        if max_val < 1.0e-28 {
            return Err(SimError::SingularMatrix { pivot_index: k });
        }
        if max_row != k {
            a.swap(k, max_row);
            piv.swap(k, max_row);
        }
        let inv_piv = 1.0 / a[k][k];
        for i in (k + 1)..n {
            let factor = a[i][k] * inv_piv;
            a[i][k] = factor;
            for j in (k + 1)..n {
                a[i][j] -= factor * a[k][j];
            }
        }
    }
    Ok(piv)
}

fn lu_backsolve(lu: &[Vec<f64>], piv: &[usize], rhs: &[f64]) -> Vec<f64> {
    let n = lu.len();
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut acc = rhs[piv[i]];
        for j in 0..i {
            acc -= lu[i][j] * y[j];
        }
        y[i] = acc;
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut acc = y[i];
        for j in (i + 1)..n {
            acc -= lu[i][j] * x[j];
        }
        x[i] = acc / lu[i][i];
    }
    x
}

/// Solves $A x = b$ using Ruiz equilibration + LU + Neumaier compensated iterative refinement (`F-023`).
pub fn solve_equilibrated_lu(matrix: &[Vec<f64>], rhs: &[f64]) -> Result<Vec<f64>, SimError> {
    let n = matrix.len();
    if n == 0 {
        return Ok(Vec::new());
    }
    let eq = ruiz_equilibrate(matrix, 6);
    let mut lu = eq.scaled_matrix;
    let piv = lu_factor_in_place(&mut lu)?;

    let b_scaled: Vec<f64> = (0..n).map(|i| eq.row_scale[i] * rhs[i]).collect();
    let y0 = lu_backsolve(&lu, &piv, &b_scaled);
    let mut x: Vec<f64> = (0..n).map(|j| eq.col_scale[j] * y0[j]).collect();

    // 2 passes of compensated-residual defect correction
    for _ in 0..2 {
        let mut r_scaled = vec![0.0; n];
        let mut max_res = 0.0_f64;
        for i in 0..n {
            let ax_i = neumaier_dot(&matrix[i], &x);
            let ri = rhs[i] - ax_i;
            max_res = max_res.max(ri.abs());
            r_scaled[i] = eq.row_scale[i] * ri;
        }
        if max_res < 1.0e-15 {
            break;
        }
        let dy = lu_backsolve(&lu, &piv, &r_scaled);
        for j in 0..n {
            x[j] += eq.col_scale[j] * dy[j];
        }
    }
    Ok(x)
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };

    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn abs_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn mag(self) -> f64 {
        self.abs_sq().sqrt()
    }
}

impl std::ops::Add for Complex64 {
    type Output = Self;
    fn add(self, o: Self) -> Self::Output {
        Self::new(self.re + o.re, self.im + o.im)
    }
}

impl std::ops::Sub for Complex64 {
    type Output = Self;
    fn sub(self, o: Self) -> Self::Output {
        Self::new(self.re - o.re, self.im - o.im)
    }
}

impl std::ops::Mul for Complex64 {
    type Output = Self;
    fn mul(self, o: Self) -> Self::Output {
        Self::new(
            self.re * o.re - self.im * o.im,
            self.re * o.im + self.im * o.re,
        )
    }
}

impl std::ops::Div for Complex64 {
    type Output = Self;
    fn div(self, o: Self) -> Self::Output {
        let d = o.abs_sq().max(1.0e-300);
        Self::new(
            (self.re * o.re + self.im * o.im) / d,
            (self.im * o.re - self.re * o.im) / d,
        )
    }
}

/// Complex dense LU solver for AC small-signal and Harmonic Balance (`(G + j*w*C) X = B`).
pub fn solve_complex_lu(
    mut a: Vec<Vec<Complex64>>,
    mut b: Vec<Complex64>,
) -> Result<Vec<Complex64>, SimError> {
    let n = a.len();
    for k in 0..n {
        let mut max_r = k;
        let mut max_v = a[k][k].abs_sq();
        for (r, row) in a.iter().enumerate().skip(k + 1) {
            if row[k].abs_sq() > max_v {
                max_v = row[k].abs_sq();
                max_r = r;
            }
        }
        if max_v < 1.0e-56 {
            return Err(SimError::SingularMatrix { pivot_index: k });
        }
        if max_r != k {
            a.swap(k, max_r);
            b.swap(k, max_r);
        }
        let piv = a[k][k];
        for i in (k + 1)..n {
            let f = a[i][k] / piv;
            for j in k..n {
                a[i][j] = a[i][j] - f * a[k][j];
            }
            b[i] = b[i] - f * b[k];
        }
    }
    let mut x = vec![Complex64::ZERO; n];
    for i in (0..n).rev() {
        let mut acc = b[i];
        for j in (i + 1)..n {
            acc = acc - a[i][j] * x[j];
        }
        x[i] = acc / a[i][i];
    }
    Ok(x)
}
