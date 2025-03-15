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
mod test_u8 {
    crate::matrix3_uint_tests!(scalar, u8);
}

#[cfg(test)]
mod test_u16 {
    crate::matrix3_uint_tests!(scalar, u16);
}

#[cfg(test)]
mod test_u32 {
    crate::matrix3_uint_tests!(scalar, u32);
}

#[cfg(test)]
mod test_u64 {
    crate::matrix3_uint_tests!(scalar, u64);
}

#[cfg(test)]
mod test_i8 {
    crate::matrix3_sint_tests!(scalar, i8);
}

#[cfg(test)]
mod test_i16 {
    crate::matrix3_sint_tests!(scalar, i16);
}

#[cfg(test)]
mod test_i32 {
    crate::matrix3_sint_tests!(scalar, i32);
}

#[cfg(test)]
mod test_i64 {
    crate::matrix3_sint_tests!(scalar, i64);
}

#[cfg(test)]
mod test_f32 {
    crate::matrix3_float_tests!(scalar, f32);
}

#[cfg(test)]
mod test_f64 {
    crate::matrix3_float_tests!(scalar, f64);
}
