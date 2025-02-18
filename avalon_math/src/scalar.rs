pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod vector2;
pub mod vector3;
pub mod vector4;

pub trait HasSqrt {
    fn sqrt(self) -> Self;
}

impl HasSqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl HasSqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

pub trait Identity {
    fn identity() -> Self;
    fn zero() -> Self;
}
impl Identity for u8  { fn identity() -> Self { 1_u8 } fn zero() -> Self  { 0_u8 } }
impl Identity for u16 { fn identity() -> Self { 1_u16 } fn zero() -> Self { 0_u16 } }
impl Identity for u32 { fn identity() -> Self { 1_u32 } fn zero() -> Self { 0_u32 } }
impl Identity for u64 { fn identity() -> Self { 1_u64 } fn zero() -> Self { 0_u64 } }
impl Identity for i8  { fn identity() -> Self { 1_i8 } fn zero() -> Self  { 0_i8 } }
impl Identity for i16 { fn identity() -> Self { 1_i16 } fn zero() -> Self { 0_i16 } }
impl Identity for i32 { fn identity() -> Self { 1_i32 } fn zero() -> Self { 0_i32 } }
impl Identity for i64 { fn identity() -> Self { 1_i64 } fn zero() -> Self { 0_i64 } }
impl Identity for f32 { fn identity() -> Self { 1.0_f32 } fn zero() -> Self { 0.0_f32 } }
impl Identity for f64 { fn identity() -> Self { 1.0_f64 } fn zero() -> Self { 0.0_f64 } }

pub trait Inverse {
    fn inverse(self) -> Self;
}
impl Inverse for f32 { fn inverse(self) -> Self { 1.0_f32 / self } }
impl Inverse for f64 { fn inverse(self) -> Self { 1.0_f64 / self } }
