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
    T: Copy + Mul<Output = T> + Sub<Output = T>,
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
mod test_u8 {
    crate::matrix2_uint_tests!(scalar, u8);
}

#[cfg(test)]
mod test_u16 {
    crate::matrix2_uint_tests!(scalar, u16);
}

#[cfg(test)]
mod test_u32 {
    crate::matrix2_uint_tests!(scalar, u32);
}

#[cfg(test)]
mod test_u64 {
    crate::matrix2_uint_tests!(scalar, u64);
}

#[cfg(test)]
mod test_i8 {
    crate::matrix2_sint_tests!(scalar, i8);
}

#[cfg(test)]
mod test_i16 {
    crate::matrix2_sint_tests!(scalar, i16);
}

#[cfg(test)]
mod test_i32 {
    crate::matrix2_sint_tests!(scalar, i32);
}

#[cfg(test)]
mod test_i64 {
    crate::matrix2_sint_tests!(scalar, i64);
}

#[cfg(test)]
mod test_f32 {
    crate::matrix2_float_tests!(scalar, f32);
}

#[cfg(test)]
mod test_f64 {
    crate::matrix2_float_tests!(scalar, f64);
}
