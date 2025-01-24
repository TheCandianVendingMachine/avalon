use gl;

use crate::shader;
use crate::viewport;

pub struct Pass {
    shader: shader::Program,
}

pub struct RenderEngine {
    viewport: viewport::Viewport,
}

impl RenderEngine {
    pub fn new() -> RenderEngine {
        let mut viewport = viewport::Viewport::screen_viewport();
        let mut bind = viewport.bind_mut();
        bind.enable_srgb(true);
        bind.set_clear_colour(nalgebra_glm::vec3(1.0, 0.6, 0.8));
        RenderEngine {
            viewport: viewport::Viewport::screen_viewport()
        }
    }

    pub fn render(&self) {
        self.viewport.bind().clear();
    }
}
