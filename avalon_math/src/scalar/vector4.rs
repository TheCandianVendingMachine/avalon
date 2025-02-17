use std::ops::{ Add, Sub, Mul, Div, Neg };
use crate::Vector4;
use crate::scalar::HasSqrt;

pub fn add<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T : Copy + Add<Output = U> {
    Vector4 {
        x: lhs.x.add(rhs.x),
        y: lhs.y.add(rhs.y),
        z: lhs.z.add(rhs.z),
        w: lhs.w.add(rhs.w),
    }
}

pub fn sub<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T : Copy + Sub<Output = U> {
    Vector4 {
        x: lhs.x.sub(rhs.x),
        y: lhs.y.sub(rhs.y),
        z: lhs.z.sub(rhs.z),
        w: lhs.w.sub(rhs.w),
    }
}

pub fn mul<U, T>(lhs: Vector4<T>, rhs: T) -> Vector4<U> where
    T : Copy + Mul<Output = U> {
    Vector4 {
        x: lhs.x.mul(rhs),
        y: lhs.y.mul(rhs),
        z: lhs.z.mul(rhs),
        w: lhs.w.mul(rhs),
    }
}

pub fn div_with_numerator<U, T>(lhs: T, rhs: Vector4<T>) -> Vector4<U> where
    T : Copy + Div<Output = U> {
    Vector4 {
        x: lhs.div(rhs.x),
        y: lhs.div(rhs.y),
        z: lhs.div(rhs.z),
        w: lhs.div(rhs.w),
    }
}

pub fn div_with_denominator<U, T>(lhs: Vector4<T>, rhs: T) -> Vector4<U> where
    T : Copy + Div<Output = U> {
    Vector4 {
        x: lhs.x.div(rhs),
        y: lhs.y.div(rhs),
        z: lhs.z.div(rhs),
        w: lhs.w.div(rhs),
    }
}

pub fn component_mul<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<U> where
    T : Copy + Mul<Output = U> {
    Vector4 {
        x: lhs.x.mul(rhs.x),
        y: lhs.y.mul(rhs.y),
        z: lhs.z.mul(rhs.z),
        w: lhs.w.mul(rhs.w),
    }
}

pub fn dot<U, T>(lhs: Vector4<T>, rhs: Vector4<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    lhs.x.mul(rhs.x) + lhs.y.mul(rhs.y) + lhs.z.mul(rhs.z) + lhs.w.mul(rhs.w)
}

pub fn magnitude<U, T>(vec: Vector4<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> + HasSqrt {
    U::sqrt(magnitude_sqr(vec))
}

pub fn magnitude_sqr<U, T>(vec: Vector4<T>) -> U where
    T : Copy + Mul<Output = U>,
    U: Add<Output = U> {
    dot(vec, vec)
}

pub fn negate<U, T>(vec: Vector4<T>) -> Vector4<U> where
    T: Neg<Output = U> {
    Vector4 {
        x: -vec.x,
        y: -vec.y,
        z: -vec.z,
        w: -vec.w
    }
}

pub fn normalize<T>(vec: Vector4<T>) -> Vector4<T> where
    T: Copy + HasSqrt + Add<Output = T> + Mul<Output = T> + Div<Output = T> {
    div_with_denominator(vec, magnitude(vec))
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use crate::Vector4;
    use crate::scalar::vector4;

    #[test]
    fn addition() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let b = Vector4 {
            x: 3.0,
            y: 9.0,
            z: 0.8,
            w: 7.0
        };
        let c = vector4::add(a, b);
        assert_abs_diff_eq!(c.x, 8.0);
        assert_abs_diff_eq!(c.y, 21.0);
        assert_abs_diff_eq!(c.z, -2.7);
        assert_abs_diff_eq!(c.w, 9.0);
    }

    #[test]
    fn sub() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let b = Vector4 {
            x: 3.0,
            y: 9.0,
            z: 0.8,
            w: 7.0
        };
        let c = vector4::sub(b, a);
        assert_abs_diff_eq!(c.x, -2.0);
        assert_abs_diff_eq!(c.y, -3.0);
        assert_abs_diff_eq!(c.z, 4.3);
        assert_abs_diff_eq!(c.w, 5.0);
    }

    #[test]
    fn mul() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: 0.8,
            w: 2.0
        };
        let b = 3.0;
        let c = vector4::mul(a, b);
        assert_abs_diff_eq!(c.x, 15.0);
        assert_abs_diff_eq!(c.y, 36.0);
        assert_abs_diff_eq!(c.z, 2.4000000000000004);
        assert_abs_diff_eq!(c.w, 6.0);
    }

    #[test]
    fn div_with_numerator() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let c = vector4::div_with_numerator(10.0, a);
        assert_abs_diff_eq!(c.x, 2.0);
        assert_abs_diff_eq!(c.y, 0.8333333333333334);
        assert_abs_diff_eq!(c.z, -2.857142857142857);
        assert_abs_diff_eq!(c.w, 5.0);
    }

    #[test]
    fn div_with_denominator() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let c = vector4::div_with_denominator(a, 10.0);
        assert_abs_diff_eq!(c.x, 0.5);
        assert_abs_diff_eq!(c.y, 1.2);
        assert_abs_diff_eq!(c.z, -0.35);
        assert_abs_diff_eq!(c.w, 0.2);
    }

    #[test]
    fn component_mul() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let b = Vector4 {
            x: 6.7,
            y: 8.7,
            z: 5.0,
            w: 3.4,
        };
        let c = vector4::component_mul(a, b);
        assert_abs_diff_eq!(c.x, 33.5);
        assert_abs_diff_eq!(c.y, 104.39999999999999);
        assert_abs_diff_eq!(c.z, -17.5);
        assert_abs_diff_eq!(c.w, 6.8);
    }

    #[test]
    fn dot() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let b = Vector4 {
            x: 6.7,
            y: 8.7,
            z: 5.0,
            w: 3.4
        };
        let c = vector4::dot(a, b);
        assert_abs_diff_eq!(c, 127.19999999999997);
    }

    #[test]
    fn magnitude() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let c = vector4::magnitude(a);
        assert_abs_diff_eq!(c, 13.6106575888162);
    }

    #[test]
    fn magnitude_sqr() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let c = vector4::magnitude_sqr(a);
        assert_abs_diff_eq!(c, 185.25);
    }

    #[test]
    fn neg() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let c = vector4::negate(a);
        assert_abs_diff_eq!(c.x, -5.0);
        assert_abs_diff_eq!(c.y, -12.0);
        assert_abs_diff_eq!(c.z, 3.5);
        assert_abs_diff_eq!(c.w, -2.0);
    }

    #[test]
    fn normalize() {
        let a = Vector4 {
            x: 5.0,
            y: 12.0,
            z: -3.5,
            w: 2.0
        };
        let c = vector4::normalize(a);
        assert_abs_diff_eq!(vector4::magnitude_sqr(c), 1.0);
        assert_abs_diff_eq!(c.x, 0.3673591791853225);
        assert_abs_diff_eq!(c.y, 0.8816620300447741);
        assert_abs_diff_eq!(c.z, -0.25715142542972574);
        assert_abs_diff_eq!(c.w, 0.146943671674129);
    }
}
