use gl;

use crate::shader::{
    Vertex as VertexShader,
    Fragment as FragmentShader,
    Compute as ComputeShader,
    Shader
};
use crate::shader::uniform::Uniform;
use crate::shader::error;

pub struct Program {
    program: gl::types::GLuint,
    shaders: Vec<Shader>
}

pub struct AttachedProgram<'program> {
    program: &'program Program
}

impl Drop for AttachedProgram<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl Program {
    pub fn new() -> ProgramBuilder {
        ProgramBuilder {
            shaders: Vec::new(),
        }
    }

    pub fn attach(&self) -> AttachedProgram {
        unsafe {
            gl::UseProgram(self.program);
        }
        AttachedProgram {
            program: self
        }
    }

    pub fn info_log(&self) -> String {
        let log_length = unsafe {
            let mut length = 0;
            gl::GetProgramiv(self.program, gl::INFO_LOG_LENGTH, &mut length);
            length as usize
        };

        let log_buffer = unsafe {
            let mut log_buffer = Vec::new();
            log_buffer.resize(log_length, 0);
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
    pub fn uniform(&self, uniform: impl Into<String>) -> Result<Uniform, error::Program> {
        let uniform = uniform.into();
        let location = unsafe {
            let uniform_cstr = std::ffi::CString::new(uniform.clone()).expect("Uniform variable must not contain \\0 bytes");
            gl::GetUniformLocation(self.program.program, uniform_cstr.as_ptr())
        };

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
}

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
