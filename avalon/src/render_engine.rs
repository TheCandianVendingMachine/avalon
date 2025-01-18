use gl;

use crate::shader;

pub struct Pass {
    shader: shader::Program,
}

pub struct RenderEngine {
}

impl RenderEngine {
    pub fn new() -> RenderEngine {
        unsafe {
            gl::Enable(gl::FRAMEBUFFER_SRGB);
        }
        RenderEngine {
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::ClearColor(1.0, 0.6, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
