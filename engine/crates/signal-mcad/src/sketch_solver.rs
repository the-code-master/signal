// Module: engine/crates/signal-mcad/src/sketch_solver.rs — 2D Damped Gauss-Newton / Dogleg Sketch Constraint Solver
// Invariants: Solves distance/coincident/parallel/perpendicular/tangent constraints to sub-nm residual (19-*).
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
use crate::McadError;
use signal_geom::Point2Nm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SketchPoint {
    pub id: usize,
    pub x_nm: i64,
    pub y_nm: i64,
    pub fixed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SketchConstraint {
    Coincident {
        p1: usize,
        p2: usize,
    },
    Horizontal {
        p1: usize,
        p2: usize,
    },
    Vertical {
        p1: usize,
        p2: usize,
    },
    Distance {
        p1: usize,
        p2: usize,
        target_nm: i64,
    },
    Parallel {
        l1_p1: usize,
        l1_p2: usize,
        l2_p1: usize,
        l2_p2: usize,
    },
    Perpendicular {
        l1_p1: usize,
        l1_p2: usize,
        l2_p1: usize,
        l2_p2: usize,
    },
    TangentCircleLine {
        center: usize,
        radius_nm: i64,
        line_p1: usize,
        line_p2: usize,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolvedSketch {
    pub points_nm: Vec<Point2Nm>,
    pub iterations: usize,
    pub max_residual_nm: f64,
}

fn eval_residuals(coords: &[f64], constraints: &[SketchConstraint]) -> Vec<f64> {
    let mut res = Vec::with_capacity(constraints.len() * 2);
    for c in constraints {
        match *c {
            SketchConstraint::Coincident { p1, p2 } => {
                res.push(coords[2 * p1] - coords[2 * p2]);
                res.push(coords[2 * p1 + 1] - coords[2 * p2 + 1]);
            }
            SketchConstraint::Horizontal { p1, p2 } => {
                res.push(coords[2 * p1 + 1] - coords[2 * p2 + 1]);
            }
            SketchConstraint::Vertical { p1, p2 } => {
                res.push(coords[2 * p1] - coords[2 * p2]);
            }
            SketchConstraint::Distance { p1, p2, target_nm } => {
                let dx = coords[2 * p2] - coords[2 * p1];
                let dy = coords[2 * p2 + 1] - coords[2 * p1 + 1];
                res.push(dx.hypot(dy) - (target_nm as f64));
            }
            SketchConstraint::Parallel {
                l1_p1,
                l1_p2,
                l2_p1,
                l2_p2,
            } => {
                let ux = coords[2 * l1_p2] - coords[2 * l1_p1];
                let uy = coords[2 * l1_p2 + 1] - coords[2 * l1_p1 + 1];
                let vx = coords[2 * l2_p2] - coords[2 * l2_p1];
                let vy = coords[2 * l2_p2 + 1] - coords[2 * l2_p1 + 1];
                let norm = (ux.hypot(uy) * vx.hypot(vy)).max(1.0);
                res.push((ux * vy - uy * vx) / norm.sqrt());
            }
            SketchConstraint::Perpendicular {
                l1_p1,
                l1_p2,
                l2_p1,
                l2_p2,
            } => {
                let ux = coords[2 * l1_p2] - coords[2 * l1_p1];
                let uy = coords[2 * l1_p2 + 1] - coords[2 * l1_p1 + 1];
                let vx = coords[2 * l2_p2] - coords[2 * l2_p1];
                let vy = coords[2 * l2_p2 + 1] - coords[2 * l2_p1 + 1];
                let norm = (ux.hypot(uy) * vx.hypot(vy)).max(1.0);
                res.push((ux * vx + uy * vy) / norm.sqrt());
            }
            SketchConstraint::TangentCircleLine {
                center,
                radius_nm,
                line_p1,
                line_p2,
            } => {
                let lx = coords[2 * line_p2] - coords[2 * line_p1];
                let ly = coords[2 * line_p2 + 1] - coords[2 * line_p1 + 1];
                let len = lx.hypot(ly).max(1.0);
                let cx = coords[2 * center] - coords[2 * line_p1];
                let cy = coords[2 * center + 1] - coords[2 * line_p1 + 1];
                let signed_dist = (lx * cy - ly * cx).abs() / len;
                res.push(signed_dist - (radius_nm as f64));
            }
        }
    }
    res
}

fn solve_dense_linear(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let n = b.len();
    for i in 0..n {
        let mut pivot = i;
        for r in (i + 1)..n {
            if a[r][i].abs() > a[pivot][i].abs() {
                pivot = r;
            }
        }
        a.swap(i, pivot);
        b.swap(i, pivot);
        let diag = a[i][i];
        if diag.abs() < 1e-12 {
            continue;
        }
        for r in (i + 1)..n {
            let factor = a[r][i] / diag;
            for c in i..n {
                a[r][c] -= factor * a[i][c];
            }
            b[r] -= factor * b[i];
        }
    }
    let mut x = vec![0.0_f64; n];
    for i in (0..n).rev() {
        let mut sum = b[i];
        for c in (i + 1)..n {
            sum -= a[i][c] * x[c];
        }
        if a[i][i].abs() >= 1e-12 {
            x[i] = sum / a[i][i];
        }
    }
    x
}

/// Solves a 2D parametric sketch using damped Gauss-Newton / Dogleg trust-region steps.
pub fn solve_sketch_2d(
    points: &[SketchPoint],
    constraints: &[SketchConstraint],
    max_iters: usize,
) -> Result<SolvedSketch, McadError> {
    let n_vars = points.len() * 2;
    let mut coords = Vec::with_capacity(n_vars);
    for p in points {
        coords.push(p.x_nm as f64);
        coords.push(p.y_nm as f64);
    }

    let mut lambda = 1e-3_f64;
    let trust_radius_nm = 50_000_000.0_f64;
    let eps = 1.0_f64;

    for iter in 0..max_iters.max(1) {
        let r0 = eval_residuals(&coords, constraints);
        let max_res = r0.iter().fold(0.0_f64, |acc, v| acc.max(v.abs()));
        if max_res <= 0.5 {
            let out_pts = points
                .iter()
                .enumerate()
                .map(|(idx, _)| {
                    Point2Nm::new(
                        coords[2 * idx].round() as i64,
                        coords[2 * idx + 1].round() as i64,
                    )
                })
                .collect();
            return Ok(SolvedSketch {
                points_nm: out_pts,
                iterations: iter,
                max_residual_nm: max_res,
            });
        }

        let m = r0.len();
        let mut jac = vec![vec![0.0_f64; n_vars]; m];
        for j in 0..n_vars {
            if points[j / 2].fixed {
                continue;
            }
            let orig = coords[j];
            coords[j] = orig + eps;
            let r_plus = eval_residuals(&coords, constraints);
            coords[j] = orig;
            for i in 0..m {
                jac[i][j] = (r_plus[i] - r0[i]) / eps;
            }
        }

        let mut jtj = vec![vec![0.0_f64; n_vars]; n_vars];
        let mut neg_jtr = vec![0.0_f64; n_vars];
        for p in 0..n_vars {
            for i in 0..m {
                neg_jtr[p] -= jac[i][p] * r0[i];
                for q in 0..n_vars {
                    jtj[p][q] += jac[i][p] * jac[i][q];
                }
            }
            jtj[p][p] += lambda;
        }

        let mut step = solve_dense_linear(jtj, neg_jtr);
        let step_norm = step.iter().map(|s| s * s).sum::<f64>().sqrt();
        if step_norm > trust_radius_nm {
            let scale = trust_radius_nm / step_norm;
            for s in &mut step {
                *s *= scale;
            }
        }

        let mut trial = coords.clone();
        for (j, s) in step.iter().enumerate() {
            if !points[j / 2].fixed {
                trial[j] += *s;
            }
        }
        let cost0: f64 = r0.iter().map(|v| v * v).sum();
        let r_trial = eval_residuals(&trial, constraints);
        let cost_trial: f64 = r_trial.iter().map(|v| v * v).sum();
        if cost_trial < cost0 {
            coords = trial;
            lambda = (lambda * 0.5).max(1e-6);
        } else {
            lambda = (lambda * 4.0).min(1e6);
        }
    }

    let final_res = eval_residuals(&coords, constraints)
        .iter()
        .fold(0.0_f64, |a, v| a.max(v.abs()));
    Err(McadError::SketchDidNotConverge(final_res.round() as i64))
}
