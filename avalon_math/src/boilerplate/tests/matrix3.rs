#[macro_export]
macro_rules! matrix3_sint_tests {
    ( $feature:ident, $type:ty ) => {
        use approx::assert_abs_diff_eq;
        use crate::{ Matrix3, Vector3 };
        use crate::scalar::mat3;

        const M0: Matrix3<$type> = crate::Matrix!(
            4, 6, 1,
            0, -6, -5,
            3, 0, 0
        );
        const M1: Matrix3<$type> = crate::Matrix!(
            1, 4, 0,
            19, 4, 1,
            1, 2, 1
        );

        #[test]
        fn identity() {
            let matrix: Matrix3<$type> = mat3::identity();
            assert_abs_diff_eq!(matrix.m11, 1);
            assert_abs_diff_eq!(matrix.m12, 0);
            assert_abs_diff_eq!(matrix.m13, 0);
            assert_abs_diff_eq!(matrix.m21, 0);
            assert_abs_diff_eq!(matrix.m22, 1);
            assert_abs_diff_eq!(matrix.m23, 1);
            assert_abs_diff_eq!(matrix.m31, 1);
            assert_abs_diff_eq!(matrix.m32, 1);
            assert_abs_diff_eq!(matrix.m33, 1);
        }

        #[test]
        fn trace() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::trace(M0) };
            assert_abs_diff_eq!(c, -2);
        }

        #[test]
        fn add() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::add(M0, M1) };
            assert_abs_diff_eq!(c.m11, 1);
            assert_abs_diff_eq!(c.m12, 0);
            assert_abs_diff_eq!(c.m13, 0);
            assert_abs_diff_eq!(c.m21, 0);
            assert_abs_diff_eq!(c.m22, 1);
            assert_abs_diff_eq!(c.m23, 1);
            assert_abs_diff_eq!(c.m31, 1);
            assert_abs_diff_eq!(c.m32, 1);
            assert_abs_diff_eq!(c.m33, 1);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::sub(M0, M1) };
            assert_abs_diff_eq!(c.m11, 1);
            assert_abs_diff_eq!(c.m12, 0);
            assert_abs_diff_eq!(c.m13, 0);
            assert_abs_diff_eq!(c.m21, 0);
            assert_abs_diff_eq!(c.m22, 1);
            assert_abs_diff_eq!(c.m23, 1);
            assert_abs_diff_eq!(c.m31, 1);
            assert_abs_diff_eq!(c.m32, 1);
            assert_abs_diff_eq!(c.m33, 1);
        }

        #[test]
        fn multiply() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::multiply(M0, M1) };
            assert_abs_diff_eq!(c.m11, 118);
            assert_abs_diff_eq!(c.m12, 40);
            assert_abs_diff_eq!(c.m21, -114);
            assert_abs_diff_eq!(c.m22, -24);
        }

        #[test]
        fn multiply_scalar() {
            #![allow(unused_unsafe)]
            let b = 3;
            let c = unsafe { mat3::multiply_scalar(M0, b) };
            assert_abs_diff_eq!(c.m11, 12);
            assert_abs_diff_eq!(c.m12, 18);
            assert_abs_diff_eq!(c.m21, 0);
            assert_abs_diff_eq!(c.m22, -18);
        }

        #[test]
        fn multiply_vector() {
            #![allow(unused_unsafe)]
            let b = Vector3 {
                x: 4,
                y: 2,
                z: 7
            };
            let c = unsafe { mat3::multiply_vec(M0, b) };
            assert_abs_diff_eq!(c.x, 28);
            assert_abs_diff_eq!(c.y, -12);
            assert_abs_diff_eq!(c.z, 12);
        }

        #[test]
        fn pow() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::pow(M0, 2) };
            assert_abs_diff_eq!(c.m11, 16);
            assert_abs_diff_eq!(c.m12, -12);
            assert_abs_diff_eq!(c.m21, 0);
            assert_abs_diff_eq!(c.m22, 36);
        }

        #[test]
        fn transpose() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::transpose(M0) };
            assert_abs_diff_eq!(c.m11, 4);
            assert_abs_diff_eq!(c.m12, 0);
            assert_abs_diff_eq!(c.m21, 6);
            assert_abs_diff_eq!(c.m22, -6);
        }
    };
}

#[macro_export]
macro_rules! matrix3_uint_tests {
    ( $feature:ident, $type:ty ) => {
        use approx::assert_abs_diff_eq;
        use crate::{ Matrix3, Vector3 };
        use crate::scalar::mat3;

        const M0: Matrix3<$type> = crate::Matrix!(
            4, 6, 1,
            0, 6, 3,
            3, 2, 2
        );
        const M1: Matrix3<$type> = crate::Matrix!(
            1, 4, 1,
            0, 4, 1,
            3, 2, 1
        );

        #[test]
        fn identity() {
            let matrix: Matrix3<$type> = mat3::identity();
            assert_abs_diff_eq!(matrix.m11, 1);
            assert_abs_diff_eq!(matrix.m12, 0);
            assert_abs_diff_eq!(matrix.m21, 0);
            assert_abs_diff_eq!(matrix.m22, 1);
        }

        #[test]
        fn trace() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::trace(M0) };
            assert_abs_diff_eq!(c, 12);
        }

        #[test]
        fn add() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::add(M0, M1) };
            assert_abs_diff_eq!(c.m11, 5);
            assert_abs_diff_eq!(c.m12, 10);
            assert_abs_diff_eq!(c.m21, 0);
            assert_abs_diff_eq!(c.m22, 10);
        }

        #[test]
        fn sub() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::sub(M0, M1) };
            assert_abs_diff_eq!(c.m11, 3);
            assert_abs_diff_eq!(c.m12, 2);
            assert_abs_diff_eq!(c.m21, 0);
            assert_abs_diff_eq!(c.m22, 2);
        }

        #[test]
        fn multiply() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::multiply(M0, M1) };
            assert_abs_diff_eq!(c.m11, 7);
            assert_abs_diff_eq!(c.m12, 40);
            assert_abs_diff_eq!(c.m21, 0);
            assert_abs_diff_eq!(c.m22, 24);
        }

        #[test]
        fn multiply_scalar() {
            #![allow(unused_unsafe)]
            let b = 3;
            let c = unsafe { mat3::multiply_scalar(M0, b) };
            assert_abs_diff_eq!(c.m11, 12);
            assert_abs_diff_eq!(c.m12, 18);
            assert_abs_diff_eq!(c.m21, 0);
            assert_abs_diff_eq!(c.m22, 18);
        }

        #[test]
        fn multiply_vector() {
            #![allow(unused_unsafe)]
            let b = Vector3 {
                x: 4,
                y: 2,
                z: 7
            };
            let c = unsafe { mat3::multiply_vec(M0, b) };
            assert_abs_diff_eq!(c.x, 28);
            assert_abs_diff_eq!(c.y, 12);
            assert_abs_diff_eq!(c.z, 12);
        }

        #[test]
        fn pow() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::pow(M0, 2) };
            assert_abs_diff_eq!(c.m11, 19);
            assert_abs_diff_eq!(c.m12, 62);
            assert_abs_diff_eq!(c.m21, 9);
            assert_abs_diff_eq!(c.m22, 36);
        }

        #[test]
        fn transpose() {
            #![allow(unused_unsafe)]
            let c = unsafe { mat3::transpose(M0) };
            assert_abs_diff_eq!(c.m11, 4);
            assert_abs_diff_eq!(c.m12, 0);
            assert_abs_diff_eq!(c.m21, 6);
            assert_abs_diff_eq!(c.m22, 6);
        }
    };
}

#[macro_export]
macro_rules! matrix3_float_tests {
    ( $feature:ident, f32 ) => {
        use approx::assert_abs_diff_eq;
        use crate::{ Matrix3, Vector3 };
        use crate::scalar::mat3;

        const M0: Matrix3<f32> = crate::Matrix!(
            4.7, 0.6, 19.0,
            19.4, 6.444, -3.4,
            3.56, 22.0, 1.01
        );
        const M1: Matrix3<f32> = crate::Matrix!(
            1.0, 4.0, -3.3,
            0.0, 4.0, 1.0,
            1.0, 1.0, 0.5
        );

        #[test]
        fn identity() {
            let matrix: Matrix3<f64> = mat3::identity();
            assert_abs_diff_eq!(matrix.m11, 1.0);
            assert_abs_diff_eq!(matrix.m12, 0.0);
            assert_abs_diff_eq!(matrix.m21, 0.0);
            assert_abs_diff_eq!(matrix.m22, 1.0);
        }

        #[test]
        fn determinate() {
            let c = mat3::determinate(M0);
            assert_abs_diff_eq!(c, 18.646797);
        }

        #[test]
        fn trace() {
            let c = mat3::trace(M0);
            assert_abs_diff_eq!(c, 11.143999);
        }

        #[test]
        fn add() {
            let c = mat3::add(M0, M1);
            assert_abs_diff_eq!(c.m11, 5.7);
            assert_abs_diff_eq!(c.m12, 4.6);
            assert_abs_diff_eq!(c.m21, 19.4);
            assert_abs_diff_eq!(c.m22, 10.443999999999999);
        }

        #[test]
        fn sub() {
            let c = mat3::sub(M0, M1);
            assert_abs_diff_eq!(c.m11, 3.6999998);
            assert_abs_diff_eq!(c.m12, -3.4);
            assert_abs_diff_eq!(c.m21, 19.4);
            assert_abs_diff_eq!(c.m22, 2.4439998);
        }

        #[test]
        fn multiply() {
            let c = mat3::multiply(M0, M1);
            assert_abs_diff_eq!(c.m11, 4.7);
            assert_abs_diff_eq!(c.m12, 21.199999);
            assert_abs_diff_eq!(c.m21, 19.4);
            assert_abs_diff_eq!(c.m22, 103.37599999999999);
        }

        #[test]
        fn multiply_scalar() {
            let b = 3.125;
            let c = mat3::multiply_scalar(M0, b);
            assert_abs_diff_eq!(c.m11, 14.687499);
            assert_abs_diff_eq!(c.m12, 1.875);
            assert_abs_diff_eq!(c.m21, 60.62499999999999);
            assert_abs_diff_eq!(c.m22, 20.137499);
        }

        #[test]
        fn multiply_vector() {
            let b = Vector3 {
                x: 4.0,
                y: 77.7,
                z: 6.4
            };
            let c = mat3::multiply_vec(M0, b);
            assert_abs_diff_eq!(c.x, 65.42);
            assert_abs_diff_eq!(c.y, 578.29877);
            assert_abs_diff_eq!(c.z, 12.0);
        }

        #[test]
        fn pow() {
            let c = mat3::pow(M0, 7);
            assert_abs_diff_eq!(c.m11, 1934257.3);
            assert_abs_diff_eq!(c.m12, 438006.75);
            assert_abs_diff_eq!(c.m21, 14162217.0);
            assert_abs_diff_eq!(c.m22, 3207397.0);
        }

        #[test]
        fn transpose() {
            let c = mat3::transpose(M0);
            assert_abs_diff_eq!(c.m11, 4.7);
            assert_abs_diff_eq!(c.m12, 19.4);
            assert_abs_diff_eq!(c.m21, 0.6);
            assert_abs_diff_eq!(c.m22, 6.444);
        }

        #[test]
        fn inverse() {
            let c = mat3::inverse(M0);
            assert_abs_diff_eq!(c.m11, 0.34558208378917562259);
            assert_abs_diff_eq!(c.m12, -0.032177102773666259087);
            assert_abs_diff_eq!(c.m21, -1.0403929896818757104);
            assert_abs_diff_eq!(c.m22, 0.25205397172705236286);
        }
    };
    ( $feature:ident, f64 ) => {
        use approx::assert_abs_diff_eq;
        use crate::{ Matrix3, Vector3 };
        use crate::scalar::mat3;

        const M0: Matrix3<f64> = crate::Matrix!(
            4.7, 0.6, 19.0,
            19.4, 6.444, -3.4,
            3.56, 22.0, 1.01
        );
        const M1: Matrix3<f64> = crate::Matrix!(
            1.0, 4.0, -3.3,
            0.0, 4.0, 1.0,
            1.0, 1.0, 0.5
        );

        #[test]
        fn identity() {
            let matrix: Matrix3<f64> = mat3::identity();
            assert_abs_diff_eq!(matrix.m11, 1.0);
            assert_abs_diff_eq!(matrix.m12, 0.0);
            assert_abs_diff_eq!(matrix.m21, 0.0);
            assert_abs_diff_eq!(matrix.m22, 1.0);
        }

        #[test]
        fn determinate() {
            let c = mat3::determinate(M0);
            assert_abs_diff_eq!(c, 18.6468);
        }

        #[test]
        fn trace() {
            let c = mat3::trace(M0);
            assert_abs_diff_eq!(c, 11.144);
        }

        #[test]
        fn add() {
            let c = mat3::add(M0, M1);
            assert_abs_diff_eq!(c.m11, 5.7);
            assert_abs_diff_eq!(c.m12, 4.6);
            assert_abs_diff_eq!(c.m21, 19.4);
            assert_abs_diff_eq!(c.m22, 10.443999999999999);
        }

        #[test]
        fn sub() {
            let c = mat3::sub(M0, M1);
            assert_abs_diff_eq!(c.m11, 3.7);
            assert_abs_diff_eq!(c.m12, -3.4);
            assert_abs_diff_eq!(c.m21, 19.4);
            assert_abs_diff_eq!(c.m22, 2.444);
        }

        #[test]
        fn multiply() {
            let c = mat3::multiply(M0, M1);
            assert_abs_diff_eq!(c.m11, 4.7);
            assert_abs_diff_eq!(c.m12, 21.2);
            assert_abs_diff_eq!(c.m21, 19.4);
            assert_abs_diff_eq!(c.m22, 103.37599999999999);
        }

        #[test]
        fn multiply_scalar() {
            let b = 3.125;
            let c = mat3::multiply_scalar(M0, b);
            assert_abs_diff_eq!(c.m11, 14.6875);
            assert_abs_diff_eq!(c.m12, 1.875);
            assert_abs_diff_eq!(c.m21, 60.62499999999999);
            assert_abs_diff_eq!(c.m22, 20.1375);
        }

        #[test]
        fn multiply_vector() {
            let b = Vector3 {
                x: 4.0,
                y: 77.7,
                z: 6.4,
            };
            let c = mat3::multiply_vec(M0, b);
            assert_abs_diff_eq!(c.x, 65.42);
            assert_abs_diff_eq!(c.y, 578.2988);
            assert_abs_diff_eq!(c.z, 1.2);
        }

        #[test]
        fn pow() {
            let c = mat3::pow(M0, 7);
            assert_abs_diff_eq!(c.m11, 1934257.574745446);
            assert_abs_diff_eq!(c.m12, 438006.80663268637091);
            assert_abs_diff_eq!(c.m21, 14162220.081123523);
            assert_abs_diff_eq!(c.m22, 3207397.359357787976);
        }

        #[test]
        fn transpose() {
            let c = mat3::transpose(M0);
            assert_abs_diff_eq!(c.m11, 4.7);
            assert_abs_diff_eq!(c.m12, 19.4);
            assert_abs_diff_eq!(c.m21, 0.6);
            assert_abs_diff_eq!(c.m22, 6.444);
        }

        #[test]
        fn inverse() {
            let c = mat3::inverse(M0);
            assert_abs_diff_eq!(c.m11, 0.34558208378917562259);
            assert_abs_diff_eq!(c.m12, -0.032177102773666259087);
            assert_abs_diff_eq!(c.m21, -1.0403929896818757104);
            assert_abs_diff_eq!(c.m22, 0.25205397172705236286);
        }
    };
}
