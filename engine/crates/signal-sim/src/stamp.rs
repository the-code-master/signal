// Module: engine/crates/signal-sim/src/stamp.rs — Conservative DAE Element Stamp Kernel (q(v), Phi(i), G, C)
// Invariants: Stamps unified DAE d(q(x))/dt + f(x, t) = 0 with Gmin & branch algebraic regularizers (F-024, F-027).
// Spec: architecture/03-rust-eda-engine.md
use crate::circuit::{AssembledMnaState, CircuitElement, MnaCircuit};
use signal_component::ElectronicModule;
use signal_core::{Microvolts, det_sin};

pub(crate) fn assemble_mna_system(
    ckt: &MnaCircuit,
    x: &[f64],
    t_sec: f64,
    temp_mk: i64,
    gmin: f64,
    src_scale: f64,
) -> AssembledMnaState {
    let dim = ckt.total_unknowns();
    let mut f = vec![0.0; dim];
    let mut q = vec![0.0; dim];
    let mut g = vec![vec![0.0; dim]; dim];
    let mut c = vec![vec![0.0; dim]; dim];

    for i in 0..ckt.num_nodes {
        f[i] += gmin * x[i];
        g[i][i] += gmin;
    }
    for (b, row) in g.iter_mut().enumerate().take(dim).skip(ckt.num_nodes) {
        row[b] += 1.0e-15;
    }

    for el in &ckt.elements {
        stamp_element(
            el, x, t_sec, temp_mk, src_scale, &mut f, &mut q, &mut g, &mut c,
        );
    }

    AssembledMnaState {
        f_static: f,
        q_dynamic: q,
        g_jacobian: g,
        c_jacobian: c,
    }
}

#[allow(clippy::too_many_arguments)]
fn stamp_2port(
    np: usize,
    nn: usize,
    i_br: f64,
    q_br: f64,
    g_br: f64,
    c_br: f64,
    f: &mut [f64],
    q: &mut [f64],
    g: &mut [Vec<f64>],
    c: &mut [Vec<f64>],
) {
    if np > 0 {
        f[np - 1] += i_br;
        q[np - 1] += q_br;
        g[np - 1][np - 1] += g_br;
        c[np - 1][np - 1] += c_br;
        if nn > 0 {
            g[np - 1][nn - 1] -= g_br;
            c[np - 1][nn - 1] -= c_br;
        }
    }
    if nn > 0 {
        f[nn - 1] -= i_br;
        q[nn - 1] -= q_br;
        g[nn - 1][nn - 1] += g_br;
        c[nn - 1][nn - 1] += c_br;
        if np > 0 {
            g[nn - 1][np - 1] -= g_br;
            c[nn - 1][np - 1] -= c_br;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn stamp_inductor_branch(
    np: usize,
    nn: usize,
    b: usize,
    il: f64,
    r_s: f64,
    phi: f64,
    l_diff: f64,
    x: &[f64],
    f: &mut [f64],
    q: &mut [f64],
    g: &mut [Vec<f64>],
    c: &mut [Vec<f64>],
) {
    let vp = MnaCircuit::node_v(x, np);
    let vn = MnaCircuit::node_v(x, nn);
    if np > 0 {
        f[np - 1] += il;
        g[np - 1][b] += 1.0;
        g[b][np - 1] -= 1.0;
    }
    if nn > 0 {
        f[nn - 1] -= il;
        g[nn - 1][b] -= 1.0;
        g[b][nn - 1] += 1.0;
    }
    f[b] += r_s * il - (vp - vn);
    g[b][b] += r_s;
    q[b] += phi;
    c[b][b] += l_diff;
}

fn stamp_vsource(
    np: usize,
    nn: usize,
    b: usize,
    vs: f64,
    x: &[f64],
    f: &mut [f64],
    g: &mut [Vec<f64>],
) {
    let vp = MnaCircuit::node_v(x, np);
    let vn = MnaCircuit::node_v(x, nn);
    let iv = x[b];
    if np > 0 {
        f[np - 1] += iv;
        g[np - 1][b] += 1.0;
        g[b][np - 1] += 1.0;
    }
    if nn > 0 {
        f[nn - 1] -= iv;
        g[nn - 1][b] -= 1.0;
        g[b][nn - 1] -= 1.0;
    }
    f[b] += (vp - vn) - vs;
}

#[allow(clippy::too_many_arguments)]
fn stamp_element(
    el: &CircuitElement,
    x: &[f64],
    t_sec: f64,
    temp_mk: i64,
    src_scale: f64,
    f: &mut [f64],
    q: &mut [f64],
    g: &mut [Vec<f64>],
    c: &mut [Vec<f64>],
) {
    match el {
        CircuitElement::Resistor { np, nn, model, .. } => {
            let v = MnaCircuit::node_v(x, *np) - MnaCircuit::node_v(x, *nn);
            let cond = 1.0 / model.effective_resistance_ohms(temp_mk);
            stamp_2port(*np, *nn, cond * v, 0.0, cond, 0.0, f, q, g, c);
        }
        CircuitElement::LinearCapacitor {
            np, nn, c_farads, ..
        } => {
            let v = MnaCircuit::node_v(x, *np) - MnaCircuit::node_v(x, *nn);
            stamp_2port(*np, *nn, 0.0, c_farads * v, 0.0, *c_farads, f, q, g, c);
        }
        CircuitElement::Mlcc { np, nn, model, .. } => {
            let v = MnaCircuit::node_v(x, *np) - MnaCircuit::node_v(x, *nn);
            let (qv, cd) = model.evaluate_charge_and_diff_cap(v, temp_mk);
            stamp_2port(
                *np,
                *nn,
                model.leak_g_siemens * v,
                qv,
                model.leak_g_siemens,
                cd,
                f,
                q,
                g,
                c,
            );
        }
        CircuitElement::LinearInductor {
            np,
            nn,
            l_henries,
            r_series_ohms,
            branch_idx,
            ..
        } => {
            let il = x[*branch_idx];
            stamp_inductor_branch(
                *np,
                *nn,
                *branch_idx,
                il,
                *r_series_ohms,
                l_henries * il,
                *l_henries,
                x,
                f,
                q,
                g,
                c,
            );
        }
        CircuitElement::SatInductor {
            np,
            nn,
            model,
            branch_idx,
            ..
        } => {
            let il = x[*branch_idx];
            let (phi, l_diff) = model.evaluate_flux_and_diff_inductance(il);
            stamp_inductor_branch(
                *np,
                *nn,
                *branch_idx,
                il,
                model.dcr_ohms,
                phi,
                l_diff,
                x,
                f,
                q,
                g,
                c,
            );
        }
        CircuitElement::DcAcVoltage {
            np,
            nn,
            dc_v,
            branch_idx,
            ..
        } => {
            stamp_vsource(*np, *nn, *branch_idx, src_scale * *dc_v, x, f, g);
        }
        CircuitElement::SineVoltage {
            np,
            nn,
            offset_v,
            amp_v,
            freq_hz,
            branch_idx,
            ..
        } => {
            let vs = src_scale
                * (*offset_v + *amp_v * det_sin(std::f64::consts::TAU * *freq_hz * t_sec));
            stamp_vsource(*np, *nn, *branch_idx, vs, x, f, g);
        }
        CircuitElement::StepPulseVoltage {
            np,
            nn,
            v_initial,
            v_pulsed,
            delay_s,
            rise_s,
            branch_idx,
            ..
        } => {
            let frac = ((t_sec - *delay_s) / *rise_s).clamp(0.0, 1.0);
            let vs = src_scale * (*v_initial + frac * (*v_pulsed - *v_initial));
            stamp_vsource(*np, *nn, *branch_idx, vs, x, f, g);
        }
        CircuitElement::CurrentSource { np, nn, dc_a, .. } => {
            let isrc = src_scale * *dc_a;
            if *np > 0 {
                f[*np - 1] += isrc;
            }
            if *nn > 0 {
                f[*nn - 1] -= isrc;
            }
        }
        CircuitElement::Diode { na, nc, model, .. } => {
            let vd = MnaCircuit::node_v(x, *na) - MnaCircuit::node_v(x, *nc);
            let (id, gd, qd, cd) = model.eval_iv_qv(vd, temp_mk);
            stamp_2port(*na, *nc, id, qd, gd, cd, f, q, g, c);
        }
        CircuitElement::Mosfet {
            nd,
            ng,
            ns,
            nb,
            model,
            ..
        } => {
            let nodes = [*nd, *ng, *ns, *nb];
            let bias: Vec<Microvolts> = nodes
                .iter()
                .map(|&n| Microvolts((MnaCircuit::node_v(x, n) * 1.0e6).round() as i64))
                .collect();
            let st = model.evaluate_multi_terminal(&bias, temp_mk);
            for (a, &na) in nodes.iter().enumerate() {
                if na > 0 {
                    f[na - 1] += st.terminal_currents_a[a];
                    q[na - 1] += st.terminal_charges_c[a];
                    for (b, &nb_node) in nodes.iter().enumerate() {
                        if nb_node > 0 {
                            g[na - 1][nb_node - 1] += st.g_jacobian[a][b];
                            c[na - 1][nb_node - 1] += st.c_jacobian[a][b];
                        }
                    }
                }
            }
        }
        CircuitElement::PwmSwitch {
            np,
            nn,
            r_on,
            r_off,
            period_s,
            duty,
            high_side,
            ..
        } => {
            let phase = (t_sec / *period_s).rem_euclid(1.0);
            let hs_closed = phase < *duty;
            let closed = if *high_side { hs_closed } else { !hs_closed };
            let cond = 1.0 / if closed { *r_on } else { *r_off };
            let v = MnaCircuit::node_v(x, *np) - MnaCircuit::node_v(x, *nn);
            stamp_2port(*np, *nn, cond * v, 0.0, cond, 0.0, f, q, g, c);
        }
    }
}
