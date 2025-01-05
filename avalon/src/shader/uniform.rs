use gl;
use nalgebra_glm::{ Vec2, Vec3, Vec4, Mat2, Mat3, Mat4 };

use crate::shader::program::AttachedProgram;
use crate::texture::gpu;

#[derive(Copy, Clone)]
pub struct Uniform<'program> {
    pub(super) program: &'program AttachedProgram<'program>,
    pub(super) location: gl::types::GLint,
}

impl Uniform<'_> {
    pub fn set_f32(self, value: impl Into<f32>) {
        unsafe { gl::Uniform1f(self.location, value.into()); }
    }

    pub fn set_f64(self, value: impl Into<f64>) {
        unsafe { gl::Uniform1d(self.location, value.into()); }
    }

    pub fn set_i32(self, value: impl Into<i32>) {
        unsafe { gl::Uniform1i(self.location, value.into()); }
    }

    pub fn set_u32(self, value: impl Into<u32>) {
        unsafe { gl::Uniform1ui(self.location, value.into()); }
    }

    pub fn set_vec2(self, value: impl AsRef<Vec2>) {
        unsafe { gl::Uniform2fv(self.location, 1, value.as_ref().as_slice().as_ptr()); }
    }

    pub fn set_vec3(self, value: impl AsRef<Vec3>) {
        unsafe { gl::Uniform3fv(self.location, 1, value.as_ref().as_slice().as_ptr()); }
    }

    pub fn set_vec4(self, value: impl AsRef<Vec4>) {
        unsafe { gl::Uniform4fv(self.location, 1, value.as_ref().as_slice().as_ptr()); }
    }

    pub fn set_mat2(self, value: impl AsRef<Mat2>) {
        unsafe { gl::UniformMatrix2fv(self.location, 1, gl::FALSE, value.as_ref().as_slice().as_ptr()); }
    }

    pub fn set_mat3(self, value: impl AsRef<Mat3>) {
        unsafe { gl::UniformMatrix3fv(self.location, 1, gl::FALSE, value.as_ref().as_slice().as_ptr()); }
    }

    pub fn set_mat4(self, value: impl AsRef<Mat4>) {
        unsafe { gl::UniformMatrix4fv(self.location, 1, gl::FALSE, value.as_ref().as_slice().as_ptr()); }
    }

    pub fn set_texture(self, value: &gpu::TextureAttachment) {
        unsafe { gl::Uniform1i(self.location, value.unit() as i32); }
    }

    pub fn set_image(self, value: &gpu::ImageAttachment) {
        unsafe { gl::Uniform1i(self.location, value.unit() as i32); }
    }
}
