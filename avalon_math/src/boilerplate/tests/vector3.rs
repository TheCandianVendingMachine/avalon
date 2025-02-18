#[macro_export]
macro_rules! vector3_sint_tests {
    ( $feature:ident, $type:ty ) => {
        use approx::assert_abs_diff_eq;
        use crate::Vector3;
        use crate::$feature::vector3;

        const V0: Vector3<$type> = crate::Vector!(7, -4, 1);
        const V1: Vector3<$type> = crate::Vector!(0, 3, 10);

        #[test]
        fn addition() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::add(V0, V1) };
            assert_abs_diff_eq!(c.x, 7);
            assert_abs_diff_eq!(c.y, -1);
            assert_abs_diff_eq!(c.z, 11);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::sub(V0, V1) };
            assert_abs_diff_eq!(c.x, 7);
            assert_abs_diff_eq!(c.y, -7);
            assert_abs_diff_eq!(c.z, -9);
        }

        #[test]
        fn mul() {
            #![allow(unused_unsafe)]
            let b = 3;
            let c = unsafe { vector3::mul(V0, b) };
            assert_abs_diff_eq!(c.x, 21);
            assert_abs_diff_eq!(c.y, -12);
            assert_abs_diff_eq!(c.z, 3);
        }

        #[test]
        fn div_with_numerator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::div_with_numerator(10, V0) };
            assert_abs_diff_eq!(c.x, 1);
            assert_abs_diff_eq!(c.y, -2);
            assert_abs_diff_eq!(c.z, 10);
        }

        #[test]
        fn div_with_denominator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::div_with_denominator(V0, 10) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, 0);
            assert_abs_diff_eq!(c.z, 0);
        }

        #[test]
        fn component_mul() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::component_mul(V0, V1) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, -12);
            assert_abs_diff_eq!(c.z, 10);
        }

        #[test]
        fn dot() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::dot(V0, V1) };
            assert_abs_diff_eq!(c, -2);
        }

        #[test]
        fn magnitude_sqr() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::magnitude_sqr(V0) };
            assert_abs_diff_eq!(c, 66);
        }

        #[test]
        fn neg() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::negate(V0) };
            assert_abs_diff_eq!(c.x, -7);
            assert_abs_diff_eq!(c.y, 4);
            assert_abs_diff_eq!(c.z, -1);
        }

        #[test]
        fn project() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::project(V0, V1) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, 0);
            assert_abs_diff_eq!(c.z, 0);
        }
    }
}
#[macro_export]
macro_rules! vector3_uint_tests {
    ( $feature:ident, $type:ty ) => {
        use approx::assert_abs_diff_eq;
        use crate::Vector3;
        use crate::$feature::vector3;

        const V0: Vector3<$type> = crate::Vector!(3, 4, 1);
        const V1: Vector3<$type> = crate::Vector!(3, 3, 0);

        #[test]
        fn addition() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::add(V0, V1) };
            assert_abs_diff_eq!(c.x, 6);
            assert_abs_diff_eq!(c.y, 7);
            assert_abs_diff_eq!(c.z, 1);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::sub(V0, V1) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, 1);
            assert_abs_diff_eq!(c.z, 1);
        }

        #[test]
        fn mul() {
            #![allow(unused_unsafe)]
            let b = 3;
            let c = unsafe { vector3::mul(V0, b) };
            assert_abs_diff_eq!(c.x, 9);
            assert_abs_diff_eq!(c.y, 12);
            assert_abs_diff_eq!(c.z, 3);
        }

        #[test]
        fn div_with_numerator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::div_with_numerator(10, V0) };
            assert_abs_diff_eq!(c.x, 3);
            assert_abs_diff_eq!(c.y, 2);
            assert_abs_diff_eq!(c.z, 10);
        }

        #[test]
        fn div_with_denominator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::div_with_denominator(V0, 10) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, 0);
            assert_abs_diff_eq!(c.z, 0);
        }

        #[test]
        fn component_mul() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::component_mul(V0, V1) };
            assert_abs_diff_eq!(c.x, 9);
            assert_abs_diff_eq!(c.y, 12);
            assert_abs_diff_eq!(c.z, 0);
        }

        #[test]
        fn dot() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::dot(V0, V1) };
            assert_abs_diff_eq!(c, 21);
        }

        #[test]
        fn magnitude_sqr() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::magnitude_sqr(V0) };
            assert_abs_diff_eq!(c, 26);
        }

        #[test]
        fn project() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::project(V0, V1) };
            assert_abs_diff_eq!(c.x, 3);
            assert_abs_diff_eq!(c.y, 3);
            assert_abs_diff_eq!(c.z, 0);
        }
    };
}
#[macro_export]
macro_rules! vector3_float_tests {
    ( $feature:ident, f32 ) => {
        use approx::assert_abs_diff_eq;
        use crate::Vector3;
        use crate::$feature::vector3;

        const V0: Vector3<f32> = crate::Vector!(5.0, 12.0, -3.5);
        const V1: Vector3<f32> = crate::Vector!(6.7, 8.7, 5.0);

        #[test]
        fn addition() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::add(V0, V1) };
            assert_abs_diff_eq!(c.x, 11.7);
            assert_abs_diff_eq!(c.y, 20.7);
            assert_abs_diff_eq!(c.z, 1.5);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::sub(V0, V1) };
            assert_abs_diff_eq!(c.x, -1.6999998);
            assert_abs_diff_eq!(c.y, 3.3000002);
            assert_abs_diff_eq!(c.z, -8.5);
        }

        #[test]
        fn mul() {
            #![allow(unused_unsafe)]
            let b = 3.0;
            let c = unsafe { vector3::mul(V0, b) };
            assert_abs_diff_eq!(c.x, 15.0);
            assert_abs_diff_eq!(c.y, 36.0);
            assert_abs_diff_eq!(c.z, -10.5);
        }

        #[test]
        fn div_with_numerator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::div_with_numerator(10.0, V0) };
            assert_abs_diff_eq!(c.x, 2.0);
            assert_abs_diff_eq!(c.y, 0.8333333333333334);
            assert_abs_diff_eq!(c.z, -2.857142857142857);
        }

        #[test]
        fn div_with_denominator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::div_with_denominator(V0, 10.0) };
            assert_abs_diff_eq!(c.x, 0.5);
            assert_abs_diff_eq!(c.y, 1.2);
            assert_abs_diff_eq!(c.z, -0.35);
        }

        #[test]
        fn component_mul() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::component_mul(V0, V1) };
            assert_abs_diff_eq!(c.x, 33.5);
            assert_abs_diff_eq!(c.y, 104.399994);
            assert_abs_diff_eq!(c.z, -17.5);
        }

        #[test]
        fn dot() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::dot(V0, V1) };
            assert_abs_diff_eq!(c, 120.399994);
        }

        #[test]
        fn magnitude() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::magnitude(V0) };
            assert_abs_diff_eq!(c, 13.46291201783626);
        }

        #[test]
        fn magnitude_sqr() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::magnitude_sqr(V0) };
            assert_abs_diff_eq!(c, 181.25);
        }

        #[test]
        fn neg() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::negate(V0) };
            assert_abs_diff_eq!(c.x, -5.0);
            assert_abs_diff_eq!(c.y, -12.0);
            assert_abs_diff_eq!(c.z, 3.5);
        }

        #[test]
        fn normalize() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::normalize(V0) };
            assert_abs_diff_eq!(vector3::magnitude_sqr(c), 1.0);
            assert_abs_diff_eq!(c.x, 0.3713906763541037);
            assert_abs_diff_eq!(c.y, 0.891337623249849);
            assert_abs_diff_eq!(c.z, -0.25997347344787264);
        }
    };
    ( $feature:ident, f64 ) => {
        use approx::assert_abs_diff_eq;
        use crate::Vector3;
        use crate::$feature::vector3;

        const V0: Vector3<f64> = crate::Vector!(5.0, 12.0, -3.5);
        const V1: Vector3<f64> = crate::Vector!(6.7, 8.7, 5.0);

        #[test]
        fn addition() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::add(V0, V1) };
            assert_abs_diff_eq!(c.x, 11.7);
            assert_abs_diff_eq!(c.y, 20.7);
            assert_abs_diff_eq!(c.z, 1.5);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::sub(V0, V1) };
            assert_abs_diff_eq!(c.x, -1.7000000000000002);
            assert_abs_diff_eq!(c.y, 3.3000000000000007);
            assert_abs_diff_eq!(c.z, -8.5);
        }

        #[test]
        fn mul() {
            #![allow(unused_unsafe)]
            let b = 3.0;
            let c = unsafe { vector3::mul(V0, b) };
            assert_abs_diff_eq!(c.x, 15.0);
            assert_abs_diff_eq!(c.y, 36.0);
            assert_abs_diff_eq!(c.z, -10.5);
        }

        #[test]
        fn div_with_numerator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::div_with_numerator(10.0, V0) };
            assert_abs_diff_eq!(c.x, 2.0);
            assert_abs_diff_eq!(c.y, 0.8333333333333334);
            assert_abs_diff_eq!(c.z, -2.857142857142857);
        }

        #[test]
        fn div_with_denominator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::div_with_denominator(V0, 10.0) };
            assert_abs_diff_eq!(c.x, 0.5);
            assert_abs_diff_eq!(c.y, 1.2);
            assert_abs_diff_eq!(c.z, -0.35);
        }

        #[test]
        fn component_mul() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::component_mul(V0, V1) };
            assert_abs_diff_eq!(c.x, 33.5);
            assert_abs_diff_eq!(c.y, 104.39999999999999);
            assert_abs_diff_eq!(c.z, -17.5);
        }

        #[test]
        fn dot() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::dot(V0, V1) };
            assert_abs_diff_eq!(c, 120.39999999999998);
        }

        #[test]
        fn magnitude() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::magnitude(V0) };
            assert_abs_diff_eq!(c, 13.46291201783626);
        }

        #[test]
        fn magnitude_sqr() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::magnitude_sqr(V0) };
            assert_abs_diff_eq!(c, 181.25);
        }

        #[test]
        fn neg() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::negate(V0) };
            assert_abs_diff_eq!(c.x, -5.0);
            assert_abs_diff_eq!(c.y, -12.0);
            assert_abs_diff_eq!(c.z, 3.5);
        }

        #[test]
        fn normalize() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector3::normalize(V0) };
            assert_abs_diff_eq!(vector3::magnitude_sqr(c), 1.0);
            assert_abs_diff_eq!(c.x, 0.3713906763541037);
            assert_abs_diff_eq!(c.y, 0.891337623249849);
            assert_abs_diff_eq!(c.z, -0.25997347344787264);
        }
    };
}
