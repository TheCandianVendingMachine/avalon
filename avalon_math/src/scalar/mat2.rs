use std::ops::{ Add, Sub, Mul, Neg };
use crate::{ Matrix2, Vector2 };
use crate::scalar::{ Inverse, Identity };

pub fn identity<T: Identity>() -> Matrix2<T> {
    Matrix2 {
        m11: T::identity(), m12: T::zero(),
        m21: T::zero(),     m22: T::identity()
    }
}

pub fn determinate<T>(matrix: Matrix2<T>) -> T where
    T: Copy + Inverse + Mul<Output = T> + Sub<Output = T>,
    {
    matrix.m11 * matrix.m22 - matrix.m21 * matrix.m12
}

pub fn trace<T>(matrix: Matrix2<T>) -> T where
    T: Copy + Add<Output = T> {
    matrix.m11 + matrix.m22
}

pub fn add<U, T>(lhs: Matrix2<T>, rhs: Matrix2<T>) -> Matrix2<U> where
    T: Copy + Add<Output = U> {
    Matrix2 {
        m11: lhs.m11 + rhs.m11,
        m12: lhs.m12 + rhs.m12,
        m21: lhs.m21 + rhs.m21,
        m22: lhs.m22 + rhs.m22,
    }
}

pub fn sub<U, T>(lhs: Matrix2<T>, rhs: Matrix2<T>) -> Matrix2<U> where
    T: Copy + Sub<Output = U> {
    Matrix2 {
        m11: lhs.m11 - rhs.m11,
        m12: lhs.m12 - rhs.m12,
        m21: lhs.m21 - rhs.m21,
        m22: lhs.m22 - rhs.m22,
    }
}

pub fn multiply<U, T>(lhs: Matrix2<T>, rhs: Matrix2<T>) -> Matrix2<U> where
    T: Copy + Mul<Output = U>,
    U: Add<Output = U> {
    Matrix2 {
        m11: lhs.m11 * rhs.m11 + lhs.m12 * rhs.m21,
        m12: lhs.m11 * rhs.m12 + lhs.m12 * rhs.m22,
        m21: lhs.m21 * rhs.m11 + lhs.m22 * rhs.m21,
        m22: lhs.m21 * rhs.m12 + lhs.m22 * rhs.m22,
    }
}

pub fn multiply_scalar<U, T>(lhs: Matrix2<T>, rhs: T) -> Matrix2<U> where
    T: Copy + Mul<Output = U> {
    Matrix2 {
        m11: lhs.m11 * rhs,
        m12: lhs.m12 * rhs,
        m21: lhs.m21 * rhs,
        m22: lhs.m22 * rhs,
    }
}

pub fn multiply_vec<U, T>(lhs: Matrix2<T>, rhs: Vector2<T>) -> Vector2<U> where
    T: Copy + Mul<Output = U>,
    U: Add<Output = U> {
    Vector2 {
        x: lhs.m11 * rhs.x + lhs.m12 * rhs.y,
        y: lhs.m21 * rhs.x + lhs.m22 * rhs.y,
    }
}

pub fn pow<T>(matrix: Matrix2<T>, power: u64) -> Matrix2<T> where
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

pub fn transpose<T>(matrix: Matrix2<T>) -> Matrix2<T> {
    Matrix2 {
        m11: matrix.m11, m12: matrix.m21,
        m21: matrix.m12, m22: matrix.m22,
    }
}

pub fn inverse<T>(matrix: Matrix2<T>) -> Matrix2<T> where
    T: Copy + Inverse + Mul<Output = T> + Sub<Output = T> + Neg<Output = T> {
    let i_det = determinate(matrix).inverse();
    Matrix2 {
        m11: i_det * matrix.m22, m12: i_det * -matrix.m12,
        m21: i_det * -matrix.m21, m22: i_det * matrix.m11,
    }
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use crate::{ Matrix2, Vector2 };
    use crate::scalar::mat2;

    #[test]
    fn identity() {
        let matrix: Matrix2<f64> = mat2::identity();
        assert_abs_diff_eq!(matrix.m11, 1.0);
        assert_abs_diff_eq!(matrix.m12, 0.0);
        assert_abs_diff_eq!(matrix.m21, 0.0);
        assert_abs_diff_eq!(matrix.m22, 1.0);
    }

    #[test]
    fn determinate() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let c = mat2::determinate(a);
        assert_abs_diff_eq!(c, 18.6468);
    }

    #[test]
    fn trace() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let c = mat2::trace(a);
        assert_abs_diff_eq!(c, 11.144);
    }

    #[test]
    fn add() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let b = Matrix2 {
            m11: 1.0,
            m12: 4.0,
            m21: 0.0,
            m22: 4.0
        };
        let c = mat2::add(a, b);
        assert_abs_diff_eq!(c.m11, 5.7);
        assert_abs_diff_eq!(c.m12, 4.6);
        assert_abs_diff_eq!(c.m21, 19.4);
        assert_abs_diff_eq!(c.m22, 10.443999999999999);
    }

    #[test]
    fn sub() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let b = Matrix2 {
            m11: 1.0,
            m12: 4.0,
            m21: 0.0,
            m22: 4.0
        };
        let c = mat2::sub(b, a);
        assert_abs_diff_eq!(c.m11, -3.7);
        assert_abs_diff_eq!(c.m12, 3.4);
        assert_abs_diff_eq!(c.m21, -19.4);
        assert_abs_diff_eq!(c.m22, -2.444);
    }

    #[test]
    fn multiply() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let b = Matrix2 {
            m11: 1.0,
            m12: 4.0,
            m21: 0.0,
            m22: 4.0
        };
        let c = mat2::multiply(a, b);
        assert_abs_diff_eq!(c.m11, 4.7);
        assert_abs_diff_eq!(c.m12, 21.2);
        assert_abs_diff_eq!(c.m21, 19.4);
        assert_abs_diff_eq!(c.m22, 103.37599999999999);
    }

    #[test]
    fn multiply_scalar() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let b = 3.125;
        let c = mat2::multiply_scalar(a, b);
        assert_abs_diff_eq!(c.m11, 14.6875);
        assert_abs_diff_eq!(c.m12, 1.875);
        assert_abs_diff_eq!(c.m21, 60.62499999999999);
        assert_abs_diff_eq!(c.m22, 20.1375);
    }

    #[test]
    fn multiply_vector() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let b = Vector2 {
            x: 4.0,
            y: 77.7
        };
        let c = mat2::multiply_vec(a, b);
        assert_abs_diff_eq!(c.x, 65.42);
        assert_abs_diff_eq!(c.y, 578.2988);
    }

    #[test]
    fn pow() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let c = mat2::pow(a, 7);
        assert_abs_diff_eq!(c.m11, 1934257.574745446);
        assert_abs_diff_eq!(c.m12, 438006.80663268637091);
        assert_abs_diff_eq!(c.m21, 14162220.081123523);
        assert_abs_diff_eq!(c.m22, 3207397.359357787976);
    }

    #[test]
    fn transpose() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let c = mat2::transpose(a);
        assert_abs_diff_eq!(c.m11, 4.7);
        assert_abs_diff_eq!(c.m12, 19.4);
        assert_abs_diff_eq!(c.m21, 0.6);
        assert_abs_diff_eq!(c.m22, 6.444);
    }

    #[test]
    fn inverse() {
        let a = Matrix2 {
            m11: 4.7,
            m12: 0.6,
            m21: 19.4,
            m22: 6.444,
        };
        let c = mat2::inverse(a);
        assert_abs_diff_eq!(c.m11, 0.34558208378917562259);
        assert_abs_diff_eq!(c.m12, -0.032177102773666259087);
        assert_abs_diff_eq!(c.m21, -1.0403929896818757104);
        assert_abs_diff_eq!(c.m22, 0.25205397172705236286);
    }
}
