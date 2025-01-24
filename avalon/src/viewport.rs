pub mod builder;
pub mod depth_options;
pub mod error;

use crate::debug::GpuAnnotation;
use crate::texture;
use nalgebra_glm::{ IVec2, Vec3, vec2, vec3 };

#[derive(Debug, Clone)]
pub enum Attachment {
    ColourIdx(usize),
    ColourTag(String),
    DepthStencil
}

#[derive(Debug, Clone)]
pub enum BlitTarget<'t> {
    Screen(Attachment),
    Viewport((&'t Viewport, Attachment))
}

#[derive(Debug, Clone)]
pub struct ViewportBind<'v> {
    viewport: &'v Viewport
}

#[derive(Debug)]
pub struct MutViewportBind<'v> {
    viewport: &'v mut Viewport
}

#[derive(Debug, Clone)]
pub struct DepthStencilTexture {
    pub kind: DepthStencil,
    pub texture: texture::gpu::Texture2d
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
pub struct ColourAttachment {
    pub colour: texture::gpu::Texture2d,
    tag: Option<String>,
    unit: gl::types::GLenum,
}

#[derive(Debug, Copy, Clone)]
enum Handle {
    RenderTarget(gl::types::GLuint),
    Screen
}

#[derive(Debug, Clone)]
pub struct Viewport {
    colours: Vec<ColourAttachment>,
    depth_stencil: Option<DepthStencilTexture>,
    dimensions: IVec2,
    handle: Handle,
    clear_colour: Vec3,
    depth_options: depth_options::DepthOptions,
}

impl Viewport {
    fn handle(&self) -> gl::types::GLuint {
        match self.handle {
            Handle::Screen => 0,
            Handle::RenderTarget(handle) => handle
        }
    }

    pub fn new(dimensions: IVec2) -> builder::ViewportBuilder {
        builder::ViewportBuilder {
            dimensions,
            colour_attachments: Vec::new(),
            depth_stencil: None
        }
    }

    pub fn screen_viewport() -> Viewport {
        let dimensions = unsafe {
            let mut data = [0; 4];
            gl::GetIntegerv(gl::VIEWPORT, data.as_mut_ptr());
            vec2(data[2], data[3])
        };

        let clear_colour = unsafe {
            let mut colour_clear = [0.0; 4];
            gl::GetFloatv(gl::COLOR_CLEAR_VALUE, colour_clear.as_mut_ptr());
            vec3(colour_clear[0], colour_clear[1], colour_clear[2])
        };

        Viewport {
            colours: Vec::new(),
            depth_stencil: None,
            dimensions,
            handle: Handle::Screen,
            clear_colour,
            depth_options: depth_options::DepthOptions::existing()
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
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle());
        }

        match self.handle {
            Handle::RenderTarget(_) => unsafe {
                let targets: Vec<_> = self.colours.iter()
                    .map(|colour| colour.unit)
                    .collect();
                gl::DrawBuffers(
                    targets.len() as i32,
                    targets.as_ptr()
                );
            },
            Handle::Screen => {}
        }

        ViewportBind {
            viewport: self
        }
    }

    pub fn bind_mut<'v>(&'v mut self) -> MutViewportBind<'v> {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle());
        }
        MutViewportBind {
            viewport: self
        }
    }

    pub fn blit_attachment(&self, source: Attachment, target: BlitTarget) -> Result<(), error::Viewport> {
        let _annotation = GpuAnnotation::push("Blit viewport");
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.handle());

            let target = match &source {
                Attachment::ColourIdx(idx) => Some(self.colour_attachment(*idx)?.unit),
                Attachment::ColourTag(tag) => Some(self.tagged_colour(tag)?.unit),
                Attachment::DepthStencil => None
            };

            if let Some(target) = target {
                gl::DrawBuffer(target);
            }
        }

        {
            let (handle, target) = match target {
                BlitTarget::Viewport(ref viewport) => match viewport {
                    (viewport, Attachment::ColourIdx(idx)) => (viewport.handle(), Some(viewport.colour_attachment(*idx)?.unit)),
                    (viewport, Attachment::ColourTag(tag)) => (viewport.handle(), Some(viewport.tagged_colour(tag)?.unit)),
                    (viewport, Attachment::DepthStencil) => (viewport.handle(), None),
                },
                BlitTarget::Screen(ref target) => match target {
                    Attachment::ColourIdx(idx) => (0, Some(gl::COLOR_ATTACHMENT0 + *idx as u32)),
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

        let dest_dimensions = match target {
            BlitTarget::Viewport((viewport, _)) => viewport.dimensions,
            BlitTarget::Screen(_) => unsafe {
                let mut data = [0; 4];
                gl::GetIntegerv(gl::VIEWPORT, data.as_mut_ptr());
                vec2(data[2], data[3])
            },
        };

        unsafe {
            gl::BlitFramebuffer(
                0, 0, self.dimensions.x, self.dimensions.y,
                0, 0, dest_dimensions.x, dest_dimensions.y,
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
        if let Handle::RenderTarget(handle) = self.handle {
            unsafe {
                gl::DeleteFramebuffers(1, &handle);
            }
        }
    }
}

impl ViewportBind<'_> {
    pub fn clear(&self) {
        unsafe {
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

impl MutViewportBind<'_> {
    pub fn enable_srgb(&mut self, enable: bool) {
        if enable {
            unsafe {
                gl::Enable(gl::FRAMEBUFFER_SRGB);
            }
        } else {
            unsafe {
                gl::Disable(gl::FRAMEBUFFER_SRGB);
            }
        }
    }

    pub fn set_clear_colour(&mut self, colour: Vec3) {
        self.viewport.clear_colour = colour;
        unsafe {
            gl::ClearColor(
                self.viewport.clear_colour.x,
                self.viewport.clear_colour.y,
                self.viewport.clear_colour.z,
                1.0
            );
        }
    }
}

impl<'d, 'b: 'd> MutViewportBind<'b> {
    pub fn depth_test(self) -> depth_options::DepthOptionsBuilder<'d> {
        let options = self.viewport.depth_options;
        depth_options::DepthOptionsBuilder {
            viewport: self,
            options
        }
    }
}

impl Drop for MutViewportBind<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}
