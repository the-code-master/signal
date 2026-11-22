// Module: engine/crates/signal-geom/src/lib.rs — Overflow-Proof i128 Geometry, Shewchuk Predicates, R*-Tree & 6-DOF Camera
// Invariants: Never multiplies i64 * i64 directly; adaptive floating predicates fall back to exact expansions (F-030, F-031).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
#![forbid(unsafe_code)]

use thiserror::Error;

pub mod camera6dof;
pub mod curves;
pub mod predicates;
pub mod primitives;
pub mod rtree;

pub use camera6dof::{Camera6Dof, Quat64};
pub use curves::{CubicBezier2Nm, EulerSpiral2d, sample_catmull_rom_spline};
pub use predicates::{
    InCircle, Orientation, incircle_adaptive, incircle_nm, orient2d_adaptive, orient2d_i128,
};
pub use primitives::{BBox2Nm, BBox3Nm, Point2Nm, Point3Nm};
pub use rtree::SpatialIndex;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GeomError {
    #[error("invalid curve parameter: {0}")]
    InvalidCurveParameter(String),
}

#[cfg(test)]
mod tests {
    use super::{Orientation, Point2Nm, orient2d_i128};
    use signal_core::Nanometers;

    #[test]
    fn handles_meter_scale_coordinates_beyond_i64_square_limit() {
        let a = Point2Nm {
            x: Nanometers(0),
            y: Nanometers(0),
        };
        let b = Point2Nm {
            x: Nanometers(5_000_000_000),
            y: Nanometers(0),
        };
        let c = Point2Nm {
            x: Nanometers(5_000_000_000),
            y: Nanometers(5_000_000_000),
        };
        assert_eq!(orient2d_i128(a, b, c), Orientation::CounterClockwise);
    }
}
