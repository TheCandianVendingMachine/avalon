use std::ops::{ Add, Sub, Mul };
use crate::{ scalar, Matrix1, Vector1 };

impl<T> Matrix1<T> where
    T: scalar::Identity {
    pub fn identity() -> Matrix1<T> {
        scalar::mat1::identity()
    }
}

impl<T> Matrix1<T> where
    T: Copy + scalar::Inverse {
    pub fn determinate(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::determinate(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::determinate(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::determinate(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::determinate(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::determinate(self)
        } else {
            scalar::mat1::determinate(self)
        }
    }
}

impl<T> Matrix1<T> where
    T: Copy {
    pub fn trace(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::trace(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::trace(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::trace(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::trace(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::trace(self)
        } else {
            scalar::mat1::trace(self)
        }
    }
}

impl<T> Matrix1<T> where
    T: Copy + scalar::Identity + Mul<Output = T> {
    pub fn pow(self, power: u64) -> Matrix1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::pow(self, power)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::pow(self, power)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::pow(self, power)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::pow(self, power)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::pow(self, power)
        } else {
            scalar::mat1::pow(self, power)
        }
    }
}

impl<T> Matrix1<T> {
    pub fn transpose(self) -> Matrix1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::transpose(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::transpose(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::transpose(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::transpose(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::transpose(self)
        } else {
            scalar::mat1::transpose(self)
        }
    }
}

impl<T> Matrix1<T> where
    T: scalar::Inverse {
    pub fn inverse(self) -> Matrix1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::inverse(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::inverse(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::inverse(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::inverse(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::inverse(self)
        } else {
            scalar::mat1::inverse(self)
        }
    }
}

impl<T> Add for Matrix1<T> where
    T: Copy + Add<Output = T>
    {
    type Output = Matrix1<T>;
    fn add(self, rhs: Matrix1<T>) -> Matrix1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::add(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::add(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::add(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::add(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::add(self, rhs)
        } else {
            scalar::mat1::add(self, rhs)
        }
    }
}

impl<T> Sub for Matrix1<T> where
    T: Copy + Sub<Output = T>
    {
    type Output = Matrix1<T>;
    fn sub(self, rhs: Matrix1<T>) -> Matrix1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::sub(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::sub(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::sub(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::sub(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::sub(self, rhs)
        } else {
            scalar::mat1::sub(self, rhs)
        }
    }
}

impl<T> Mul for Matrix1<T> where
    T: Copy + Mul<Output = T>
    {
    type Output = Matrix1<T>;
    fn mul(self, rhs: Matrix1<T>) -> Matrix1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::multiply(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::multiply(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::multiply(self, rhs)
        } else {
            scalar::mat1::multiply(self, rhs)
        }
    }
}

impl<T> Mul<T> for Matrix1<T> where
    T: Copy + Mul<Output = T>
    {
    type Output = Matrix1<T>;
    fn mul(self, rhs: T) -> Matrix1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::multiply_scalar(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::multiply_scalar(self, rhs)
        } else {
            scalar::mat1::multiply_scalar(self, rhs)
        }
    }
}

impl<T> Mul<Vector1<T>> for Matrix1<T> where
    T: Copy + Mul<Output = T>
    {
    type Output = Vector1<T>;
    fn mul(self, rhs: Vector1<T>) -> Vector1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::mat1::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::mat1::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::mat1::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::mat1::multiply_vec(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::mat1::multiply_vec(self, rhs)
        } else {
            scalar::mat1::multiply_vec(self, rhs)
        }
    }
}
