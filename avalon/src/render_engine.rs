use crate::shader;

pub struct Pass {
    shader: shader::Program,
}

pub struct RenderEngine {

}

impl RenderEngine {
    pub fn new() -> RenderEngine {
        RenderEngine {
        }
    }

    pub fn render(&self) {
    }
}
