use std::ops::{ Add, Sub, Mul, Div };
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

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use crate::Vector3;
    use crate::scalar::vector3;

    #[test]
    fn addition() {
        let a = Vector3 {
            x: 5.0,
            y: 12.0,
            z: -3.5
        };
        let b = Vector3 {
            x: 3.0,
            y: 9.0,
            z: 0.8
        };
        let c = vector3::add(a, b);
        assert_abs_diff_eq!(c.x, 8.0);
        assert_abs_diff_eq!(c.y, 21.0);
        assert_abs_diff_eq!(c.z, -2.7);
    }

    #[test]
    fn sub() {
        let a = Vector3 {
            x: 5.0,
            y: 12.0,
            z: -3.5
        };
        let b = Vector3 {
            x: 3.0,
            y: 9.0,
            z: 0.8
        };
        let c = vector3::sub(b, a);
        assert_abs_diff_eq!(c.x, -2.0);
        assert_abs_diff_eq!(c.y, -3.0);
        assert_abs_diff_eq!(c.z, 4.3);
    }

    #[test]
    fn div_with_numerator() {
        let a = Vector3 {
            x: 5.0,
            y: 12.0,
            z: -3.5
        };
        let c = vector3::div_with_numerator(10.0, a);
        assert_abs_diff_eq!(c.x, 2.0);
        assert_abs_diff_eq!(c.y, 0.8333333333333334);
        assert_abs_diff_eq!(c.z, -2.857142857142857);
    }

    #[test]
    fn div_with_denominator() {
        let a = Vector3 {
            x: 5.0,
            y: 12.0,
            z: -3.5
        };
        let c = vector3::div_with_denominator(a, 10.0);
        assert_abs_diff_eq!(c.x, 0.5);
        assert_abs_diff_eq!(c.y, 1.2);
        assert_abs_diff_eq!(c.z, -0.35);
    }

    #[test]
    fn component_mul() {
        let a = Vector3 {
            x: 5.0,
            y: 12.0,
            z: -3.5
        };
        let b = Vector3 {
            x: 6.7,
            y: 8.7,
            z: 5.0
        };
        let c = vector3::component_mul(a, b);
        assert_abs_diff_eq!(c.x, 33.5);
        assert_abs_diff_eq!(c.y, 104.39999999999999);
        assert_abs_diff_eq!(c.z, -17.5);
    }

    #[test]
    fn dot() {
        let a = Vector3 {
            x: 5.0,
            y: 12.0,
            z: -3.5
        };
        let b = Vector3 {
            x: 6.7,
            y: 8.7,
            z: 5.0
        };
        let c = vector3::dot(a, b);
        assert_abs_diff_eq!(c, 120.39999999999998);
    }

    #[test]
    fn magnitude() {
        let a = Vector3 {
            x: 5.0,
            y: 12.0,
            z: -3.5
        };
        let c = vector3::magnitude(a);
        assert_abs_diff_eq!(c, 13.46291201783626);
    }

    #[test]
    fn magnitude_sqr() {
        let a = Vector3 {
            x: 5.0,
            y: 12.0,
            z: -3.5
        };
        let c = vector3::magnitude_sqr(a);
        assert_abs_diff_eq!(c, 181.25);
    }
}
