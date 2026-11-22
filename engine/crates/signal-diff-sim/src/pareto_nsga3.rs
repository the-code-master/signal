// Module: engine/crates/signal-diff-sim/src/pareto_nsga3.rs — NSGA-III Multi-Objective Pareto Optimizer (FEAT-103)
// Invariants: True 5-objective non-dominated sorting + Das-Dennis reference-direction niching (Cost, Eff, Area, Tj, CO2e).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md

use signal_proto::v1::{
    ExploreParetoFrontierRequest, ExploreParetoFrontierResponse, ParetoCandidate,
};

/// Minimized 5-vector representation: `[cost_uusd, -efficiency_pct, area_sq_nm, peak_tj_mk, carbon_mg_co2e]`.
fn minimization_vector(c: &ParetoCandidate) -> [f64; 5] {
    [
        c.bom_cost_micro_usd as f64,
        -c.efficiency_pct,
        c.pcb_area_sq_nm as f64,
        c.peak_tj_mkelvin as f64,
        c.embodied_carbon_mg_co2e as f64,
    ]
}

/// Returns `true` iff candidate `a` Pareto-dominates `b` across all 5 engineering objectives (`FEAT-103`).
pub fn pareto_dominates(a: &ParetoCandidate, b: &ParetoCandidate) -> bool {
    let va = minimization_vector(a);
    let vb = minimization_vector(b);
    let all_leq = va.iter().zip(vb.iter()).all(|(x, y)| x <= y);
    let any_lt = va.iter().zip(vb.iter()).any(|(x, y)| x < y);
    all_leq && any_lt
}

/// Partitions candidates into Pareto fronts `F_0, F_1, ...` via fast non-dominated sorting.
pub fn fast_non_dominated_sort(pop: &[ParetoCandidate]) -> Vec<Vec<usize>> {
    let n = pop.len();
    let mut dom_sets = vec![Vec::new(); n];
    let mut dom_counts = vec![0_usize; n];
    let mut fronts: Vec<Vec<usize>> = Vec::new();
    let mut first_front = Vec::new();

    for p in 0..n {
        for q in 0..n {
            if p == q {
                continue;
            }
            if pareto_dominates(&pop[p], &pop[q]) {
                dom_sets[p].push(q);
            } else if pareto_dominates(&pop[q], &pop[p]) {
                dom_counts[p] += 1;
            }
        }
        if dom_counts[p] == 0 {
            first_front.push(p);
        }
    }

    if first_front.is_empty() {
        return fronts;
    }
    fronts.push(first_front);

    let mut f_idx = 0;
    while f_idx < fronts.len() {
        let mut next_front = Vec::new();
        for &p in &fronts[f_idx] {
            for &q in &dom_sets[p] {
                dom_counts[q] = dom_counts[q].saturating_sub(1);
                if dom_counts[q] == 0 {
                    next_front.push(q);
                }
            }
        }
        if !next_front.is_empty() {
            fronts.push(next_front);
        }
        f_idx += 1;
    }
    fronts
}

/// Generates Das-Dennis unit-simplex reference directions in $\mathbb{R}^5$ ($p = 3$ divisions $\implies 35$ rays).
fn generate_das_dennis_5d(divisions: u32) -> Vec<[f64; 5]> {
    let p = divisions.max(1);
    let mut dirs = Vec::new();
    for i0 in 0..=p {
        for i1 in 0..=(p - i0) {
            for i2 in 0..=(p - i0 - i1) {
                for i3 in 0..=(p - i0 - i1 - i2) {
                    let i4 = p - i0 - i1 - i2 - i3;
                    let denom = p as f64;
                    dirs.push([
                        (i0 as f64 / denom).max(1e-6),
                        (i1 as f64 / denom).max(1e-6),
                        (i2 as f64 / denom).max(1e-6),
                        (i3 as f64 / denom).max(1e-6),
                        (i4 as f64 / denom).max(1e-6),
                    ]);
                }
            }
        }
    }
    dirs
}

fn perp_distance_5d(point: &[f64; 5], dir: &[f64; 5]) -> f64 {
    let dir_norm_sq: f64 = dir.iter().map(|w| w * w).sum();
    let dot: f64 = point.iter().zip(dir.iter()).map(|(x, w)| x * w).sum();
    let scale = dot / dir_norm_sq.max(1e-18);
    point
        .iter()
        .zip(dir.iter())
        .map(|(x, w)| {
            let d = x - scale * w;
            d * d
        })
        .sum::<f64>()
        .sqrt()
}

fn evaluate_design_point(
    branch_prefix: &str,
    id_idx: usize,
    fsw_khz: f64,
    l_uh: f64,
    cu_sq_mm: f64,
    fet_tier: f64,
) -> ParetoCandidate {
    // Physics-coupled trade-off model:
    // - Higher f_sw + smaller L reduces PCB area & embodied carbon, but raises switching loss
    // - Higher fet_tier (lower R_ds_on / GaN) raises efficiency & lowers T_j, but raises BOM cost
    // - Larger copper area lowers Peak T_j, but increases PCB area & embodied carbon
    let sw_loss_w = 0.08 * (fsw_khz / 500.0) / fet_tier.sqrt();
    let cond_loss_w = 0.28 / (fet_tier * (1.0 + 0.06 * l_uh));
    let core_loss_w = 0.06 / l_uh.sqrt();
    let p_out_w = 10.0;
    let total_loss_w = sw_loss_w + cond_loss_w + core_loss_w;
    let eff_pct = (p_out_w / (p_out_w + total_loss_w) * 100.0).clamp(70.0, 99.2);

    let cost_uusd =
        ((450_000.0 * fet_tier) + (38_000.0 * l_uh) + (2_500.0 * cu_sq_mm)).round() as i64;
    let ind_area_sq_mm = 4.0 + 1.8 * l_uh;
    let total_area_sq_mm = cu_sq_mm + ind_area_sq_mm + 12.0;
    let area_sq_nm = (total_area_sq_mm * 1.0e12).round() as i64;

    let r_theta_k_per_w = 18.0 + 140.0 / cu_sq_mm.sqrt();
    let peak_tj_mk = (298_150.0 + total_loss_w * r_theta_k_per_w * 1_000.0).round() as i64;
    let carbon_mg =
        ((18_000.0 * l_uh) + (650.0 * total_area_sq_mm) + (12_000.0 * fet_tier)).round() as i64;

    ParetoCandidate {
        candidate_branch_id: format!("jj/{branch_prefix}/pareto-{id_idx:03}"),
        bom_cost_micro_usd: cost_uusd,
        efficiency_pct: (eff_pct * 1000.0).round() / 1000.0,
        pcb_area_sq_nm: area_sq_nm,
        peak_tj_mkelvin: peak_tj_mk,
        embodied_carbon_mg_co2e: carbon_mg,
    }
}

fn nsga3_select(
    pop: &[ParetoCandidate],
    target_size: usize,
    ref_dirs: &[[f64; 5]],
) -> Vec<ParetoCandidate> {
    let fronts = fast_non_dominated_sort(pop);
    let mut selected_indices = Vec::with_capacity(target_size);
    let mut split_front = Vec::new();

    for front in fronts {
        if selected_indices.len() + front.len() <= target_size {
            selected_indices.extend(front);
        } else {
            split_front = front;
            break;
        }
    }
    if selected_indices.len() == target_size || split_front.is_empty() {
        return selected_indices
            .into_iter()
            .map(|i| pop[i].clone())
            .collect();
    }

    // Ideal/nadir normalization over active pool
    let pool: Vec<usize> = selected_indices
        .iter()
        .chain(split_front.iter())
        .copied()
        .collect();
    let mut z_min = [f64::INFINITY; 5];
    let mut z_max = [f64::NEG_INFINITY; 5];
    for &idx in &pool {
        let v = minimization_vector(&pop[idx]);
        for k in 0..5 {
            z_min[k] = z_min[k].min(v[k]);
            z_max[k] = z_max[k].max(v[k]);
        }
    }

    let assoc_dir = |idx: usize| -> (usize, f64) {
        let raw = minimization_vector(&pop[idx]);
        let mut norm = [0.0_f64; 5];
        for k in 0..5 {
            norm[k] = (raw[k] - z_min[k]) / (z_max[k] - z_min[k]).max(1e-9);
        }
        let mut best_r = 0;
        let mut best_d = f64::INFINITY;
        for (r_idx, dir) in ref_dirs.iter().enumerate() {
            let d = perp_distance_5d(&norm, dir);
            if d < best_d {
                best_d = d;
                best_r = r_idx;
            }
        }
        (best_r, best_d)
    };

    let mut niche_count = vec![0_usize; ref_dirs.len()];
    for &idx in &selected_indices {
        let (r, _) = assoc_dir(idx);
        niche_count[r] += 1;
    }

    let mut remaining: Vec<(usize, usize, f64)> = split_front
        .into_iter()
        .map(|idx| {
            let (r, d) = assoc_dir(idx);
            (idx, r, d)
        })
        .collect();

    while selected_indices.len() < target_size && !remaining.is_empty() {
        remaining.sort_by(|a, b| {
            niche_count[a.1]
                .cmp(&niche_count[b.1])
                .then_with(|| a.2.total_cmp(&b.2))
        });
        let chosen = remaining.remove(0);
        niche_count[chosen.1] += 1;
        selected_indices.push(chosen.0);
    }

    selected_indices
        .into_iter()
        .map(|i| pop[i].clone())
        .collect()
}

/// Executes deterministic NSGA-III multi-objective Pareto exploration (`FEAT-103`).
pub fn explore_pareto_frontier(
    req: &ExploreParetoFrontierRequest,
) -> ExploreParetoFrontierResponse {
    let pop_size = (req.population_size as usize).clamp(8, 64);
    let gens = req.generations.clamp(1, 25);
    let branch = if req.branch_id.trim().is_empty() {
        "main"
    } else {
        req.branch_id.trim()
    };
    let ref_dirs = generate_das_dennis_5d(3);

    // Deterministic Halton/grid initial design parameter vectors (fsw_khz, l_uh, cu_sq_mm, fet_tier)
    let mut params: Vec<[f64; 4]> = (0..pop_size)
        .map(|i| {
            let t = (i as f64) / (pop_size.max(2) - 1) as f64;
            let fsw = 300.0 + 1_900.0 * ((i * 7 % pop_size) as f64 / pop_size as f64);
            let l_uh = 0.68 + 9.32 * t;
            let cu_mm2 = 25.0 + 225.0 * ((i * 11 % pop_size) as f64 / pop_size as f64);
            let fet = 0.8 + 2.4 * ((i * 5 % pop_size) as f64 / pop_size as f64);
            [fsw, l_uh, cu_mm2, fet]
        })
        .collect();

    let mut next_id = 1_usize;
    let mut pop: Vec<ParetoCandidate> = params
        .iter()
        .map(|p| {
            let c = evaluate_design_point(branch, next_id, p[0], p[1], p[2], p[3]);
            next_id += 1;
            c
        })
        .collect();

    for g in 0..gens {
        let mut offspring_params = Vec::with_capacity(pop_size);
        for i in 0..pop_size {
            let j = (i + 1 + (g as usize)) % pop_size;
            let alpha = 0.35 + 0.30 * (((i + g as usize) % 3) as f64 * 0.5);
            let mut child = [0.0_f64; 4];
            for k in 0..4 {
                child[k] = alpha * params[i][k] + (1.0 - alpha) * params[j][k];
            }
            // Deterministic local mutation nudge
            let sign = if (i + g as usize).is_multiple_of(2) {
                1.0
            } else {
                -1.0
            };
            child[0] = (child[0] + sign * 85.0).clamp(250.0, 2_500.0);
            child[1] = (child[1] - sign * 0.35).clamp(0.47, 15.0);
            child[2] = (child[2] + sign * 12.0).clamp(20.0, 300.0);
            child[3] = (child[3] + sign * 0.18).clamp(0.7, 3.5);
            offspring_params.push(child);
        }

        let mut combined = pop;
        for cp in &offspring_params {
            combined.push(evaluate_design_point(
                branch, next_id, cp[0], cp[1], cp[2], cp[3],
            ));
            next_id += 1;
        }
        params.extend(offspring_params);
        pop = nsga3_select(&combined, pop_size, &ref_dirs);
        params.truncate(pop_size);
    }

    let fronts = fast_non_dominated_sort(&pop);
    let frontier: Vec<ParetoCandidate> = fronts
        .first()
        .map(|f0| f0.iter().map(|&idx| pop[idx].clone()).collect())
        .unwrap_or_default();

    ExploreParetoFrontierResponse { frontier }
}
