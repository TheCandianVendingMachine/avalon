use std::ops::{ Add, Sub, Mul, Neg };
use crate::{ scalar, Matrix3, Vector3 };

impl<T> Matrix3<T> where
    T: scalar::Identity {
    pub fn identity() -> Matrix3<T> {
        scalar::mat3::identity()
    }
}

impl<T> Matrix3<T> where
    T: Copy + scalar::Inverse + Sub<Output = T> + Mul<Output = T> + Add<Output = T> {
    pub fn determinate(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::determinate(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::determinate(self)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::determinate(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::determinate(self)
        } else {
            scalar::mat3::determinate(self)
        }
    }
}

impl<T> Matrix3<T> where
    T: Copy + Add<Output = T> {
    pub fn trace(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::trace(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::trace(self)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::trace(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::trace(self)
        } else {
            scalar::mat3::trace(self)
        }
    }
}

impl<T> Matrix3<T> where
    T: Copy + scalar::Identity + Mul<Output = T> + Add<Output = T> {
    pub fn pow(self, power: u64) -> Matrix3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::pow(self, power)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::pow(self, power)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::pow(self, power)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::pow(self, power)
        } else {
            scalar::mat3::pow(self, power)
        }
    }
}

impl<T> Matrix3<T> {
    pub fn transpose(self) -> Matrix3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::transpose(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::transpose(self)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::transpose(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::transpose(self)
        } else {
            scalar::mat3::transpose(self)
        }
    }
}

impl<T> Matrix3<T> where
    T: Copy + scalar::Inverse + Neg<Output = T> + Sub<Output = T> + Mul<Output = T> + Add<Output = T> {
    pub fn inverse(self) -> Matrix3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::inverse(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::inverse(self)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::inverse(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::inverse(self)
        } else {
            scalar::mat3::inverse(self)
        }
    }
}

impl<T> Add for Matrix3<T> where
    T: Copy + Add<Output = T>
    {
    type Output = Matrix3<T>;
    fn add(self, rhs: Matrix3<T>) -> Matrix3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::add(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::add(self, rhs)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::add(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::add(self, rhs)
        } else {
            scalar::mat3::add(self, rhs)
        }
    }
}

impl<T> Sub for Matrix3<T> where
    T: Copy + Sub<Output = T>
    {
    type Output = Matrix3<T>;
    fn sub(self, rhs: Matrix3<T>) -> Matrix3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::sub(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::sub(self, rhs)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::sub(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::sub(self, rhs)
        } else {
            scalar::mat3::sub(self, rhs)
        }
    }
}

impl<T> Mul for Matrix3<T> where
    T: Copy + Mul<Output = T> + Add<Output = T>
    {
    type Output = Matrix3<T>;
    fn mul(self, rhs: Matrix3<T>) -> Matrix3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::multiply(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::multiply(self, rhs)
        } else {
            scalar::mat3::multiply(self, rhs)
        }
    }
}

impl<T> Mul<T> for Matrix3<T> where
    T: Copy + Mul<Output = T>
    {
    type Output = Matrix3<T>;
    fn mul(self, rhs: T) -> Matrix3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::multiply_scalar(self, rhs)
        } else {
            scalar::mat3::multiply_scalar(self, rhs)
        }
    }
}

impl<T> Mul<Vector3<T>> for Matrix3<T> where
    T: Copy + Mul<Output = T> + Add<Output = T>
    {
    type Output = Vector3<T>;
    fn mul(self, rhs: Vector3<T>) -> Vector3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat3::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat3::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse4.2") {
            scalar::mat3::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat3::multiply_vec(self, rhs)
        } else {
            scalar::mat3::multiply_vec(self, rhs)
        }
    }
}
