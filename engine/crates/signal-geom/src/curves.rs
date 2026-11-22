// Module: engine/crates/signal-geom/src/curves.rs — Adiabatic Euler Clothoid Spirals & Cubic Spline Samplers
// Invariants: Cross-ISA deterministic Fresnel quadrature via core_math; exact i128 nanometer output (F-030, F-031).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
use crate::GeomError;
use crate::primitives::Point2Nm;
use signal_core::core_math::{det_cos, det_sin};

/// Adiabatic Euler (Clothoid) spiral segment with linear curvature $\kappa(s) = \kappa_0 + c \cdot s$ (`F-031`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EulerSpiral2d {
    pub origin: Point2Nm,
    pub start_heading_rad: f64,
    pub length_nm: f64,
    pub start_curvature_per_nm: f64,
    pub end_curvature_per_nm: f64,
}

impl EulerSpiral2d {
    /// Integrates exact Fresnel arc-length integrals via composite Simpson's rule (`F-031`).
    pub fn sample(&self, segments: usize) -> Result<Vec<Point2Nm>, GeomError> {
        if segments == 0 || !self.length_nm.is_finite() || self.length_nm <= 0.0 {
            return Err(GeomError::InvalidCurveParameter(
                "clothoid segments and length_nm must be positive".to_string(),
            ));
        }
        let c_rate = (self.end_curvature_per_nm - self.start_curvature_per_nm) / self.length_nm;
        let ds = self.length_nm / (segments as f64);

        let heading_at = |s: f64| -> f64 {
            self.start_heading_rad + self.start_curvature_per_nm * s + 0.5 * c_rate * s * s
        };

        let mut pts = Vec::with_capacity(segments + 1);
        pts.push(self.origin);
        let mut x_acc = 0.0_f64;
        let mut y_acc = 0.0_f64;

        for i in 0..segments {
            let s0 = (i as f64) * ds;
            let sm = s0 + 0.5 * ds;
            let s1 = s0 + ds;
            let th0 = heading_at(s0);
            let thm = heading_at(sm);
            let th1 = heading_at(s1);
            x_acc += (ds / 6.0) * (det_cos(th0) + 4.0 * det_cos(thm) + det_cos(th1));
            y_acc += (ds / 6.0) * (det_sin(th0) + 4.0 * det_sin(thm) + det_sin(th1));
            pts.push(Point2Nm::new(
                self.origin.x.0.saturating_add(x_acc.round() as i64),
                self.origin.y.0.saturating_add(y_acc.round() as i64),
            ));
        }
        Ok(pts)
    }
}

/// Exact fixed-point 2D Cubic Bézier curve (`F-030`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CubicBezier2Nm {
    pub p0: Point2Nm,
    pub p1: Point2Nm,
    pub p2: Point2Nm,
    pub p3: Point2Nm,
}

impl CubicBezier2Nm {
    pub fn eval_at(&self, t: f64) -> Point2Nm {
        let u = 1.0 - t;
        let b0 = u * u * u;
        let b1 = 3.0 * u * u * t;
        let b2 = 3.0 * u * t * t;
        let b3 = t * t * t;
        let x = b0 * (self.p0.x.0 as f64)
            + b1 * (self.p1.x.0 as f64)
            + b2 * (self.p2.x.0 as f64)
            + b3 * (self.p3.x.0 as f64);
        let y = b0 * (self.p0.y.0 as f64)
            + b1 * (self.p1.y.0 as f64)
            + b2 * (self.p2.y.0 as f64)
            + b3 * (self.p3.y.0 as f64);
        Point2Nm::new(x.round() as i64, y.round() as i64)
    }

    pub fn sample_uniform(&self, segments: usize) -> Result<Vec<Point2Nm>, GeomError> {
        if segments == 0 {
            return Err(GeomError::InvalidCurveParameter(
                "spline segments must be >= 1".to_string(),
            ));
        }
        let mut pts = Vec::with_capacity(segments + 1);
        pts.push(self.p0);
        for i in 1..segments {
            pts.push(self.eval_at((i as f64) / (segments as f64)));
        }
        pts.push(self.p3);
        Ok(pts)
    }
}

/// Samples a $C^1$-continuous Catmull-Rom cubic spline passing through `waypoints` (`F-030`).
pub fn sample_catmull_rom_spline(
    waypoints: &[Point2Nm],
    substeps_per_span: usize,
) -> Result<Vec<Point2Nm>, GeomError> {
    if waypoints.len() < 2 || substeps_per_span == 0 {
        return Err(GeomError::InvalidCurveParameter(
            "Catmull-Rom requires >= 2 waypoints and >= 1 substep".to_string(),
        ));
    }
    let mut out = Vec::with_capacity((waypoints.len() - 1) * substeps_per_span + 1);
    out.push(waypoints[0]);

    for i in 0..(waypoints.len() - 1) {
        let p_prev = if i == 0 {
            waypoints[0]
        } else {
            waypoints[i - 1]
        };
        let p0 = waypoints[i];
        let p1 = waypoints[i + 1];
        let p_next = if i + 2 < waypoints.len() {
            waypoints[i + 2]
        } else {
            waypoints[waypoints.len() - 1]
        };

        // Convert Catmull-Rom span to equivalent Cubic Bezier control points
        let c1 = Point2Nm::new(
            p0.x.0 + (p1.x.0 - p_prev.x.0) / 6,
            p0.y.0 + (p1.y.0 - p_prev.y.0) / 6,
        );
        let c2 = Point2Nm::new(
            p1.x.0 - (p_next.x.0 - p0.x.0) / 6,
            p1.y.0 - (p_next.y.0 - p0.y.0) / 6,
        );
        let bez = CubicBezier2Nm {
            p0,
            p1: c1,
            p2: c2,
            p3: p1,
        };
        let sampled = bez.sample_uniform(substeps_per_span)?;
        out.extend_from_slice(&sampled[1..]);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euler_clothoid_90_deg_adiabatic_bend_conserves_arc_length() {
        let l_nm = 20_000.0;
        // Total heading change = 0.5 * (k0 + k1) * L = pi / 2 => k1 = pi / L when k0 = 0
        let spiral = EulerSpiral2d {
            origin: Point2Nm::new(0, 0),
            start_heading_rad: 0.0,
            length_nm: l_nm,
            start_curvature_per_nm: 0.0,
            end_curvature_per_nm: std::f64::consts::PI / l_nm,
        };
        let pts = spiral.sample(64).expect("sample");
        assert_eq!(pts.len(), 65);
        assert!(pts[64].x.0 > 0 && pts[64].y.0 > 0);
        let mut poly_len = 0.0;
        for w in pts.windows(2) {
            poly_len += (w[0].dist_sq_i128(w[1]) as f64).sqrt();
        }
        assert!(((poly_len - l_nm) / l_nm).abs() < 1e-3);
    }

    #[test]
    fn cubic_bezier_and_catmull_rom_interpolate_endpoints() {
        let w = [
            Point2Nm::new(0, 0),
            Point2Nm::new(10_000, 5_000),
            Point2Nm::new(20_000, 0),
        ];
        let pts = sample_catmull_rom_spline(&w, 8).expect("spline");
        assert_eq!(pts.first(), Some(&w[0]));
        assert_eq!(pts.get(8), Some(&w[1]));
        assert_eq!(pts.last(), Some(&w[2]));
    }
}
