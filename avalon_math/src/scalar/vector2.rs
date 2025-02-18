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
    let numerator = dot(lhs, rhs);
    let denominator = dot(rhs, rhs);
    mul(rhs, numerator / denominator)
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use crate::Vector2;
    use crate::scalar::vector2;

    #[test]
    fn addition() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0
        };
        let b = Vector2 {
            x: 3.0,
            y: 9.0
        };
        let c = vector2::add(a, b);
        assert_abs_diff_eq!(c.x, 8.0);
        assert_abs_diff_eq!(c.y, 21.0);
    }

    #[test]
    fn sub() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0
        };
        let b = Vector2 {
            x: 3.0,
            y: 9.0
        };
        let c = vector2::sub(b, a);
        assert_abs_diff_eq!(c.x, -2.0);
        assert_abs_diff_eq!(c.y, -3.0);
    }

    #[test]
    fn mul() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0
        };
        let b = 3.0;
        let c = vector2::mul(a, b);
        assert_abs_diff_eq!(c.x, 15.0);
        assert_abs_diff_eq!(c.y, 36.0);
    }

    #[test]
    fn div_with_numerator() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0
        };
        let c = vector2::div_with_numerator(10.0, a);
        assert_abs_diff_eq!(c.x, 2.0);
        assert_abs_diff_eq!(c.y, 0.8333333333333334);
    }

    #[test]
    fn div_with_denominator() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0
        };
        let c = vector2::div_with_denominator(a, 10.0);
        assert_abs_diff_eq!(c.x, 0.5);
        assert_abs_diff_eq!(c.y, 1.2);
    }

    #[test]
    fn component_mul() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0
        };
        let b = Vector2 {
            x: 6.7,
            y: 8.7,
        };
        let c = vector2::component_mul(a, b);
        assert_abs_diff_eq!(c.x, 33.5);
        assert_abs_diff_eq!(c.y, 104.39999999999999);
    }

    #[test]
    fn dot() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0
        };
        let b = Vector2 {
            x: 6.7,
            y: 8.7
        };
        let c = vector2::dot(a, b);
        assert_abs_diff_eq!(c, 137.89999999999998);
    }

    #[test]
    fn magnitude() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0
        };
        let c = vector2::magnitude(a);
        assert_abs_diff_eq!(c, 13.0);
    }

    #[test]
    fn magnitude_sqr() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0
        };
        let c = vector2::magnitude_sqr(a);
        assert_abs_diff_eq!(c, 169.0);
    }

    #[test]
    fn neg() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0,
        };
        let c = vector2::negate(a);
        assert_abs_diff_eq!(c.x, -5.0);
        assert_abs_diff_eq!(c.y, -12.0);
    }

    #[test]
    fn normalize() {
        let a = Vector2 {
            x: 5.0,
            y: 12.0,
        };
        let c = vector2::normalize(a);
        assert_abs_diff_eq!(vector2::magnitude_sqr(c), 1.0);
        assert_abs_diff_eq!(c.x, 0.38461538461538464);
        assert_abs_diff_eq!(c.y, 0.9230769230769231);
    }
}
