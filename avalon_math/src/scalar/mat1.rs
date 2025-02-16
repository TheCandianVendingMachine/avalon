use std::ops::{ Add, Sub, Mul };
use crate::{ Matrix1, Vector1 };
use crate::scalar::{ Inverse, Identity };

pub fn identity<T: Identity>() -> Matrix1<T> {
    Matrix1 {
        m11: T::identity()
    }
}

pub fn determinate<T>(matrix: Matrix1<T>) -> T where
    T: Copy + Inverse {
    T::inverse(matrix.m11)
}

pub fn trace<T>(matrix: Matrix1<T>) -> T where
    T: Copy {
    matrix.m11
}

pub fn add<U, T>(lhs: Matrix1<T>, rhs: Matrix1<T>) -> Matrix1<U> where
    T: Copy + Add<Output = U> {
    Matrix1 {
        m11: lhs.m11 + rhs.m11
    }
}

pub fn sub<U, T>(lhs: Matrix1<T>, rhs: Matrix1<T>) -> Matrix1<U> where
    T: Copy + Sub<Output = U> {
    Matrix1 {
        m11: lhs.m11 - rhs.m11
    }
}

pub fn multiply<U, T>(lhs: Matrix1<T>, rhs: Matrix1<T>) -> Matrix1<U> where
    T: Copy + Mul<Output = U> {
    Matrix1 {
        m11: lhs.m11 * rhs.m11
    }
}

pub fn multiply_scalar<U, T>(lhs: Matrix1<T>, rhs: T) -> Matrix1<U> where
    T: Copy + Mul<Output = U> {
    Matrix1 {
        m11: lhs.m11 * rhs
    }
}

pub fn multiply_vec<U, T>(lhs: Matrix1<T>, rhs: Vector1<T>) -> Vector1<U> where
    T: Copy + Mul<Output = U> {
    Vector1 {
        x: lhs.m11 * rhs.x
    }
}

pub fn pow<T>(matrix: Matrix1<T>, power: u64) -> Matrix1<T> where
    T: Copy + Mul<Output = T> + Identity {
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

pub fn transpose<T>(matrix: Matrix1<T>) -> Matrix1<T> {
    Matrix1 {
        m11: matrix.m11,
    }
}

pub fn inverse<T>(matrix: Matrix1<T>) -> Matrix1<T> where
    T: Inverse {
    Matrix1 {
        m11: matrix.m11.inverse(),
    }
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use crate::{ Matrix1, Vector1 };
    use crate::scalar::mat1;

    #[test]
    fn identity() {
        let matrix: Matrix1<f64> = mat1::identity();
        assert_abs_diff_eq!(matrix.m11, 1.0);
    }

    #[test]
    fn determinate() {
        let a = Matrix1 {
            m11: 4.7
        };
        let c = mat1::determinate(a);
        assert_abs_diff_eq!(c, 1.0 / 4.7);
    }

    #[test]
    fn trace() {
        let a = Matrix1 {
            m11: 4.7
        };
        let c = mat1::trace(a);
        assert_abs_diff_eq!(c, 4.7);
    }

    #[test]
    fn add() {
        let a = Matrix1 {
            m11: 4.7
        };
        let b = Matrix1 {
            m11: 1.0
        };
        let c = mat1::add(a, b);
        assert_abs_diff_eq!(c.m11, 5.7);
    }

    #[test]
    fn sub() {
        let a = Matrix1 {
            m11: 4.7
        };
        let b = Matrix1 {
            m11: 1.0
        };
        let c = mat1::sub(b, a);
        assert_abs_diff_eq!(c.m11, -3.7);
    }

    #[test]
    fn multiply() {
        let a = Matrix1 {
            m11: 4.7
        };
        let b = Matrix1 {
            m11: 0.5
        };
        let c = mat1::multiply(a, b);
        assert_abs_diff_eq!(c.m11, 2.35);
    }

    #[test]
    fn multiply_scalar() {
        let a = Matrix1 {
            m11: 4.7
        };
        let b = 3.125;
        let c = mat1::multiply_scalar(a, b);
        assert_abs_diff_eq!(c.m11, 14.6875);
    }

    #[test]
    fn multiply_vector() {
        let a = Matrix1 {
            m11: 4.7
        };
        let b = Vector1 {
            x: 4.0
        };
        let c = mat1::multiply_vec(a, b);
        assert_abs_diff_eq!(c.x, 18.8);
    }

    #[test]
    fn pow() {
        let a = Matrix1 {
            m11: 4.7
        };
        let c = mat1::pow(a, 7);
        assert_abs_diff_eq!(c.m11, 50662.31204630003);
    }

    #[test]
    fn transpose() {
        let a = Matrix1 {
            m11: 4.7
        };
        let c = mat1::transpose(a);
        assert_abs_diff_eq!(c.m11, 4.7);
    }

    #[test]
    fn inverse() {
        let a = Matrix1 {
            m11: 4.7
        };
        let c = mat1::inverse(a);
        assert_abs_diff_eq!(c.m11, 1.0 / 4.7);
    }
}
