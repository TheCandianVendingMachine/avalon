use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::{ scalar, Matrix4, Vector4 };

impl<T> Matrix4<T> where
    T: scalar::Identity {
    pub fn identity() -> Matrix4<T> {
        scalar::mat4::identity()
    }
}

impl<T> Matrix4<T> where
    T: Copy + scalar::Inverse + Sub<Output = T> + Mul<Output = T> + Add<Output = T> {
    pub fn determinate(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::determinate(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::determinate(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::determinate(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::determinate(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::determinate(self)
        } else {
            scalar::mat4::determinate(self)
        }
    }
}

impl<T> Matrix4<T> where
    T: Copy + Add<Output = T> {
    pub fn trace(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::trace(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::trace(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::trace(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::trace(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::trace(self)
        } else {
            scalar::mat4::trace(self)
        }
    }
}

impl<T> Matrix4<T> where
    T: Copy + scalar::Identity + Mul<Output = T> + Add<Output = T> {
    pub fn pow(self, power: u64) -> Matrix4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::pow(self, power)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::pow(self, power)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::pow(self, power)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::pow(self, power)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::pow(self, power)
        } else {
            scalar::mat4::pow(self, power)
        }
    }
}

impl<T> Matrix4<T> {
    pub fn transpose(self) -> Matrix4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::transpose(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::transpose(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::transpose(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::transpose(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::transpose(self)
        } else {
            scalar::mat4::transpose(self)
        }
    }
}

impl<T> Matrix4<T> where
    T: Copy + scalar::Inverse + scalar::Identity + Neg<Output = T> + Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Div<f64, Output = T> {
    pub fn inverse(self) -> Matrix4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::inverse(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::inverse(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::inverse(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::inverse(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::inverse(self)
        } else {
            scalar::mat4::inverse(self)
        }
    }
}

impl<T> Add for Matrix4<T> where
    T: Copy + Add<Output = T>
    {
    type Output = Matrix4<T>;
    fn add(self, rhs: Matrix4<T>) -> Matrix4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::add(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::add(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::add(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::add(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::add(self, rhs)
        } else {
            scalar::mat4::add(self, rhs)
        }
    }
}

impl<T> Sub for Matrix4<T> where
    T: Copy + Sub<Output = T>
    {
    type Output = Matrix4<T>;
    fn sub(self, rhs: Matrix4<T>) -> Matrix4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::sub(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::sub(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::sub(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::sub(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::sub(self, rhs)
        } else {
            scalar::mat4::sub(self, rhs)
        }
    }
}

impl<T> Mul for Matrix4<T> where
    T: Copy + Mul<Output = T> + Add<Output = T>
    {
    type Output = Matrix4<T>;
    fn mul(self, rhs: Matrix4<T>) -> Matrix4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::multiply(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::multiply(self, rhs)
        } else {
            scalar::mat4::multiply(self, rhs)
        }
    }
}

impl<T> Mul<T> for Matrix4<T> where
    T: Copy + Mul<Output = T>
    {
    type Output = Matrix4<T>;
    fn mul(self, rhs: T) -> Matrix4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::multiply_scalar(self, rhs)
        } else {
            scalar::mat4::multiply_scalar(self, rhs)
        }
    }
}

impl<T> Mul<Vector4<T>> for Matrix4<T> where
    T: Copy + Mul<Output = T> + Add<Output = T>
    {
    type Output = Vector4<T>;
    fn mul(self, rhs: Vector4<T>) -> Vector4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat4::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat4::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat4::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat4::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat4::multiply_vec(self, rhs)
        } else {
            scalar::mat4::multiply_vec(self, rhs)
        }
    }
}
