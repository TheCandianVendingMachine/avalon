use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::scalar::vector4 as scalar;
use crate::simd::{ self, SimdType };
use crate::Vector4;
use crate::scalar::HasSqrt;
use std::arch::x86_64 as simd_inst;

#[target_feature(enable = "sse2")]
pub unsafe fn add<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T: SimdType + Copy + Add<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            unsafe {
                let simd_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let simd_rhs: simd_inst::__m128 = std::mem::transmute(rhs_pack);
                let simd_result = simd_inst::_mm_add_ps(simd_lhs, simd_rhs);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            }
        },
        _ => {
            scalar::add(lhs, rhs)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn sub<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T: SimdType + Copy + Sub<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            unsafe {
                let simd_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let simd_rhs: simd_inst::__m128 = std::mem::transmute(rhs_pack);
                let simd_result = simd_inst::_mm_sub_ps(simd_lhs, simd_rhs);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            }
        },
        _ => {
            scalar::sub(lhs, rhs)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn mul<U, T>(lhs: Vector4<T>, rhs: T) -> Vector4<U> where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs: f32 = simd::Type::convert_variable(rhs);

            unsafe {
                let simd_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let simd_rhs: simd_inst::__m128 = simd_inst::_mm_load1_ps(&rhs);
                let simd_result = simd_inst::_mm_mul_ps(simd_lhs, simd_rhs);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            }
        },
        _ => {
            scalar::mul(lhs, rhs)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn div_with_numerator<U, T>(lhs: T, rhs: Vector4<T>) -> Vector4<U> where
    T: SimdType + Copy + Div<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs: f32 = simd::Type::convert_variable(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            unsafe {
                let simd_lhs = simd_inst::_mm_load1_ps(&lhs);
                let simd_rhs: simd_inst::__m128 = std::mem::transmute(rhs_pack);
                let simd_result = simd_inst::_mm_div_ps(simd_lhs, simd_rhs);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            }
        },
        _ => {
            scalar::div_with_numerator(lhs, rhs)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn div_with_denominator<U, T>(lhs: Vector4<T>, rhs: T) -> Vector4<U> where
    T: SimdType + Copy + Div<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs: f32 = simd::Type::convert_variable(rhs);

            unsafe {
                let simd_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let simd_rhs = simd_inst::_mm_load1_ps(&rhs);
                let simd_result = simd_inst::_mm_div_ps(simd_lhs, simd_rhs);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            }
        },
        _ => {
            scalar::div_with_denominator(lhs, rhs)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn component_mul<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            unsafe {
                let simd_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let simd_rhs: simd_inst::__m128 = std::mem::transmute(rhs_pack);
                let simd_result = simd_inst::_mm_mul_ps(simd_lhs, simd_rhs);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            }
        },
        _ => {
            scalar::component_mul(lhs, rhs)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn dot<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> U where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy + Add<Output = U> {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs_pack = simd::f32x4::from(rhs);

            let result_vec: simd::f32x4 = unsafe {
                let prod_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let prod_rhs: simd_inst::__m128 = std::mem::transmute(rhs_pack);
                let prod_result = simd_inst::_mm_mul_ps(prod_lhs, prod_rhs);

                let shift_1 = simd_inst::_mm_shuffle_ps(prod_result, prod_result, 0b01_11_00_01);
                let add_1 = simd_inst::_mm_add_ps(prod_result, shift_1);
                let shift_2 = simd_inst::_mm_shuffle_ps(add_1, add_1, 0b00_00_00_10);
                let simd_result = simd_inst::_mm_add_ps(shift_2, add_1);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            };
            simd::Type::convert_variable(result_vec.0)
        },
        _ => {
            scalar::dot(lhs, rhs)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn magnitude<U, T>(vec: Vector4<T>) -> U where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy + Add<Output = U> + HasSqrt {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(vec);
            let result_vec: simd::f32x4 = unsafe {
                let prod_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let prod_result = simd_inst::_mm_mul_ps(prod_lhs, prod_lhs);

                let shift_1 = simd_inst::_mm_shuffle_ps(prod_result, prod_result, 0b01_11_00_01);
                let add_1 = simd_inst::_mm_add_ps(prod_result, shift_1);
                let shift_2 = simd_inst::_mm_shuffle_ps(add_1, add_1, 0b00_00_00_10);
                let magnitude_sqr = simd_inst::_mm_add_ps(shift_2, add_1);

                let simd_result = simd_inst::_mm_sqrt_ss(magnitude_sqr);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            };
            simd::Type::convert_variable(result_vec.0)
        },
        _ => {
            scalar::magnitude(vec)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn magnitude_sqr<U, T>(vec: Vector4<T>) -> U where
    T: SimdType + Copy + Mul<Output = U>,
    U: SimdType + Copy + Add<Output = U> {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(vec);
            let result_vec: simd::f32x4 = unsafe {
                let prod_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let prod_result = simd_inst::_mm_mul_ps(prod_lhs, prod_lhs);

                let shift_1 = simd_inst::_mm_shuffle_ps(prod_result, prod_result, 0b01_11_00_01);
                let add_1 = simd_inst::_mm_add_ps(prod_result, shift_1);
                let shift_2 = simd_inst::_mm_shuffle_ps(add_1, add_1, 0b00_00_00_10);
                let simd_result = simd_inst::_mm_add_ps(shift_2, add_1);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            };
            simd::Type::convert_variable(result_vec.0)
        },
        _ => {
            scalar::magnitude_sqr(vec)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn negate<U, T>(vec: Vector4<T>) -> Vector4<U> where
    T: SimdType + Copy + Neg<Output = U>,
    U: SimdType + Copy {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(vec);

            unsafe {
                static NEGATIVE: f32 = -0.0;
                let simd_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let simd_rhs = simd_inst::_mm_load1_ps(&NEGATIVE);
                let simd_result = simd_inst::_mm_xor_ps(simd_lhs, simd_rhs);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            }
        },
        _ => {
            scalar::negate(vec)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn normalize<T>(vec: Vector4<T>) -> Vector4<T> where
    T: SimdType + Copy + HasSqrt + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(vec);
            unsafe {
                let simd_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let prod_result = simd_inst::_mm_mul_ps(simd_lhs, simd_lhs);

                let shift_1 = simd_inst::_mm_shuffle_ps(prod_result, prod_result, 0b01_11_00_01);
                let add_1 = simd_inst::_mm_add_ps(prod_result, shift_1);
                let shift_2 = simd_inst::_mm_shuffle_ps(add_1, add_1, 0b00_00_00_10);
                let magnitude_sqr = simd_inst::_mm_add_ps(shift_2, add_1);
                let sqrt = simd_inst::_mm_sqrt_ps(magnitude_sqr);
                let sqrt_across = simd_inst::_mm_shuffle_ps(sqrt, sqrt, 0b00_00_00_00);

                let simd_result = simd_inst::_mm_div_ps(simd_lhs, sqrt_across);

                let result: simd::f32x4 = std::mem::transmute(simd_result);
                result.into()
            }
        },
        _ => {
            scalar::normalize(vec)
        }
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn project<T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<T> where
    T: std::fmt::Debug + SimdType + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
    match T::to_type() {
        simd::Type::f32 => {
            let lhs_pack = simd::f32x4::from(lhs);
            let rhs_pack = simd::f32x4::from(rhs);
            unsafe {
                let dot_num_lhs: simd_inst::__m128 = std::mem::transmute(lhs_pack);
                let dot_num_rhs: simd_inst::__m128 = std::mem::transmute(rhs_pack);
                let dot_denom_v: simd_inst::__m128 = std::mem::transmute(rhs_pack);
                let num_result = simd_inst::_mm_mul_ps(dot_num_lhs, dot_num_rhs);
                let denom_result = simd_inst::_mm_mul_ps(dot_denom_v, dot_denom_v);

                let num_shift = simd_inst::_mm_shuffle_ps(num_result, num_result, 0b10_11_00_01);
                let num_sum = simd_inst::_mm_add_ps(num_result, num_shift);

                let denom_shift = simd_inst::_mm_shuffle_ps(denom_result, denom_result, 0b10_11_00_01);
                let denom_sum = simd_inst::_mm_add_ps(denom_result, denom_shift);

                let packed_sum_top = simd_inst::_mm_shuffle_ps(num_sum, denom_sum, 0b11_00_11_00);
                let packed_sum_bottom = simd_inst::_mm_shuffle_ps(num_sum, denom_sum, 0b00_11_00_11);
                let dot_results = simd_inst::_mm_add_ps(packed_sum_bottom, packed_sum_top);

                let dot_results_denom = simd_inst::_mm_shuffle_ps(dot_results, dot_results, 0b00_00_00_11);

                let proj_numerator = simd_inst::_mm_shuffle_ps(dot_results, dot_results, 0);
                let proj_numerator = simd_inst::_mm_mul_ps(dot_num_rhs, proj_numerator);
                let proj_denominator = simd_inst::_mm_shuffle_ps(dot_results_denom, dot_results_denom, 0);
                let projection = simd_inst::_mm_div_ps(proj_numerator, proj_denominator);

                let result: simd::f32x4 = std::mem::transmute(projection);
                result.into()
            }
        },
        _ => {
            scalar::project(lhs, rhs)
        }
    }
}

#[cfg(test)]
mod test_u8 {
    crate::vector4_uint_tests!(sse2, u8);
}

#[cfg(test)]
mod test_u16 {
    crate::vector4_uint_tests!(sse2, u16);
}

#[cfg(test)]
mod test_u32 {
    crate::vector4_uint_tests!(sse2, u32);
}

#[cfg(test)]
mod test_u64 {
    crate::vector4_uint_tests!(sse2, u64);
}

#[cfg(test)]
mod test_i8 {
    crate::vector4_sint_tests!(sse2, i8);
}

#[cfg(test)]
mod test_i16 {
    crate::vector4_sint_tests!(sse2, i16);
}

#[cfg(test)]
mod test_i32 {
    crate::vector4_sint_tests!(sse2, i32);
}

#[cfg(test)]
mod test_i64 {
    crate::vector4_sint_tests!(sse2, i64);
}

#[cfg(test)]
mod test_f32 {
    crate::vector4_float_tests!(sse2, f32);
}

#[cfg(test)]
mod test_f64 {
    crate::vector4_float_tests!(sse2, f64);
}
