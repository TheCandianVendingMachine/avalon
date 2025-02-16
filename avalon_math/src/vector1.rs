use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::{ scalar, Vector1 };

impl<T> Vector1<T> where
    T: Copy + Add<Output = T> + Mul<Output = T> {
    pub fn dot(self, rhs: Vector1<T>) -> T {
        scalar::vector1::dot(self, rhs)
    }

    pub fn magnitude_sqr(self) -> T {
        scalar::vector1::magnitude_sqr(self)
    }
}

impl<T> Vector1<T> where
    T: Copy {
    pub fn magnitude(self) -> T {
        scalar::vector1::magnitude(self)
    }
}

impl<T> Add for Vector1<T> where
    T: Copy + Add<Output = T> {
    type Output = Vector1<T>;
    fn add(self, rhs: Vector1<T>) -> Vector1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector1::add(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector1::add(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector1::add(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector1::add(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector1::add(self, rhs)
        } else {
            scalar::vector1::add(self, rhs)
        }
    }
}

impl<T> Sub for Vector1<T> where
    T: Copy + Sub<Output = T> {
    type Output = Vector1<T>;
    fn sub(self, rhs: Vector1<T>) -> Vector1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector1::sub(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector1::sub(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector1::sub(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector1::sub(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector1::sub(self, rhs)
        } else {
            scalar::vector1::sub(self, rhs)
        }
    }
}

impl<T> Mul for Vector1<T> where
    T: Copy + Mul<Output = T> {
    type Output = Vector1<T>;
    fn mul(self, rhs: Vector1<T>) -> Vector1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector1::component_mul(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector1::component_mul(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector1::component_mul(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector1::component_mul(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector1::component_mul(self, rhs)
        } else {
            scalar::vector1::component_mul(self, rhs)
        }
    }
}

impl<T> Mul<T> for Vector1<T> where
    T: Copy + Mul<Output = T> {
    type Output = Vector1<T>;
    fn mul(self, rhs: T) -> Vector1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector1::mul(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector1::mul(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector1::mul(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector1::mul(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector1::mul(self, rhs)
        } else {
            scalar::vector1::mul(self, rhs)
        }
    }
}

impl<T> Div<T> for Vector1<T> where
    T: Copy + Div<Output = T> {
    type Output = Vector1<T>;
    fn div(self, rhs: T) -> Vector1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector1::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector1::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector1::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector1::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector1::div_with_denominator(self, rhs)
        } else {
            scalar::vector1::div_with_denominator(self, rhs)
        }
    }
}

impl<T> Neg for Vector1<T> where
    T: Copy + Neg<Output = T> {
    type Output = Vector1<T>;
    fn neg(self) -> Vector1<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector1::negate(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector1::negate(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector1::negate(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector1::negate(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector1::negate(self)
        } else {
            scalar::vector1::negate(self)
        }
    }
}
