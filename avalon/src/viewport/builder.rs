use nalgebra_glm::IVec2;
use crate::viewport::Viewport;
use crate::texture::gpu::{ self, SizedComponent };

pub struct ColourAttachmentBuilder {
    viewport_builder: ViewportBuilder,
    internal_format: SizedComponent,
}

impl ColourAttachmentBuilder {
    fn format(mut self, size: SizedComponent) -> ViewportBuilder {
        self.internal_format = size;
        self.viewport_builder.colour_attachments.push(ColourAttachmentData {
            internal_format: size
        });
        self.viewport_builder
    }
}

pub(super) struct ColourAttachmentData {
    internal_format: SizedComponent
}

pub(super) struct DepthAttachmentData {
    internal_format: SizedComponent,
    kind: super::DepthStencil
}

pub struct ViewportBuilder {
    pub(super) dimensions: IVec2,
    pub(super) colour_attachments: Vec<ColourAttachmentData>,
    pub(super) depth_stencil: Option<DepthAttachmentData>
}

impl ViewportBuilder {
    pub fn colour_attachment(self) -> ColourAttachmentBuilder {
        ColourAttachmentBuilder {
            viewport_builder: self,
            internal_format: SizedComponent::RGBA8
        }
    }

    pub fn depth_stencil(mut self, depth: super::DepthStencil) -> ViewportBuilder {
        self.depth_stencil = Some(DepthAttachmentData {
            internal_format: match depth {
                super::DepthStencil::DepthStencil => SizedComponent::DepthStencil,
                super::DepthStencil::Depth => SizedComponent::FloatDepth32,
            },
            kind: depth
        });
        self
    }

    pub fn build(self) -> Viewport {
        let handle = unsafe {
            let mut fbo = 0;
            gl::GenFramebuffers(1, &mut fbo);
            fbo
        };

        let textures = {
            let mut textures = vec![0; self.colour_attachments.len()];
            unsafe {
                gl::GenTextures(self.colour_attachments.len() as i32, textures.as_mut_ptr());
            }
            textures
        };

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, handle);
        }

        let mut colours = Vec::new();
        for (idx, (colour, texture)) in self.colour_attachments.iter().zip(textures.iter()).enumerate() {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, *texture);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    colour.internal_format.as_api(),
                    self.dimensions.x,
                    self.dimensions.y,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    std::ptr::null()
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            }

            unsafe {
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::COLOR_ATTACHMENT0 + idx as u32,
                    gl::TEXTURE_2D,
                    *texture,
                    0
                );
            }

            colours.push(super::ColourAttachment {
                colour: gpu::Texture2d::from_handle(
                    *texture,
                    colour.internal_format.into(),
                    colour.internal_format,
                    self.dimensions
                ),
                unit: gl::COLOR_ATTACHMENT0 + idx as u32
            });
        }

        let depth_stencil = if let Some(depth_stencil) = self.depth_stencil {
            let depth_stencil_handle = unsafe {
                let mut handle = 0;
                gl::GenTextures(1, &mut handle);
                handle
            };
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, depth_stencil_handle);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    depth_stencil.internal_format.as_api(),
                    self.dimensions.x,
                    self.dimensions.y,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    std::ptr::null()
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            }

            unsafe {
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    match depth_stencil.kind {
                        super::DepthStencil::Depth => gl::DEPTH_ATTACHMENT,
                        super::DepthStencil::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
                    },
                    gl::TEXTURE_2D,
                    depth_stencil_handle,
                    0
                );
            }

            Some(super::DepthStencilTexture {
                kind: depth_stencil.kind,
                texture: gpu::Texture2d::from_handle(
                    depth_stencil_handle,
                    depth_stencil.internal_format.into(),
                    depth_stencil.internal_format,
                    self.dimensions
                ),
            })
        } else {
            None
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Viewport {
            colours,
            depth_stencil,
            dimensions: self.dimensions,
            handle
        }
    }
}

