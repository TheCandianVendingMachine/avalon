#[cfg(test)]
pub mod test {
    #[macro_export]
    macro_rules! vector4_sint_tests {
        ( $feature:ident, $type:ty ) => {
            use approx::assert_abs_diff_eq;
            use crate::Vector4;
            use crate::$feature::vector4;

            const V0: Vector4<$type> = crate::Vector!(7, -4, 1, 2);
            const V1: Vector4<$type> = crate::Vector!(0, 3, 10, -4);

            #[test]
            fn addition() {
                let c = vector4::add(V0, V1);
                assert_abs_diff_eq!(c.x, 7);
                assert_abs_diff_eq!(c.y, -1);
                assert_abs_diff_eq!(c.z, 11);
                assert_abs_diff_eq!(c.w, -2);
            }

            #[test]
            fn sub() {
                let c = vector4::sub(V0, V1);
                assert_abs_diff_eq!(c.x, 7);
                assert_abs_diff_eq!(c.y, -7);
                assert_abs_diff_eq!(c.z, -9);
                assert_abs_diff_eq!(c.w, 6);
            }

            #[test]
            fn mul() {
                let b = 3;
                let c = vector4::mul(V0, b);
                assert_abs_diff_eq!(c.x, 21);
                assert_abs_diff_eq!(c.y, -12);
                assert_abs_diff_eq!(c.z, 3);
                assert_abs_diff_eq!(c.w, 6);
            }

            #[test]
            fn div_with_numerator() {
                let c = vector4::div_with_numerator(10, V0);
                assert_abs_diff_eq!(c.x, 1);
                assert_abs_diff_eq!(c.y, -2);
                assert_abs_diff_eq!(c.z, 10);
                assert_abs_diff_eq!(c.w, 5);
            }

            #[test]
            fn div_with_denominator() {
                let c = vector4::div_with_denominator(V0, 10);
                assert_abs_diff_eq!(c.x, 0);
                assert_abs_diff_eq!(c.y, 0);
                assert_abs_diff_eq!(c.z, 0);
                assert_abs_diff_eq!(c.w, 0);
            }

            #[test]
            fn component_mul() {
                let c = vector4::component_mul(V0, V1);
                assert_abs_diff_eq!(c.x, 0);
                assert_abs_diff_eq!(c.y, -12);
                assert_abs_diff_eq!(c.z, 10);
                assert_abs_diff_eq!(c.w, -8);
            }

            #[test]
            fn dot() {
                let c = vector4::dot(V0, V1);
                assert_abs_diff_eq!(c, -10);
            }

            #[test]
            fn magnitude_sqr() {
                let c = vector4::magnitude_sqr(V0);
                assert_abs_diff_eq!(c, 70);
            }

            #[test]
            fn neg() {
                let c = vector4::negate(V0);
                assert_abs_diff_eq!(c.x, -7);
                assert_abs_diff_eq!(c.y, 4);
                assert_abs_diff_eq!(c.z, -1);
                assert_abs_diff_eq!(c.w, -2);
            }

            #[test]
            fn project() {
                let c = vector4::project(V0, V1);
                assert_abs_diff_eq!(c.x, 0);
                assert_abs_diff_eq!(c.y, 0);
                assert_abs_diff_eq!(c.z, 0);
                assert_abs_diff_eq!(c.w, 0);
            }
        }
    }
    #[macro_export]
    macro_rules! vector4_uint_tests {
        ( $feature:ident, $type:ty ) => {
            use approx::assert_abs_diff_eq;
            use crate::Vector4;
            use crate::$feature::vector4;

            const V0: Vector4<$type> = crate::Vector!(10, 4, 1, 6);
            const V1: Vector4<$type> = crate::Vector!(3, 3, 0, 4);

            #[test]
            fn addition() {
                let c = vector4::add(V0, V1);
                assert_abs_diff_eq!(c.x, 13);
                assert_abs_diff_eq!(c.y, 7);
                assert_abs_diff_eq!(c.z, 1);
                assert_abs_diff_eq!(c.w, 10);
            }

            #[test]
            fn sub() {
                let c = vector4::sub(V0, V1);
                assert_abs_diff_eq!(c.x, 7);
                assert_abs_diff_eq!(c.y, 1);
                assert_abs_diff_eq!(c.z, 1);
                assert_abs_diff_eq!(c.w, 2);
            }

            #[test]
            fn mul() {
                let b = 3;
                let c = vector4::mul(V0, b);
                assert_abs_diff_eq!(c.x, 30);
                assert_abs_diff_eq!(c.y, 12);
                assert_abs_diff_eq!(c.z, 3);
                assert_abs_diff_eq!(c.w, 18);
            }

            #[test]
            fn div_with_numerator() {
                let c = vector4::div_with_numerator(10, V0);
                assert_abs_diff_eq!(c.x, 1);
                assert_abs_diff_eq!(c.y, 2);
                assert_abs_diff_eq!(c.z, 10);
                assert_abs_diff_eq!(c.w, 1);
            }

            #[test]
            fn div_with_denominator() {
                let c = vector4::div_with_denominator(V0, 10);
                assert_abs_diff_eq!(c.x, 1);
                assert_abs_diff_eq!(c.y, 0);
                assert_abs_diff_eq!(c.z, 0);
                assert_abs_diff_eq!(c.w, 0);
            }

            #[test]
            fn component_mul() {
                let c = vector4::component_mul(V0, V1);
                assert_abs_diff_eq!(c.x, 30);
                assert_abs_diff_eq!(c.y, 12);
                assert_abs_diff_eq!(c.z, 0);
                assert_abs_diff_eq!(c.w, 24);
            }

            #[test]
            fn dot() {
                let c = vector4::dot(V0, V1);
                assert_abs_diff_eq!(c, 66);
            }

            #[test]
            fn magnitude_sqr() {
                let c = vector4::magnitude_sqr(V0);
                assert_abs_diff_eq!(c, 153);
            }

            #[test]
            fn project() {
                let c = vector4::project(V0, V1);
                assert_abs_diff_eq!(c.x, 3);
                assert_abs_diff_eq!(c.y, 3);
                assert_abs_diff_eq!(c.z, 0);
                assert_abs_diff_eq!(c.w, 4);
            }
        };
    }
    #[macro_export]
    macro_rules! vector4_float_tests {
        ( $feature:ident, f32 ) => {
            use approx::assert_abs_diff_eq;
            use crate::Vector4;
            use crate::$feature::vector4;

            const V0: Vector4<f32> = crate::Vector!(5.0, 12.0, -3.5, 2.0);
            const V1: Vector4<f32> = crate::Vector!(6.7, 8.7, 5.0, 3.4);

            #[test]
            fn addition() {
                let c = vector4::add(V0, V1);
                assert_abs_diff_eq!(c.x, 11.7);
                assert_abs_diff_eq!(c.y, 20.7);
                assert_abs_diff_eq!(c.z, 1.5);
                assert_abs_diff_eq!(c.w, 5.4);
            }

            #[test]
            fn sub() {
                let c = vector4::sub(V0, V1);
                assert_abs_diff_eq!(c.x, -1.6999998);
                assert_abs_diff_eq!(c.y, 3.3000002);
                assert_abs_diff_eq!(c.z, -8.5);
                assert_abs_diff_eq!(c.w, -1.4);
            }

            #[test]
            fn mul() {
                let b = 3.0;
                let c = vector4::mul(V0, b);
                assert_abs_diff_eq!(c.x, 15.0);
                assert_abs_diff_eq!(c.y, 36.0);
                assert_abs_diff_eq!(c.z, -10.5);
                assert_abs_diff_eq!(c.w, 6.0);
            }

            #[test]
            fn div_with_numerator() {
                let c = vector4::div_with_numerator(10.0, V0);
                assert_abs_diff_eq!(c.x, 2.0);
                assert_abs_diff_eq!(c.y, 0.8333333333333334);
                assert_abs_diff_eq!(c.z, -2.857142857142857);
                assert_abs_diff_eq!(c.w, 5.0);
            }

            #[test]
            fn div_with_denominator() {
                let c = vector4::div_with_denominator(V0, 10.0);
                assert_abs_diff_eq!(c.x, 0.5);
                assert_abs_diff_eq!(c.y, 1.2);
                assert_abs_diff_eq!(c.z, -0.35);
                assert_abs_diff_eq!(c.w, 0.2);
            }

            #[test]
            fn component_mul() {
                let c = vector4::component_mul(V0, V1);
                assert_abs_diff_eq!(c.x, 33.5);
                assert_abs_diff_eq!(c.y, 104.399994);
                assert_abs_diff_eq!(c.z, -17.5);
                assert_abs_diff_eq!(c.w, 6.8);
            }

            #[test]
            fn dot() {
                let c = vector4::dot(V0, V1);
                assert_abs_diff_eq!(c, 127.19999999999997);
            }

            #[test]
            fn magnitude() {
                let c = vector4::magnitude(V0);
                assert_abs_diff_eq!(c, 13.6106575888162);
            }

            #[test]
            fn magnitude_sqr() {
                let c = vector4::magnitude_sqr(V0);
                assert_abs_diff_eq!(c, 185.25);
            }

            #[test]
            fn neg() {
                let c = vector4::negate(V0);
                assert_abs_diff_eq!(c.x, -5.0);
                assert_abs_diff_eq!(c.y, -12.0);
                assert_abs_diff_eq!(c.z, 3.5);
                assert_abs_diff_eq!(c.w, -2.0);
            }

            #[test]
            fn normalize() {
                let c = vector4::normalize(V0);
                assert_abs_diff_eq!(vector4::magnitude_sqr(c), 1.0);
                assert_abs_diff_eq!(c.x, 0.3673591791853225);
                assert_abs_diff_eq!(c.y, 0.8816620300447741);
                assert_abs_diff_eq!(c.z, -0.25715142542972574);
                assert_abs_diff_eq!(c.w, 0.146943671674129);
            }

            #[test]
            fn project() {
                let c = vector4::project(V0, V1);
                assert_abs_diff_eq!(c.x, 5.4234443);
                assert_abs_diff_eq!(c.y, 7.042383);
                assert_abs_diff_eq!(c.z, 4.0473466);
                assert_abs_diff_eq!(c.w, 2.7521958);
            }
        };
        ( $feature:ident, f64 ) => {
            use approx::assert_abs_diff_eq;
            use crate::Vector4;
            use crate::$feature::vector4;

            const V0: Vector4<f64> = crate::Vector!(5.0, 12.0, -3.5, 2.0);
            const V1: Vector4<f64> = crate::Vector!(6.7, 8.7, 5.0, 3.4);

            #[test]
            fn addition() {
                let c = vector4::add(V0, V1);
                assert_abs_diff_eq!(c.x, 11.7);
                assert_abs_diff_eq!(c.y, 20.7);
                assert_abs_diff_eq!(c.z, 1.5);
                assert_abs_diff_eq!(c.w, 5.4);
            }

            #[test]
            fn sub() {
                let c = vector4::sub(V0, V1);
                assert_abs_diff_eq!(c.x, -1.7000000000000002);
                assert_abs_diff_eq!(c.y, 3.3000000000000007);
                assert_abs_diff_eq!(c.z, -8.5);
                assert_abs_diff_eq!(c.w, -1.4);
            }

            #[test]
            fn mul() {
                let b = 3.0;
                let c = vector4::mul(V0, b);
                assert_abs_diff_eq!(c.x, 15.0);
                assert_abs_diff_eq!(c.y, 36.0);
                assert_abs_diff_eq!(c.z, -10.5);
                assert_abs_diff_eq!(c.w, 6.0);
            }

            #[test]
            fn div_with_numerator() {
                let c = vector4::div_with_numerator(10.0, V0);
                assert_abs_diff_eq!(c.x, 2.0);
                assert_abs_diff_eq!(c.y, 0.8333333333333334);
                assert_abs_diff_eq!(c.z, -2.857142857142857);
                assert_abs_diff_eq!(c.w, 5.0);
            }

            #[test]
            fn div_with_denominator() {
                let c = vector4::div_with_denominator(V0, 10.0);
                assert_abs_diff_eq!(c.x, 0.5);
                assert_abs_diff_eq!(c.y, 1.2);
                assert_abs_diff_eq!(c.z, -0.35);
                assert_abs_diff_eq!(c.w, 0.2);
            }

            #[test]
            fn component_mul() {
                let c = vector4::component_mul(V0, V1);
                assert_abs_diff_eq!(c.x, 33.5);
                assert_abs_diff_eq!(c.y, 104.39999999999999);
                assert_abs_diff_eq!(c.z, -17.5);
                assert_abs_diff_eq!(c.w, 6.8);
            }

            #[test]
            fn dot() {
                let c = vector4::dot(V0, V1);
                assert_abs_diff_eq!(c, 127.19999999999997);
            }

            #[test]
            fn magnitude() {
                let c = vector4::magnitude(V0);
                assert_abs_diff_eq!(c, 13.6106575888162);
            }

            #[test]
            fn magnitude_sqr() {
                let c = vector4::magnitude_sqr(V0);
                assert_abs_diff_eq!(c, 185.25);
            }

            #[test]
            fn neg() {
                let c = vector4::negate(V0);
                assert_abs_diff_eq!(c.x, -5.0);
                assert_abs_diff_eq!(c.y, -12.0);
                assert_abs_diff_eq!(c.z, 3.5);
                assert_abs_diff_eq!(c.w, -2.0);
            }

            #[test]
            fn normalize() {
                let c = vector4::normalize(V0);
                assert_abs_diff_eq!(vector4::magnitude_sqr(c), 1.0);
                assert_abs_diff_eq!(c.x, 0.3673591791853225);
                assert_abs_diff_eq!(c.y, 0.8816620300447741);
                assert_abs_diff_eq!(c.z, -0.25715142542972574);
                assert_abs_diff_eq!(c.w, 0.146943671674129);
            }

            #[test]
            fn project() {
                let c = vector4::project(V0, V1);
                assert_abs_diff_eq!(c.x, 5.42344406261932);
                assert_abs_diff_eq!(c.y, 7.04238258877434);
                assert_abs_diff_eq!(c.z, 4.047346315387552);
                assert_abs_diff_eq!(c.w, 2.7521954944635354);
            }
        };
    }
}
