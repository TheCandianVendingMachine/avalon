use crate::texture::Component;

#[derive(Clone)]
pub struct Data {
    pub(super) data: Pixels,
    pub(super) components: Component,
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
    pub fn empty(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::UnsignedByte(vec![0; size])
        }
    }
}
