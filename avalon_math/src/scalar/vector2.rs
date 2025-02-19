use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::Vector2;
use crate::scalar::HasSqrt;

pub fn add<U, T>(lhs: Vector2<T>, rhs: Vector2<T>) -> Vector2<U> where
    T : Copy + Add<Output = U> {
    Vector2 {
        x: lhs.x.add(rhs.x),
        y: lhs.y.add(rhs.y),
    }
}

pub fn sub<U, T>(lhs: Vector2<T>, rhs: Vector2<T>) -> Vector2<U> where
    T : Copy + Sub<Output = U> {
    Vector2 {
        x: lhs.x.sub(rhs.x),
        y: lhs.y.sub(rhs.y),
    }
}

pub fn div_with_numerator<U, T>(lhs: T, rhs: Vector2<T>) -> Vector2<U> where
    T : Copy + Div<Output = U> {
    Vector2 {
        x: lhs.div(rhs.x),
        y: lhs.div(rhs.y),
    }
}

pub fn div_with_denominator<U, T>(lhs: Vector2<T>, rhs: T) -> Vector2<U> where
    T : Copy + Div<Output = U> {
    Vector2 {
        x: lhs.x.div(rhs),
        y: lhs.y.div(rhs),
    }
}

pub fn mul<U, T>(lhs: Vector2<T>, rhs: T) -> Vector2<U> where
    T : Copy + Mul<Output = U> {
    Vector2 {
        x: lhs.x.mul(rhs),
        y: lhs.y.mul(rhs)
    }
}

pub fn component_mul<U, T>(lhs: Vector2<T>, rhs: Vector2<T>) -> Vector2<U> where
    T : Copy + Mul<Output = U> {
    Vector2 {
        x: lhs.x.mul(rhs.x),
        y: lhs.y.mul(rhs.y),
    }
}

pub fn dot<U, T>(lhs: Vector2<T>, rhs: Vector2<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    lhs.x.mul(rhs.x) + lhs.y.mul(rhs.y)
}

pub fn magnitude<U, T>(vec: Vector2<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> + HasSqrt {
    U::sqrt(magnitude_sqr(vec))
}

pub fn magnitude_sqr<U, T>(vec: Vector2<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    dot(vec, vec)
}

pub fn negate<U, T>(vec: Vector2<T>) -> Vector2<U> where
    T: Neg<Output = U> {
    Vector2 {
        x: -vec.x,
        y: -vec.y
    }
}

pub fn normalize<T>(vec: Vector2<T>) -> Vector2<T> where
    T: Copy + HasSqrt + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
    div_with_denominator(vec, magnitude(vec))
}

pub fn project<T>(lhs: Vector2<T>, rhs: Vector2<T>) -> Vector2<T> where
    T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
    let numerator = mul(rhs, dot(lhs, rhs));
    let denominator = dot(rhs, rhs);
    numerator / denominator
}

#[cfg(test)]
mod test_u8 {
    crate::vector2_uint_tests!(scalar, u8);
}

#[cfg(test)]
mod test_u16 {
    crate::vector2_uint_tests!(scalar, u16);
}

#[cfg(test)]
mod test_u32 {
    crate::vector2_uint_tests!(scalar, u32);
}

#[cfg(test)]
mod test_u64 {
    crate::vector2_uint_tests!(scalar, u64);
}

#[cfg(test)]
mod test_i8 {
    crate::vector2_sint_tests!(scalar, i8);
}

#[cfg(test)]
mod test_i16 {
    crate::vector2_sint_tests!(scalar, i16);
}

#[cfg(test)]
mod test_i32 {
    crate::vector2_sint_tests!(scalar, i32);
}

#[cfg(test)]
mod test_i64 {
    crate::vector2_sint_tests!(scalar, i64);
}

#[cfg(test)]
mod test_f32 {
    crate::vector2_float_tests!(scalar, f32);
}

#[cfg(test)]
mod test_f64 {
    crate::vector2_float_tests!(scalar, f64);
}
