pub mod builder;

use crate::texture;
use nalgebra_glm::{ IVec2, Vec3 };

pub struct ViewportBind<'v> {
    viewport: &'v Viewport
}

impl ViewportBind<'_> {
    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(self.viewport.clear_colour.x, self.viewport.clear_colour.y, self.viewport.clear_colour.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }
}

impl Drop for ViewportBind<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

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
    handle: gl::types::GLuint,
    clear_colour: Vec3
}

impl Viewport {
    pub fn new(dimensions: IVec2) -> builder::ViewportBuilder {
        builder::ViewportBuilder {
            dimensions,
            colour_attachments: Vec::new(),
            depth_stencil: None
        }
    }

    pub fn bind<'v>(&'v self) -> ViewportBind<'v> {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);
        }
        unsafe {
            gl::DrawBuffers(
                self.colours.len() as i32,
                self.colours.iter()
                    .map(|colour| colour.unit)
                    .collect::<Vec<_>>()
                    .as_ptr()
            );
        }
        ViewportBind {
            viewport: self
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

