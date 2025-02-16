use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::{ scalar, Vector3 };

impl<T> Vector3<T> where
    T: Copy + Add<Output = T> + Mul<Output = T> {
    pub fn dot(self, rhs: Vector3<T>) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector3::dot(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector3::dot(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector3::dot(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector3::dot(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector3::dot(self, rhs)
        } else {
            scalar::vector3::dot(self, rhs)
        }
    }

    pub fn magnitude_sqr(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector3::magnitude_sqr(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector3::magnitude_sqr(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector3::magnitude_sqr(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector3::magnitude_sqr(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector3::magnitude_sqr(self)
        } else {
            scalar::vector3::magnitude_sqr(self)
        }
    }
}

impl<T> Vector3<T> where
    T: Copy + scalar::HasSqrt + Add<Output = T> + Mul<Output = T> {
    pub fn magnitude(self) -> T {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector3::magnitude(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector3::magnitude(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector3::magnitude(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector3::magnitude(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector3::magnitude(self)
        } else {
            scalar::vector3::magnitude(self)
        }
    }
}

impl<T> Add for Vector3<T> where
    T: Copy + Add<Output = T> {
    type Output = Vector3<T>;
    fn add(self, rhs: Vector3<T>) -> Vector3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector3::add(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector3::add(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector3::add(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector3::add(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector3::add(self, rhs)
        } else {
            scalar::vector3::add(self, rhs)
        }
    }
}

impl<T> Sub for Vector3<T> where
    T: Copy + Sub<Output = T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector3::sub(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector3::sub(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector3::sub(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector3::sub(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector3::sub(self, rhs)
        } else {
            scalar::vector3::sub(self, rhs)
        }
    }
}

impl<T> Mul for Vector3<T> where
    T: Copy + Mul<Output = T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: Vector3<T>) -> Vector3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector3::component_mul(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector3::component_mul(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector3::component_mul(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector3::component_mul(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector3::component_mul(self, rhs)
        } else {
            scalar::vector3::component_mul(self, rhs)
        }
    }
}

impl<T> Mul<T> for Vector3<T> where
    T: Copy + Mul<Output = T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: T) -> Vector3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector3::mul(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector3::mul(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector3::mul(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector3::mul(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector3::mul(self, rhs)
        } else {
            scalar::vector3::mul(self, rhs)
        }
    }
}

impl<T> Div<T> for Vector3<T> where
    T: Copy + Div<Output = T> {
    type Output = Vector3<T>;
    fn div(self, rhs: T) -> Vector3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector3::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector3::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector3::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector3::div_with_denominator(self, rhs)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector3::div_with_denominator(self, rhs)
        } else {
            scalar::vector3::div_with_denominator(self, rhs)
        }
    }
}

impl<T> Neg for Vector3<T> where
    T: Copy + Neg<Output = T> {
    type Output = Vector3<T>;
    fn neg(self) -> Vector3<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            scalar::vector3::negate(self)
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            scalar::vector3::negate(self)
        } else if is_x86_feature_detected!("sse4.1") {
            scalar::vector3::negate(self)
        } else if is_x86_feature_detected!("sse3") {
            scalar::vector3::negate(self)
        } else if is_x86_feature_detected!("sse2") {
            scalar::vector3::negate(self)
        } else {
            scalar::vector3::negate(self)
        }
    }
}
