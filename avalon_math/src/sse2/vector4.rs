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
    T: SimdType + Copy + Sub<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            unsafe {
                let simd_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let simd_rhs = simd_inst::_mm_load_ps(&rhs_pack.0);
                let simd_result = simd_inst::_mm_sub_ps(simd_lhs, simd_rhs);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            }
        },
        _ => {
            scalar::sub(lhs, rhs)
        }
    }
}

pub fn mul<U, T>(lhs: Vector4<T>, rhs: T) -> Vector4<U> where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs: f32 = simd::Type::convert_variable(rhs);

            unsafe {
                let simd_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let simd_rhs = simd_inst::_mm_load1_ps(&rhs);
                let simd_result = simd_inst::_mm_mul_ps(simd_lhs, simd_rhs);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            }
        },
        _ => {
            scalar::mul(lhs, rhs)
        }
    }
}

pub fn div_with_numerator<U, T>(lhs: T, rhs: Vector4<T>) -> Vector4<U> where
    T: SimdType + Copy + Div<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs: f32 = simd::Type::convert_variable(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            unsafe {
                let simd_lhs = simd_inst::_mm_load1_ps(&lhs);
                let simd_rhs = simd_inst::_mm_load_ps(&rhs_pack.0);
                let simd_result = simd_inst::_mm_div_ps(simd_lhs, simd_rhs);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            }
        },
        _ => {
            scalar::div_with_numerator(lhs, rhs)
        }
    }
}

pub fn div_with_denominator<U, T>(lhs: Vector4<T>, rhs: T) -> Vector4<U> where
    T: SimdType + Copy + Div<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs: f32 = simd::Type::convert_variable(rhs);

            unsafe {
                let simd_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let simd_rhs = simd_inst::_mm_load1_ps(&rhs);
                let simd_result = simd_inst::_mm_div_ps(simd_lhs, simd_rhs);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            }
        },
        _ => {
            scalar::div_with_denominator(lhs, rhs)
        }
    }
}

pub fn component_mul<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            unsafe {
                let simd_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let simd_rhs = simd_inst::_mm_load_ps(&rhs_pack.0);
                let simd_result = simd_inst::_mm_mul_ps(simd_lhs, simd_rhs);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            }
        },
        _ => {
            scalar::component_mul(lhs, rhs)
        }
    }
}

pub fn dot<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> U where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy + Add<Output = U> {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            let result_vec: simd::f32x4 = unsafe {
                let prod_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let prod_rhs = simd_inst::_mm_load_ps(&rhs_pack.0);
                let prod_result = simd_inst::_mm_mul_ps(prod_lhs, prod_rhs);

                let shift_1 = simd_inst::_mm_shuffle_ps(prod_result, prod_result, 0b01_11_00_01);
                let add_1 = simd_inst::_mm_add_ps(prod_result, shift_1);
                let shift_2 = simd_inst::_mm_shuffle_ps(add_1, add_1, 0b00_00_00_10);
                let simd_result = simd_inst::_mm_add_ps(shift_2, add_1);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            };
            simd::Type::convert_variable(result_vec.0)
        },
        _ => {
            scalar::dot(lhs, rhs)
        }
    }
}

pub fn magnitude<U, T>(vec: Vector4<T>) -> U where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy + Add<Output = U> + HasSqrt {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(vec);
            let result_vec: simd::f32x4 = unsafe {
                let prod_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let prod_result = simd_inst::_mm_mul_ps(prod_lhs, prod_lhs);

                let shift_1 = simd_inst::_mm_shuffle_ps(prod_result, prod_result, 0b01_11_00_01);
                let add_1 = simd_inst::_mm_add_ps(prod_result, shift_1);
                let shift_2 = simd_inst::_mm_shuffle_ps(add_1, add_1, 0b00_00_00_10);
                let magnitude_sqr = simd_inst::_mm_add_ps(shift_2, add_1);

                let simd_result = simd_inst::_mm_sqrt_ss(magnitude_sqr);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            };
            simd::Type::convert_variable(result_vec.0)
        },
        _ => {
            scalar::magnitude(vec)
        }
    }
}

pub fn magnitude_sqr<U, T>(vec: Vector4<T>) -> U where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy + Add<Output = U> {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(vec);
            let result_vec: simd::f32x4 = unsafe {
                let prod_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let prod_result = simd_inst::_mm_mul_ps(prod_lhs, prod_lhs);

                let shift_1 = simd_inst::_mm_shuffle_ps(prod_result, prod_result, 0b01_11_00_01);
                let add_1 = simd_inst::_mm_add_ps(prod_result, shift_1);
                let shift_2 = simd_inst::_mm_shuffle_ps(add_1, add_1, 0b00_00_00_10);
                let simd_result = simd_inst::_mm_add_ps(shift_2, add_1);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            };
            simd::Type::convert_variable(result_vec.0)
        },
        _ => {
            scalar::magnitude_sqr(vec)
        }
    }
}

pub fn negate<U, T>(vec: Vector4<T>) -> Vector4<U> where
    T: SimdType + Copy + Neg<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(vec);

            unsafe {
                static NEGATIVE: f32 = -0.0;
                let simd_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let simd_rhs = simd_inst::_mm_load1_ps(&NEGATIVE);
                let simd_result = simd_inst::_mm_xor_ps(simd_lhs, simd_rhs);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            }
        },
        _ => {
            scalar::negate(vec)
        }
    }
}

pub fn normalize<T>(vec: Vector4<T>) -> Vector4<T> where
    T: SimdType + Copy + HasSqrt + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(vec);
            unsafe {
                let simd_lhs = simd_inst::_mm_load_ps(&lhs_pack.0);
                let prod_result = simd_inst::_mm_mul_ps(simd_lhs, simd_lhs);

                let shift_1 = simd_inst::_mm_shuffle_ps(prod_result, prod_result, 0b01_11_00_01);
                let add_1 = simd_inst::_mm_add_ps(prod_result, shift_1);
                let shift_2 = simd_inst::_mm_shuffle_ps(add_1, add_1, 0b00_00_00_10);
                let magnitude_sqr = simd_inst::_mm_add_ps(shift_2, add_1);
                let sqrt = simd_inst::_mm_sqrt_ps(magnitude_sqr);
                let sqrt_across = simd_inst::_mm_shuffle_ps(sqrt, sqrt, 0b00_00_00_00);

                let simd_result = simd_inst::_mm_div_ps(simd_lhs, sqrt_across);

                let mut result = simd::f32x4(0.0, 0.0, 0.0, 0.0);
                simd_inst::_mm_store_ps(&mut result.0, simd_result);

                result.into()
            }
        },
        _ => {
            scalar::normalize(vec)
        }
    }
}

#[cfg(test)]
mod test_i32 {
    use crate::Vector4;
    use crate::sse2::vector4;

    #[test]
    fn addition() {
        let a = Vector4 {
            x: 5,
            y: 12,
            z: -3,
            w: 2
        };
        let b = Vector4 {
            x: 3,
            y: 9,
            z: 0,
            w: 7
        };
        let c = vector4::add(a, b);
        assert_eq!(c.x, 8);
        assert_eq!(c.y, 21);
        assert_eq!(c.z, -3);
        assert_eq!(c.w, 9);
    }

    #[test]
    fn sub() {
        let a = Vector4 {
            x: 5,
            y: 12,
            z: -3,
            w: 2
        };
        let b = Vector4 {
            x: 3,
            y: 9,
            z: 0,
            w: 7
        };
        let c = vector4::sub(b, a);
        assert_eq!(c.x, -2);
        assert_eq!(c.y, -3);
        assert_eq!(c.z, 3);
        assert_eq!(c.w, 5);
    }

    #[test]
    fn mul() {
        let a = Vector4 {
            x: 5,
            y: 12,
            z: 0,
            w: 2
        };
        let b = 3;
        let c = vector4::mul(a, b);
        assert_eq!(c.x, 15);
        assert_eq!(c.y, 36);
        assert_eq!(c.z, 0);
        assert_eq!(c.w, 6);
    }

    #[test]
    fn div_with_numerator() {
        let a = Vector4 {
            x: 5,
            y: 12,
            z: 3,
            w: -2
        };
        let c = vector4::div_with_numerator(10, a);
        assert_eq!(c.x, 2);
        assert_eq!(c.y, 0);
        assert_eq!(c.z, 3);
        assert_eq!(c.w, -5);
    }

    #[test]
    fn div_with_denominator() {
        let a = Vector4 {
            x: 5,
            y: 12,
            z: 3,
            w: -2
        };
        let c = vector4::div_with_denominator(a, 10);
        assert_eq!(c.x, 0);
        assert_eq!(c.y, 1);
        assert_eq!(c.z, 0);
        assert_eq!(c.w, 0);
    }

    #[test]
    fn component_mul() {
        let a = Vector4 {
            x: 5,
            y: 12,
            z: 3,
            w: -2
        };
        let b = Vector4 {
            x: 3,
            y: 9,
            z: 0,
            w: 7
        };
        let c = vector4::component_mul(a, b);
        assert_eq!(c.x, 15);
        assert_eq!(c.y, 108);
        assert_eq!(c.z, 0);
        assert_eq!(c.w, -14);
    }

    #[test]
    fn dot() {
        let a = Vector4 {
            x: 5,
            y: 12,
            z: 3,
            w: -2
        };
        let b = Vector4 {
            x: 3,
            y: 9,
            z: 0,
            w: 7
        };
        let c = vector4::dot(a, b);
        assert_eq!(c, 109)
    }

    #[test]
    fn magnitude_sqr() {
        let a = Vector4 {
            x: 5,
            y: 12,
            z: 3,
            w: -2
        };
        let c = vector4::magnitude_sqr(a);
        assert_eq!(c, 182);
    }

    #[test]
    fn neg() {
        let a = Vector4 {
            x: 5,
            y: 12,
            z: 3,
            w: -2
        };
        let c = vector4::negate(a);
        assert_eq!(c.x, -5);
        assert_eq!(c.y, -12);
        assert_eq!(c.z, -3);
        assert_eq!(c.w, 2);
    }
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

    #[test]
    fn normalize() {
        let a = Vector4 {
            x: 5.0_f32,
            y: 12.0_f32,
            z: -3.5_f32,
            w: 2.0_f32
        };
        let c = vector4::normalize(a);
        assert_abs_diff_eq!(vector4::magnitude_sqr(c), 1.0);
        assert_abs_diff_eq!(c.x, 0.3673591791853225);
        assert_abs_diff_eq!(c.y, 0.8816620300447741);
        assert_abs_diff_eq!(c.z, -0.25715142542972574);
        assert_abs_diff_eq!(c.w, 0.146943671674129);
    }
}
