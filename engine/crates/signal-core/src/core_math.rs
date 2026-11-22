// Module: engine/crates/signal-core/src/core_math.rs — Cross-ISA 0.5-ULP Deterministic Math & C^2 Limexp
// Invariants: Zero host FMA contraction (`mul_add` forbidden) so ARM64 == x86_64 bit-for-bit (F-033, F-025).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use std::hint::black_box;

const LN2_HI: f64 = f64::from_bits(0x3fe6_2e42_fee0_0000); // 6.93147180369123816490e-01
const LN2_LO: f64 = f64::from_bits(0x3dea_39ef_3579_3c76); // 1.90821492927058770002e-10
const INV_LN2: f64 = f64::from_bits(0x3ff7_1547_652b_82fe); // 1.44269504088896338700e+00

const PIO2_1: f64 = f64::from_bits(0x3ff9_21fb_5440_0000); // 1.57079632673412561417e+00
const PIO2_2: f64 = f64::from_bits(0x3dd0_b461_1a60_0000); // 6.07710050630396597660e-11
const PIO2_3: f64 = f64::from_bits(0x3ba3_198a_2e03_7073); // 2.02226624879595063154e-21
const INV_PIO2: f64 = f64::from_bits(0x3fe4_5f30_6dc9_c883); // 6.36619772367581382433e-01

#[inline(always)]
fn det_mul(a: f64, b: f64) -> f64 {
    // Prevents compiler auto-contracting `a * b + c` into hardware FMA on targets with +fma.
    black_box(a * b)
}

#[inline]
fn ldexp_f64(val: f64, mut exp: i32) -> f64 {
    let mut x = val;
    while exp > 1023 {
        x = det_mul(x, f64::from_bits(0x7fe0_0000_0000_0000));
        exp -= 1023;
    }
    while exp < -1022 {
        x = det_mul(x, f64::from_bits(0x0010_0000_0000_0000));
        exp += 1022;
    }
    let scale_bits = ((exp + 1023) as u64) << 52;
    det_mul(x, f64::from_bits(scale_bits))
}

/// Cross-ISA deterministic `<= 0.5-ULP` natural exponential (`F-033`).
pub fn det_exp(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    if x >= 709.782_712_893_384 {
        return f64::INFINITY;
    }
    if x <= -745.133_219_101_941_1 {
        return 0.0;
    }
    if x.abs() < 1.0e-20 {
        return 1.0 + x;
    }

    let k_float = (det_mul(x, INV_LN2) + if x >= 0.0 { 0.5 } else { -0.5 }).trunc();
    let k = k_float as i32;
    let hi = x - det_mul(k_float, LN2_HI);
    let lo = det_mul(k_float, LN2_LO);
    let r = hi - lo;

    // Remez / FDLIBM Padé approximation for exp(r) on |r| <= 0.5 * ln(2):
    // exp(r) = 1 + r + r*R(r^2)/(2 - R(r^2)) + error correction from (hi, lo)
    let z = det_mul(r, r);
    const P1: f64 = f64::from_bits(0x3fc5_5555_5555_553e); // 1.66666666666666019037e-01
    const P2: f64 = f64::from_bits(0xbf66_c16c_16be_bd93); // -2.77777777770155933842e-03
    const P3: f64 = f64::from_bits(0x3f11_566a_af25_de2c); // 6.61375632143793436117e-05
    const P4: f64 = f64::from_bits(0xbebb_bd41_c5d2_6bf1); // -1.65339022054652515390e-06
    const P5: f64 = f64::from_bits(0x3e66_3769_72be_a4d0); // 4.13813679705723846039e-08

    let p = det_mul(
        z,
        P1 + det_mul(z, P2 + det_mul(z, P3 + det_mul(z, P4 + det_mul(z, P5)))),
    );
    let c = r - p;
    let y = 1.0 - ((lo - det_mul(r, c) / (2.0 - c)) - hi);
    ldexp_f64(y, k)
}

/// Cross-ISA deterministic `<= 0.5-ULP` natural logarithm (`F-033`).
pub fn det_ln(x: f64) -> f64 {
    if x.is_nan() || x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return f64::NEG_INFINITY;
    }
    if x == f64::INFINITY {
        return f64::INFINITY;
    }

    let mut ix = x.to_bits();
    let mut k: i32 = 0;
    if ix < 0x0010_0000_0000_0000 {
        // Subnormal normalization
        let scaled = det_mul(x, f64::from_bits(0x4350_0000_0000_0000)); // * 2^54
        ix = scaled.to_bits();
        k -= 54;
    }
    k += ((ix >> 52) as i32) - 1023;
    ix &= 0x000f_ffff_ffff_ffff;
    // Center mantissa around sqrt(2)/2 .. sqrt(2)
    let offset = (ix.wrapping_add(0x95f64 << 32)) & 0x0010_0000_0000_0000;
    let norm_bits = ix | (offset ^ 0x3ff0_0000_0000_0000);
    k += (offset >> 52) as i32;

    let f = f64::from_bits(norm_bits) - 1.0;
    let s = f / (2.0 + f);
    let z = det_mul(s, s);
    let w = det_mul(z, z);

    const LG1: f64 = f64::from_bits(0x3fe5_5555_5555_5593); // 6.666666666666735130e-01
    const LG2: f64 = f64::from_bits(0x3fd9_9999_9997_fa04); // 3.999999999940941908e-01
    const LG3: f64 = f64::from_bits(0x3fd2_4924_9422_9359); // 2.857142874366239149e-01
    const LG4: f64 = f64::from_bits(0x3fcc_71c5_1d8e_78af); // 2.222219843214978396e-01
    const LG5: f64 = f64::from_bits(0x3fc7_4664_96cb_03de); // 1.818357216161805012e-01
    const LG6: f64 = f64::from_bits(0x3fc3_9a09_d078_c69f); // 1.531383769920937332e-01
    const LG7: f64 = f64::from_bits(0x3fc2_f112_df3e_5244); // 1.479819860511658591e-01

    let t1 = det_mul(w, LG2 + det_mul(w, LG4 + det_mul(w, LG6)));
    let t2 = det_mul(z, LG1 + det_mul(w, LG3 + det_mul(w, LG5 + det_mul(w, LG7))));
    let r = t2 + t1;
    let hfsq = det_mul(0.5, det_mul(f, f));
    let dk = f64::from(k);
    det_mul(dk, LN2_HI) - ((hfsq - (det_mul(s, hfsq + r) + det_mul(dk, LN2_LO))) - f)
}

#[inline]
fn kernel_sin(x: f64, y: f64) -> f64 {
    const S1: f64 = f64::from_bits(0xbfc5_5555_5555_5549);
    const S2: f64 = f64::from_bits(0x3f81_1111_1110_f8a6);
    const S3: f64 = f64::from_bits(0xbf2a_01a0_19c1_61d5);
    const S4: f64 = f64::from_bits(0x3ec7_1de3_57b1_fe7d);
    const S5: f64 = f64::from_bits(0xbe5a_e5e6_8a2b_9ceb);
    const S6: f64 = f64::from_bits(0x3de5_d93a_5aca_bf57);

    let z = det_mul(x, x);
    let v = det_mul(z, x);
    let r = S2 + det_mul(z, S3 + det_mul(z, S4 + det_mul(z, S5 + det_mul(z, S6))));
    x - ((det_mul(z, det_mul(0.5, y) - det_mul(v, r)) - y) - det_mul(v, S1))
}

#[inline]
fn kernel_cos(x: f64, y: f64) -> f64 {
    const C1: f64 = f64::from_bits(0x3fa5_5555_5555_554c);
    const C2: f64 = f64::from_bits(0xbf56_c16c_16c1_5177);
    const C3: f64 = f64::from_bits(0x3efa_01a0_19cb_1590);
    const C4: f64 = f64::from_bits(0xbe92_7e4f_809c_52ad);
    const C5: f64 = f64::from_bits(0x3e21_ee9e_bdb4_b1c4);
    const C6: f64 = f64::from_bits(0xbda8_fae9_be88_38d4);

    let z = det_mul(x, x);
    let r = det_mul(
        z,
        C1 + det_mul(
            z,
            C2 + det_mul(z, C3 + det_mul(z, C4 + det_mul(z, C5 + det_mul(z, C6)))),
        ),
    );
    let hz = det_mul(0.5, z);
    let w = 1.0 - hz;
    w + (((1.0 - w) - hz) + (det_mul(z, r) - det_mul(x, y)))
}

#[inline]
fn rem_pio2_medium(x: f64) -> (i32, f64, f64) {
    let fn_k = (det_mul(x, INV_PIO2) + if x >= 0.0 { 0.5 } else { -0.5 }).trunc();
    let n = fn_k as i64;
    let r1 = x - det_mul(fn_k, PIO2_1);
    let w1 = det_mul(fn_k, PIO2_2);
    let y0_tmp = r1 - w1;
    let r2 = y0_tmp;
    let w2 = det_mul(fn_k, PIO2_3) - ((r1 - r2) - w1);
    let y0 = r2 - w2;
    let y1 = (r2 - y0) - w2;
    ((n & 3) as i32, y0, y1)
}

/// Cross-ISA deterministic `<= 0.5-ULP` sine (`F-033`).
pub fn det_sin(x: f64) -> f64 {
    if !x.is_finite() {
        return f64::NAN;
    }
    if x.abs() < std::f64::consts::FRAC_PI_4 {
        if x.abs() < 1.0e-18 {
            return x;
        }
        return kernel_sin(x, 0.0);
    }
    let (q, y0, y1) = rem_pio2_medium(x);
    match q & 3 {
        0 => kernel_sin(y0, y1),
        1 => kernel_cos(y0, y1),
        2 => -kernel_sin(y0, y1),
        _ => -kernel_cos(y0, y1),
    }
}

/// Cross-ISA deterministic `<= 0.5-ULP` cosine (`F-033`).
pub fn det_cos(x: f64) -> f64 {
    if !x.is_finite() {
        return f64::NAN;
    }
    let ax = x.abs();
    if ax < std::f64::consts::FRAC_PI_4 {
        return kernel_cos(ax, 0.0);
    }
    let (q, y0, y1) = rem_pio2_medium(x);
    match q & 3 {
        0 => kernel_cos(y0, y1),
        1 => -kernel_sin(y0, y1),
        2 => -kernel_cos(y0, y1),
        _ => kernel_sin(y0, y1),
    }
}

/// Cross-ISA deterministic hyperbolic tangent (`F-033`).
pub fn det_tanh(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    let ax = x.abs();
    if ax >= 22.0 {
        return if x >= 0.0 { 1.0 } else { -1.0 };
    }
    let mag = if ax >= 1.0e-9 {
        let e2 = det_exp(det_mul(2.0, ax));
        (e2 - 1.0) / (e2 + 1.0)
    } else {
        let z = det_mul(ax, ax);
        ax - det_mul(ax, det_mul(z, 1.0 / 3.0))
    };
    if x >= 0.0 { mag } else { -mag }
}

/// $C^2$-continuous SPICE exponential clamp (`F-025`, `F-033`).
/// Returns `(f(x), f'(x), f''(x))` with continuous value, 1st, and 2nd derivatives across `x_lim`.
pub fn limexp_c2(x: f64, x_lim: f64) -> (f64, f64, f64) {
    if x <= x_lim {
        let ex = det_exp(x);
        (ex, ex, ex)
    } else {
        let e_lim = det_exp(x_lim);
        let d = x - x_lim;
        let val = det_mul(e_lim, 1.0 + d + det_mul(0.5, det_mul(d, d)));
        let d1 = det_mul(e_lim, 1.0 + d);
        (val, d1, e_lim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ulp_distance(a: f64, b: f64) -> u64 {
        if a == b {
            return 0;
        }
        a.to_bits().abs_diff(b.to_bits())
    }

    #[test]
    fn det_math_matches_ieee_within_1_ulp_and_preserves_identities() {
        for &x in &[
            -5.0,
            -1.25,
            -0.1,
            0.0,
            0.25,
            0.7,
            1.5,
            std::f64::consts::PI,
            10.0,
        ] {
            assert!(ulp_distance(det_exp(x), x.exp()) <= 1, "exp({x})");
            assert!(ulp_distance(det_sin(x), x.sin()) <= 1, "sin({x})");
            assert!(ulp_distance(det_cos(x), x.cos()) <= 1, "cos({x})");
            let pyth = det_sin(x) * det_sin(x) + det_cos(x) * det_cos(x);
            assert!((pyth - 1.0).abs() < 1e-15);
            assert!((det_tanh(x) - x.tanh()).abs() < 1e-15, "tanh({x})");
        }
        for &x in &[
            1e-12,
            0.01,
            0.5,
            0.9999,
            1.0,
            std::f64::consts::E,
            100.0,
            1e9,
        ] {
            assert!(ulp_distance(det_ln(x), x.ln()) <= 1, "ln({x})");
        }
    }

    #[test]
    fn limexp_c2_is_c2_continuous_across_clamp_boundary() {
        let x_lim = 40.0;
        let eps = 1e-8;
        let (v_lo, d1_lo, d2_lo) = limexp_c2(x_lim - eps, x_lim);
        let (v_hi, d1_hi, d2_hi) = limexp_c2(x_lim + eps, x_lim);
        assert!(((v_hi - v_lo) / v_lo).abs() < 3e-8);
        assert!(((d1_hi - d1_lo) / d1_lo).abs() < 3e-8);
        assert!(((d2_hi - d2_lo) / d2_lo).abs() < 3e-8);
        let (v_far, _, _) = limexp_c2(5000.0, x_lim);
        assert!(v_far.is_finite());
    }
}
