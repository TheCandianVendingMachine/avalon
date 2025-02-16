use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::{ scalar, Vector2 };

impl<T> Vector2<T> where
    T: Copy + Add<Output = T> + Mul<Output = T> {
    pub fn dot(self, rhs: Vector2<T>) -> T {
        scalar::vector2::dot(self, rhs)
    }

    pub fn magnitude_sqr(self) -> T {
        scalar::vector2::magnitude_sqr(self)
    }
}

impl<T> Vector2<T> where
    T: Copy + scalar::HasSqrt + Add<Output = T> + Mul<Output = T> {
    pub fn magnitude(self) -> T {
        scalar::vector2::magnitude(self)
    }
}

impl<T> Add for Vector2<T> where
    T: Copy + Add<Output = T> {
    type Output = Vector2<T>;
    fn add(self, rhs: Vector2<T>) -> Vector2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("sse4.1") {
            todo!()
        } else if is_x86_feature_detected!("sse3") {
            todo!()
        } else if is_x86_feature_detected!("sse2") {
            todo!()
        } else {
            scalar::vector2::add(self, rhs)
        }
    }
}

impl<T> Sub for Vector2<T> where
    T: Copy + Sub<Output = T> {
    type Output = Vector2<T>;
    fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("sse4.1") {
            todo!()
        } else if is_x86_feature_detected!("sse3") {
            todo!()
        } else if is_x86_feature_detected!("sse2") {
            todo!()
        } else {
            scalar::vector2::sub(self, rhs)
        }
    }
}

impl<T> Mul for Vector2<T> where
    T: Copy + Mul<Output = T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: Vector2<T>) -> Vector2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("sse4.1") {
            todo!()
        } else if is_x86_feature_detected!("sse3") {
            todo!()
        } else if is_x86_feature_detected!("sse2") {
            todo!()
        } else {
            scalar::vector2::component_mul(self, rhs)
        }
    }
}

impl<T> Mul<T> for Vector2<T> where
    T: Copy + Mul<Output = T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: T) -> Vector2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("sse4.1") {
            todo!()
        } else if is_x86_feature_detected!("sse3") {
            todo!()
        } else if is_x86_feature_detected!("sse2") {
            todo!()
        } else {
            scalar::vector2::mul(self, rhs)
        }
    }
}

impl<T> Div<T> for Vector2<T> where
    T: Copy + Div<Output = T> {
    type Output = Vector2<T>;
    fn div(self, rhs: T) -> Vector2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("sse4.1") {
            todo!()
        } else if is_x86_feature_detected!("sse3") {
            todo!()
        } else if is_x86_feature_detected!("sse2") {
            todo!()
        } else {
            scalar::vector2::div_with_denominator(self, rhs)
        }
    }
}

impl<T> Neg for Vector2<T> where
    T: Copy + Neg<Output = T> {
    type Output = Vector2<T>;
    fn neg(self) -> Vector2<T> {
        if is_x86_feature_detected!("avx2") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("avx") {
            // FMA maybe available
            todo!()
        } else if is_x86_feature_detected!("sse4.1") {
            todo!()
        } else if is_x86_feature_detected!("sse3") {
            todo!()
        } else if is_x86_feature_detected!("sse2") {
            todo!()
        } else {
            scalar::vector2::negate(self)
        }
    }
}
