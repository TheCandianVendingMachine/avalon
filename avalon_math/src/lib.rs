
pub mod scalar;
pub mod sse2;
pub mod vector1;
pub mod vector2;
pub mod vector3;
pub mod vector4;

#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Vector1<TElem> {
    pub x: TElem
}

#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Vector2<TElem> {
    pub x: TElem,
    pub y: TElem,
}

#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Vector3<TElem> {
    pub x: TElem,
    pub y: TElem,
    pub z: TElem,
}

#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Vector4<TElem> {
    pub x: TElem,
    pub y: TElem,
    pub z: TElem,
    pub w: TElem,
}

#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix1<TElem> {
    pub m11: TElem
}

#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix2<TElem> {
    pub m11: TElem,
    pub m12: TElem,
    pub m21: TElem,
    pub m22: TElem,
}

#[repr(align(8))]
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

#[repr(align(8))]
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

