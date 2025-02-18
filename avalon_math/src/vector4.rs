use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::{ sse2, scalar, Vector4 };

impl<T> Vector4<T> where
    T: Copy + Add<Output = T> + Mul<Output = T> {
    pub fn dot(self, rhs: Vector4<T>) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector4::dot(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector4::dot(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector4::dot(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector4::dot(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector4::dot(self, rhs)
        } else {
            scalar::vector4::dot(self, rhs)
        }
    }

    pub fn magnitude_sqr(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector4::magnitude_sqr(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector4::magnitude_sqr(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector4::magnitude_sqr(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector4::magnitude_sqr(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector4::magnitude_sqr(self)
        } else {
            scalar::vector4::magnitude_sqr(self)
        }
    }
}

impl<T> Vector4<T> where
    T: Copy + scalar::HasSqrt + Add<Output = T> + Mul<Output = T> {
    pub fn magnitude(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector4::magnitude(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector4::magnitude(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector4::magnitude(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector4::magnitude(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector4::magnitude(self)
        } else {
            scalar::vector4::magnitude(self)
        }
    }
}

impl<T> Add for Vector4<T> where
    T: Copy + Add<Output = T> {
    type Output = Vector4<T>;
    fn add(self, rhs: Vector4<T>) -> Vector4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector4::add(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector4::add(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector4::add(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector4::add(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector4::add(self, rhs)
        } else {
            scalar::vector4::add(self, rhs)
        }
    }
}

impl<T> Sub for Vector4<T> where
    T: Copy + Sub<Output = T> {
    type Output = Vector4<T>;
    fn sub(self, rhs: Vector4<T>) -> Vector4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector4::sub(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector4::sub(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector4::sub(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector4::sub(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector4::sub(self, rhs)
        } else {
            scalar::vector4::sub(self, rhs)
        }
    }
}

impl<T> Mul for Vector4<T> where
    T: Copy + Mul<Output = T> {
    type Output = Vector4<T>;
    fn mul(self, rhs: Vector4<T>) -> Vector4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector4::component_mul(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector4::component_mul(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector4::component_mul(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector4::component_mul(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector4::component_mul(self, rhs)
        } else {
            scalar::vector4::component_mul(self, rhs)
        }
    }
}

impl<T> Mul<T> for Vector4<T> where
    T: Copy + Mul<Output = T> {
    type Output = Vector4<T>;
    fn mul(self, rhs: T) -> Vector4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector4::mul(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector4::mul(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector4::mul(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector4::mul(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector4::mul(self, rhs)
        } else {
            scalar::vector4::mul(self, rhs)
        }
    }
}

impl<T> Div<T> for Vector4<T> where
    T: Copy + Div<Output = T> {
    type Output = Vector4<T>;
    fn div(self, rhs: T) -> Vector4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector4::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector4::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector4::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector4::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector4::div_with_denominator(self, rhs)
        } else {
            scalar::vector4::div_with_denominator(self, rhs)
        }
    }
}

impl<T> Neg for Vector4<T> where
    T: Copy + Neg<Output = T> {
    type Output = Vector4<T>;
    fn neg(self) -> Vector4<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector4::negate(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector4::negate(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector4::negate(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector4::negate(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector4::negate(self)
        } else {
            scalar::vector4::negate(self)
        }
    }
}
