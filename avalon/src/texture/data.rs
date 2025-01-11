use crate::texture::Component;

pub mod error {
    use thiserror::Error;
    #[derive(Debug, Error)]
    pub enum DatumError {
        #[error("Datum type and desired component have a mismatch")]
        ComponentTypeMismatch
    }
}

#[derive(Clone)]
pub struct Data {
    pub(super) data: Pixels,
    pub(super) components: Component,
}

#[derive(Debug, Copy, Clone)]
pub enum Datum {
    UnsignedByte(u8),
    Byte(i8),
    UnsignedShort(u16),
    Short(i16),
    UnsignedInt(u32),
    Int(i32),
    //Float16(f16),
    Float32(f32),
}

#[derive(Clone)]
pub enum Pixels {
    UnsignedByte(Vec<u8>),
    Byte(Vec<i8>),
    UnsignedShort(Vec<u16>),
    Short(Vec<i16>),
    UnsignedInt(Vec<u32>),
    Int(Vec<i32>),
    //Float16(f16),
    Float32(Vec<f32>),
    RGB3_3_2(Vec<u8>),
    RGB5_6_5(Vec<u16>),
    RGBA4(Vec<u16>),
    RGBA5_5_5_1(Vec<u16>),
    RGBA8(Vec<u32>),
    RGBA10_10_10_2(Vec<u32>),
}

impl Pixels {
    fn set(&mut self, idx: usize, datum: Datum) {
        match self {
            Pixels::UnsignedByte(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::Byte(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::UnsignedShort(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::Short(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::UnsignedInt(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::Int(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::Float32(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::RGB3_3_2(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::RGB5_6_5(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::RGBA4(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::RGBA5_5_5_1(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::RGBA8(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
            Pixels::RGBA10_10_10_2(data) => data[idx] = datum.try_into().expect("datum needs to be same type as component"),
        }
    }

    pub(super) fn as_api(&self) -> gl::types::GLenum {
        match self {
            Pixels::UnsignedByte(..) => gl::UNSIGNED_BYTE,
            Pixels::Byte(..) => gl::BYTE,
            Pixels::UnsignedShort(..) => gl::UNSIGNED_SHORT,
            Pixels::Short(..) => gl::SHORT,
            Pixels::UnsignedInt(..) => gl::UNSIGNED_INT,
            Pixels::Int(..) => gl::INT,
            Pixels::Float32(..) => gl::FLOAT,
            Pixels::RGB3_3_2(..) => gl::UNSIGNED_BYTE_3_3_2,
            Pixels::RGB5_6_5(..) => gl::UNSIGNED_SHORT_5_6_5,
            Pixels::RGBA4(..) => gl::UNSIGNED_SHORT_4_4_4_4,
            Pixels::RGBA5_5_5_1(..) => gl::UNSIGNED_SHORT_5_5_5_1,
            Pixels::RGBA8(..) => gl::UNSIGNED_INT_8_8_8_8,
            Pixels::RGBA10_10_10_2(..) => gl::UNSIGNED_INT_10_10_10_2,
        }
    }

    pub(super) fn from_api(api: gl::types::GLenum, size: usize) -> Pixels {
        match api {
            gl::UNSIGNED_BYTE => Pixels::UnsignedByte(vec![0; size]),
            gl::BYTE => Pixels::Byte(vec![0; size]),
            gl::UNSIGNED_SHORT => Pixels::UnsignedShort(vec![0; size]),
            gl::SHORT => Pixels::Short(vec![0; size]),
            gl::UNSIGNED_INT => Pixels::UnsignedInt(vec![0; size]),
            gl::INT => Pixels::Int(vec![0; size]),
            gl::FLOAT => Pixels::Float32(vec![0.0; size]),
            gl::UNSIGNED_BYTE_3_3_2 => Pixels::RGB3_3_2(vec![0; size]),
            gl::UNSIGNED_SHORT_5_6_5 => Pixels::RGB5_6_5(vec![0; size]),
            gl::UNSIGNED_SHORT_4_4_4_4 => Pixels::RGBA4(vec![0; size]),
            gl::UNSIGNED_SHORT_5_5_5_1 => Pixels::RGBA5_5_5_1(vec![0; size]),
            gl::UNSIGNED_INT_8_8_8_8 => Pixels::RGBA8(vec![0; size]),
            gl::UNSIGNED_INT_10_10_10_2 => Pixels::RGBA10_10_10_2(vec![0; size]),
            _ => panic!("Invalid API parameter passed")
        }
    }

    pub(super) fn as_ptr(&self) -> *const std::ffi::c_void {
        match self {
            Pixels::UnsignedByte(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::Byte(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::UnsignedShort(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::Short(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::UnsignedInt(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::Int(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::Float32(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::RGB3_3_2(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::RGB5_6_5(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::RGBA4(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::RGBA5_5_5_1(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::RGBA8(data) => data.as_ptr() as *const std::ffi::c_void,
            Pixels::RGBA10_10_10_2(data) => data.as_ptr() as *const std::ffi::c_void,
        }
    }

    pub(super) fn as_mut(&mut self) -> *mut std::ffi::c_void {
        match self {
            Pixels::UnsignedByte(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::Byte(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::UnsignedShort(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::Short(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::UnsignedInt(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::Int(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::Float32(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::RGB3_3_2(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::RGB5_6_5(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::RGBA4(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::RGBA5_5_5_1(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::RGBA8(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
            Pixels::RGBA10_10_10_2(data) => data.as_mut_ptr() as *mut std::ffi::c_void,
        }
    }
}

impl Data {
    pub fn empty_u8(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::UnsignedByte(vec![0; size * components.component_count()])
        }
    }

    pub fn empty_u16(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::UnsignedShort(vec![0; size * components.component_count()])
        }
    }

    pub fn empty_u32(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::UnsignedInt(vec![0; size * components.component_count()])
        }
    }

    pub fn empty_i8(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::Byte(vec![0; size * components.component_count()])
        }
    }

    pub fn empty_i16(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::Short(vec![0; size * components.component_count()])
        }
    }

    pub fn empty_i32(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::Int(vec![0; size * components.component_count()])
        }
    }

    pub fn empty_f32(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::Float32(vec![0.0; size * components.component_count()])
        }
    }

    pub fn set(&mut self, idx: usize, component: impl Into<Datum>) {
        self.data.set(idx, component.into());
    }
}

impl From<u8> for Datum {
    fn from(data: u8) -> Datum {
        Datum::UnsignedByte(data)
    }
}

impl From<u16> for Datum {
    fn from(data: u16) -> Datum {
        Datum::UnsignedShort(data)
    }
}

impl From<u32> for Datum {
    fn from(data: u32) -> Datum {
        Datum::UnsignedInt(data)
    }
}

impl From<i8> for Datum {
    fn from(data: i8) -> Datum {
        Datum::Byte(data)
    }
}

impl From<i16> for Datum {
    fn from(data: i16) -> Datum {
        Datum::Short(data)
    }
}

impl From<i32> for Datum {
    fn from(data: i32) -> Datum {
        Datum::Int(data)
    }
}

impl From<f32> for Datum {
    fn from(data: f32) -> Datum {
        Datum::Float32(data)
    }
}

impl TryFrom<Datum> for u8 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        if let Datum::UnsignedByte(d) = data {
            Ok(d)
        } else {
            Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for u16 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        if let Datum::UnsignedShort(d) = data {
            Ok(d)
        } else {
            Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for u32 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        if let Datum::UnsignedInt(d) = data {
            Ok(d)
        } else {
            Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for i8 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        if let Datum::Byte(d) = data {
            Ok(d)
        } else {
            Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for i16 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        if let Datum::Short(d) = data {
            Ok(d)
        } else {
            Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for i32 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        if let Datum::Int(d) = data {
            Ok(d)
        } else {
            Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for f32 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        if let Datum::Float32(d) = data {
            Ok(d)
        } else {
            Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}
