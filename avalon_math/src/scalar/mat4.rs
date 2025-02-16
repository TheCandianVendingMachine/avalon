use std::ops::{ Add, Sub, Mul, Neg, Div };
use crate::{ Matrix4, Vector4 };
use crate::scalar::{ Inverse, Identity };

pub fn identity<T: Identity>() -> Matrix4<T> {
    Matrix4 {
        m11: T::identity(), m12: T::zero(),     m13: T::zero(),     m14: T::zero(),
        m21: T::zero(),     m22: T::identity(), m23: T::zero(),     m24: T::zero(),
        m31: T::zero(),     m32: T::zero(),     m33: T::identity(), m34: T::zero(),
        m41: T::zero(),     m42: T::zero(),     m43: T::zero(),     m44: T::identity(),
    }
}

pub fn determinate<T>(matrix: Matrix4<T>) -> T where
    T: Copy + Inverse + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
    {
    let a = matrix.m11;
    let b = matrix.m12;
    let c = matrix.m13;
    let d = matrix.m14;

    let w = crate::scalar::mat3::determinate(crate::Matrix3 {
        m11: matrix.m22, m12: matrix.m23, m13: matrix.m24,
        m21: matrix.m32, m22: matrix.m33, m23: matrix.m34,
        m31: matrix.m42, m32: matrix.m43, m33: matrix.m44,
    });

    let x = crate::scalar::mat3::determinate(crate::Matrix3 {
        m11: matrix.m21, m12: matrix.m23, m13: matrix.m24,
        m21: matrix.m31, m22: matrix.m33, m23: matrix.m34,
        m31: matrix.m41, m32: matrix.m43, m33: matrix.m44,
    });

    let y = crate::scalar::mat3::determinate(crate::Matrix3 {
        m11: matrix.m21, m12: matrix.m22, m13: matrix.m24,
        m21: matrix.m31, m22: matrix.m32, m23: matrix.m34,
        m31: matrix.m41, m32: matrix.m42, m33: matrix.m44,
    });

    let z = crate::scalar::mat3::determinate(crate::Matrix3 {
        m11: matrix.m21, m12: matrix.m22, m13: matrix.m23,
        m21: matrix.m31, m22: matrix.m32, m23: matrix.m33,
        m31: matrix.m41, m32: matrix.m42, m33: matrix.m43,
    });

    a * w - b * x + c * y - d * z
}

pub fn trace<T>(matrix: Matrix4<T>) -> T where
    T: Copy + Add<Output = T> {
    matrix.m11 + matrix.m22 + matrix.m33 + matrix.m44
}

pub fn add<U, T>(lhs: Matrix4<T>, rhs: Matrix4<T>) -> Matrix4<U> where
    T: Copy + Add<Output = U> {
    Matrix4 {
        m11: lhs.m11 + rhs.m11,
        m12: lhs.m12 + rhs.m12,
        m13: lhs.m13 + rhs.m13,
        m14: lhs.m14 + rhs.m14,
        m21: lhs.m21 + rhs.m21,
        m22: lhs.m22 + rhs.m22,
        m23: lhs.m23 + rhs.m23,
        m24: lhs.m24 + rhs.m24,
        m31: lhs.m31 + rhs.m31,
        m32: lhs.m32 + rhs.m32,
        m33: lhs.m33 + rhs.m33,
        m34: lhs.m34 + rhs.m34,
        m41: lhs.m41 + rhs.m41,
        m42: lhs.m42 + rhs.m42,
        m43: lhs.m43 + rhs.m43,
        m44: lhs.m44 + rhs.m44,
    }
}

pub fn sub<U, T>(lhs: Matrix4<T>, rhs: Matrix4<T>) -> Matrix4<U> where
    T: Copy + Sub<Output = U> {
    Matrix4 {
        m11: lhs.m11 - rhs.m11,
        m12: lhs.m12 - rhs.m12,
        m13: lhs.m13 - rhs.m13,
        m14: lhs.m14 - rhs.m14,
        m21: lhs.m21 - rhs.m21,
        m22: lhs.m22 - rhs.m22,
        m23: lhs.m23 - rhs.m23,
        m24: lhs.m24 - rhs.m24,
        m31: lhs.m31 - rhs.m31,
        m32: lhs.m32 - rhs.m32,
        m33: lhs.m33 - rhs.m33,
        m34: lhs.m34 - rhs.m34,
        m41: lhs.m41 - rhs.m41,
        m42: lhs.m42 - rhs.m42,
        m43: lhs.m43 - rhs.m43,
        m44: lhs.m44 - rhs.m44,
    }
}

pub fn multiply<U, T>(lhs: Matrix4<T>, rhs: Matrix4<T>) -> Matrix4<U> where
    T: Copy + Mul<Output = U>,
    U: Add<Output = U> {
    Matrix4 {
        m11: lhs.m11 * rhs.m11 + lhs.m12 * rhs.m21 + lhs.m13 * rhs.m31 + lhs.m14 * rhs.m41,
        m12: lhs.m11 * rhs.m12 + lhs.m12 * rhs.m22 + lhs.m13 * rhs.m32 + lhs.m14 * rhs.m42,
        m13: lhs.m11 * rhs.m13 + lhs.m12 * rhs.m23 + lhs.m13 * rhs.m33 + lhs.m14 * rhs.m43,
        m14: lhs.m11 * rhs.m14 + lhs.m12 * rhs.m24 + lhs.m13 * rhs.m34 + lhs.m14 * rhs.m44,

        m21: lhs.m21 * rhs.m11 + lhs.m22 * rhs.m21 + lhs.m23 * rhs.m31 + lhs.m24 * rhs.m41,
        m22: lhs.m21 * rhs.m12 + lhs.m22 * rhs.m22 + lhs.m23 * rhs.m32 + lhs.m24 * rhs.m42,
        m23: lhs.m21 * rhs.m13 + lhs.m22 * rhs.m23 + lhs.m23 * rhs.m33 + lhs.m24 * rhs.m43,
        m24: lhs.m21 * rhs.m14 + lhs.m22 * rhs.m24 + lhs.m23 * rhs.m34 + lhs.m24 * rhs.m44,

        m31: lhs.m31 * rhs.m11 + lhs.m32 * rhs.m21 + lhs.m33 * rhs.m31 + lhs.m34 * rhs.m41,
        m32: lhs.m31 * rhs.m12 + lhs.m32 * rhs.m22 + lhs.m33 * rhs.m32 + lhs.m34 * rhs.m42,
        m33: lhs.m31 * rhs.m13 + lhs.m32 * rhs.m23 + lhs.m33 * rhs.m33 + lhs.m34 * rhs.m43,
        m34: lhs.m31 * rhs.m14 + lhs.m32 * rhs.m24 + lhs.m33 * rhs.m34 + lhs.m34 * rhs.m44,

        m41: lhs.m41 * rhs.m11 + lhs.m42 * rhs.m21 + lhs.m43 * rhs.m31 + lhs.m44 * rhs.m41,
        m42: lhs.m41 * rhs.m12 + lhs.m42 * rhs.m22 + lhs.m43 * rhs.m32 + lhs.m44 * rhs.m42,
        m43: lhs.m41 * rhs.m13 + lhs.m42 * rhs.m23 + lhs.m43 * rhs.m33 + lhs.m44 * rhs.m43,
        m44: lhs.m41 * rhs.m14 + lhs.m42 * rhs.m24 + lhs.m43 * rhs.m34 + lhs.m44 * rhs.m44,
    }
}

pub fn multiply_scalar<U, T>(lhs: Matrix4<T>, rhs: T) -> Matrix4<U> where
    T: Copy + Mul<Output = U> {
    Matrix4 {
        m11: lhs.m11 * rhs,
        m12: lhs.m12 * rhs,
        m13: lhs.m13 * rhs,
        m14: lhs.m14 * rhs,
        m21: lhs.m21 * rhs,
        m22: lhs.m22 * rhs,
        m23: lhs.m23 * rhs,
        m24: lhs.m24 * rhs,
        m31: lhs.m31 * rhs,
        m32: lhs.m32 * rhs,
        m33: lhs.m33 * rhs,
        m34: lhs.m34 * rhs,
        m41: lhs.m41 * rhs,
        m42: lhs.m42 * rhs,
        m43: lhs.m43 * rhs,
        m44: lhs.m44 * rhs,
    }
}

pub fn multiply_vec<U, T>(lhs: Matrix4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T: Copy + Mul<Output = U>,
    U: Add<Output = U> {
    Vector4 {
        x: lhs.m11 * rhs.x + lhs.m12 * rhs.y + lhs.m13 * rhs.z + lhs.m14 * rhs.w,
        y: lhs.m21 * rhs.x + lhs.m22 * rhs.y + lhs.m23 * rhs.z + lhs.m24 * rhs.w,
        z: lhs.m31 * rhs.x + lhs.m32 * rhs.y + lhs.m33 * rhs.z + lhs.m34 * rhs.w,
        w: lhs.m41 * rhs.x + lhs.m42 * rhs.y + lhs.m43 * rhs.z + lhs.m44 * rhs.w,
    }
}

pub fn pow<T>(matrix: Matrix4<T>, power: u64) -> Matrix4<T> where
    T: Copy + Mul<Output = T> + Add<Output = T> + Identity {
    if power == 0 {
        identity()
    } else if power == 1 {
        matrix
    } else if power % 2 == 0{
        pow(multiply(matrix, matrix), power / 2)
    } else {
        multiply(matrix, pow(multiply(matrix, matrix), (power - 1) / 2))
    }
}

pub fn transpose<T>(matrix: Matrix4<T>) -> Matrix4<T> {
    Matrix4 {
        m11: matrix.m11, m12: matrix.m21, m13: matrix.m31, m14: matrix.m41,
        m21: matrix.m12, m22: matrix.m22, m23: matrix.m32, m24: matrix.m42,
        m31: matrix.m13, m32: matrix.m23, m33: matrix.m33, m34: matrix.m43,
        m41: matrix.m14, m42: matrix.m24, m43: matrix.m34, m44: matrix.m44,
    }
}

pub fn inverse<T>(matrix: Matrix4<T>) -> Matrix4<T> where
    T: Copy + Inverse + Identity + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Neg<Output = T> + Div<f64, Output = T> {
    let i_det = determinate(matrix).inverse();
    let m2 = pow(matrix, 2);
    let m3 = multiply(matrix, m2);
    let tr1 = trace(matrix);
    let tr2 = trace(m2);
    let tr3 = trace(m3);

    let a = tr1 * tr1 * tr1 / 6.0 - tr1 * tr2 / 2.0 + tr3 / 3.0;
    let b = (tr1 * tr1 - tr2) / 2.0;
    let c = tr1;

    let x = multiply_scalar(identity(), a);
    let y = multiply_scalar(matrix, b);
    let z = multiply_scalar(m2, c);

    let inverted = sub(x, y);
    let inverted = add(inverted, z);
    let inverted = sub(inverted, m3);

    multiply_scalar(inverted, i_det)
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use crate::{ Matrix4, Vector4 };
    use crate::scalar::mat4;

    #[test]
    fn identity() {
        let matrix: Matrix4<f64> = mat4::identity();
        assert_abs_diff_eq!(matrix.m11, 1.0);
        assert_abs_diff_eq!(matrix.m12, 0.0);
        assert_abs_diff_eq!(matrix.m13, 0.0);
        assert_abs_diff_eq!(matrix.m13, 0.0);
        assert_abs_diff_eq!(matrix.m21, 0.0);
        assert_abs_diff_eq!(matrix.m22, 1.0);
        assert_abs_diff_eq!(matrix.m23, 0.0);
        assert_abs_diff_eq!(matrix.m24, 0.0);
        assert_abs_diff_eq!(matrix.m31, 0.0);
        assert_abs_diff_eq!(matrix.m32, 0.0);
        assert_abs_diff_eq!(matrix.m33, 1.0);
        assert_abs_diff_eq!(matrix.m34, 0.0);
        assert_abs_diff_eq!(matrix.m41, 0.0);
        assert_abs_diff_eq!(matrix.m42, 0.0);
        assert_abs_diff_eq!(matrix.m43, 0.0);
        assert_abs_diff_eq!(matrix.m44, 1.0);
    }

    #[test]
    fn determinate() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let c = mat4::determinate(a);
        assert_abs_diff_eq!(c, 211210084.8514032);
    }

    #[test]
    fn trace() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let c = mat4::trace(a);
        assert_abs_diff_eq!(c, 120351.144);
    }

    #[test]
    fn add() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let b = Matrix4 {
            m11: 1.0,
            m12: 4.0,
            m13: 4.0,
            m14: 1.0,
            m21: 0.0,
            m22: 4.0,
            m23: 0.0,
            m24: 1.0,
            m31: 9.0,
            m32: 8.0,
            m33: 5.0,
            m34: 1.0,
            m41: 8.0,
            m42: 4.0,
            m43: 4.0,
            m44: 5.0,
        };
        let c = mat4::add(a, b);
        assert_abs_diff_eq!(c.m11, 5.7);
        assert_abs_diff_eq!(c.m12, 4.6);
        assert_abs_diff_eq!(c.m13, 12.1);
        assert_abs_diff_eq!(c.m14, 3.0);
        assert_abs_diff_eq!(c.m21, 19.4);
        assert_abs_diff_eq!(c.m22, 10.443999999999999);
        assert_abs_diff_eq!(c.m23, 0.0);
        assert_abs_diff_eq!(c.m24, 1.4);
        assert_abs_diff_eq!(c.m31, 9.05);
        assert_abs_diff_eq!(c.m32, 18.0);
        assert_abs_diff_eq!(c.m33, 15.0);
        assert_abs_diff_eq!(c.m34, 1.0);
        assert_abs_diff_eq!(c.m41, 8.9);
        assert_abs_diff_eq!(c.m42, 4.03);
        assert_abs_diff_eq!(c.m43, 4.0001);
        assert_abs_diff_eq!(c.m44, 120335.0);
    }

    #[test]
    fn sub() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let b = Matrix4 {
            m11: 1.0,
            m12: 4.0,
            m13: 4.0,
            m14: 1.0,
            m21: 0.0,
            m22: 4.0,
            m23: 0.0,
            m24: 1.0,
            m31: 9.0,
            m32: 8.0,
            m33: 5.0,
            m34: 1.0,
            m41: 8.0,
            m42: 4.0,
            m43: 4.0,
            m44: 5.0,
        };
        let c = mat4::sub(a, b);
        assert_abs_diff_eq!(c.m11, 3.7);
        assert_abs_diff_eq!(c.m12, -3.4);
        assert_abs_diff_eq!(c.m13, 4.1);
        assert_abs_diff_eq!(c.m14, 1.0);
        assert_abs_diff_eq!(c.m21, 19.4);
        assert_abs_diff_eq!(c.m22, 2.444);
        assert_abs_diff_eq!(c.m23, 0.0);
        assert_abs_diff_eq!(c.m24, -0.6);
        assert_abs_diff_eq!(c.m31, -8.95);
        assert_abs_diff_eq!(c.m32, 2.0);
        assert_abs_diff_eq!(c.m33, 5.0);
        assert_abs_diff_eq!(c.m34, -1.0);
        assert_abs_diff_eq!(c.m41, -7.1);
        assert_abs_diff_eq!(c.m42, -3.97);
        assert_abs_diff_eq!(c.m43, -3.9999);
        assert_abs_diff_eq!(c.m44, 120325.0);
    }

    #[test]
    fn multiply() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let b = Matrix4 {
            m11: 1.0,
            m12: 4.0,
            m13: 4.0,
            m14: 1.0,
            m21: 0.0,
            m22: 4.0,
            m23: 0.0,
            m24: 1.0,
            m31: 9.0,
            m32: 8.0,
            m33: 5.0,
            m34: 1.0,
            m41: 8.0,
            m42: 4.0,
            m43: 4.0,
            m44: 5.0,
        };
        let c = mat4::multiply(a, b);
        assert_abs_diff_eq!(c.m11, 93.6);
        assert_abs_diff_eq!(c.m12, 94.0);
        assert_abs_diff_eq!(c.m13, 67.3);
        assert_abs_diff_eq!(c.m14, 23.4);
        assert_abs_diff_eq!(c.m21, 22.599999999999998);
        assert_abs_diff_eq!(c.m22, 104.97599999999998);
        assert_abs_diff_eq!(c.m23, 79.19999999999999);
        assert_abs_diff_eq!(c.m24, 27.843999999999998);
        assert_abs_diff_eq!(c.m31, 90.05);
        assert_abs_diff_eq!(c.m32, 120.2);
        assert_abs_diff_eq!(c.m33, 50.2);
        assert_abs_diff_eq!(c.m34, 20.05);
        assert_abs_diff_eq!(c.m41, 962640.9009);
        assert_abs_diff_eq!(c.m42, 481323.7208);
        assert_abs_diff_eq!(c.m43, 481323.6005);
        assert_abs_diff_eq!(c.m44, 601650.9301);
    }

    #[test]
    fn multiply_scalar() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let b = 3.125;
        let c = mat4::multiply_scalar(a, b);
        assert_abs_diff_eq!(c.m11, 14.6875);
        assert_abs_diff_eq!(c.m12, 1.875);
        assert_abs_diff_eq!(c.m13, 25.3125);
        assert_abs_diff_eq!(c.m14, 6.25);
        assert_abs_diff_eq!(c.m21, 60.62499999999999);
        assert_abs_diff_eq!(c.m22, 20.1375);
        assert_abs_diff_eq!(c.m23, 0.0);
        assert_abs_diff_eq!(c.m24, 1.25);
        assert_abs_diff_eq!(c.m31, 0.15625);
        assert_abs_diff_eq!(c.m32, 31.25);
        assert_abs_diff_eq!(c.m33, 31.25);
        assert_abs_diff_eq!(c.m34, 0.0);
        assert_abs_diff_eq!(c.m41, 2.8125);
        assert_abs_diff_eq!(c.m42, 0.09375);
        assert_abs_diff_eq!(c.m43, 0.0003125);
        assert_abs_diff_eq!(c.m44, 376031.25);
    }

    #[test]
    fn multiply_vector() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let b = Vector4 {
            x: 4.0,
            y: 77.7,
            z: 0.00322,
            w: 1.5
        };
        let c = mat4::multiply_vec(a, b);
        assert_abs_diff_eq!(c.x, 68.446082);
        assert_abs_diff_eq!(c.y, 578.8988);
        assert_abs_diff_eq!(c.z, 777.2322);
        assert_abs_diff_eq!(c.w, 180500.931000322);
    }

    #[test]
    fn pow() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let c = mat4::pow(a, 2);
        assert_abs_diff_eq!(c.m11, 35.935);
        assert_abs_diff_eq!(c.m12, 87.7464);
        assert_abs_diff_eq!(c.m13, 119.0702);
        assert_abs_diff_eq!(c.m14, 240669.64);
        assert_abs_diff_eq!(c.m21, 216.55360000000002);
        assert_abs_diff_eq!(c.m22, 53.177136);
        assert_abs_diff_eq!(c.m23, 157.14004);
        assert_abs_diff_eq!(c.m24, 48173.3776);
        assert_abs_diff_eq!(c.m31, 194.735);
        assert_abs_diff_eq!(c.m32, 164.47);
        assert_abs_diff_eq!(c.m33, 100.405);
        assert_abs_diff_eq!(c.m34, 4.1);
        assert_abs_diff_eq!(c.m41, 108301.812005);
        assert_abs_diff_eq!(c.m42, 3610.63432);
        assert_abs_diff_eq!(c.m43, 19.324);
        assert_abs_diff_eq!(c.m44, 14479308901.812);
    }

    #[test]
    fn transpose() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let c = mat4::transpose(a);
        assert_abs_diff_eq!(c.m11, 4.7);
        assert_abs_diff_eq!(c.m12, 19.4);
        assert_abs_diff_eq!(c.m13, 0.05);
        assert_abs_diff_eq!(c.m14, 0.9);
        assert_abs_diff_eq!(c.m21, 0.6);
        assert_abs_diff_eq!(c.m22, 6.444);
        assert_abs_diff_eq!(c.m23, 10.0);
        assert_abs_diff_eq!(c.m24, 0.03);
        assert_abs_diff_eq!(c.m31, 8.1);
        assert_abs_diff_eq!(c.m32, 0.0);
        assert_abs_diff_eq!(c.m33, 10.0);
        assert_abs_diff_eq!(c.m34, 0.0001);
        assert_abs_diff_eq!(c.m41, 2.0);
        assert_abs_diff_eq!(c.m42, 0.4);
        assert_abs_diff_eq!(c.m43, 0.0);
        assert_abs_diff_eq!(c.m44, 120330.0);
    }

    #[test]
    fn inverse() {
        let a = Matrix4 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m14: 2.0,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m24: 0.4,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
            m34: 0.0,
            m41: 0.9,
            m42: 0.03,
            m43: 0.0001,
            m44: 120330.0,
        };
        let c = mat4::inverse(a);
        assert_abs_diff_eq!(c.m11, 0.03671257034845203);
        assert_abs_diff_eq!(c.m12, 0.042728786385126255759);
        assert_abs_diff_eq!(c.m13, -0.02973718190565867);
        assert_abs_diff_eq!(c.m14, -7.522368128391939e-7);
        assert_abs_diff_eq!(c.m21, -0.11052510308126179);
        assert_abs_diff_eq!(c.m22, 0.02654588379131693);
        assert_abs_diff_eq!(c.m23, 0.089525333478328814141);
        assert_abs_diff_eq!(c.m24, 1.7487895986197027e-6);
        assert_abs_diff_eq!(c.m31, 0.11034154022993928233);
        assert_abs_diff_eq!(c.m32, -0.026759527637977593);
        assert_abs_diff_eq!(c.m33, 0.010623352516466624);
        assert_abs_diff_eq!(c.m34, -0.0000017450284168926193504);
        assert_abs_diff_eq!(c.m41, -0.00000024712535775326271738);
        assert_abs_diff_eq!(c.m42, -0.00000032618306577770539103);
        assert_abs_diff_eq!(c.m43, 0.00000020008843493307856558);
        assert_abs_diff_eq!(c.m44, 8.310445977212242e-6);
    }
}
