// Module: engine/crates/signal-geom/src/primitives.rs — Exact i128 Fixed-Point 2D/3D Nanometer Points & Bounding Boxes
// Invariants: Never multiplies i64 * i64 directly; all 1 nm products/areas/distances widen to i128 first (F-030).
// Spec: architecture/03-rust-eda-engine.md
use signal_core::Nanometers;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point2Nm {
    pub x: Nanometers,
    pub y: Nanometers,
}

impl Point2Nm {
    pub const ORIGIN: Self = Self {
        x: Nanometers(0),
        y: Nanometers(0),
    };

    pub const fn new(x_nm: i64, y_nm: i64) -> Self {
        Self {
            x: Nanometers(x_nm),
            y: Nanometers(y_nm),
        }
    }

    pub fn dist_sq_i128(self, other: Self) -> i128 {
        let dx = i128::from(self.x.0) - i128::from(other.x.0);
        let dy = i128::from(self.y.0) - i128::from(other.y.0);
        dx * dx + dy * dy
    }

    pub fn manhattan_i128(self, other: Self) -> i128 {
        (i128::from(self.x.0) - i128::from(other.x.0)).abs()
            + (i128::from(self.y.0) - i128::from(other.y.0)).abs()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point3Nm {
    pub x: Nanometers,
    pub y: Nanometers,
    pub z: Nanometers,
}

impl Point3Nm {
    pub const fn new(x_nm: i64, y_nm: i64, z_nm: i64) -> Self {
        Self {
            x: Nanometers(x_nm),
            y: Nanometers(y_nm),
            z: Nanometers(z_nm),
        }
    }

    pub fn dist_sq_i128(self, other: Self) -> i128 {
        let dx = i128::from(self.x.0) - i128::from(other.x.0);
        let dy = i128::from(self.y.0) - i128::from(other.y.0);
        let dz = i128::from(self.z.0) - i128::from(other.z.0);
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BBox2Nm {
    pub min: Point2Nm,
    pub max: Point2Nm,
}

impl BBox2Nm {
    pub fn from_points(a: Point2Nm, b: Point2Nm) -> Self {
        Self {
            min: Point2Nm::new(a.x.0.min(b.x.0), a.y.0.min(b.y.0)),
            max: Point2Nm::new(a.x.0.max(b.x.0), a.y.0.max(b.y.0)),
        }
    }

    pub const fn from_point(p: Point2Nm) -> Self {
        Self { min: p, max: p }
    }

    pub fn width_i128(&self) -> i128 {
        (i128::from(self.max.x.0) - i128::from(self.min.x.0)).max(0)
    }

    pub fn height_i128(&self) -> i128 {
        (i128::from(self.max.y.0) - i128::from(self.min.y.0)).max(0)
    }

    pub fn area_i128(&self) -> i128 {
        self.width_i128() * self.height_i128()
    }

    pub fn half_perimeter_i128(&self) -> i128 {
        self.width_i128() + self.height_i128()
    }

    pub fn center_nm(&self) -> Point2Nm {
        let cx = (i128::from(self.min.x.0) + i128::from(self.max.x.0)) / 2;
        let cy = (i128::from(self.min.y.0) + i128::from(self.max.y.0)) / 2;
        Point2Nm::new(cx as i64, cy as i64)
    }

    pub fn contains_point(&self, p: Point2Nm) -> bool {
        p.x.0 >= self.min.x.0
            && p.x.0 <= self.max.x.0
            && p.y.0 >= self.min.y.0
            && p.y.0 <= self.max.y.0
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x.0 <= other.max.x.0
            && self.max.x.0 >= other.min.x.0
            && self.min.y.0 <= other.max.y.0
            && self.max.y.0 >= other.min.y.0
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: Point2Nm::new(
                self.min.x.0.min(other.min.x.0),
                self.min.y.0.min(other.min.y.0),
            ),
            max: Point2Nm::new(
                self.max.x.0.max(other.max.x.0),
                self.max.y.0.max(other.max.y.0),
            ),
        }
    }

    pub fn enlargement_i128(&self, other: &Self) -> i128 {
        self.union(other).area_i128() - self.area_i128()
    }

    pub fn min_dist_sq_to_point_i128(&self, p: Point2Nm) -> i128 {
        let px = i128::from(p.x.0);
        let py = i128::from(p.y.0);
        let cx = px.clamp(i128::from(self.min.x.0), i128::from(self.max.x.0));
        let cy = py.clamp(i128::from(self.min.y.0), i128::from(self.max.y.0));
        let dx = px - cx;
        let dy = py - cy;
        dx * dx + dy * dy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BBox3Nm {
    pub min: Point3Nm,
    pub max: Point3Nm,
}

impl BBox3Nm {
    pub fn from_points(a: Point3Nm, b: Point3Nm) -> Self {
        Self {
            min: Point3Nm::new(a.x.0.min(b.x.0), a.y.0.min(b.y.0), a.z.0.min(b.z.0)),
            max: Point3Nm::new(a.x.0.max(b.x.0), a.y.0.max(b.y.0), a.z.0.max(b.z.0)),
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x.0 <= other.max.x.0
            && self.max.x.0 >= other.min.x.0
            && self.min.y.0 <= other.max.y.0
            && self.max.y.0 >= other.min.y.0
            && self.min.z.0 <= other.max.z.0
            && self.max.z.0 >= other.min.z.0
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: Point3Nm::new(
                self.min.x.0.min(other.min.x.0),
                self.min.y.0.min(other.min.y.0),
                self.min.z.0.min(other.min.z.0),
            ),
            max: Point3Nm::new(
                self.max.x.0.max(other.max.x.0),
                self.max.y.0.max(other.max.y.0),
                self.max.z.0.max(other.max.z.0),
            ),
        }
    }
}
