use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::Vector3;
use crate::scalar::HasSqrt;

pub fn add<U, T>(lhs: Vector3<T>, rhs: Vector3<T>) -> Vector3<U> where
    T : Copy + Add<Output = U> {
    Vector3 {
        x: lhs.x.add(rhs.x),
        y: lhs.y.add(rhs.y),
        z: lhs.z.add(rhs.z),
    }
}

pub fn sub<U, T>(lhs: Vector3<T>, rhs: Vector3<T>) -> Vector3<U> where
    T : Copy + Sub<Output = U> {
    Vector3 {
        x: lhs.x.sub(rhs.x),
        y: lhs.y.sub(rhs.y),
        z: lhs.z.sub(rhs.z),
    }
}

pub fn div_with_numerator<U, T>(lhs: T, rhs: Vector3<T>) -> Vector3<U> where
    T : Copy + Div<Output = U> {
    Vector3 {
        x: lhs.div(rhs.x),
        y: lhs.div(rhs.y),
        z: lhs.div(rhs.z),
    }
}

pub fn div_with_denominator<U, T>(lhs: Vector3<T>, rhs: T) -> Vector3<U> where
    T : Copy + Div<Output = U> {
    Vector3 {
        x: lhs.x.div(rhs),
        y: lhs.y.div(rhs),
        z: lhs.z.div(rhs),
    }
}

pub fn mul<U, T>(lhs: Vector3<T>, rhs: T) -> Vector3<U> where
    T : Copy + Mul<Output = U> {
    Vector3 {
        x: lhs.x.mul(rhs),
        y: lhs.y.mul(rhs),
        z: lhs.z.mul(rhs),
    }
}

pub fn component_mul<U, T>(lhs: Vector3<T>, rhs: Vector3<T>) -> Vector3<U> where
    T : Copy + Mul<Output = U> {
    Vector3 {
        x: lhs.x.mul(rhs.x),
        y: lhs.y.mul(rhs.y),
        z: lhs.z.mul(rhs.z),
    }
}

pub fn dot<U, T>(lhs: Vector3<T>, rhs: Vector3<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    lhs.x.mul(rhs.x) + lhs.y.mul(rhs.y) + lhs.z.mul(rhs.z)
}

pub fn magnitude<U, T>(vec: Vector3<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> + HasSqrt {
    U::sqrt(magnitude_sqr(vec))
}

pub fn magnitude_sqr<U, T>(vec: Vector3<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    dot(vec, vec)
}

pub fn negate<U, T>(vec: Vector3<T>) -> Vector3<U> where
    T: Neg<Output = U> {
    Vector3 {
        x: -vec.x,
        y: -vec.y,
        z: -vec.z,
    }
}

pub fn normalize<T>(vec: Vector3<T>) -> Vector3<T> where
    T: Copy + HasSqrt + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
    div_with_denominator(vec, magnitude(vec))
}

pub fn project<T>(lhs: Vector3<T>, rhs: Vector3<T>) -> Vector3<T> where
    T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
    let numerator = mul(rhs, dot(lhs, rhs));
    let denominator = dot(rhs, rhs);
    numerator / denominator
}

#[cfg(test)]
mod test_u8 {
    crate::vector3_uint_tests!(scalar, u8);
}

#[cfg(test)]
mod test_u16 {
    crate::vector3_uint_tests!(scalar, u16);
}

#[cfg(test)]
mod test_u32 {
    crate::vector3_uint_tests!(scalar, u32);
}

#[cfg(test)]
mod test_u64 {
    crate::vector3_uint_tests!(scalar, u64);
}

#[cfg(test)]
mod test_i8 {
    crate::vector3_sint_tests!(scalar, i8);
}

#[cfg(test)]
mod test_i16 {
    crate::vector3_sint_tests!(scalar, i16);
}

#[cfg(test)]
mod test_i32 {
    crate::vector3_sint_tests!(scalar, i32);
}

#[cfg(test)]
mod test_i64 {
    crate::vector3_sint_tests!(scalar, i64);
}

#[cfg(test)]
mod test_f32 {
    crate::vector3_float_tests!(scalar, f32);
}

#[cfg(test)]
mod test_f64 {
    crate::vector3_float_tests!(scalar, f64);
}
