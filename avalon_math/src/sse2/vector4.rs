use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::scalar::vector4 as scalar;
use crate::simd::{ self, SimdType };
use crate::Vector4;
use crate::scalar::HasSqrt;
use std::arch::x86_64 as simd_inst;

pub fn add<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T: SimdType + Copy + Add<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            unsafe {
                let simd_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let simd_rhs = simd_inst::_mm_load_ps(&rhs_pack.0);
                let simd_result = simd_inst::_mm_add_ps(simd_lhs, simd_rhs);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            }
        },
        _ => {
            scalar::add(lhs, rhs)
        }
    }
}

pub fn sub<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T : Copy + Sub<Output = U> {
    scalar::sub(lhs, rhs)
}

pub fn mul<U, T>(lhs: Vector4<T>, rhs: T) -> Vector4<U> where
    T : Copy + Mul<Output = U> {
    scalar::mul(lhs, rhs)
}

pub fn div_with_numerator<U, T>(lhs: T, rhs: Vector4<T>) -> Vector4<U> where
    T : Copy + Div<Output = U> {
    scalar::div_with_numerator(lhs, rhs)
}

pub fn div_with_denominator<U, T>(lhs: Vector4<T>, rhs: T) -> Vector4<U> where
    T : Copy + Div<Output = U> {
    scalar::div_with_denominator(lhs, rhs)
}

pub fn component_mul<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T : Copy + Mul<Output = U> {
    scalar::component_mul(lhs, rhs)
}

pub fn dot<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    scalar::dot(lhs, rhs)
}

pub fn magnitude<U, T>(vec: Vector4<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> + HasSqrt {
    scalar::magnitude(vec)
}

pub fn magnitude_sqr<U, T>(vec: Vector4<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    scalar::magnitude_sqr(vec)
}

pub fn negate<U, T>(vec: Vector4<T>) -> Vector4<U> where
    T: Neg<Output = U> {
    scalar::negate(vec)
}

#[cfg(test)]
mod test_f32 {
    use approx::assert_abs_diff_eq;
    use crate::Vector4;
    use crate::sse2::vector4;

    #[test]
    fn addition() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let b = Vector4 {
            x: 3.0_f32,
            y: 9.0_f32,
            z: 0.8_f32,
            w: 7.0_f32
        };
        let c = vector4::add(a, b);
        assert_abs_diff_eq!(c.x, 8.0_f32);
        assert_abs_diff_eq!(c.y, 21.0_f32);
        assert_abs_diff_eq!(c.z, -2.7_f32);
        assert_abs_diff_eq!(c.w, 9.0_f32);
    }

    #[test]
    fn sub() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let b = Vector4 {
            x: 3.0_f32,
            y: 9.0_f32,
            z: 0.8_f32,
            w: 7.0_f32
        };
        let c = vector4::sub(b, a);
        assert_abs_diff_eq!(c.x, -2.0_f32);
        assert_abs_diff_eq!(c.y, -3.0_f32);
        assert_abs_diff_eq!(c.z, 4.3_f32);
        assert_abs_diff_eq!(c.w, 5.0_f32);
    }

    #[test]
    fn mul() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: 0.8_f32,
            w: 2.0_f32
        };
        let b = 3.0_f32;
        let c = vector4::mul(a, b);
        assert_abs_diff_eq!(c.x, 15.0_f32);
        assert_abs_diff_eq!(c.y, 36.0_f32);
        assert_abs_diff_eq!(c.z, 2.4000000000000004_f32);
        assert_abs_diff_eq!(c.w, 6.0_f32);
    }

    #[test]
    fn div_with_numerator() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let c = vector4::div_with_numerator(10.0_f32, a);
        assert_abs_diff_eq!(c.x, 2.0_f32);
        assert_abs_diff_eq!(c.y, 0.8333333333333334_f32);
        assert_abs_diff_eq!(c.z, -2.857142857142857_f32);
        assert_abs_diff_eq!(c.w, 5.0_f32);
    }

    #[test]
    fn div_with_denominator() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let c = vector4::div_with_denominator(a, 10.0_f32);
        assert_abs_diff_eq!(c.x, 0.5_f32);
        assert_abs_diff_eq!(c.y, 1.2_f32);
        assert_abs_diff_eq!(c.z, -0.35_f32);
        assert_abs_diff_eq!(c.w, 0.2_f32);
    }

    #[test]
    fn component_mul() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let b = Vector4 {
            x: 6.7_f32,
            y: 8.7_f32,
            z: 5.0_f32,
            w: 3.4_f32,
        };
        let c = vector4::component_mul(a, b);
        assert_abs_diff_eq!(c.x, 33.5_f32);
        assert_abs_diff_eq!(c.y, 104.399994_f32);
        assert_abs_diff_eq!(c.z, -17.5_f32);
        assert_abs_diff_eq!(c.w, 6.8_f32);
    }

    #[test]
    fn dot() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let b = Vector4 {
            x: 6.7_f32,
            y: 8.7_f32,
            z: 5.0_f32,
            w: 3.4_f32
        };
        let c = vector4::dot(a, b);
        assert_abs_diff_eq!(c, 127.19999999999997_f32);
    }

    #[test]
    fn magnitude() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let c = vector4::magnitude(a);
        assert_abs_diff_eq!(c, 13.6106575888162_f32);
    }

    #[test]
    fn magnitude_sqr() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let c = vector4::magnitude_sqr(a);
        assert_abs_diff_eq!(c, 185.25_f32);
    }

    #[test]
    fn neg() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let c = vector4::negate(a);
        assert_abs_diff_eq!(c.x, -5.0_f32);
        assert_abs_diff_eq!(c.y, -12.0_f32);
        assert_abs_diff_eq!(c.z, 3.5_f32);
        assert_abs_diff_eq!(c.w, -2.0_f32);
    }
}
