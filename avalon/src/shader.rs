use std::path::Path;
use std::fs;

mod program;
mod uniform;
pub mod error;
pub use program::Program;

mod utils {
    pub fn string_as_char_buf(source: String) -> Vec<i8> {
        Vec::from(source.as_bytes())
            .iter()
            .map(|i| (*i) as i8)
            .collect()
    }
}

trait MetaShader {
    fn from_id(id: gl::types::GLuint) -> Self;
    fn id(&self) -> gl::types::GLuint;
    fn shader_type() -> gl::types::GLuint;
}

pub trait Source: MetaShader {
    fn info_log(&self) -> String {
        let log_length = unsafe {
            let mut length = 0;
            gl::GetShaderiv(self.id(), gl::INFO_LOG_LENGTH, &mut length);
            length as usize
        };

        let log_buffer = unsafe {
            let mut log_buffer = Vec::new();
            log_buffer.resize(log_length, 0);
            gl::GetShaderInfoLog(
                self.id(),
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

    fn load_from_source(source: String) -> Result<Self, error::Creation> where Self: Sized {
        let shader = Self::from_id(unsafe { gl::CreateShader(Self::shader_type()) });

        let source_buffer = utils::string_as_char_buf(source);
        unsafe {
            gl::ShaderSource(shader.id(), 1, &source_buffer.as_ptr(), std::ptr::null());
        }

        unsafe {
            gl::CompileShader(shader.id());
        }

        let successful_compile = unsafe {
            let mut successful = 0;
            gl::GetShaderiv(shader.id(), gl::COMPILE_STATUS, &mut successful);
            successful == gl::TRUE.into()
        };

        if !successful_compile {
            return Err(error::Creation::FailedToCompile {
                reason: shader.info_log()
            });
        }

        Ok(shader)
    }
    fn load_from_path(path: impl AsRef<Path>) -> Result<Self, error::Creation> where Self: Sized {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(error::Creation::ShaderNotFound { path: path.to_string_lossy().to_string() });
        }
        let file = fs::read(path).map_err(|e| error::Creation::FileReadError(e))?;

        Self::load_from_source(String::from_utf8(file).map_err(|e| error::Creation::FileParseError(e))?)
    }
}

pub struct Vertex {
    id: gl::types::GLuint
}
pub struct Fragment {
    id: gl::types::GLuint
}
pub struct Compute {
    id: gl::types::GLuint
}

impl MetaShader for Vertex {
    fn from_id(id: gl::types::GLuint) -> Self {
        Vertex {
            id
        }
    }

    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn shader_type() -> gl::types::GLuint {
        gl::VERTEX_SHADER
    }
}

impl MetaShader for Fragment {
    fn from_id(id: gl::types::GLuint) -> Self {
        Fragment {
            id
        }
    }

    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn shader_type() -> gl::types::GLuint {
        gl::FRAGMENT_SHADER
    }
}

impl MetaShader for Compute {
    fn from_id(id: gl::types::GLuint) -> Self {
        Compute {
            id
        }
    }

    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn shader_type() -> gl::types::GLuint {
        gl::COMPUTE_SHADER
    }
}

impl Source for Vertex {}
impl Source for Fragment {}
impl Source for Compute {}

enum Shader {
    Vertex(Vertex),
    Fragment(Fragment),
    Compute(Compute)
}

impl Shader {
    fn shader(&self) -> gl::types::GLuint {
        match self {
            Shader::Vertex(shader) => shader.id,
            Shader::Fragment(shader) => shader.id,
            Shader::Compute(shader) => shader.id,
        }
    }
}
