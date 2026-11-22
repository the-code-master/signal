// Module: engine/crates/signal-geom/src/lib.rs — Overflow-Proof i128 Fixed-Point Geometry Predicates
// Invariants: Never multiplies i64 * i64 directly; all 1 nm cross/dot products widen to i128 first (F-030).
// Spec: architecture/03-rust-eda-engine.md
#![forbid(unsafe_code)]

use signal_core::Nanometers;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2Nm {
    pub x: Nanometers,
    pub y: Nanometers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    CounterClockwise,
    Clockwise,
    Collinear,
}

pub fn orient2d_i128(a: Point2Nm, b: Point2Nm, c: Point2Nm) -> Orientation {
    let ab_x = i128::from(b.x.0) - i128::from(a.x.0);
    let ab_y = i128::from(b.y.0) - i128::from(a.y.0);
    let ac_x = i128::from(c.x.0) - i128::from(a.x.0);
    let ac_y = i128::from(c.y.0) - i128::from(a.y.0);
    let det = ab_x * ac_y - ab_y * ac_x;
    match det.cmp(&0) {
        std::cmp::Ordering::Greater => Orientation::CounterClockwise,
        std::cmp::Ordering::Less => Orientation::Clockwise,
        std::cmp::Ordering::Equal => Orientation::Collinear,
    }
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
