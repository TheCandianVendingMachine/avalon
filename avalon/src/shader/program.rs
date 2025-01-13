use gl;
use std::collections::HashMap;

use crate::shader::{
    Vertex as VertexShader,
    Fragment as FragmentShader,
    Compute as ComputeShader,
    Shader
};
use crate::shader::uniform::Uniform;
use crate::shader::error;
use crate::texture::gpu::{ Access, Sampler, Image, TextureAttachment, ImageAttachment };

#[derive(Clone)]
pub struct Program {
    program: gl::types::GLuint,
    shaders: Vec<Shader>
}

pub struct AttachedProgram<'program> {
    program: &'program Program,
    texture_attachments: HashMap<gl::types::GLuint, TextureAttachment<'program>>,
    image_attachments: HashMap<(gl::types::GLuint, Access), ImageAttachment<'program>>
}

impl Program {
    pub fn new() -> ProgramBuilder {
        ProgramBuilder {
            shaders: Vec::new(),
        }
    }

    pub fn activate(&self) -> AttachedProgram {
        unsafe {
            gl::UseProgram(self.program);
        }
        AttachedProgram {
            program: self,
            texture_attachments: HashMap::new(),
            image_attachments: HashMap::new()
        }
    }

    pub fn info_log(&self) -> String {
        let log_length = unsafe {
            let mut length = 0;
            gl::GetProgramiv(self.program, gl::INFO_LOG_LENGTH, &mut length);
            length as usize
        };

        let log_buffer = unsafe {
            let mut log_buffer = vec![0; log_length];
            gl::GetProgramInfoLog(
                self.program,
                log_length as i32,
                std::ptr::null_mut(),
                log_buffer.as_mut_ptr()
            );
            log_buffer.iter()
                .map(|i| (*i) as u8)
                .collect()
        };

        String::from_utf8(log_buffer).unwrap()
    }

    fn link(self) -> Result<Self, error::Creation> {
        for shader in self.shaders.iter() {
            unsafe {
                gl::AttachShader(self.program, shader.shader());
            }
        }

        unsafe {
            gl::LinkProgram(self.program);
        }

        unsafe {
            let mut successful = 0;
            gl::GetProgramiv(self.program, gl::LINK_STATUS, &mut successful);
            if successful == gl::FALSE.into() {
                return Err(error::Creation::FailedToLink {
                    reason: self.info_log()
                });
            }
        }

        Ok(self)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        for shader in self.shaders.iter() {
            unsafe {
                gl::DetachShader(self.program, shader.shader());
                gl::DeleteShader(shader.shader());
            }
        }
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

impl AttachedProgram<'_> {
    fn location_from_uniform(&self, uniform: &String) -> gl::types::GLint {
        unsafe {
            let uniform_cstr = std::ffi::CString::new(uniform.clone()).expect("Uniform variable must not contain \\0 bytes");
            gl::GetUniformLocation(self.program.program, uniform_cstr.as_ptr())
        }
    }

    pub fn uniform(&self, uniform: impl Into<String>) -> Result<Uniform, error::Program> {
        let uniform = uniform.into();
        let location = self.location_from_uniform(&uniform);
        if location == -1 {
            return Err(error::Program::UniformNotFound { name: uniform });
        }

        Ok(Uniform {
            program: self,
            location
        })
    }

    pub fn location(&self, location: u32) -> Result<Uniform, error::Program> {
        Ok(Uniform {
            program: self,
            location: location as i32
        })
    }

    pub fn dispatch_compute(&self, groups_x: u32, groups_y: u32, groups_z: u32) {
        unsafe {
            gl::DispatchCompute(groups_x, groups_y, groups_z);
        }
    }
}

impl<'p, 't: 'p> AttachedProgram<'p> {
    pub fn sampler(&mut self, name: impl Into<String>, texture: &'t impl Sampler) -> Result<(), error::Program> {
        let name = name.into();
        let location = self.location_from_uniform(&name);
        self.sampler_with_location(location as u32, texture)
    }

    pub fn sampler_with_location(&mut self, location: u32, texture: &'t impl Sampler) -> Result<(), error::Program> {
        if !self.texture_attachments.contains_key(&texture.handle()) {
            self.texture_attachments.insert(texture.handle(), texture.sampler(self.texture_attachments.len() as u32));
        }
        let attachment = self.texture_attachments.get(&texture.handle()).unwrap();
        self.location(location)?.set_texture(attachment);
        Ok(())
    }

    pub fn image(&mut self, name: impl Into<String>, texture: &'t impl Image, access: Access) -> Result<(), error::Program> {
        let name = name.into();
        let location = self.location_from_uniform(&name);
        self.image_with_location(location as u32, texture, access)
    }

    pub fn image_with_location(&mut self, location: u32, texture: &'t impl Image, access: Access) -> Result<(), error::Program> {
        let key = (texture.handle(), access);
        if !self.image_attachments.contains_key(&key) {
            self.image_attachments.insert(key, texture.image(self.image_attachments.len() as u32, access));
        }
        let attachment = self.image_attachments.get(&key).unwrap();
        self.location(location)?.set_image(attachment);
        Ok(())
    }
}

impl Drop for AttachedProgram<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

#[derive(Clone)]
pub struct ProgramBuilder {
    shaders: Vec<Shader>
}

impl ProgramBuilder {
    pub fn vertex(mut self, shader: VertexShader) -> Self {
        self.shaders.push(Shader::Vertex(shader));
        self
    }

    pub fn fragment(mut self, shader: FragmentShader) -> Self {
        self.shaders.push(Shader::Fragment(shader));
        self
    }

    pub fn compute(mut self, shader: ComputeShader) -> Self {
        self.shaders.push(Shader::Compute(shader));
        self
    }

    pub fn build(self) -> Result<Program, error::Creation> {
        Program {
            program: unsafe {
                gl::CreateProgram()
            },
            shaders: self.shaders
        }.link()
    }
}
