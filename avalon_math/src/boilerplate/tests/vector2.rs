#[macro_export]
macro_rules! vector2_sint_tests {
    ( $feature:ident, $type:ty ) => {
        use approx::assert_abs_diff_eq;
        use crate::Vector2;
        use crate::$feature::vector2;

        const V0: Vector2<$type> = crate::Vector!(7, -4);
        const V1: Vector2<$type> = crate::Vector!(0, 3);

        #[test]
        fn addition() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::add(V0, V1) };
            assert_abs_diff_eq!(c.x, 7);
            assert_abs_diff_eq!(c.y, -1);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::sub(V0, V1) };
            assert_abs_diff_eq!(c.x, 7);
            assert_abs_diff_eq!(c.y, -7);
        }

        #[test]
        fn mul() {
            #![allow(unused_unsafe)]
            let b = 3;
            let c = unsafe { vector2::mul(V0, b) };
            assert_abs_diff_eq!(c.x, 21);
            assert_abs_diff_eq!(c.y, -12);
        }

        #[test]
        fn div_with_numerator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::div_with_numerator(10, V0) };
            assert_abs_diff_eq!(c.x, 1);
            assert_abs_diff_eq!(c.y, -2);
        }

        #[test]
        fn div_with_denominator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::div_with_denominator(V0, 10) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, 0);
        }

        #[test]
        fn component_mul() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::component_mul(V0, V1) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, -12);
        }

        #[test]
        fn dot() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::dot(V0, V1) };
            assert_abs_diff_eq!(c, -12);
        }

        #[test]
        fn magnitude_sqr() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::magnitude_sqr(V0) };
            assert_abs_diff_eq!(c, 65);
        }

        #[test]
        fn neg() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::negate(V0) };
            assert_abs_diff_eq!(c.x, -7);
            assert_abs_diff_eq!(c.y, 4);
        }

        #[test]
        fn project() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::project(V0, V1) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, -4);
        }
    }
}
#[macro_export]
macro_rules! vector2_uint_tests {
    ( $feature:ident, $type:ty ) => {
        use approx::assert_abs_diff_eq;
        use crate::Vector2;
        use crate::$feature::vector2;

        const V0: Vector2<$type> = crate::Vector!(3, 4);
        const V1: Vector2<$type> = crate::Vector!(3, 3);

        #[test]
        fn addition() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::add(V0, V1) };
            assert_abs_diff_eq!(c.x, 6);
            assert_abs_diff_eq!(c.y, 7);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::sub(V0, V1) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, 1);
        }

        #[test]
        fn mul() {
            #![allow(unused_unsafe)]
            let b = 3;
            let c = unsafe { vector2::mul(V0, b) };
            assert_abs_diff_eq!(c.x, 9);
            assert_abs_diff_eq!(c.y, 12);
        }

        #[test]
        fn div_with_numerator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::div_with_numerator(10, V0) };
            assert_abs_diff_eq!(c.x, 3);
            assert_abs_diff_eq!(c.y, 2);
        }

        #[test]
        fn div_with_denominator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::div_with_denominator(V0, 10) };
            assert_abs_diff_eq!(c.x, 0);
            assert_abs_diff_eq!(c.y, 0);
        }

        #[test]
        fn component_mul() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::component_mul(V0, V1) };
            assert_abs_diff_eq!(c.x, 9);
            assert_abs_diff_eq!(c.y, 12);
        }

        #[test]
        fn dot() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::dot(V0, V1) };
            assert_abs_diff_eq!(c, 21);
        }

        #[test]
        fn magnitude_sqr() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::magnitude_sqr(V0) };
            assert_abs_diff_eq!(c, 25);
        }

        #[test]
        fn project() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::project(V0, V1) };
            assert_abs_diff_eq!(c.x, 3);
            assert_abs_diff_eq!(c.y, 3);
        }
    };
}
#[macro_export]
macro_rules! vector2_float_tests {
    ( $feature:ident, f32 ) => {
        use approx::assert_abs_diff_eq;
        use crate::Vector2;
        use crate::$feature::vector2;

        const V0: Vector2<f32> = crate::Vector!(5.0, 12.0);
        const V1: Vector2<f32> = crate::Vector!(6.7, 8.7);

        #[test]
        fn addition() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::add(V0, V1) };
            assert_abs_diff_eq!(c.x, 11.7);
            assert_abs_diff_eq!(c.y, 20.7);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::sub(V0, V1) };
            assert_abs_diff_eq!(c.x, -1.6999998);
            assert_abs_diff_eq!(c.y, 3.3000002);
        }

        #[test]
        fn mul() {
            #![allow(unused_unsafe)]
            let b = 3.0;
            let c = unsafe { vector2::mul(V0, b) };
            assert_abs_diff_eq!(c.x, 15.0);
            assert_abs_diff_eq!(c.y, 36.0);
        }

        #[test]
        fn div_with_numerator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::div_with_numerator(10.0, V0) };
            assert_abs_diff_eq!(c.x, 2.0);
            assert_abs_diff_eq!(c.y, 0.8333333333333334);
        }

        #[test]
        fn div_with_denominator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::div_with_denominator(V0, 10.0) };
            assert_abs_diff_eq!(c.x, 0.5);
            assert_abs_diff_eq!(c.y, 1.2);
        }

        #[test]
        fn component_mul() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::component_mul(V0, V1) };
            assert_abs_diff_eq!(c.x, 33.5);
            assert_abs_diff_eq!(c.y, 104.399994);
        }

        #[test]
        fn dot() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::dot(V0, V1) };
            assert_abs_diff_eq!(c, 137.89999999999998);
        }

        #[test]
        fn magnitude() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::magnitude(V0) };
            assert_abs_diff_eq!(c, 13.0);
        }

        #[test]
        fn magnitude_sqr() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::magnitude_sqr(V0) };
            assert_abs_diff_eq!(c, 169.0);
        }

        #[test]
        fn neg() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::negate(V0) };
            assert_abs_diff_eq!(c.x, -5.0);
            assert_abs_diff_eq!(c.y, -12.0);
        }

        #[test]
        fn normalize() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::normalize(V0) };
            assert_abs_diff_eq!(vector2::magnitude_sqr(c), 1.0);
            assert_abs_diff_eq!(c.x, 0.38461538461538464);
            assert_abs_diff_eq!(c.y, 0.9230769230769231);
        }
    };
    ( $feature:ident, f64 ) => {
        use approx::assert_abs_diff_eq;
        use crate::Vector2;
        use crate::$feature::vector2;

        const V0: Vector2<f64> = crate::Vector!(5.0, 12.0);
        const V1: Vector2<f64> = crate::Vector!(6.7, 8.7);

        #[test]
        fn addition() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::add(V0, V1) };
            assert_abs_diff_eq!(c.x, 11.7);
            assert_abs_diff_eq!(c.y, 20.7);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::sub(V0, V1) };
            assert_abs_diff_eq!(c.x, -1.7000000000000002);
            assert_abs_diff_eq!(c.y, 3.3000000000000007);
        }

        #[test]
        fn mul() {
            #![allow(unused_unsafe)]
            let b = 3.0;
            let c = unsafe { vector2::mul(V0, b) };
            assert_abs_diff_eq!(c.x, 15.0);
            assert_abs_diff_eq!(c.y, 36.0);
        }

        #[test]
        fn div_with_numerator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::div_with_numerator(10.0, V0) };
            assert_abs_diff_eq!(c.x, 2.0);
            assert_abs_diff_eq!(c.y, 0.8333333333333334);
        }

        #[test]
        fn div_with_denominator() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::div_with_denominator(V0, 10.0) };
            assert_abs_diff_eq!(c.x, 0.5);
            assert_abs_diff_eq!(c.y, 1.2);
        }

        #[test]
        fn component_mul() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::component_mul(V0, V1) };
            assert_abs_diff_eq!(c.x, 33.5);
            assert_abs_diff_eq!(c.y, 104.39999999999999);
        }

        #[test]
        fn dot() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::dot(V0, V1) };
            assert_abs_diff_eq!(c, 137.89999999999998);
        }

        #[test]
        fn magnitude() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::magnitude(V0) };
            assert_abs_diff_eq!(c, 13.0);
        }

        #[test]
        fn magnitude_sqr() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::magnitude_sqr(V0) };
            assert_abs_diff_eq!(c, 169.0);
        }

        #[test]
        fn neg() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::negate(V0) };
            assert_abs_diff_eq!(c.x, -5.0);
            assert_abs_diff_eq!(c.y, -12.0);
        }

        #[test]
        fn normalize() {
            #![allow(unused_unsafe)]
            let c = unsafe { vector2::normalize(V0) };
            assert_abs_diff_eq!(vector2::magnitude_sqr(c), 1.0);
            assert_abs_diff_eq!(c.x, 0.38461538461538464);
            assert_abs_diff_eq!(c.y, 0.9230769230769231);
        }
    };
}
