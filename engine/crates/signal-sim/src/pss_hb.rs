// Module: engine/crates/signal-sim/src/pss_hb.rs — Shooting-Newton Periodic Steady-State (PSS) & Harmonic Balance (HB)
// Invariants: Solves (I - Phi(T,0)) dx0 = x(T; x0) - x0 via equilibrated LU; multi-harmonic Galerkin HB (F-028).
// Spec: architecture/03-rust-eda-engine.md
use crate::SimError;
use crate::circuit::MnaCircuit;
use crate::equilibrate_solver::{Complex64, solve_complex_lu, solve_equilibrated_lu};
use crate::mna_engine::step_tr_bdf2;
use signal_core::{det_cos, det_sin};

#[derive(Debug, Clone)]
pub struct PssResult {
    pub periodic_initial_state: Vec<f64>,
    pub period_times_s: Vec<f64>,
    pub period_states: Vec<Vec<f64>>,
    pub iterations: usize,
    pub closure_residual: f64,
}

fn integrate_fixed_period(
    ckt: &MnaCircuit,
    x0: &[f64],
    period_s: f64,
    steps_per_period: usize,
    temp_mk: i64,
) -> Result<(Vec<f64>, Vec<Vec<f64>>), SimError> {
    let n_steps = steps_per_period.max(8);
    let pwm_duty = ckt.elements.iter().find_map(|el| match el {
        crate::circuit::CircuitElement::PwmSwitch { duty, .. } if *duty > 0.0 && *duty < 1.0 => {
            Some(*duty)
        }
        _ => None,
    });

    let mut times = Vec::with_capacity(n_steps + 1);
    let mut states = Vec::with_capacity(n_steps + 1);
    times.push(0.0);
    states.push(x0.to_vec());
    let mut cur = x0.to_vec();

    if let Some(d) = pwm_duty {
        let n_on = ((n_steps as f64 * d).round() as usize).clamp(2, n_steps - 2);
        let n_off = n_steps - n_on;
        let t_sw = d * period_s;
        let h_on = t_sw / (n_on as f64);
        for k in 0..n_on {
            let t = (k as f64) * h_on;
            cur = step_tr_bdf2(ckt, t, h_on, &cur, temp_mk)?;
            times.push((k + 1) as f64 * h_on);
            states.push(cur.clone());
        }
        let h_off = (period_s - t_sw) / (n_off as f64);
        for k in 0..n_off {
            let t = t_sw + (k as f64) * h_off;
            cur = step_tr_bdf2(ckt, t, h_off, &cur, temp_mk)?;
            times.push(t_sw + (k + 1) as f64 * h_off);
            states.push(cur.clone());
        }
        return Ok((times, states));
    }

    let h = period_s / (n_steps as f64);
    for k in 0..n_steps {
        let t = (k as f64) * h;
        cur = step_tr_bdf2(ckt, t, h, &cur, temp_mk)?;
        times.push((k + 1) as f64 * h);
        states.push(cur.clone());
    }
    Ok((times, states))
}

/// **Shooting-Newton Periodic Steady-State (PSS)** solver (`F-028`).
/// Finds $x_0^*$ such that $x(T; x_0^*) - x_0^* = 0$ without brute-forcing thousands of lightly-damped startup cycles.
pub fn solve_shooting_pss(
    ckt: &MnaCircuit,
    period_s: f64,
    steps_per_period: usize,
    initial_guess: Option<&[f64]>,
    temp_mk: i64,
) -> Result<PssResult, SimError> {
    let dim = ckt.total_unknowns();
    let mut x0 = initial_guess
        .filter(|g| g.len() == dim)
        .map_or_else(|| vec![0.0; dim], <[f64]>::to_vec);

    let fd_eps = 1.0e-6;
    for iter in 1..=20 {
        let (times, states) =
            integrate_fixed_period(ckt, &x0, period_s, steps_per_period, temp_mk)?;
        let x_end = states
            .last()
            .ok_or(SimError::InvalidParameter("empty PSS period"))?;
        let mut r_vec = vec![0.0; dim];
        let mut max_err = 0.0_f64;
        for i in 0..dim {
            r_vec[i] = x_end[i] - x0[i];
            max_err = max_err.max(r_vec[i].abs());
        }
        if max_err < 1.0e-7 {
            return Ok(PssResult {
                periodic_initial_state: x0,
                period_times_s: times,
                period_states: states,
                iterations: iter,
                closure_residual: max_err,
            });
        }

        // Build shooting Jacobian J_pss = I - Phi(T, 0) where column j of Phi is d x(T)/d x0_j
        let mut j_pss = vec![vec![0.0; dim]; dim];
        for (i, row) in j_pss.iter_mut().enumerate().take(dim) {
            row[i] = 1.0;
        }
        for j in 0..dim {
            let mut x_pert = x0.clone();
            let delta = fd_eps * x0[j].abs().max(1.0);
            x_pert[j] += delta;
            let (_, st_pert) =
                integrate_fixed_period(ckt, &x_pert, period_s, steps_per_period, temp_mk)?;
            let x_pert_end = st_pert
                .last()
                .ok_or(SimError::InvalidParameter("empty pert"))?;
            for i in 0..dim {
                let phi_ij = (x_pert_end[i] - x_end[i]) / delta;
                j_pss[i][j] -= phi_ij;
            }
        }

        // Solve (I - Phi) * dx0 = x(T) - x0 => x0_next = x0 + dx0
        let dx0 = solve_equilibrated_lu(&j_pss, &r_vec)?;
        for j in 0..dim {
            x0[j] += dx0[j];
        }
    }

    let (times, states) = integrate_fixed_period(ckt, &x0, period_s, steps_per_period, temp_mk)?;
    let x_end = states
        .last()
        .ok_or(SimError::InvalidParameter("empty PSS"))?;
    let res = x_end
        .iter()
        .zip(&x0)
        .fold(0.0_f64, |m, (a, b)| m.max((a - b).abs()));
    Ok(PssResult {
        periodic_initial_state: x0,
        period_times_s: times,
        period_states: states,
        iterations: 20,
        closure_residual: res,
    })
}

#[derive(Debug, Clone)]
pub struct HarmonicBalanceResult {
    pub fundamental_hz: f64,
    pub harmonic_phasors: Vec<Vec<Complex64>>, // [harmonic k = 0..=K][unknown i]
}

/// Multi-harmonic **Harmonic Balance (HB)** solver over $k = 0 \dots K$ harmonics (`F-028`).
/// Uses $M = 2K + 1$ uniform time-domain collocation points + Picard/Newton spectral admittance iteration.
pub fn solve_harmonic_balance(
    ckt: &MnaCircuit,
    fundamental_hz: f64,
    num_harmonics: usize,
    temp_mk: i64,
) -> Result<HarmonicBalanceResult, SimError> {
    if fundamental_hz <= 0.0 || num_harmonics == 0 {
        return Err(SimError::InvalidParameter("HB requires f0 > 0 and K >= 1"));
    }
    let dim = ckt.total_unknowns();
    let k_max = num_harmonics;
    let m_pts = 2 * k_max + 1;
    let period_s = 1.0 / fundamental_hz;

    // Linearize reference G0, C0 around x = 0 so Y(k*w0) = G0 + j*k*w0*C0
    let zero = vec![0.0; dim];
    let base = ckt.assemble(&zero, 0.0, temp_mk, 1.0e-12, 0.0);
    let mut x_spec = vec![vec![Complex64::ZERO; dim]; k_max + 1];

    for _iter in 0..30 {
        // Inverse DFT: reconstruct time-domain samples x(t_m) for m = 0..M
        let mut x_time = vec![vec![0.0; dim]; m_pts];
        for (m, xm) in x_time.iter_mut().enumerate() {
            let phase_base = std::f64::consts::TAU * (m as f64) / (m_pts as f64);
            for i in 0..dim {
                let mut val = x_spec[0][i].re;
                for (k, xk) in x_spec.iter().enumerate().skip(1) {
                    let ang = (k as f64) * phase_base;
                    val += 2.0 * (xk[i].re * det_cos(ang) - xk[i].im * det_sin(ang));
                }
                xm[i] = val;
            }
        }

        // Evaluate nonlinear excess current f_nl(x(t_m), t_m) = f(x_m, t_m) - G0 * x_m
        let mut fnl_time = vec![vec![0.0; dim]; m_pts];
        for m in 0..m_pts {
            let t_m = (m as f64) * period_s / (m_pts as f64);
            let st_m = ckt.assemble(&x_time[m], t_m, temp_mk, 1.0e-12, 1.0);
            for i in 0..dim {
                let mut g0_x = 0.0;
                for j in 0..dim {
                    g0_x += base.g_jacobian[i][j] * x_time[m][j];
                }
                fnl_time[m][i] = st_m.f_static[i] - g0_x;
            }
        }

        // Forward DFT of -f_nl and solve Y(k*w0) * X_k_new = -F_nl_k with damped relaxation
        let mut max_delta = 0.0_f64;
        for (k, xk_slot) in x_spec.iter_mut().enumerate() {
            let omega_k = std::f64::consts::TAU * (k as f64) * fundamental_hz;
            let mut rhs_k = vec![Complex64::ZERO; dim];
            for (m, fnl_m) in fnl_time.iter().enumerate() {
                let ang = std::f64::consts::TAU * (k as f64) * (m as f64) / (m_pts as f64);
                let c_ang = det_cos(ang);
                let s_ang = -det_sin(ang);
                for i in 0..dim {
                    let scale = -fnl_m[i] / (m_pts as f64);
                    rhs_k[i] = rhs_k[i] + Complex64::new(scale * c_ang, scale * s_ang);
                }
            }
            let mut y_k = vec![vec![Complex64::ZERO; dim]; dim];
            for i in 0..dim {
                for j in 0..dim {
                    y_k[i][j] =
                        Complex64::new(base.g_jacobian[i][j], omega_k * base.c_jacobian[i][j]);
                }
            }
            let sol_k = solve_complex_lu(y_k, rhs_k)?;
            for i in 0..dim {
                let diff = (sol_k[i] - xk_slot[i]).mag();
                max_delta = max_delta.max(diff);
                xk_slot[i] = Complex64::new(
                    0.35 * xk_slot[i].re + 0.65 * sol_k[i].re,
                    0.35 * xk_slot[i].im + 0.65 * sol_k[i].im,
                );
            }
        }
        if max_delta < 1.0e-8 {
            break;
        }
    }

    Ok(HarmonicBalanceResult {
        fundamental_hz,
        harmonic_phasors: x_spec,
    })
}
