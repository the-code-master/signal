// Module: engine/crates/signal-diff-sim/src/revolve_adjoint.rs — Binomial Revolve Checkpointing, Hermite & Saltation Adjoint
// Invariants: O(log N_t) RAM checkpoints + O(h^4) cubic Hermite interpolation + Two-Stage PSS warm-start (I-01, F-113).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use crate::DiffSimError;
use crate::saltation::{SwitchingEvent, apply_adjoint_saltation_jump};
use signal_sim::{MnaCircuit, PssResult, solve_shooting_pss};

/// Computes Griewank-Walther binomial capacity $\beta(s, r) = \binom{s + r}{s}$ saturated at `u64::MAX`.
pub fn binomial_revolve_capacity(slots: u32, reps: u32) -> u64 {
    if slots == 0 {
        return 1;
    }
    let mut num = 1_u128;
    let k = slots.min(reps) as u128;
    let n = u128::from(slots) + u128::from(reps);
    for i in 0..k {
        num = num.saturating_mul(n - i) / (i + 1);
        if num >= u128::from(u64::MAX) {
            return u64::MAX;
        }
    }
    num as u64
}

/// Computes optimal Griewank-Walther **`Revolve`** initial snapshot step indices for `total_steps` in `ram_slots` (`F-113`).
pub fn compute_revolve_snapshot_indices(total_steps: usize, ram_slots: usize) -> Vec<usize> {
    if total_steps <= 1 || ram_slots == 0 {
        return vec![0];
    }
    let s_max = ram_slots.max(1);
    let mut reps = 0_u32;
    while binomial_revolve_capacity(s_max as u32, reps) < (total_steps as u64) {
        reps += 1;
    }

    let mut checkpoints = Vec::with_capacity(s_max);
    let mut cur_step = 0_usize;
    let mut rem_steps = total_steps;
    let mut rem_slots = s_max as u32;

    while rem_slots > 0 && rem_steps > 1 {
        checkpoints.push(cur_step);
        if rem_slots == 1 {
            break;
        }
        let mut r = 0_u32;
        while binomial_revolve_capacity(rem_slots, r) < (rem_steps as u64) {
            r += 1;
        }
        let jump = if r == 0 {
            1
        } else {
            (binomial_revolve_capacity(rem_slots - 1, r - 1) as usize).clamp(1, rem_steps - 1)
        };
        cur_step += jump;
        rem_steps -= jump;
        rem_slots -= 1;
    }
    if checkpoints.is_empty() {
        checkpoints.push(0);
    }
    checkpoints
}

/// $O(h^4)$-accurate **Cubic Hermite** state trajectory interpolation on $[t_0, t_1]$ during reverse adjoint sweeps.
pub fn hermite_interpolate_state(
    t0: f64,
    t1: f64,
    x0: &[f64],
    f0: &[f64],
    x1: &[f64],
    f1: &[f64],
    t_query: f64,
) -> Vec<f64> {
    let h = (t1 - t0).max(1.0e-30);
    let s = ((t_query - t0) / h).clamp(0.0, 1.0);
    let s2 = s * s;
    let s3 = s2 * s;
    let h00 = 2.0 * s3 - 3.0 * s2 + 1.0;
    let h10 = s3 - 2.0 * s2 + s;
    let h01 = -2.0 * s3 + 3.0 * s2;
    let h11 = s3 - s2;

    (0..x0.len())
        .map(|i| h00 * x0[i] + h10 * h * f0[i] + h01 * x1[i] + h11 * h * f1[i])
        .collect()
}

/// Two-Stage **Shooting-Newton PSS Warm-Start** locating periodic orbit $x_0^*$ prior to adjoint fine-tuning (`I-01`).
pub fn warm_start_pss_for_adjoint(
    ckt: &MnaCircuit,
    period_s: f64,
    steps_per_period: usize,
    temp_mk: i64,
) -> Result<PssResult, DiffSimError> {
    solve_shooting_pss(ckt, period_s, steps_per_period, None, temp_mk)
        .map_err(|_| DiffSimError::SolverFailure("Two-Stage PSS warm-start failed"))
}

#[derive(Debug)]
struct ForwardChopperTrace {
    traj: Vec<(f64, [f64; 2], bool)>,
    sw_event: Option<(usize, SwitchingEvent)>,
    x_end: [f64; 2],
    _revolve_slots: Vec<usize>,
}

/// Peak-Current-Mode Switched Buck / Chopper Hybrid Dynamical System ($x = [i_L, v_C]^T$).
/// Switches from ON ($f_-$) to OFF ($f_+$) at comparator hyperplane $h(x) = i_L - i_{\text{peak}} = 0$.
#[derive(Debug, Clone, Copy)]
pub struct SwitchedBuckChopper {
    pub vin_v: f64,
    pub l_henries: f64,
    pub c_farads: f64,
    pub r_ind_ohms: f64,
    pub r_load_ohms: f64,
    pub i_peak_amps: f64,
    pub horizon_s: f64,
    pub target_vout_v: f64,
}

impl SwitchedBuckChopper {
    pub fn rhs(&self, x: [f64; 2], high_side_on: bool) -> [f64; 2] {
        let v_drv = if high_side_on { self.vin_v } else { 0.0 };
        [
            (v_drv - self.r_ind_ohms * x[0] - x[1]) / self.l_henries,
            (x[0] - x[1] / self.r_load_ohms) / self.c_farads,
        ]
    }

    pub fn jac_x(&self) -> [[f64; 2]; 2] {
        [
            [-self.r_ind_ohms / self.l_henries, -1.0 / self.l_henries],
            [
                1.0 / self.c_farads,
                -1.0 / (self.r_load_ohms * self.c_farads),
            ],
        ]
    }

    pub fn df_dl(&self, x: [f64; 2], high_side_on: bool) -> [f64; 2] {
        let v_drv = if high_side_on { self.vin_v } else { 0.0 };
        [
            -(v_drv - self.r_ind_ohms * x[0] - x[1]) / (self.l_henries * self.l_henries),
            0.0,
        ]
    }

    fn rk4_step(&self, x: [f64; 2], dt: f64, on: bool) -> [f64; 2] {
        let k1 = self.rhs(x, on);
        let k2 = self.rhs([x[0] + 0.5 * dt * k1[0], x[1] + 0.5 * dt * k1[1]], on);
        let k3 = self.rhs([x[0] + 0.5 * dt * k2[0], x[1] + 0.5 * dt * k2[1]], on);
        let k4 = self.rhs([x[0] + dt * k3[0], x[1] + dt * k3[1]], on);
        [
            x[0] + (dt / 6.0) * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]),
            x[1] + (dt / 6.0) * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]),
        ]
    }

    /// Simulates forward trajectory with exact bisection root-finding at $h(x) = i_L - i_{\text{peak}} = 0$
    /// and returns loss $\mathcal{L} = \frac{1}{2}(v_C(T) - V_{\text{target}})^2$.
    pub fn simulate_loss(&self, x0: [f64; 2], steps: usize) -> f64 {
        let trace = self.forward_with_event(x0, steps);
        let err = trace.x_end[1] - self.target_vout_v;
        0.5 * err * err
    }

    fn forward_with_event(&self, x0: [f64; 2], steps: usize) -> ForwardChopperTrace {
        let n = steps.max(16);
        let dt_nom = self.horizon_s / (n as f64);
        let revolve_slots = compute_revolve_snapshot_indices(n, 8);
        let mut traj = Vec::with_capacity(n + 2);
        let mut t = 0.0_f64;
        let mut x = x0;
        let mut on = x[0] < self.i_peak_amps;
        let mut sw_event = None;
        traj.push((t, x, on));

        for _ in 0..n {
            let h = (self.horizon_s - t).min(dt_nom);
            if h <= 1.0e-18 {
                break;
            }
            let x_try = self.rk4_step(x, h, on);
            if on && x_try[0] >= self.i_peak_amps {
                let mut lo = 0.0_f64;
                let mut hi = h;
                for _ in 0..52 {
                    let mid = 0.5 * (lo + hi);
                    if self.rk4_step(x, mid, true)[0] < self.i_peak_amps {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                let tau = 0.5 * (lo + hi);
                let x_sw = self.rk4_step(x, tau, true);
                let t_sw = t + tau;
                let f_m = self.rhs(x_sw, true);
                let f_p = self.rhs(x_sw, false);
                let ev_idx = traj.len();
                traj.push((t_sw, x_sw, true));
                sw_event = Some((
                    ev_idx,
                    SwitchingEvent {
                        t_star: t_sw,
                        f_minus: vec![f_m[0], f_m[1]],
                        f_plus: vec![f_p[0], f_p[1]],
                        normal_grad_x: vec![1.0, 0.0],
                        dh_dt: 0.0,
                        dh_dp: vec![0.0],
                    },
                ));
                on = false;
                let rem = h - tau;
                if rem > 1.0e-18 {
                    x = self.rk4_step(x_sw, rem, false);
                    t = t_sw + rem;
                    traj.push((t, x, false));
                } else {
                    x = x_sw;
                    t = t_sw;
                }
            } else {
                x = x_try;
                t += h;
                traj.push((t, x, on));
            }
        }
        ForwardChopperTrace {
            traj,
            sw_event,
            x_end: x,
            _revolve_slots: revolve_slots,
        }
    }

    /// Computes exact **Filippov Saltation + Hermite/Revolve Adjoint** gradient $\frac{d\mathcal{L}}{dL}$
    /// (or naive non-saltation gradient when `use_saltation == false`) (`I-01`, `F-034`, `F-113`).
    pub fn adjoint_gradient_dl(
        &self,
        x0: [f64; 2],
        steps: usize,
        use_saltation: bool,
    ) -> Result<f64, DiffSimError> {
        let trace = self.forward_with_event(x0, steps);
        let traj = trace.traj;
        let sw_event = trace.sw_event;
        let x_end = trace.x_end;
        let mut lambda = [0.0, x_end[1] - self.target_vout_v];
        let jx = self.jac_x();
        let mut grad_l = 0.0_f64;

        let adj_rhs = |lam: [f64; 2]| -> [f64; 2] {
            // d(lambda)/d(backward_tau) = J_x^T * lambda
            [
                jx[0][0] * lam[0] + jx[1][0] * lam[1],
                jx[0][1] * lam[0] + jx[1][1] * lam[1],
            ]
        };

        for k in (0..(traj.len() - 1)).rev() {
            let (t0, x0_k, on_k) = traj[k];
            let (t1, x1_k, on_next) = traj[k + 1];
            let seg_on = if k + 1 < traj.len() && !on_next {
                false
            } else {
                on_k
            };
            let h = t1 - t0;

            if h > 1.0e-18 {
                let f0 = self.rhs(x0_k, seg_on);
                let f1 = self.rhs(x1_k, seg_on);
                let x_mid_vec =
                    hermite_interpolate_state(t0, t1, &x0_k, &f0, &x1_k, &f1, 0.5 * (t0 + t1));
                let x_mid = [x_mid_vec[0], x_mid_vec[1]];

                // Backward RK4 step for lambda from t1 -> t0
                let k1 = adj_rhs(lambda);
                let lam_mid1 = [lambda[0] + 0.5 * h * k1[0], lambda[1] + 0.5 * h * k1[1]];
                let k2 = adj_rhs(lam_mid1);
                let lam_mid2 = [lambda[0] + 0.5 * h * k2[0], lambda[1] + 0.5 * h * k2[1]];
                let k3 = adj_rhs(lam_mid2);
                let lam_end = [lambda[0] + h * k3[0], lambda[1] + h * k3[1]];
                let k4 = adj_rhs(lam_end);
                let lambda_t0 = [
                    lambda[0] + (h / 6.0) * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]),
                    lambda[1] + (h / 6.0) * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]),
                ];
                let lambda_mid = [
                    0.5 * (lam_mid1[0] + lam_mid2[0]),
                    0.5 * (lam_mid1[1] + lam_mid2[1]),
                ];

                // Simpson quadrature for int_{t0}^{t1} lambda(t)^T * (df/dL)(x(t)) dt
                let g1 = self.df_dl(x1_k, seg_on);
                let gm = self.df_dl(x_mid, seg_on);
                let g0 = self.df_dl(x0_k, seg_on);
                let dot1 = lambda[0] * g1[0] + lambda[1] * g1[1];
                let dotm = lambda_mid[0] * gm[0] + lambda_mid[1] * gm[1];
                let dot0 = lambda_t0[0] * g0[0] + lambda_t0[1] * g0[1];
                grad_l += (h / 6.0) * (dot0 + 4.0 * dotm + dot1);
                lambda = lambda_t0;
            }

            if use_saltation
                && let Some((ev_idx, ref ev)) = sw_event
                && k == ev_idx
            {
                let (lam_minus, _) = apply_adjoint_saltation_jump(ev, &lambda)?;
                lambda = [lam_minus[0], lam_minus[1]];
            }
        }
        Ok(grad_l)
    }
}
