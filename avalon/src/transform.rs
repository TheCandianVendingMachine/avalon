use nalgebra_glm::{ Quat, quat_to_mat4, quat_to_mat3, vec3, Vec3, vec4, Mat4 };
use nalgebra::Unit;

#[derive(Debug, Copy, Clone)]
pub struct Euler {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    position: Vec3,
    right: Vec3,
    up: Vec3,
    orientation_quaternion: Quat,
    euler_angles: Euler,
    transformation_matrix: Mat4,
    dirty: bool
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vec3::zeros(),
            right: vec3(1.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            orientation_quaternion: Quat::default(),
            euler_angles: Euler::default(),
            transformation_matrix: Mat4::identity(),
            dirty: true
        }
    }

    pub fn left(&self) -> Vec3 {
        quat_to_mat3(&self.orientation_quaternion) * self.right
    }

    pub fn up(&self) -> Vec3 {
        quat_to_mat3(&self.orientation_quaternion) * self.up
    }

    pub fn forward(&self) -> Vec3 {
        let left = self.left();
        let up = self.up();
        left.cross(&up)
    }

    pub fn matrix(&self) -> Mat4 {
        if self.dirty {
            let rotation_matrix = quat_to_mat4(&self.orientation_quaternion);
            let translation_matrix = Mat4::from_columns(&[
                vec4(1.0, 0.0, 0.0, 0.0),
                vec4(0.0, 1.0, 0.0, 0.0),
                vec4(0.0, 0.0, 1.0, 0.0),
                vec4(-self.position.x, self.position.y, self.position.z, 1.0)
            ]);
            translation_matrix * rotation_matrix
        } else {
            self.transformation_matrix
        }
    }

    pub fn clean(&mut self) {
        if self.dirty {
            self.transformation_matrix = self.matrix();
            self.dirty = false;
        }
    }

    pub fn set_euler_angles(&mut self, euler: Euler) {
        self.dirty = true;
        self.euler_angles = euler;
        self.orientation_quaternion = Quat::from_polar_decomposition(1.0, 0.5 * euler.yaw, Unit::new_normalize(vec3(0.0, 1.0, 0.0)));
        self.orientation_quaternion *= Quat::from_polar_decomposition(1.0, 0.5 * euler.pitch, Unit::new_normalize(vec3(1.0, 0.0, 0.0)));
        self.orientation_quaternion *= Quat::from_polar_decomposition(1.0, 0.5 * euler.roll, Unit::new_normalize(vec3(0.0, 0.0, 1.0)));
    }

    pub fn euler_angles(&self) -> Euler {
        self.euler_angles
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.dirty = true;
        self.position = position;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn translate(&mut self, offset: Vec3) {
        self.set_position(self.position() + offset);
    }
}

impl From<&Transform> for Euler {
    fn from(transform: &Transform) -> Euler {
        transform.euler_angles()
    }
}

impl std::ops::Mul for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Transform {
        let mut transform = Transform {
            position: self.position + rhs.position,
            right: self.right,
            up: self.up,
            orientation_quaternion: self.orientation_quaternion * rhs.orientation_quaternion,
            euler_angles: self.euler_angles,
            transformation_matrix: Mat4::identity(),
            dirty: true
        };
        transform.clean();
        transform
    }
}

impl Default for Euler {
    fn default() -> Euler {
        Euler {
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0
        }
    }
}

impl From<Mat4> for Euler {
    fn from(transform: Mat4) -> Euler {
        let (theta, psi, phi) = if transform.m31.abs() != 1.0 {
            let theta = -transform.m31.asin();
            let cos_theta = theta.cos();
            let psi = (transform.m32 / cos_theta).atan2(transform.m33 / cos_theta);
            let phi = (transform.m21 / cos_theta).atan2(transform.m11 / cos_theta);

            (theta, psi, phi)
        } else {
            let phi = 0.0;
            let (theta, psi) = if transform.m31 == -1.0 {
                let theta = std::f32::consts::FRAC_PI_2;
                let psi = phi + transform.m12.atan2(transform.m13);
                (theta, psi)
            } else {
                let theta = -std::f32::consts::FRAC_PI_2;
                let psi = -phi + (-transform.m12).atan2(-transform.m13);
                (theta, psi)
            };

            (theta, psi, phi)
        };

        Euler {
            pitch: psi,
            yaw: theta,
            roll: phi
        }
    }
}

impl From<Euler> for Mat4 {
    fn from(euler: Euler) -> Mat4 {
        let cos_theta = euler.yaw.cos();
        let sin_theta = euler.yaw.sin();

        let cos_psi = euler.pitch.cos();
        let sin_psi = euler.pitch.sin();

        let cos_phi = euler.roll.cos();
        let sin_phi = euler.roll.sin();

        let r1 = vec4(
            cos_theta * cos_phi,
            sin_psi * sin_theta * cos_phi - cos_psi * sin_phi,
            cos_psi * sin_theta * cos_phi + sin_psi * sin_phi,
            0.0
        ).transpose();
        let r2 = vec4(
            cos_theta * sin_phi,
            sin_psi * sin_theta * sin_phi + cos_psi * cos_phi,
            cos_psi * sin_theta * sin_phi - sin_psi * cos_phi,
            0.0
        ).transpose();
        let r3 = vec4(
            -sin_theta,
            sin_psi * cos_theta,
            cos_psi * cos_theta,
            0.0
        ).transpose();
        let r4 = vec4(
            0.0,
            0.0,
            0.0,
            1.0
        ).transpose();

        Mat4::from_rows(&[r1, r2, r3, r4])
    }
}

#[cfg(test)]
mod euler_tests {
    use super::Euler;
    use nalgebra::Matrix4;

    #[test]
    fn test_mat4_to_euler() {
        let r = Matrix4::new(
            0.5, -0.1464, 0.8536, 0.0,
            0.5, 0.8536, -0.1464, 0.0,
            -0.7071, 0.5, 0.5, 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        let e = Euler::from(r);
        assert!((e.yaw - std::f32::consts::FRAC_PI_4).abs() < 0.01);
        assert!((e.pitch - std::f32::consts::FRAC_PI_4).abs() < 0.01);
        assert!((e.roll - std::f32::consts::FRAC_PI_4).abs() < 0.01);
    }

    #[test]
    fn test_euler_to_mat4() {
        let e = Euler {
            pitch: std::f32::consts::FRAC_PI_4,
            yaw: std::f32::consts::FRAC_PI_4,
            roll: std::f32::consts::FRAC_PI_4,
        };
        let const_mat = Matrix4::new(
            0.5, -0.1464, 0.8536, 0.0,
            0.5, 0.8536, -0.1464, 0.0,
            -0.7071, 0.5, 0.5, 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        let m = Matrix4::from(e);
        assert!((m.m11 - const_mat.m11).abs() < 0.01);
        assert!((m.m12 - const_mat.m12).abs() < 0.01);
        assert!((m.m13 - const_mat.m13).abs() < 0.01);
        assert!((m.m14 - const_mat.m14).abs() < 0.01);

        assert!((m.m21 - const_mat.m21).abs() < 0.01);
        assert!((m.m22 - const_mat.m22).abs() < 0.01);
        assert!((m.m23 - const_mat.m23).abs() < 0.01);
        assert!((m.m24 - const_mat.m24).abs() < 0.01);

        assert!((m.m31 - const_mat.m31).abs() < 0.01);
        assert!((m.m32 - const_mat.m32).abs() < 0.01);
        assert!((m.m33 - const_mat.m33).abs() < 0.01);
        assert!((m.m34 - const_mat.m34).abs() < 0.01);

        assert!((m.m41 - const_mat.m41).abs() < 0.01);
        assert!((m.m42 - const_mat.m42).abs() < 0.01);
        assert!((m.m43 - const_mat.m43).abs() < 0.01);
        assert!((m.m44 - const_mat.m44).abs() < 0.01);
    }

    #[test]
    fn test_euler_recreate() {
        let e = Euler {
            pitch: std::f32::consts::FRAC_PI_4 * 0.01,
            yaw: std::f32::consts::FRAC_PI_6 * -0.555,
            roll: std::f32::consts::FRAC_PI_8 * 0.0533,
        };

        let m = Matrix4::from(e);
        let e_tick = Euler::from(m);
        assert!((e.yaw - e_tick.yaw).abs() < 0.0001);
        assert!((e.pitch - e_tick.pitch).abs() < 0.0001);
        assert!((e.roll - e_tick.roll).abs() < 0.0001);
    }

    #[test]
    fn test_matrix_recreate() {
        let r = Matrix4::new(
            0.5, -0.1464, 0.8536, 0.0,
            0.5, 0.8536, -0.1464, 0.0,
            -0.7071, 0.5, 0.5, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let e = Euler::from(r);
        let r_tick = Matrix4::from(e);

        assert!((r.m11 - r_tick.m11).abs() < 0.0001);
        assert!((r.m12 - r_tick.m12).abs() < 0.0001);
        assert!((r.m13 - r_tick.m13).abs() < 0.0001);
        assert!((r.m14 - r_tick.m14).abs() < 0.0001);

        assert!((r.m21 - r_tick.m21).abs() < 0.0001);
        assert!((r.m22 - r_tick.m22).abs() < 0.0001);
        assert!((r.m23 - r_tick.m23).abs() < 0.0001);
        assert!((r.m24 - r_tick.m24).abs() < 0.0001);

        assert!((r.m31 - r_tick.m31).abs() < 0.0001);
        assert!((r.m32 - r_tick.m32).abs() < 0.0001);
        assert!((r.m33 - r_tick.m33).abs() < 0.0001);
        assert!((r.m34 - r_tick.m34).abs() < 0.0001);

        assert!((r.m41 - r_tick.m41).abs() < 0.0001);
        assert!((r.m42 - r_tick.m42).abs() < 0.0001);
        assert!((r.m43 - r_tick.m43).abs() < 0.0001);
        assert!((r.m44 - r_tick.m44).abs() < 0.0001);
    }
}
