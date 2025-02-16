use std::ops::{ Add, Sub, Mul, Neg };
use crate::{ scalar, Matrix2, Vector2 };

impl<T> Matrix2<T> where
    T: scalar::Identity {
    pub fn identity() -> Matrix2<T> {
        scalar::mat2::identity()
    }
}

impl<T> Matrix2<T> where
    T: Copy +  Sub<Output = T> + Mul<Output = T> {
    pub fn determinate(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::determinate(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::determinate(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::determinate(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::determinate(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::determinate(self)
        } else {
            scalar::mat2::determinate(self)
        }
    }
}

impl<T> Matrix2<T> where
    T: Copy + Add<Output = T> {
    pub fn trace(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::trace(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::trace(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::trace(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::trace(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::trace(self)
        } else {
            scalar::mat2::trace(self)
        }
    }
}

impl<T> Matrix2<T> where
    T: Copy + scalar::Identity + Mul<Output = T> + Add<Output = T> {
    pub fn pow(self, power: u64) -> Matrix2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::pow(self, power)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::pow(self, power)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::pow(self, power)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::pow(self, power)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::pow(self, power)
        } else {
            scalar::mat2::pow(self, power)
        }
    }
}

impl<T> Matrix2<T> {
    pub fn transpose(self) -> Matrix2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::transpose(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::transpose(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::transpose(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::transpose(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::transpose(self)
        } else {
            scalar::mat2::transpose(self)
        }
    }
}

impl<T> Matrix2<T> where
    T: Copy + scalar::Inverse + Neg<Output = T> + Sub<Output = T> + Mul<Output = T> {
    pub fn inverse(self) -> Matrix2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::inverse(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::inverse(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::inverse(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::inverse(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::inverse(self)
        } else {
            scalar::mat2::inverse(self)
        }
    }
}

impl<T> Add for Matrix2<T> where
    T: Copy + Add<Output = T>
    {
    type Output = Matrix2<T>;
    fn add(self, rhs: Matrix2<T>) -> Matrix2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::add(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::add(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::add(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::add(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::add(self, rhs)
        } else {
            scalar::mat2::add(self, rhs)
        }
    }
}

impl<T> Sub for Matrix2<T> where
    T: Copy + Sub<Output = T>
    {
    type Output = Matrix2<T>;
    fn sub(self, rhs: Matrix2<T>) -> Matrix2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::sub(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::sub(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::sub(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::sub(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::sub(self, rhs)
        } else {
            scalar::mat2::sub(self, rhs)
        }
    }
}

impl<T> Mul for Matrix2<T> where
    T: Copy + Mul<Output = T> + Add<Output = T>
    {
    type Output = Matrix2<T>;
    fn mul(self, rhs: Matrix2<T>) -> Matrix2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::multiply(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::multiply(self, rhs)
        } else {
            scalar::mat2::multiply(self, rhs)
        }
    }
}

impl<T> Mul<T> for Matrix2<T> where
    T: Copy + Mul<Output = T>
    {
    type Output = Matrix2<T>;
    fn mul(self, rhs: T) -> Matrix2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::multiply_scalar(self, rhs)
        } else {
            scalar::mat2::multiply_scalar(self, rhs)
        }
    }
}

impl<T> Mul<Vector2<T>> for Matrix2<T> where
    T: Copy + Mul<Output = T> + Add<Output = T>
    {
    type Output = Vector2<T>;
    fn mul(self, rhs: Vector2<T>) -> Vector2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat2::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat2::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat2::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat2::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat2::multiply_vec(self, rhs)
        } else {
            scalar::mat2::multiply_vec(self, rhs)
        }
    }
}
