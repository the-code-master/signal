// Module: engine/crates/signal-component/src/dual.rs — Forward-Mode Dual64 Automatic Differentiation
// Invariants: Lifts core_math (det_exp, det_ln, det_tanh, limexp_c2) for exact cross-ISA AD Jacobians (F-024, F-033).
// Spec: architecture/03-rust-eda-engine.md
use signal_core::core_math::{det_cos, det_exp, det_ln, det_sin, det_tanh, limexp_c2};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// First-order forward automatic differentiation scalar $a + b \varepsilon$ ($\varepsilon^2 = 0$).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Dual64 {
    pub val: f64,
    pub eps: f64,
}

impl Dual64 {
    pub const fn constant(val: f64) -> Self {
        Self { val, eps: 0.0 }
    }

    pub const fn variable(val: f64) -> Self {
        Self { val, eps: 1.0 }
    }

    pub fn exp(self) -> Self {
        let ex = det_exp(self.val);
        Self {
            val: ex,
            eps: ex * self.eps,
        }
    }

    pub fn limexp(self, x_lim: f64) -> Self {
        let (v, d1, _) = limexp_c2(self.val, x_lim);
        Self {
            val: v,
            eps: d1 * self.eps,
        }
    }

    pub fn ln(self) -> Self {
        Self {
            val: det_ln(self.val),
            eps: self.eps / self.val,
        }
    }

    pub fn tanh(self) -> Self {
        let t = det_tanh(self.val);
        Self {
            val: t,
            eps: (1.0 - t * t) * self.eps,
        }
    }

    pub fn sin(self) -> Self {
        Self {
            val: det_sin(self.val),
            eps: det_cos(self.val) * self.eps,
        }
    }

    pub fn cos(self) -> Self {
        Self {
            val: det_cos(self.val),
            eps: -det_sin(self.val) * self.eps,
        }
    }

    pub fn sqrt(self) -> Self {
        let s = self.val.sqrt();
        Self {
            val: s,
            eps: if s > 0.0 { self.eps / (2.0 * s) } else { 0.0 },
        }
    }

    pub fn ln1p_exp_softplus(self) -> Self {
        if self.val > 35.0 {
            self
        } else if self.val < -35.0 {
            let ex = det_exp(self.val);
            Self {
                val: ex,
                eps: ex * self.eps,
            }
        } else {
            (Self::constant(1.0) + self.exp()).ln()
        }
    }
}

impl Add for Dual64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            val: self.val + rhs.val,
            eps: self.eps + rhs.eps,
        }
    }
}

impl Sub for Dual64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            val: self.val - rhs.val,
            eps: self.eps - rhs.eps,
        }
    }
}

impl Mul for Dual64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            val: self.val * rhs.val,
            eps: self.val * rhs.eps + self.eps * rhs.val,
        }
    }
}

impl Div for Dual64 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let inv = 1.0 / rhs.val;
        Self {
            val: self.val * inv,
            eps: (self.eps * rhs.val - self.val * rhs.eps) * inv * inv,
        }
    }
}

impl Neg for Dual64 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            val: -self.val,
            eps: -self.eps,
        }
    }
}

/// Evaluates `f(x)` and its dense Jacobian $J_{ij} = \partial f_i / \partial x_j$ via forward-mode `Dual64`.
pub fn forward_ad_jacobian<F>(x: &[f64], out_dim: usize, f: F) -> (Vec<f64>, Vec<Vec<f64>>)
where
    F: Fn(&[Dual64]) -> Vec<Dual64>,
{
    let n = x.len();
    let mut base_vals = vec![0.0; out_dim];
    let mut jac = vec![vec![0.0; n]; out_dim];
    let mut dual_in: Vec<Dual64> = x.iter().map(|&v| Dual64::constant(v)).collect();

    if n == 0 {
        let res = f(&dual_in);
        for (i, r) in res.iter().take(out_dim).enumerate() {
            base_vals[i] = r.val;
        }
        return (base_vals, jac);
    }

    for j in 0..n {
        dual_in[j].eps = 1.0;
        let res = f(&dual_in);
        dual_in[j].eps = 0.0;
        for (i, r) in res.iter().take(out_dim).enumerate() {
            if j == 0 {
                base_vals[i] = r.val;
            }
            jac[i][j] = r.eps;
        }
    }
    (base_vals, jac)
}
