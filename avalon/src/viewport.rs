pub mod builder;

use crate::texture;
use nalgebra_glm::IVec2;

struct DepthStencilTexture {
    kind: DepthStencil,
    texture: texture::gpu::Texture2d
}

#[derive(Debug)]
pub enum DepthStencil {
    Depth,
    DepthStencil,
}

struct ColourAttachment {
    colour: texture::gpu::Texture2d,
    unit: gl::types::GLenum,
}

pub struct Viewport {
    colours: Vec<ColourAttachment>,
    depth_stencil: Option<DepthStencilTexture>,
    dimensions: IVec2,
    handle: gl::types::GLuint
}

impl Viewport {
    pub fn new(dimensions: IVec2) -> builder::ViewportBuilder {
        builder::ViewportBuilder {
            dimensions,
            colour_attachments: Vec::new(),
            depth_stencil: None
        }
    }
}

impl Drop for Viewport {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.handle);
        }
    }
}

