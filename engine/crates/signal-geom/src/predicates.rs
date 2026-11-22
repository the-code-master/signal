// Module: engine/crates/signal-geom/src/predicates.rs — Exact i128/i256 & Shewchuk Adaptive orient2d / incircle Predicates
// Invariants: Adaptive floating filter falls back to exact error-free floating expansion when uncertain (F-030).
// Spec: architecture/03-rust-eda-engine.md
use crate::primitives::Point2Nm;
use std::cmp::Ordering;
use std::hint::black_box;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    CounterClockwise,
    Clockwise,
    Collinear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InCircle {
    Inside,
    Outside,
    Cocircular,
}

/// Exact integer `orient2d` promoted to `i128` (`F-030`).
pub fn orient2d_i128(a: Point2Nm, b: Point2Nm, c: Point2Nm) -> Orientation {
    let ab_x = i128::from(b.x.0) - i128::from(a.x.0);
    let ab_y = i128::from(b.y.0) - i128::from(a.y.0);
    let ac_x = i128::from(c.x.0) - i128::from(a.x.0);
    let ac_y = i128::from(c.y.0) - i128::from(a.y.0);
    let det = ab_x * ac_y - ab_y * ac_x;
    match det.cmp(&0) {
        Ordering::Greater => Orientation::CounterClockwise,
        Ordering::Less => Orientation::Clockwise,
        Ordering::Equal => Orientation::Collinear,
    }
}

#[inline(always)]
fn no_fma_mul(a: f64, b: f64) -> f64 {
    black_box(a * b)
}

#[inline]
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let x = black_box(a + b);
    let bv = black_box(x - a);
    let av = black_box(x - bv);
    let err = black_box(b - bv) + black_box(a - av);
    (x, err)
}

#[inline]
fn two_diff(a: f64, b: f64) -> (f64, f64) {
    two_sum(a, -b)
}

#[inline]
fn split_dekker(a: f64) -> (f64, f64) {
    const SPLITTER: f64 = 134_217_729.0; // 2^27 + 1
    let c = no_fma_mul(SPLITTER, a);
    let abig = black_box(c - a);
    let ahi = black_box(c - abig);
    let alo = black_box(a - ahi);
    (ahi, alo)
}

#[inline]
fn two_product(a: f64, b: f64) -> (f64, f64) {
    let x = no_fma_mul(a, b);
    let (ahi, alo) = split_dekker(a);
    let (bhi, blo) = split_dekker(b);
    let err1 = x - no_fma_mul(ahi, bhi);
    let err2 = err1 - no_fma_mul(alo, bhi);
    let err3 = err2 - no_fma_mul(ahi, blo);
    let y = no_fma_mul(alo, blo) - err3;
    (x, y)
}

fn grow_expansion(e: &[f64], b: f64) -> Vec<f64> {
    let mut out = Vec::with_capacity(e.len() + 1);
    let mut q = b;
    for &ei in e {
        let (q_new, h) = two_sum(q, ei);
        if h != 0.0 {
            out.push(h);
        }
        q = q_new;
    }
    if q != 0.0 || out.is_empty() {
        out.push(q);
    }
    out
}

fn expansion_sum(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut acc = a.to_vec();
    for &bi in b {
        acc = grow_expansion(&acc, bi);
    }
    acc
}

fn scale_expansion(e: &[f64], b: f64) -> Vec<f64> {
    if e.is_empty() || b == 0.0 {
        return vec![0.0];
    }
    let mut out = Vec::with_capacity(e.len() * 2);
    let (mut q, hh0) = two_product(e[0], b);
    if hh0 != 0.0 {
        out.push(hh0);
    }
    for &ei in &e[1..] {
        let (ti, ti_lo) = two_product(ei, b);
        let (q_mid, h1) = two_sum(q, ti_lo);
        if h1 != 0.0 {
            out.push(h1);
        }
        let (q_next, h2) = two_sum(ti, q_mid);
        if h2 != 0.0 {
            out.push(h2);
        }
        q = q_next;
    }
    if q != 0.0 || out.is_empty() {
        out.push(q);
    }
    out
}

fn negate_expansion(mut e: Vec<f64>) -> Vec<f64> {
    for v in &mut e {
        *v = -*v;
    }
    e
}

fn expansion_sign(e: &[f64]) -> Ordering {
    for &v in e.iter().rev() {
        if v > 0.0 {
            return Ordering::Greater;
        }
        if v < 0.0 {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn exact_2x2_det_expansion(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> Vec<f64> {
    let (acx, acx_tail) = two_diff(ax, cx);
    let (bcx, bcx_tail) = two_diff(bx, cx);
    let (acy, acy_tail) = two_diff(ay, cy);
    let (bcy, bcy_tail) = two_diff(by, cy);

    let (p1, p1_lo) = two_product(acx, bcy);
    let (p2, p2_lo) = two_product(acy, bcx);
    let mut det = expansion_sum(&[p1_lo, p1], &[-p2_lo, -p2]);

    if acx_tail != 0.0 || acy_tail != 0.0 || bcx_tail != 0.0 || bcy_tail != 0.0 {
        let t1 = scale_expansion(&[bcy_tail, bcy], acx_tail);
        let t2 = scale_expansion(&[acx], bcy_tail);
        let t3 = negate_expansion(scale_expansion(&[bcx_tail, bcx], acy_tail));
        let t4 = negate_expansion(scale_expansion(&[acy], bcx_tail));
        det = expansion_sum(&det, &t1);
        det = expansion_sum(&det, &t2);
        det = expansion_sum(&det, &t3);
        det = expansion_sum(&det, &t4);
    }
    det
}

/// Shewchuk adaptive `orient2d` predicate with error-bound filter + exact expansion fallback (`F-030`).
pub fn orient2d_adaptive(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> Orientation {
    const CCW_ERR_BOUND_A: f64 = (3.0 + 16.0 * f64::EPSILON) * f64::EPSILON;
    let detleft = no_fma_mul(a.0 - c.0, b.1 - c.1);
    let detright = no_fma_mul(a.1 - c.1, b.0 - c.0);
    let det = detleft - detright;
    let detsum = detleft.abs() + detright.abs();
    if det.abs() > CCW_ERR_BOUND_A * detsum {
        return if det > 0.0 {
            Orientation::CounterClockwise
        } else {
            Orientation::Clockwise
        };
    }
    match expansion_sign(&exact_2x2_det_expansion(a.0, a.1, b.0, b.1, c.0, c.1)) {
        Ordering::Greater => Orientation::CounterClockwise,
        Ordering::Less => Orientation::Clockwise,
        Ordering::Equal => Orientation::Collinear,
    }
}

/// Shewchuk adaptive `incircle` predicate (`d` vs circumcircle of CounterClockwise `(a, b, c)`, `F-030`).
pub fn incircle_adaptive(a: (f64, f64), b: (f64, f64), c: (f64, f64), d: (f64, f64)) -> InCircle {
    const ICC_ERR_BOUND_A: f64 = (10.0 + 96.0 * f64::EPSILON) * f64::EPSILON;
    let adx = a.0 - d.0;
    let bdx = b.0 - d.0;
    let cdx = c.0 - d.0;
    let ady = a.1 - d.1;
    let bdy = b.1 - d.1;
    let cdy = c.1 - d.1;

    let bdxcdy = no_fma_mul(bdx, cdy);
    let cdxbdy = no_fma_mul(cdx, bdy);
    let alift = no_fma_mul(adx, adx) + no_fma_mul(ady, ady);

    let cdxady = no_fma_mul(cdx, ady);
    let adxcdy = no_fma_mul(adx, cdy);
    let blift = no_fma_mul(bdx, bdx) + no_fma_mul(bdy, bdy);

    let adxbdy = no_fma_mul(adx, bdy);
    let bdxady = no_fma_mul(bdx, ady);
    let clift = no_fma_mul(cdx, cdx) + no_fma_mul(cdy, cdy);

    let det = alift * (bdxcdy - cdxbdy) + blift * (cdxady - adxcdy) + clift * (adxbdy - bdxady);
    let permanent = (bdxcdy.abs() + cdxbdy.abs()) * alift
        + (cdxady.abs() + adxcdy.abs()) * blift
        + (adxbdy.abs() + bdxady.abs()) * clift;

    if det.abs() > ICC_ERR_BOUND_A * permanent {
        return if det > 0.0 {
            InCircle::Inside
        } else {
            InCircle::Outside
        };
    }

    let bc = exact_2x2_det_expansion(b.0, b.1, c.0, c.1, d.0, d.1);
    let ca = exact_2x2_det_expansion(c.0, c.1, a.0, a.1, d.0, d.1);
    let ab = exact_2x2_det_expansion(a.0, a.1, b.0, b.1, d.0, d.1);

    let lift_term = |px: f64, py: f64, minor: &[f64]| -> Vec<f64> {
        let (dx, dx_lo) = two_diff(px, d.0);
        let (dy, dy_lo) = two_diff(py, d.1);
        let dx_exp = [dx_lo, dx];
        let dy_exp = [dy_lo, dy];
        let x2 = expansion_sum(
            &scale_expansion(&dx_exp, dx_lo),
            &scale_expansion(&dx_exp, dx),
        );
        let y2 = expansion_sum(
            &scale_expansion(&dy_exp, dy_lo),
            &scale_expansion(&dy_exp, dy),
        );
        let r2 = expansion_sum(&x2, &y2);
        let mut acc = vec![0.0];
        for &ri in &r2 {
            acc = expansion_sum(&acc, &scale_expansion(minor, ri));
        }
        acc
    };

    let t_a = lift_term(a.0, a.1, &bc);
    let t_b = lift_term(b.0, b.1, &ca);
    let t_c = lift_term(c.0, c.1, &ab);
    let total = expansion_sum(&expansion_sum(&t_a, &t_b), &t_c);
    match expansion_sign(&total) {
        Ordering::Greater => InCircle::Inside,
        Ordering::Less => InCircle::Outside,
        Ordering::Equal => InCircle::Cocircular,
    }
}

/// Exact nanometer `incircle` predicate backed by Shewchuk expansion arithmetic (`F-030`).
pub fn incircle_nm(a: Point2Nm, b: Point2Nm, c: Point2Nm, d: Point2Nm) -> InCircle {
    incircle_adaptive(
        (a.x.0 as f64, a.y.0 as f64),
        (b.x.0 as f64, b.y.0 as f64),
        (c.x.0 as f64, c.y.0 as f64),
        (d.x.0 as f64, d.y.0 as f64),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adaptive_orient2d_resolves_near_collinear_perturbation_at_1e_minus_14() {
        let base = 12.0;
        let eps = 1.0e-14;
        assert_eq!(
            orient2d_adaptive((0.0, 0.0), (base, base), (24.0, 24.0)),
            Orientation::Collinear
        );
        assert_eq!(
            orient2d_adaptive((0.0, 0.0), (base, base + eps), (24.0, 24.0)),
            Orientation::Clockwise
        );
        assert_eq!(
            orient2d_adaptive((0.0, 0.0), (base, base - eps), (24.0, 24.0)),
            Orientation::CounterClockwise
        );
    }

    #[test]
    fn adaptive_and_nm_incircle_detect_cocircular_inside_and_outside() {
        let a = Point2Nm::new(1_000_000_000, 0);
        let b = Point2Nm::new(0, 1_000_000_000);
        let c = Point2Nm::new(-1_000_000_000, 0);
        let d_on = Point2Nm::new(0, -1_000_000_000);
        let d_in = Point2Nm::new(0, -999_999_999);
        let d_out = Point2Nm::new(0, -1_000_000_001);
        assert_eq!(incircle_nm(a, b, c, d_on), InCircle::Cocircular);
        assert_eq!(incircle_nm(a, b, c, d_in), InCircle::Inside);
        assert_eq!(incircle_nm(a, b, c, d_out), InCircle::Outside);
    }
}
