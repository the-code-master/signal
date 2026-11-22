// Module: engine/crates/signal-geom/src/camera6dof.rs — Gimbal-Lock-Free Quaternion 6-DOF SpaceMouse Camera
// Invariants: Pure unit-quaternion SO(3) exponential map via core_math; zero Euler-angle singularities (`FEAT-093`).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_core::core_math::{det_cos, det_sin};

/// Unit quaternion `(w, x, y, z)` representing a 3D orientation without gimbal lock (`FEAT-093`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat64 {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Quat64 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quat64 {
    pub const IDENTITY: Self = Self {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn normalized(self) -> Self {
        let norm_sq = self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z;
        if norm_sq <= 1.0e-30 {
            return Self::IDENTITY;
        }
        let inv = 1.0 / norm_sq.sqrt();
        Self {
            w: self.w * inv,
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }

    /// Exponential map from body-frame rotation vector `(rx, ry, rz)` in radians.
    pub fn from_rotation_vector(rx: f64, ry: f64, rz: f64) -> Self {
        let angle = (rx * rx + ry * ry + rz * rz).sqrt();
        if angle < 1.0e-12 {
            return Self {
                w: 1.0,
                x: 0.5 * rx,
                y: 0.5 * ry,
                z: 0.5 * rz,
            }
            .normalized();
        }
        let half = 0.5 * angle;
        let s = det_sin(half) / angle;
        Self {
            w: det_cos(half),
            x: rx * s,
            y: ry * s,
            z: rz * s,
        }
        .normalized()
    }

    pub fn hamilton_product(self, rhs: Self) -> Self {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
        .normalized()
    }

    pub fn conjugate(self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn rotate_vec3(self, v: [f64; 3]) -> [f64; 3] {
        // v' = v + 2 * w * (q_xyz x v) + 2 * (q_xyz x (q_xyz x v))
        let tx = 2.0 * (self.y * v[2] - self.z * v[1]);
        let ty = 2.0 * (self.z * v[0] - self.x * v[2]);
        let tz = 2.0 * (self.x * v[1] - self.y * v[0]);
        [
            v[0] + self.w * tx + (self.y * tz - self.z * ty),
            v[1] + self.w * ty + (self.z * tx - self.x * tz),
            v[2] + self.w * tz + (self.x * ty - self.y * tx),
        ]
    }
}

/// 6-DOF SpaceMouse / CAD viewport camera (`FEAT-093`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera6Dof {
    pub eye_nm: [f64; 3],
    pub orientation: Quat64,
    pub translation_speed_nm_per_unit: f64,
}

impl Default for Camera6Dof {
    fn default() -> Self {
        Self {
            eye_nm: [0.0, 0.0, 50_000_000.0],
            orientation: Quat64::IDENTITY,
            translation_speed_nm_per_unit: 1_000_000.0,
        }
    }
}

impl Camera6Dof {
    /// Applies a 6-DOF SpaceMouse delta (`tx, ty, tz` in milli-units, `rx, ry, rz` in milliradians).
    pub fn apply_spacemouse_milli(
        &mut self,
        tx_milli: i32,
        ty_milli: i32,
        tz_milli: i32,
        rx_milli_rad: i32,
        ry_milli_rad: i32,
        rz_milli_rad: i32,
    ) {
        let dq = Quat64::from_rotation_vector(
            f64::from(rx_milli_rad) * 1.0e-3,
            f64::from(ry_milli_rad) * 1.0e-3,
            f64::from(rz_milli_rad) * 1.0e-3,
        );
        self.orientation = self.orientation.hamilton_product(dq);

        let local_step = [
            f64::from(tx_milli) * 1.0e-3 * self.translation_speed_nm_per_unit,
            f64::from(ty_milli) * 1.0e-3 * self.translation_speed_nm_per_unit,
            f64::from(tz_milli) * 1.0e-3 * self.translation_speed_nm_per_unit,
        ];
        let world_delta = self.orientation.rotate_vec3(local_step);
        for (coord, d) in self.eye_nm.iter_mut().zip(world_delta) {
            *coord += d;
        }
    }

    pub fn forward_axis(&self) -> [f64; 3] {
        self.orientation.rotate_vec3([0.0, 0.0, -1.0])
    }

    /// Column-major 4x4 rigid view matrix transforming world coordinates into camera space.
    pub fn view_matrix_col_major(&self) -> [f64; 16] {
        let inv = self.orientation.conjugate();
        let x_ax = inv.rotate_vec3([1.0, 0.0, 0.0]);
        let y_ax = inv.rotate_vec3([0.0, 1.0, 0.0]);
        let z_ax = inv.rotate_vec3([0.0, 0.0, 1.0]);
        let t = inv.rotate_vec3([-self.eye_nm[0], -self.eye_nm[1], -self.eye_nm[2]]);
        [
            x_ax[0], y_ax[0], z_ax[0], 0.0, x_ax[1], y_ax[1], z_ax[1], 0.0, x_ax[2], y_ax[2],
            z_ax[2], 0.0, t[0], t[1], t[2], 1.0,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_6dof_rotates_past_90_deg_pitch_without_gimbal_lock() {
        let mut cam = Camera6Dof::default();
        // Pitch +90 deg (1571 mrad), then yaw +90 deg, then roll +90 deg
        cam.apply_spacemouse_milli(0, 0, 0, 1571, 0, 0);
        cam.apply_spacemouse_milli(0, 0, 0, 0, 1571, 0);
        cam.apply_spacemouse_milli(1000, 0, 0, 0, 0, 1571);
        let fwd = cam.forward_axis();
        let norm = (fwd[0] * fwd[0] + fwd[1] * fwd[1] + fwd[2] * fwd[2]).sqrt();
        assert!((norm - 1.0).abs() < 1e-12);
        let m = cam.view_matrix_col_major();
        assert!((m[15] - 1.0).abs() < 1e-12);
    }
}
