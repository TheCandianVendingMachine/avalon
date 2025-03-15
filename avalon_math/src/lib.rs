mod boilerplate;

pub mod simd;
pub mod scalar;
pub mod sse2;
pub mod sse42;
pub mod avx;

pub mod matrix2;
pub mod matrix3;
pub mod matrix4;
pub mod vector2;
pub mod vector3;
pub mod vector4;

#[macro_export]
macro_rules! Vector {
    ( $x:expr, $y: expr ) => {
        Vector2 {
            x: $x,
            y: $y,
        }
    };
    ( $x:expr, $y: expr, $z: expr ) => {
        Vector3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
    ( $x:expr, $y: expr, $z: expr, $w: expr ) => {
        Vector4 {
            x: $x,
            y: $y,
            z: $z,
            w: $w
        }
    };
}

#[macro_export]
macro_rules! Matrix {
    (
        $c1r1:expr, $c2r1:expr,
        $c1r2:expr, $c2r2:expr
    ) => {
        Matrix2 {
            m11: $c1r1, m12: $c2r1,
            m21: $c1r2, m22: $c2r2
        }
    };
    (
        $c1r1:expr, $c2r1:expr, $c3r1:expr,
        $c1r2:expr, $c2r2:expr, $c3r2:expr,
        $c1r3:expr, $c2r3:expr, $c3r3:expr
    ) => {
        Matrix3 {
            m11: $c1r1, m12: $c2r1, m13: $c3r1,
            m21: $c1r2, m22: $c2r2, m23: $c3r2,
            m31: $c1r3, m32: $c2r3, m33: $c3r3,
        }
    };
    (
        $c1r1:expr, $c2r1:expr, $c3r1:expr, $c4r1:expr,
        $c1r2:expr, $c2r2:expr, $c3r2:expr, $c4r2:expr,
        $c1r3:expr, $c2r3:expr, $c3r3:expr, $c4r3:expr,
        $c1r4:expr, $c2r4:expr, $c3r4:expr, $c4r4:expr
    ) => {
        Matrix4 {
            m11: $c1r1, m12: $c2r1, m13: $c3r1, m14: $c4r1,
            m21: $c1r2, m22: $c2r2, m23: $c3r2, m24: $c4r2,
            m31: $c1r3, m32: $c2r3, m33: $c3r3, m34: $c4r3,
            m41: $c1r4, m42: $c2r4, m43: $c3r4, m44: $c4r4,
        }
    };
}

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vector2<TElem> {
    pub x: TElem,
    pub y: TElem,
}

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vector3<TElem> {
    pub x: TElem,
    pub y: TElem,
    pub z: TElem,
}

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vector4<TElem> {
    pub x: TElem,
    pub y: TElem,
    pub z: TElem,
    pub w: TElem,
}

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix2<TElem> {
    pub m11: TElem,
    pub m12: TElem,
    pub m21: TElem,
    pub m22: TElem,
}

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix3<TElem> {
    pub m11: TElem,
    pub m12: TElem,
    pub m13: TElem,
    pub m21: TElem,
    pub m22: TElem,
    pub m23: TElem,
    pub m31: TElem,
    pub m32: TElem,
    pub m33: TElem,
}

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix4<TElem> {
    pub m11: TElem,
    pub m12: TElem,
    pub m13: TElem,
    pub m14: TElem,
    pub m21: TElem,
    pub m22: TElem,
    pub m23: TElem,
    pub m24: TElem,
    pub m31: TElem,
    pub m32: TElem,
    pub m33: TElem,
    pub m34: TElem,
    pub m41: TElem,
    pub m42: TElem,
    pub m43: TElem,
    pub m44: TElem,
}

