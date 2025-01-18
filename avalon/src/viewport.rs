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

pub struct DepthStencilTexture {
    pub kind: DepthStencil,
    pub texture: texture::gpu::Texture2d
}

#[derive(Debug)]
pub enum DepthStencil {
    Depth,
    DepthStencil,
}

pub struct ColourAttachment {
    pub colour: texture::gpu::Texture2d,
    tag: Option<String>,
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

    pub fn colour_attachment(&self, idx: usize) -> &ColourAttachment {
        &self.colours[idx]
    }

    pub fn tagged_colour(&self, tag: impl AsRef<str>) -> Option<&ColourAttachment> {
        let tag = tag.as_ref();
        self.colours.iter()
            .filter(|attachment| attachment.tag.is_some())
            .find(|attachment| *attachment.tag.as_ref().unwrap() == *tag) }

    pub fn depth_attachment(&self) -> Option<&DepthStencilTexture> {
        self.depth_stencil.as_ref()
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

