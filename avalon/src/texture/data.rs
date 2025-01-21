use image::{ self, GenericImageView };
use nalgebra_glm::IVec2;
use aligned_vec::{ avec, AVec, ConstAlign };
use crate::texture::Component;

pub mod error {
    use thiserror::Error;
    #[derive(Debug, Error)]
    pub enum DatumError {
        #[error("Datum type and desired component have a mismatch")]
        ComponentTypeMismatch
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Pixels {
    UnsignedByte(AVec<u8, ConstAlign<4>>),
    Byte(AVec<i8, ConstAlign<4>>),
    UnsignedShort(AVec<u16, ConstAlign<4>>),
    Short(AVec<i16, ConstAlign<4>>),
    UnsignedInt(AVec<u32, ConstAlign<4>>),
    Int(AVec<i32, ConstAlign<4>>),
    //Float16(f16),
    Float32(AVec<f32, ConstAlign<4>>),
    RGB3_3_2(AVec<u8, ConstAlign<4>>),
    RGB5_6_5(AVec<u16, ConstAlign<4>>),
    RGBA4(AVec<u16, ConstAlign<4>>),
    RGBA5_5_5_1(AVec<u16, ConstAlign<4>>),
    RGBA8(AVec<u32, ConstAlign<4>>),
    RGBA10_10_10_2(AVec<u32, ConstAlign<4>>),
}

impl Pixels {
    fn from_datum(datum: Vec<Datum>) -> Pixels {
        if datum.is_empty() {
            return Pixels::UnsignedByte(AVec::new(4));
        }

        let first_datum = datum[0];
        match first_datum {
            Datum::UnsignedByte(..) => Pixels::UnsignedByte(AVec::from_iter(
                4, datum.iter().map(|d| (*d).try_into().unwrap())
            )),
            Datum::Byte(..) => Pixels::Byte(AVec::from_iter(
                4, datum.iter().map(|d| (*d).try_into().unwrap())
            )),
            Datum::UnsignedShort(..) => Pixels::UnsignedShort(AVec::from_iter(
                4, datum.iter().map(|d| (*d).try_into().unwrap())
            )),
            Datum::Short(..) => Pixels::Short(AVec::from_iter(
                4, datum.iter().map(|d| (*d).try_into().unwrap())
            )),
            Datum::UnsignedInt(..) => Pixels::UnsignedInt(AVec::from_iter(
                4, datum.iter().map(|d| (*d).try_into().unwrap())
            )),
            Datum::Int(..) => Pixels::Int(AVec::from_iter(
                4, datum.iter().map(|d| (*d).try_into().unwrap())
            )),
            Datum::Float32(..) => Pixels::Float32(AVec::from_iter(
                4, datum.iter().map(|d| (*d).try_into().unwrap())
            )),
        }
    }

    fn len(&self) -> usize {
        match self {
            Pixels::UnsignedByte(data) => data.len(),
            Pixels::Byte(data) => data.len(),
            Pixels::UnsignedShort(data) => data.len(),
            Pixels::Short(data) => data.len(),
            Pixels::UnsignedInt(data) => data.len(),
            Pixels::Int(data) => data.len(),
            Pixels::Float32(data) => data.len(),
            Pixels::RGB3_3_2(data) => data.len(),
            Pixels::RGB5_6_5(data) => data.len(),
            Pixels::RGBA4(data) => data.len(),
            Pixels::RGBA5_5_5_1(data) => data.len(),
            Pixels::RGBA8(data) => data.len(),
            Pixels::RGBA10_10_10_2(data) => data.len(),
        }
    }

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

    fn get(&self, idx: usize) -> Datum {
        match self {
            Pixels::UnsignedByte(data) => data[idx].into(),
            Pixels::Byte(data) => data[idx].into(),
            Pixels::UnsignedShort(data) => data[idx].into(),
            Pixels::Short(data) => data[idx].into(),
            Pixels::UnsignedInt(data) => data[idx].into(),
            Pixels::Int(data) => data[idx].into(),
            Pixels::Float32(data) => data[idx].into(),
            Pixels::RGB3_3_2(data) => data[idx].into(),
            Pixels::RGB5_6_5(data) => data[idx].into(),
            Pixels::RGBA4(data) => data[idx].into(),
            Pixels::RGBA5_5_5_1(data) => data[idx].into(),
            Pixels::RGBA8(data) => data[idx].into(),
            Pixels::RGBA10_10_10_2(data) => data[idx].into(),
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
            gl::UNSIGNED_BYTE => Pixels::UnsignedByte(avec![[4] | 0; size]),
            gl::BYTE => Pixels::Byte(avec![[4] | 0; size]),
            gl::UNSIGNED_SHORT => Pixels::UnsignedShort(avec![[4] | 0; size]),
            gl::SHORT => Pixels::Short(avec![[4] | 0; size]),
            gl::UNSIGNED_INT => Pixels::UnsignedInt(avec![[4] | 0; size]),
            gl::INT => Pixels::Int(avec![[4] | 0; size]),
            gl::FLOAT => Pixels::Float32(avec![[4] | 0.0; size]),
            gl::UNSIGNED_BYTE_3_3_2 => Pixels::RGB3_3_2(avec![[4] | 0; size]),
            gl::UNSIGNED_SHORT_5_6_5 => Pixels::RGB5_6_5(avec![[4] | 0; size]),
            gl::UNSIGNED_SHORT_4_4_4_4 => Pixels::RGBA4(avec![[4] | 0; size]),
            gl::UNSIGNED_SHORT_5_5_5_1 => Pixels::RGBA5_5_5_1(avec![[4] | 0; size]),
            gl::UNSIGNED_INT_8_8_8_8 => Pixels::RGBA8(avec![[4] | 0; size]),
            gl::UNSIGNED_INT_10_10_10_2 => Pixels::RGBA10_10_10_2(avec![[4] | 0; size]),
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
    fn from_image(image: image::DynamicImage) -> Data {
        let (components, buffer): (Component, Vec<Datum>) = match image {
            image::DynamicImage::ImageRgb8(img) => (
                Component::RGB,
                img.as_raw().iter()
                    .map(|p| (*p).into())
                    .collect()
            ),
            image::DynamicImage::ImageRgba8(img) => (
                Component::RGBA,
                img.as_raw().iter()
                    .map(|p| (*p).into())
                    .collect()
            ),
            image::DynamicImage::ImageRgb16(img) => (
                Component::RGB,
                img.as_raw().iter()
                    .map(|p| (*p).into())
                    .collect()
            ),
            image::DynamicImage::ImageRgba16(img) => (
                Component::RGBA,
                img.as_raw().iter()
                    .map(|p| (*p).into())
                    .collect()
            ),
            image::DynamicImage::ImageRgb32F(img) => (
                Component::RGB,
                img.as_raw().iter()
                    .map(|p| (*p).into())
                    .collect()
            ),
            image::DynamicImage::ImageRgba32F(img) => (
                Component::RGBA,
                img.as_raw().iter()
                    .map(|p| (*p).into())
                    .collect()
            ),
            _ => panic!("unsupported image type")
        };
        Data {
            components,
            data: match components {
                Component::RGB => Pixels::from_datum(buffer),
                Component::RGBA => Pixels::from_datum(buffer),
                _ => panic!("unsupported components")
            }
        }
    }
    pub fn from_file(path: impl AsRef<std::path::Path>) -> (Data, IVec2) {
        let image = image::ImageReader::open(path).unwrap().decode().unwrap();
        let dimension = image.dimensions();
        (Data::from_image(image), IVec2::new(dimension.0 as i32, dimension.1 as i32))
    }
    pub fn from_buffer(buffer: Vec<u8>) -> (Data, IVec2) {
        let cursor = std::io::Cursor::new(buffer);
        let image = image::ImageReader::new(cursor).with_guessed_format().unwrap().decode().unwrap();
        let dimension = image.dimensions();
        (Data::from_image(image), IVec2::new(dimension.0 as i32, dimension.1 as i32))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn empty_u8(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::UnsignedByte(avec![[4] | 0; size * components.component_count()])
        }
    }

    pub fn empty_u16(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::UnsignedShort(avec![[4] | 0; size * components.component_count()])
        }
    }

    pub fn empty_u32(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::UnsignedInt(avec![[4] | 0; size * components.component_count()])
        }
    }

    pub fn empty_i8(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::Byte(avec![[4] | 0; size * components.component_count()])
        }
    }

    pub fn empty_i16(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::Short(avec![[4] | 0; size * components.component_count()])
        }
    }

    pub fn empty_i32(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::Int(avec![[4] | 0; size * components.component_count()])
        }
    }

    pub fn empty_f32(components: Component, size: usize) -> Data {
        Data {
            components,
            data: Pixels::Float32(avec![[4] | 0.0; size * components.component_count()])
        }
    }

    pub fn set(&mut self, idx: usize, component: impl Into<Datum>) {
        self.data.set(idx, component.into());
    }

    pub fn get(&self, idx: usize) -> Datum {
        self.data.get(idx)
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
        match data {
            Datum::UnsignedByte(d) => Ok(d),
            _ => Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for u16 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        match data {
            Datum::UnsignedByte(d) => Ok(d.into()),
            Datum::UnsignedShort(d) => Ok(d),
            _ => Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for u32 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        match data {
            Datum::UnsignedByte(d) => Ok(d.into()),
            Datum::UnsignedShort(d) => Ok(d.into()),
            Datum::UnsignedInt(d) => Ok(d),
            _ => Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for i8 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        match data {
            Datum::Byte(d) => Ok(d),
            _ => Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for i16 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        match data {
            Datum::Byte(d) => Ok(d.into()),
            Datum::Short(d) => Ok(d),
            _ => Err(error::DatumError::ComponentTypeMismatch)
        }
    }
}

impl TryFrom<Datum> for i32 {
    type Error = error::DatumError;
    fn try_from(data: Datum) -> Result<Self, Self::Error> {
        match data {
            Datum::Byte(d) => Ok(d.into()),
            Datum::Short(d) => Ok(d.into()),
            Datum::Int(d) => Ok(d),
            _ => Err(error::DatumError::ComponentTypeMismatch)
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
