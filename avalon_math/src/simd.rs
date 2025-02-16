use crate::{ Vector4, Vector3, Vector2, Vector1 };
#[allow(non_camel_case_types)]
#[repr(C)]
#[repr(align(16))]
pub struct f32x4(pub f32, pub f32, pub f32, pub f32);

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        if T::to_type() != U::to_type() {
            panic!("Attempting to convert a variable which does not have same type");
        }
        let var_ptr = &var as *const T;
        let var_ptr = var_ptr as *const std::ffi::c_void;
        unsafe {
            let dst_ptr: *const U = std::mem::transmute(var_ptr);
            *dst_ptr
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

impl<T: SimdType> From<Vector4<T>> for f32x4 {
    fn from(vec: Vector4<T>) -> f32x4 {
        f32x4(
            Type::convert_variable(vec.w),
            Type::convert_variable(vec.z),
            Type::convert_variable(vec.y),
            Type::convert_variable(vec.x),
        )
    }
}

impl<T: Copy + SimdType> From<f32x4> for Vector4<T> {
    fn from(pack: f32x4) -> Vector4<T> {
        Vector4 {
            x: Type::convert_variable(pack.3),
            y: Type::convert_variable(pack.2),
            z: Type::convert_variable(pack.1),
            w: Type::convert_variable(pack.0),
        }
    }
}

impl<T: SimdType> From<Vector3<T>> for f32x4 {
    fn from(vec: Vector3<T>) -> f32x4 {
        f32x4(
            0.0,
            Type::convert_variable(vec.z),
            Type::convert_variable(vec.y),
            Type::convert_variable(vec.x),
        )
    }
}

impl<T: Copy + SimdType> From<f32x4> for Vector3<T> {
    fn from(pack: f32x4) -> Vector3<T> {
        Vector3 {
            x: Type::convert_variable(pack.3),
            y: Type::convert_variable(pack.2),
            z: Type::convert_variable(pack.1),
        }
    }
}

impl<T: SimdType> From<Vector2<T>> for f32x4 {
    fn from(vec: Vector2<T>) -> f32x4 {
        f32x4(
            0.0,
            0.0,
            Type::convert_variable(vec.y),
            Type::convert_variable(vec.x),
        )
    }
}

impl<T: Copy + SimdType> From<f32x4> for Vector2<T> {
    fn from(pack: f32x4) -> Vector2<T> {
        Vector2 {
            x: Type::convert_variable(pack.3),
            y: Type::convert_variable(pack.2),
        }
    }
}

impl<T: SimdType> From<Vector1<T>> for f32x4 {
    fn from(vec: Vector1<T>) -> f32x4 {
        f32x4(
            0.0,
            0.0,
            0.0,
            Type::convert_variable(vec.x),
        )
    }
}

impl<T: Copy + SimdType> From<f32x4> for Vector1<T> {
    fn from(pack: f32x4) -> Vector1<T> {
        Vector1 {
            x: Type::convert_variable(pack.3),
        }
    }
}
