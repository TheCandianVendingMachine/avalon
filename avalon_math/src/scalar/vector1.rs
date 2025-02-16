use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::Vector1;

pub fn add<U, T>(lhs: Vector1<T>, rhs: Vector1<T>) -> Vector1<U> where
    T : Copy + Add<Output = U> {
    Vector1 { x: lhs.x.add(rhs.x) }
}

pub fn sub<U, T>(lhs: Vector1<T>, rhs: Vector1<T>) -> Vector1<U> where
    T : Copy + Sub<Output = U> {
    Vector1 { x: lhs.x.sub(rhs.x) }
}

pub fn div_with_numerator<U, T>(lhs: T, rhs: Vector1<T>) -> Vector1<U> where
    T : Copy + Div<Output = U> {
    Vector1 { x: lhs.div(rhs.x) }
}

pub fn div_with_denominator<U, T>(lhs: Vector1<T>, rhs: T) -> Vector1<U> where
    T : Copy + Div<Output = U> {
    Vector1 { x: lhs.x.div(rhs) }
}

pub fn mul<U, T>(lhs: Vector1<T>, rhs: T) -> Vector1<U> where
    T : Copy + Mul<Output = U> {
    Vector1 { x: lhs.x.mul(rhs) }
}

pub fn component_mul<U, T>(lhs: Vector1<T>, rhs: Vector1<T>) -> Vector1<U> where
    T : Copy + Mul<Output = U> {
    Vector1 { x: lhs.x.mul(rhs.x) }
}

pub fn dot<U, T>(lhs: Vector1<T>, rhs: Vector1<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    lhs.x.mul(rhs.x)
}

pub fn magnitude<T: Copy>(vec: Vector1<T>) -> T {
    vec.x
}

pub fn magnitude_sqr<U, T>(vec: Vector1<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    dot(vec, vec)
}

pub fn negate<U, T>(vec: Vector1<T>) -> Vector1<U> where
    T: Neg<Output = U> {
    Vector1 {
        x: -vec.x
    }
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use crate::Vector1;
    use crate::scalar::vector1;

    #[test]
    fn addition() {
        let a = Vector1 {
            x: 5.0
        };
        let b = Vector1 {
            x: 3.0
        };
        let c = vector1::add(a, b);
        assert_abs_diff_eq!(c.x, 8.0);
    }

    #[test]
    fn sub() {
        let a = Vector1 {
            x: 5.0
        };
        let b = Vector1 {
            x: 3.0
        };
        let c = vector1::sub(b, a);
        assert_abs_diff_eq!(c.x, -2.0);
    }

    #[test]
    fn mul() {
        let a = Vector1 {
            x: 5.0
        };
        let b = 3.0;
        let c = vector1::mul(a, b);
        assert_abs_diff_eq!(c.x, 15.0);
    }

    #[test]
    fn div_with_numerator() {
        let a = Vector1 {
            x: 5.0
        };
        let c = vector1::div_with_numerator(10.0, a);
        assert_abs_diff_eq!(c.x, 2.0);
    }

    #[test]
    fn div_with_denominator() {
        let a = Vector1 {
            x: 5.0
        };
        let c = vector1::div_with_denominator(a, 10.0);
        assert_abs_diff_eq!(c.x, 0.5);
    }

    #[test]
    fn component_mul() {
        let a = Vector1 {
            x: 5.0
        };
        let b = Vector1 {
            x: 6.7
        };
        let c = vector1::component_mul(a, b);
        assert_abs_diff_eq!(c.x, 33.5);
    }

    #[test]
    fn dot() {
        let a = Vector1 {
            x: 5.0
        };
        let b = Vector1 {
            x: 6.7
        };
        let c = vector1::dot(a, b);
        assert_abs_diff_eq!(c, 33.5);
    }

    #[test]
    fn magnitude() {
        let a = Vector1 {
            x: 5.0
        };
        let c = vector1::magnitude(a);
        assert_abs_diff_eq!(c, 5.0);
    }

    #[test]
    fn magnitude_sqr() {
        let a = Vector1 {
            x: 5.0
        };
        let c = vector1::magnitude_sqr(a);
        assert_abs_diff_eq!(c, 25.0);
    }

    #[test]
    fn neg() {
        let a = Vector1 {
            x: 5.0,
        };
        let c = vector1::negate(a);
        assert_abs_diff_eq!(c.x, -5.0);
    }
}
