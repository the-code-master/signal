// Module: engine/crates/signal-sim/src/mna_engine.rs — Newton DC-OP, Complex AC Sweep & Adaptive TR-BDF2 Transient
// Invariants: Gmin/source-stepped DC-OP (F-025); exact 2-stage charge/flux-conserving L-stable TR-BDF2 (F-026, F-027).
// Spec: architecture/03-rust-eda-engine.md
use crate::SimError;
use crate::circuit::{CircuitElement, MnaCircuit, pnjlim};
use crate::equilibrate_solver::{Complex64, solve_complex_lu, solve_equilibrated_lu};

#[derive(Debug, Clone)]
pub struct AcPointResult {
    pub freq_hz: f64,
    pub node_phasors: Vec<Complex64>,
}

#[derive(Debug, Clone)]
pub struct TransientTrajectory {
    pub times_s: Vec<f64>,
    pub states: Vec<Vec<f64>>,
}

fn apply_diode_pnjlim_damping(ckt: &MnaCircuit, x_old: &[f64], x_cand: &mut [f64], temp_mk: i64) {
    let vt = signal_lib_semiconductor::ShockleyDiode::thermal_voltage(temp_mk);
    let v_crit = 0.65;
    for el in &ckt.elements {
        if let CircuitElement::Diode { na, nc, .. } = el {
            let vd_old = MnaCircuit::node_v(x_old, *na) - MnaCircuit::node_v(x_old, *nc);
            let vd_new = MnaCircuit::node_v(x_cand, *na) - MnaCircuit::node_v(x_cand, *nc);
            let vd_lim = pnjlim(vd_new, vd_old, vt, v_crit);
            let corr = vd_lim - vd_new;
            if corr.abs() > 1.0e-15 {
                if *na > 0 && *nc == 0 {
                    x_cand[*na - 1] += corr;
                } else if *nc > 0 && *na == 0 {
                    x_cand[*nc - 1] -= corr;
                } else if *na > 0 && *nc > 0 {
                    x_cand[*na - 1] += 0.5 * corr;
                    x_cand[*nc - 1] -= 0.5 * corr;
                }
            }
        }
    }
}

fn newton_dc_stage(
    ckt: &MnaCircuit,
    x_init: &[f64],
    temp_mk: i64,
    gmin: f64,
    src_scale: f64,
    max_iters: usize,
) -> Result<Vec<f64>, SimError> {
    let mut x = x_init.to_vec();
    for _ in 0..max_iters {
        let st = ckt.assemble(&x, 0.0, temp_mk, gmin, src_scale);
        let max_f = st.f_static.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        if max_f < 1.0e-11 {
            return Ok(x);
        }
        let neg_f: Vec<f64> = st.f_static.iter().map(|v| -*v).collect();
        let dx = solve_equilibrated_lu(&st.g_jacobian, &neg_f)?;
        let max_dx = dx.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        let mut x_next: Vec<f64> = x.iter().zip(&dx).map(|(xi, dxi)| xi + dxi).collect();
        apply_diode_pnjlim_damping(ckt, &x, &mut x_next, temp_mk);
        x = x_next;
        if max_dx < 1.0e-12 && max_f < 1.0e-9 {
            return Ok(x);
        }
    }
    Err(SimError::NewtonNonConvergence(
        "DC operating point stage exceeded max_iters",
    ))
}

/// Solves DC Operating Point with `pnjlim` + Gmin stepping + Source stepping homotopy (`F-025`).
pub fn solve_dc_op(
    ckt: &MnaCircuit,
    warm_guess: Option<&[f64]>,
    temp_mk: i64,
) -> Result<Vec<f64>, SimError> {
    let dim = ckt.total_unknowns();
    let x0 = warm_guess
        .filter(|g| g.len() == dim)
        .map_or_else(|| vec![0.0; dim], <[f64]>::to_vec);

    if let Ok(sol) = newton_dc_stage(ckt, &x0, temp_mk, 1.0e-12, 1.0, 60) {
        return Ok(sol);
    }

    // Homotopy fallback 1: Gmin stepping from 1e-3 S -> 1e-12 S
    let mut x_gmin = x0.clone();
    let mut gmin_ok = true;
    for &gmin in &[1.0e-3, 1.0e-5, 1.0e-7, 1.0e-9, 1.0e-12] {
        match newton_dc_stage(ckt, &x_gmin, temp_mk, gmin, 1.0, 50) {
            Ok(sol) => x_gmin = sol,
            Err(_) => {
                gmin_ok = false;
                break;
            }
        }
    }
    if gmin_ok {
        return Ok(x_gmin);
    }

    // Homotopy fallback 2: Source stepping alpha = 0.2 -> 1.0
    let mut x_src = vec![0.0; dim];
    for step in 1..=5 {
        let alpha = f64::from(step) * 0.2;
        x_src = newton_dc_stage(ckt, &x_src, temp_mk, 1.0e-11, alpha, 60)?;
    }
    newton_dc_stage(ckt, &x_src, temp_mk, 1.0e-12, 1.0, 40)
}

/// Solves linearized small-signal AC sweep $(G_{\text{op}} + j \omega C_{\text{op}}) X(\omega) = B_{\text{ac}}$.
pub fn solve_ac_sweep(
    ckt: &MnaCircuit,
    freqs_hz: &[f64],
    temp_mk: i64,
) -> Result<Vec<AcPointResult>, SimError> {
    let x_op = solve_dc_op(ckt, None, temp_mk)?;
    let op_state = ckt.assemble(&x_op, 0.0, temp_mk, 1.0e-12, 1.0);
    let rhs = ckt.assemble_ac_rhs();
    let dim = ckt.total_unknowns();

    let mut out = Vec::with_capacity(freqs_hz.len());
    for &freq in freqs_hz {
        let omega = std::f64::consts::TAU * freq;
        let mut y_mat = vec![vec![Complex64::ZERO; dim]; dim];
        for i in 0..dim {
            for j in 0..dim {
                y_mat[i][j] =
                    Complex64::new(op_state.g_jacobian[i][j], omega * op_state.c_jacobian[i][j]);
            }
        }
        let sol = solve_complex_lu(y_mat, rhs.clone())?;
        out.push(AcPointResult {
            freq_hz: freq,
            node_phasors: sol,
        });
    }
    Ok(out)
}

/// Solves one 2-stage **TR-BDF2** step from `(t_n, x_n)` to `t_n + h` (`F-026`).
pub fn step_tr_bdf2(
    ckt: &MnaCircuit,
    t_n: f64,
    h: f64,
    x_n: &[f64],
    temp_mk: i64,
) -> Result<Vec<f64>, SimError> {
    let gamma = 2.0 - std::f64::consts::SQRT_2; // ~0.5857864376269049
    let dim = ckt.total_unknowns();
    let st_n = ckt.assemble(x_n, t_n, temp_mk, 1.0e-12, 1.0);

    // Stage 1: Trapezoidal step to t_gamma = t_n + gamma * h
    let t_g = t_n + gamma * h;
    let c_scale_1 = 2.0 / (gamma * h);
    let mut x_g = x_n.to_vec();
    for _ in 0..35 {
        let st_g = ckt.assemble(&x_g, t_g, temp_mk, 1.0e-12, 1.0);
        let mut res = vec![0.0; dim];
        let mut jac = vec![vec![0.0; dim]; dim];
        let mut max_r = 0.0_f64;
        for i in 0..dim {
            res[i] = c_scale_1 * (st_g.q_dynamic[i] - st_n.q_dynamic[i])
                + st_g.f_static[i]
                + st_n.f_static[i];
            max_r = max_r.max(res[i].abs());
            for j in 0..dim {
                jac[i][j] = c_scale_1 * st_g.c_jacobian[i][j] + st_g.g_jacobian[i][j];
            }
        }
        if max_r < 1.0e-10 {
            break;
        }
        let neg_r: Vec<f64> = res.iter().map(|v| -*v).collect();
        let dx = solve_equilibrated_lu(&jac, &neg_r)?;
        let mut x_cand: Vec<f64> = x_g.iter().zip(&dx).map(|(a, b)| a + b).collect();
        apply_diode_pnjlim_damping(ckt, &x_g, &mut x_cand, temp_mk);
        let step_norm = dx.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        x_g = x_cand;
        if step_norm < 1.0e-11 && max_r < 1.0e-8 {
            break;
        }
    }

    let st_g_final = ckt.assemble(&x_g, t_g, temp_mk, 1.0e-12, 1.0);

    // Stage 2: BDF2 step to t_{n+1}^- (left-limit preserves smooth mode on breakpoint-aligned switches)
    let t_eval_end = t_n + h * (1.0 - 1.0e-12);
    let a2 = (2.0 - gamma) / ((1.0 - gamma) * h);
    let a1 = -1.0 / (gamma * (1.0 - gamma) * h);
    let a0 = (1.0 - gamma) / (gamma * h);
    let mut x_next = x_g;

    for _ in 0..35 {
        let st_1 = ckt.assemble(&x_next, t_eval_end, temp_mk, 1.0e-12, 1.0);
        let mut res = vec![0.0; dim];
        let mut jac = vec![vec![0.0; dim]; dim];
        let mut max_r = 0.0_f64;
        for i in 0..dim {
            res[i] = a2 * st_1.q_dynamic[i]
                + a1 * st_g_final.q_dynamic[i]
                + a0 * st_n.q_dynamic[i]
                + st_1.f_static[i];
            max_r = max_r.max(res[i].abs());
            for j in 0..dim {
                jac[i][j] = a2 * st_1.c_jacobian[i][j] + st_1.g_jacobian[i][j];
            }
        }
        if max_r < 1.0e-10 {
            return Ok(x_next);
        }
        let neg_r: Vec<f64> = res.iter().map(|v| -*v).collect();
        let dx = solve_equilibrated_lu(&jac, &neg_r)?;
        let mut x_cand: Vec<f64> = x_next.iter().zip(&dx).map(|(a, b)| a + b).collect();
        apply_diode_pnjlim_damping(ckt, &x_next, &mut x_cand, temp_mk);
        let step_norm = dx.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        x_next = x_cand;
        if step_norm < 1.0e-11 && max_r < 1.0e-8 {
            return Ok(x_next);
        }
    }
    Ok(x_next)
}

/// Adaptive charge/flux-conserving **TR-BDF2** transient solver (`F-026`, `F-027`).
pub fn solve_transient_tr_bdf2(
    ckt: &MnaCircuit,
    stop_time_s: f64,
    max_step_s: f64,
    initial_state: Option<&[f64]>,
    temp_mk: i64,
) -> Result<TransientTrajectory, SimError> {
    let x0 = match initial_state {
        Some(s) if s.len() == ckt.total_unknowns() => s.to_vec(),
        _ => solve_dc_op(ckt, None, temp_mk)?,
    };

    let mut times = vec![0.0];
    let mut states = vec![x0];
    let mut t = 0.0_f64;
    let mut h = (max_step_s * 0.25).clamp(1.0e-13, max_step_s);
    let min_h = (stop_time_s * 1.0e-8).max(1.0e-14);

    while t < stop_time_s - 1.0e-15 {
        if t + h > stop_time_s {
            h = stop_time_s - t;
        }
        let cur_x = states
            .last()
            .ok_or(SimError::InvalidParameter("empty state"))?;
        match step_tr_bdf2(ckt, t, h, cur_x, temp_mk) {
            Ok(x_next) => {
                let max_delta = cur_x
                    .iter()
                    .zip(&x_next)
                    .fold(0.0_f64, |m, (a, b)| m.max((b - a).abs()));
                t += h;
                times.push(t);
                states.push(x_next);
                if max_delta < 0.02 {
                    h = (h * 1.5).min(max_step_s);
                } else if max_delta > 0.5 {
                    h = (h * 0.5).max(min_h);
                }
            }
            Err(e) => {
                if h <= min_h * 1.01 {
                    return Err(e);
                }
                h = (h * 0.5).max(min_h);
            }
        }
    }

    Ok(TransientTrajectory {
        times_s: times,
        states,
    })
}
