pub mod simd {
    #[allow(non_camel_case_types)]
    #[repr(C)]
    #[repr(align(16))]
    pub struct f32x4(pub f32, pub f32, pub f32, pub f32);

    #[allow(non_camel_case_types)]
    pub enum Type {
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64
    }

    impl Type {
        pub fn convert_variable<T: SimdType, U: SimdType + Copy>(var: T) -> U {
            let var_ptr = &var as *const T;
            let var_ptr = var_ptr as *const std::ffi::c_void;
            match (T::to_type(), U::to_type()) {
                (Type::f64, Type::f64) |
                (Type::f32, Type::f32) |
                (Type::i64, Type::i64) |
                (Type::i32, Type::i32) |
                (Type::i16, Type::i16) |
                (Type::i8,  Type::i8)  |
                (Type::u64, Type::u64) |
                (Type::u32, Type::u32) |
                (Type::u16, Type::u16) |
                (Type::u8, Type::u8) => unsafe {
                    let dst_ptr: *const U = std::mem::transmute(var_ptr);
                    *dst_ptr
                },
                _ => {
                    panic!("Attempting to convert a variable which does not have same type");
                }
            }
        }
    }

    pub trait SimdType {
        fn to_type() -> Type;
    }

    impl SimdType for i8 { fn to_type() -> Type { Type::i8 } }
    impl SimdType for i16 { fn to_type() -> Type { Type::i16 } }
    impl SimdType for i32 { fn to_type() -> Type { Type::i32 } }
    impl SimdType for i64 { fn to_type() -> Type { Type::i64 } }
    impl SimdType for u8 { fn to_type() -> Type { Type::u8 } }
    impl SimdType for u16 { fn to_type() -> Type { Type::u16 } }
    impl SimdType for u32 { fn to_type() -> Type { Type::u32 } }
    impl SimdType for u64 { fn to_type() -> Type { Type::u64 } }
    impl SimdType for f32 { fn to_type() -> Type { Type::f32 } }
    impl SimdType for f64 { fn to_type() -> Type { Type::f64 } }
}

pub mod scalar;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2"
    )
)]

pub mod sse2;
pub mod matrix1;
pub mod matrix2;
pub mod matrix3;
pub mod matrix4;
pub mod vector1;
pub mod vector2;
pub mod vector3;
pub mod vector4;

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vector1<TElem> {
    pub x: TElem
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
pub struct Matrix1<TElem> {
    pub m11: TElem
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

