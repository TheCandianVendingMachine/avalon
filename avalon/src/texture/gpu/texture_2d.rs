use nalgebra_glm::IVec2;
use crate::texture::{ texture_2d, data, data::Data, Component };
use crate::texture::gpu::{ Mipmap, SizedComponent };

pub struct TextureBind2d<'t> {
    texture: &'t mut Texture2d
}

impl TextureBind2d<'_> {
    pub fn fetch_pixels(&self, mip_level: u32) -> Data {
        let pixels = unsafe {
            let mut pixels = data::Pixels::from_api(self.texture.internal_size.map_to_cpu_types(), self.texture.internal_size.component_count());
            gl::GetTextureImage(
                self.texture.handle,
                mip_level as i32,
                self.texture.internal_components.as_api(),
                self.texture.internal_size.map_to_cpu_types(),
                self.texture.internal_size.component_count() as i32,
                pixels.as_mut()
            );
            pixels
        };
        Data {
            data: pixels,
            components: self.texture.internal_components
        }
    }

    pub fn write_pixels(&self, mip_level: u32, data: Data) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                mip_level as i32,
                self.texture.internal_size.as_api(),
                self.texture.dimensions.x,
                self.texture.dimensions.y,
                0,
                data.components.as_api(),
                data.data.as_api(),
                data.data.as_ptr()
            );
        }
    }

    pub fn clear(&self, mip_level: u32) {
        let pixels = data::Pixels::from_api(self.texture.internal_size.map_to_cpu_types(), self.texture.internal_size.component_count());
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                mip_level as i32,
                self.texture.internal_size.as_api(),
                self.texture.dimensions.x,
                self.texture.dimensions.y,
                0,
                self.texture.internal_size.map_to_cpu_types(),
                gl::RGBA,
                pixels.as_ptr()
            );
        }
    }
}

impl Drop for TextureBind2d<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

#[derive(Clone)]
pub struct TextureAttachment2d<'t> {
    texture: &'t Texture2d,
    unit: gl::types::GLenum
}

impl Drop for TextureAttachment2d<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl TextureAttachment2d<'_> {
    pub fn unit(&self) -> gl::types::GLenum {
        self.unit
    }
}

#[derive(Clone)]
pub struct Texture2d {
    handle: gl::types::GLuint,
    internal_components: Component,
    internal_size: SizedComponent,
    dimensions: IVec2,
}

impl Texture2d {
    pub fn generate(arguments: Arguments) -> Texture2d {
        arguments.verify();
        let handle = unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            texture
        };

        let (data_format, data_type, data) = if let Some(data) = arguments.data {
            (
                data.components.as_api(),
                data.data.as_api(),
                data.data.as_ptr()
            )
        } else {
            (gl::RED, gl::UNSIGNED_BYTE, std::ptr::null())
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, handle);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                arguments.internal_size.as_api(),
                arguments.dimensions.x,
                arguments.dimensions.y,
                0,
                data_format,
                data_type,
                data
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture2d {
            handle,
            internal_components: arguments.internal_components,
            internal_size: arguments.internal_size,
            dimensions: arguments.dimensions
        }
    }

    pub fn bind(&mut self) -> TextureBind2d {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
        TextureBind2d {
            texture: self
        }
    }

    pub fn attach(&self, unit: gl::types::GLenum) -> TextureAttachment2d {
        unsafe {
            gl::ActiveTexture(unit);
            gl::BindTexture(unit, self.handle);
        }
        TextureAttachment2d {
            texture: self,
            unit
        }
    }
}

impl Drop for Texture2d {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
    }
}

#[derive(Clone)]
pub struct Arguments {
    pub dimensions: IVec2,
    pub internal_components: Component,
    pub internal_size: SizedComponent,
    pub mipmap_type: Mipmap,
    pub data: Option<Data>
}

impl Arguments {
    fn verify(&self) {
        match self.internal_components {
            Component::IntR | Component::R => {
                match self.internal_size {
                    SizedComponent::R8 => (),
                    SizedComponent::NormalR8 => (),
                    SizedComponent::R16 => (),
                    SizedComponent::NormalR16 => (),
                    SizedComponent::FloatR16 => (),
                    SizedComponent::FloatR32 => (),
                    SizedComponent::IntR8 => (),
                    SizedComponent::UnsignedIntR8 => (),
                    SizedComponent::IntR16 => (),
                    SizedComponent::UnsignedIntR16 => (),
                    SizedComponent::IntR32 => (),
                    SizedComponent::UnsignedIntR32 => (),
                    _ => panic!("Mismatched components and desired size: components[{:?}] vs size[{:?}]", self.internal_components, self.internal_size),
                }
            },
            Component::IntRG | Component::RG => {
                match self.internal_size {
                    SizedComponent::RG8 => (),
                    SizedComponent::NormalRG8 => (),
                    SizedComponent::RG16 => (),
                    SizedComponent::NormalRG16 => (),
                    SizedComponent::FloatRG16 => (),
                    SizedComponent::FloatRG32 => (),
                    SizedComponent::IntRG8 => (),
                    SizedComponent::UnsignedIntRG8 => (),
                    SizedComponent::IntRG16 => (),
                    SizedComponent::UnsignedIntRG16 => (),
                    SizedComponent::IntRG32 => (),
                    SizedComponent::UnsignedIntRG32 => (),
                    _ => panic!("Mismatched components and desired size: components[{:?}] vs size[{:?}]", self.internal_components, self.internal_size),
                }
            },
            Component::IntRGB | Component::RGB => {
                match self.internal_size {
                    SizedComponent::RGB332 => (),
                    SizedComponent::RGB4 => (),
                    SizedComponent::RGB5 => (),
                    SizedComponent::RGB8 => (),
                    SizedComponent::NormalRGB8 => (),
                    SizedComponent::RGB10 => (),
                    SizedComponent::RGB12 => (),
                    SizedComponent::NormalRGB16 => (),
                    SizedComponent::RGBA2 => (),
                    SizedComponent::RGBA4 => (),
                    SizedComponent::SRGB8 => (),
                    SizedComponent::FloatRGB16 => (),
                    SizedComponent::FloatRGB32 => (),
                    SizedComponent::IntRGB8 => (),
                    SizedComponent::UnsignedIntRGB8 => (),
                    SizedComponent::IntRGB16 => (),
                    SizedComponent::UnsignedIntRGB16 => (),
                    SizedComponent::IntRGB32 => (),
                    SizedComponent::UnsignedIntRGB32 => (),
                    _ => panic!("Mismatched components and desired size: components[{:?}] vs size[{:?}]", self.internal_components, self.internal_size),
                }
            },
            Component::IntRGBA | Component::RGBA => {
                match self.internal_size {
                    SizedComponent::RGB5A1 => (),
                    SizedComponent::RGBA8 => (),
                    SizedComponent::NormalRGBA8 => (),
                    SizedComponent::RGB10A2 => (),
                    SizedComponent::RGBA12 => (),
                    SizedComponent::SRGB8A8 => (),
                    SizedComponent::FloatRGBA16 => (),
                    SizedComponent::FloatRGBA32 => (),
                    SizedComponent::IntRGBA8 => (),
                    SizedComponent::UnsignedIntRGBA8 => (),
                    SizedComponent::IntRGBA16 => (),
                    SizedComponent::UnsignedIntRGBA16 => (),
                    SizedComponent::IntRGBA32 => (),
                    SizedComponent::UnsignedIntRGBA32 => (),
                    _ => panic!("Mismatched components and desired size: components[{:?}] vs size[{:?}]", self.internal_components, self.internal_size),
                }
            },
            Component::Depth => {
                match self.internal_size {
                    SizedComponent::Depth => (),
                    SizedComponent::Depth16 => (),
                    SizedComponent::Depth24 => (),
                    SizedComponent::FloatDepth32 => (),
                    _ => panic!("Mismatched components and desired size: components[{:?}] vs size[{:?}]", self.internal_components, self.internal_size),
                }
            },
            Component::DepthStencil => {},
        };
    }
}

pub struct TextureBuilder2d {
    meta_builder: texture_2d::Texture2dBuilder,
    size: SizedComponent,
    mipmap_type: Mipmap
}

impl TextureBuilder2d {
    pub fn new(meta_builder: texture_2d::Texture2dBuilder) -> TextureBuilder2d {
        TextureBuilder2d {
            meta_builder,
            size: SizedComponent::RGBA8,
            mipmap_type: Mipmap::None,
        }
    }
    pub fn mipmap(mut self, mipmap: Mipmap) -> Self {
        self.mipmap_type = mipmap;
        self
    }

    pub fn finish(mut self) -> texture_2d::Texture2dBuilder {
        self.meta_builder.gpu_texture_arguments = Some(Arguments {
            dimensions: self.meta_builder.dimensions,
            internal_components: Component::RGBA,
            internal_size: self.size,
            data: None,
            mipmap_type: self.mipmap_type
        });
        self.meta_builder
    }
}
