use std::ops::{ Add, Sub, Mul, Neg };
use crate::{ Matrix3, Vector3 };
use crate::scalar::{ Inverse, Identity };

pub fn identity<T: Identity>() -> Matrix3<T> {
    Matrix3 {
        m11: T::identity(), m12: T::zero(),     m13: T::zero(),
        m21: T::zero(),     m22: T::identity(), m23: T::zero(),
        m31: T::zero(),     m32: T::zero(),     m33: T::identity(),
    }
}

pub fn determinate<T>(matrix: Matrix3<T>) -> T where
    T: Copy + Inverse + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
    {
    matrix.m11 * matrix.m22 * matrix.m33 +
    matrix.m12 * matrix.m23 * matrix.m31 +
    matrix.m13 * matrix.m21 * matrix.m32 -
    matrix.m13 * matrix.m22 * matrix.m31 -
    matrix.m12 * matrix.m21 * matrix.m33 -
    matrix.m11 * matrix.m23 * matrix.m32
}

pub fn trace<T>(matrix: Matrix3<T>) -> T where
    T: Copy + Add<Output = T> {
    matrix.m11 + matrix.m22 + matrix.m33
}

pub fn add<U, T>(lhs: Matrix3<T>, rhs: Matrix3<T>) -> Matrix3<U> where
    T: Copy + Add<Output = U> {
    Matrix3 {
        m11: lhs.m11 + rhs.m11,
        m12: lhs.m12 + rhs.m12,
        m13: lhs.m13 + rhs.m13,
        m21: lhs.m21 + rhs.m21,
        m22: lhs.m22 + rhs.m22,
        m23: lhs.m23 + rhs.m23,
        m31: lhs.m31 + rhs.m31,
        m32: lhs.m32 + rhs.m32,
        m33: lhs.m33 + rhs.m33,
    }
}

pub fn sub<U, T>(lhs: Matrix3<T>, rhs: Matrix3<T>) -> Matrix3<U> where
    T: Copy + Sub<Output = U> {
    Matrix3 {
        m11: lhs.m11 - rhs.m11,
        m12: lhs.m12 - rhs.m12,
        m13: lhs.m13 - rhs.m13,
        m21: lhs.m21 - rhs.m21,
        m22: lhs.m22 - rhs.m22,
        m23: lhs.m23 - rhs.m23,
        m31: lhs.m31 - rhs.m31,
        m32: lhs.m32 - rhs.m32,
        m33: lhs.m33 - rhs.m33,
    }
}

pub fn multiply<U, T>(lhs: Matrix3<T>, rhs: Matrix3<T>) -> Matrix3<U> where
    T: Copy + Mul<Output = U>,
    U: Add<Output = U> {
    Matrix3 {
        m11: lhs.m11 * rhs.m11 + lhs.m12 * rhs.m21 + lhs.m13 * rhs.m31,
        m12: lhs.m11 * rhs.m12 + lhs.m12 * rhs.m22 + lhs.m13 * rhs.m32,
        m13: lhs.m11 * rhs.m13 + lhs.m12 * rhs.m23 + lhs.m13 * rhs.m33,

        m21: lhs.m21 * rhs.m11 + lhs.m22 * rhs.m21 + lhs.m23 * rhs.m31,
        m22: lhs.m21 * rhs.m12 + lhs.m22 * rhs.m22 + lhs.m23 * rhs.m32,
        m23: lhs.m21 * rhs.m13 + lhs.m22 * rhs.m23 + lhs.m23 * rhs.m33,

        m31: lhs.m31 * rhs.m11 + lhs.m32 * rhs.m21 + lhs.m33 * rhs.m31,
        m32: lhs.m31 * rhs.m12 + lhs.m32 * rhs.m22 + lhs.m33 * rhs.m32,
        m33: lhs.m31 * rhs.m13 + lhs.m32 * rhs.m23 + lhs.m33 * rhs.m33,
    }
}

pub fn multiply_scalar<U, T>(lhs: Matrix3<T>, rhs: T) -> Matrix3<U> where
    T: Copy + Mul<Output = U> {
    Matrix3 {
        m11: lhs.m11 * rhs,
        m12: lhs.m12 * rhs,
        m13: lhs.m13 * rhs,
        m21: lhs.m21 * rhs,
        m22: lhs.m22 * rhs,
        m23: lhs.m23 * rhs,
        m31: lhs.m31 * rhs,
        m32: lhs.m32 * rhs,
        m33: lhs.m33 * rhs,
    }
}

pub fn multiply_vec<U, T>(lhs: Matrix3<T>, rhs: Vector3<T>) -> Vector3<U> where
    T: Copy + Mul<Output = U>,
    U: Add<Output = U> {
    Vector3 {
        x: lhs.m11 * rhs.x + lhs.m12 * rhs.y + lhs.m13 * rhs.z,
        y: lhs.m21 * rhs.x + lhs.m22 * rhs.y + lhs.m23 * rhs.z,
        z: lhs.m31 * rhs.x + lhs.m32 * rhs.y + lhs.m33 * rhs.z,
    }
}

pub fn pow<T>(matrix: Matrix3<T>, power: u64) -> Matrix3<T> where
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

pub fn transpose<T>(matrix: Matrix3<T>) -> Matrix3<T> {
    Matrix3 {
        m11: matrix.m11, m12: matrix.m21, m13: matrix.m31,
        m21: matrix.m12, m22: matrix.m22, m23: matrix.m32,
        m31: matrix.m13, m32: matrix.m23, m33: matrix.m33
    }
}

pub fn inverse<T>(matrix: Matrix3<T>) -> Matrix3<T> where
    T: Copy + Inverse + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Neg<Output = T> {
    let i_det = determinate(matrix).inverse();
    Matrix3 {
        m11: i_det *  (matrix.m22 * matrix.m33 - matrix.m23 * matrix.m32),
        m12: i_det * -(matrix.m12 * matrix.m33 - matrix.m13 * matrix.m32),
        m13: i_det *  (matrix.m12 * matrix.m23 - matrix.m13 * matrix.m22),

        m21: i_det * -(matrix.m21 * matrix.m33 - matrix.m23 * matrix.m31),
        m22: i_det *  (matrix.m11 * matrix.m33 - matrix.m13 * matrix.m31),
        m23: i_det * -(matrix.m11 * matrix.m23 - matrix.m13 * matrix.m21),

        m31: i_det *  (matrix.m21 * matrix.m32 - matrix.m22 * matrix.m31),
        m32: i_det * -(matrix.m11 * matrix.m32 - matrix.m12 * matrix.m31),
        m33: i_det *  (matrix.m11 * matrix.m22 - matrix.m12 * matrix.m21),
    }
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use crate::{ Matrix3, Vector3 };
    use crate::scalar::mat3;

    #[test]
    fn identity() {
        let matrix: Matrix3<f64> = mat3::identity();
        assert_abs_diff_eq!(matrix.m11, 1.0);
        assert_abs_diff_eq!(matrix.m12, 0.0);
        assert_abs_diff_eq!(matrix.m13, 0.0);
        assert_abs_diff_eq!(matrix.m21, 0.0);
        assert_abs_diff_eq!(matrix.m22, 1.0);
        assert_abs_diff_eq!(matrix.m23, 0.0);
        assert_abs_diff_eq!(matrix.m31, 0.0);
        assert_abs_diff_eq!(matrix.m32, 0.0);
        assert_abs_diff_eq!(matrix.m33, 1.0);
    }

    #[test]
    fn determinate() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let c = mat3::determinate(a);
        assert_abs_diff_eq!(c, 1755.2581799999998);
    }

    #[test]
    fn trace() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let c = mat3::trace(a);
        assert_abs_diff_eq!(c, 21.144);
    }

    #[test]
    fn add() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let b = Matrix3 {
            m11: 1.0,
            m12: 4.0,
            m13: 4.0,
            m21: 0.0,
            m22: 4.0,
            m23: 0.0,
            m31: 9.0,
            m32: 8.0,
            m33: 5.0,
        };
        let c = mat3::add(a, b);
        assert_abs_diff_eq!(c.m11, 5.7);
        assert_abs_diff_eq!(c.m12, 4.6);
        assert_abs_diff_eq!(c.m13, 12.1);
        assert_abs_diff_eq!(c.m21, 19.4);
        assert_abs_diff_eq!(c.m22, 10.443999999999999);
        assert_abs_diff_eq!(c.m23, 0.0);
        assert_abs_diff_eq!(c.m31, 9.05);
        assert_abs_diff_eq!(c.m32, 18.0);
        assert_abs_diff_eq!(c.m33, 15.0);
    }

    #[test]
    fn sub() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let b = Matrix3 {
            m11: 1.0,
            m12: 4.0,
            m13: 4.0,
            m21: 0.0,
            m22: 4.0,
            m23: 0.0,
            m31: 9.0,
            m32: 8.0,
            m33: 5.0,
        };
        let c = mat3::sub(a, b);
        assert_abs_diff_eq!(c.m11, 3.7);
        assert_abs_diff_eq!(c.m12, -3.4);
        assert_abs_diff_eq!(c.m13, 4.1);
        assert_abs_diff_eq!(c.m21, 19.4);
        assert_abs_diff_eq!(c.m22, 2.444);
        assert_abs_diff_eq!(c.m23, 0.0);
        assert_abs_diff_eq!(c.m31, -8.95);
        assert_abs_diff_eq!(c.m32, 2.0);
        assert_abs_diff_eq!(c.m33, 5.0);
    }

    #[test]
    fn multiply() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let b = Matrix3 {
            m11: 1.0,
            m12: 4.0,
            m13: 4.0,
            m21: 0.0,
            m22: 4.0,
            m23: 0.0,
            m31: 9.0,
            m32: 8.0,
            m33: 5.0,
        };
        let c = mat3::multiply(a, b);
        assert_abs_diff_eq!(c.m11, 77.6);
        assert_abs_diff_eq!(c.m12, 86.0);
        assert_abs_diff_eq!(c.m13, 59.3);
        assert_abs_diff_eq!(c.m21, 19.4);
        assert_abs_diff_eq!(c.m22, 103.37599999999999);
        assert_abs_diff_eq!(c.m23, 77.6);
        assert_abs_diff_eq!(c.m31, 90.05);
        assert_abs_diff_eq!(c.m32, 120.2);
        assert_abs_diff_eq!(c.m33, 50.2);
    }

    #[test]
    fn multiply_scalar() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let b = 3.125;
        let c = mat3::multiply_scalar(a, b);
        assert_abs_diff_eq!(c.m11, 14.6875);
        assert_abs_diff_eq!(c.m12, 1.875);
        assert_abs_diff_eq!(c.m13, 25.3125);
        assert_abs_diff_eq!(c.m21, 60.62499999999999);
        assert_abs_diff_eq!(c.m22, 20.1375);
        assert_abs_diff_eq!(c.m23, 0.0);
        assert_abs_diff_eq!(c.m31, 0.15625);
        assert_abs_diff_eq!(c.m32, 31.25);
        assert_abs_diff_eq!(c.m33, 31.25);
    }

    #[test]
    fn multiply_vector() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let b = Vector3 {
            x: 4.0,
            y: 77.7,
            z: 0.00322
        };
        let c = mat3::multiply_vec(a, b);
        assert_abs_diff_eq!(c.x, 65.446082);
        assert_abs_diff_eq!(c.y, 578.2988);
        assert_abs_diff_eq!(c.z, 777.2322);
    }

    #[test]
    fn pow() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let c = mat3::pow(a, 3);
        assert_abs_diff_eq!(c.m11, 1867.50416);
        assert_abs_diff_eq!(c.m12, 1776.2321616);
        assert_abs_diff_eq!(c.m13, 1467.1934999999999);
        assert_abs_diff_eq!(c.m21, 2055.3705584);
        assert_abs_diff_eq!(c.m22, 2043.7122963839997);
        assert_abs_diff_eq!(c.m23, 3322.56816);
        assert_abs_diff_eq!(c.m31, 4110.99275);
        assert_abs_diff_eq!(c.m32, 2180.73568);
        assert_abs_diff_eq!(c.m33, 2581.4035);
    }

    #[test]
    fn transpose() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let c = mat3::transpose(a);
        assert_abs_diff_eq!(c.m11, 4.7);
        assert_abs_diff_eq!(c.m12, 19.4);
        assert_abs_diff_eq!(c.m13, 0.05);
        assert_abs_diff_eq!(c.m21, 0.6);
        assert_abs_diff_eq!(c.m22, 6.444);
        assert_abs_diff_eq!(c.m23, 10.0);
        assert_abs_diff_eq!(c.m31, 8.1);
        assert_abs_diff_eq!(c.m32, 0.0);
        assert_abs_diff_eq!(c.m33, 10.0);
    }

    #[test]
    fn inverse() {
        let a = Matrix3 {
            m11: 4.7,
            m12: 0.6,
            m13: 8.1,
            m21: 19.4,
            m22: 6.444,
            m23: 0.0,
            m31: 0.05,
            m32: 10.0,
            m33: 10.0,
        };
        let c = mat3::inverse(a);
        assert_abs_diff_eq!(c.m11, 0.03671254789423627703);
        assert_abs_diff_eq!(c.m12, 0.042728756860144642649);
        assert_abs_diff_eq!(c.m13, -0.029737163794331384343);
        assert_abs_diff_eq!(c.m21, -0.11052505107824080909);
        assert_abs_diff_eq!(c.m22, 0.02654595234531252834);
        assert_abs_diff_eq!(c.m23, 0.089525291373375055287);
        assert_abs_diff_eq!(c.m31, 0.11034148833876962759);
        assert_abs_diff_eq!(c.m32, -0.026759596129613251539);
        assert_abs_diff_eq!(c.m33, 0.010623394445596601636);
    }
}
