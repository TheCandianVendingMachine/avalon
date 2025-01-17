use nalgebra_glm::{ quat, Quat, quat_to_mat4, vec3, Vec3, vec4, Mat4 };
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
    orientation_quaternion: Quat,
    transformation_matrix: Mat4,
    dirty: bool
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vec3::zeros(),
            orientation_quaternion: Quat::default(),
            transformation_matrix: Mat4::identity(),
            dirty: true
        }
    }

    pub fn matrix(&self) -> Mat4 {
        if self.dirty {
            quat_to_mat4(&self.orientation_quaternion) * Mat4::from_columns(&[
                vec4(1.0, 0.0, 0.0, 0.0),
                vec4(0.0, 1.0, 0.0, 0.0),
                vec4(0.0, 0.0, 1.0, 0.0),
                vec4(self.position.x, self.position.y, self.position.z, 1.0)
            ])
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
        self.orientation_quaternion =
            Quat::from_polar_decomposition(1.0, euler.pitch, Unit::new_normalize(vec3(1.0, 0.0, 0.0))) *
            Quat::from_polar_decomposition(1.0, euler.yaw, Unit::new_normalize(vec3(0.0, 1.0, 0.0))) *
            Quat::from_polar_decomposition(1.0, euler.roll, Unit::new_normalize(vec3(0.0, 0.0, 1.0)));
    }

    pub fn euler_angles(&self) -> Euler {
        self.matrix().into()
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
            orientation_quaternion: self.orientation_quaternion * rhs.orientation_quaternion,
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
            let theta = -transform.m32.asin();
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
            sin_psi * sin_theta * sin_phi - cos_psi * sin_phi,
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
