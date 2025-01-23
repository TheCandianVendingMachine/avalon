pub mod builder;
pub mod error;

use crate::debug::GpuAnnotation;
use crate::texture;
use nalgebra_glm::{ IVec2, Vec3 };

pub enum Attachment {
    ColourIdx(usize),
    ColourTag(String),
    DepthStencil
}

pub enum BlitTarget<'t> {
    Screen(Attachment),
    Viewport((&'t Viewport, Attachment))
}

pub struct ViewportBind<'v> {
    viewport: &'v Viewport
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

impl DepthStencil {
    fn unit(&self) -> gl::types::GLenum {
        match self {
            DepthStencil::Depth => gl::DEPTH_ATTACHMENT,
            DepthStencil::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
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

    pub fn colour_attachment(&self, idx: usize) -> Result<&ColourAttachment, error::Viewport> {
        self.colours.get(idx).ok_or(error::Viewport::NoColourAtIndex(idx))
    }

    pub fn tagged_colour(&self, tag: impl AsRef<str>) -> Result<&ColourAttachment, error::Viewport> {
        let tag = tag.as_ref();
        self.colours.iter()
            .filter(|attachment| attachment.tag.is_some())
            .find(|attachment| *attachment.tag.as_ref().unwrap() == *tag)
            .ok_or(error::Viewport::NoColourWithName(tag.to_string()))
    }

    pub fn depth_attachment(&self) -> Result<&DepthStencilTexture, error::Viewport> {
        self.depth_stencil.as_ref().ok_or(error::Viewport::NoDepthStencilAttachment)
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

        if self.depth_stencil.is_some() {
            unsafe {
                gl::Enable(gl::DEPTH_TEST);
                gl::DepthFunc(gl::ALWAYS);
                gl::DepthMask(gl::TRUE);
            }
        }

        ViewportBind {
            viewport: self
        }
    }

    pub fn blit_attachment(&self, source: Attachment, target: BlitTarget) -> Result<(), error::Viewport> {
        let _annotation = GpuAnnotation::push("Blitting attachment onto target");
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.handle);

            let target = match &source {
                Attachment::ColourIdx(idx) => Some(self.colour_attachment(*idx)?.unit),
                Attachment::ColourTag(tag) => Some(self.tagged_colour(tag)?.unit),
                Attachment::DepthStencil => None
            };

            if let Some(target) = target {
                gl::DrawBuffer(target);
            }
        }

        let (handle, target) = match target {
            BlitTarget::Viewport(viewport) => match viewport {
                (viewport, Attachment::ColourIdx(idx)) => (viewport.handle, Some(viewport.colour_attachment(idx)?.unit)),
                (viewport, Attachment::ColourTag(tag)) => (viewport.handle, Some(viewport.tagged_colour(tag)?.unit)),
                (viewport, Attachment::DepthStencil) => (viewport.handle, None),
            },
            BlitTarget::Screen(target) => match target {
                Attachment::ColourIdx(idx) => (0, Some(gl::COLOR_ATTACHMENT0 + idx as u32)),
                Attachment::DepthStencil =>   (0, None),
                Attachment::ColourTag(_tag) => panic!("unknown operation"),
            }
        };

        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, handle);
            if let Some(target) = target {
                gl::DrawBuffer(target);
            }
        }

        let mask = match source {
            Attachment::ColourTag(_) | Attachment::ColourIdx(_) => gl::COLOR_BUFFER_BIT,
            Attachment::DepthStencil => {
                match self.depth_attachment()?.kind {
                    DepthStencil::Depth => gl::DEPTH_BUFFER_BIT,
                    DepthStencil::DepthStencil => gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
                }
            }
        };

        unsafe {
            gl::BlitFramebuffer(
                0, 0, self.dimensions.x, self.dimensions.y,
                0, 0, self.dimensions.x, self.dimensions.y,
                mask,
                gl::NEAREST
            );
        }

        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, 0);
        }

        Ok(())
    }
}

impl Drop for Viewport {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.handle);
        }
    }
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

