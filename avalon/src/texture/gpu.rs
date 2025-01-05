use nalgebra_glm::IVec2;
use crate::shader;
use crate::texture::{ texture_2d, data::Data, Component };

#[derive(Debug)]
pub enum SizedComponent {
    R8,
    R16,
    RG8,
    RG16,
    RGB332,
    RGB4,
    RGB5,
    RGB8,
    RGB10,
    RGB12,
    RGBA2,
    RGBA4,
    RGB5A1,
    RGBA8,
    RGB10A2,
    UnsignedIntRGB10A2,
    RGBA12,
    RGBA16,
    SRGB8,
    SRGB8A8,
    FloatR16,
    FloatRG16,
    FloatRGB16,
    FloatRGBA16,
    FloatR32,
    FloatRG32,
    FloatRGB32,
    FloatRGBA32,
    FloatR11G11B10,
    IntR8,
    UnsignedIntR8,
    IntR16,
    UnsignedIntR16,
    IntR32,
    UnsignedIntR32,
    IntRG8,
    UnsignedIntRG8,
    IntRG16,
    UnsignedIntRG16,
    IntRG32,
    UnsignedIntRG32,
    IntRGB8,
    UnsignedIntRGB8,
    IntRGB16,
    UnsignedIntRGB16,
    IntRGB32,
    UnsignedIntRGB32,
    IntRGBA8,
    UnsignedIntRGBA8,
    IntRGBA16,
    UnsignedIntRGBA16,
    IntRGBA32,
    UnsignedIntRGBA32,
    NormalR8,
    NormalR16,
    NormalRG8,
    NormalRG16,
    NormalRGB8,
    NormalRGB16,
    NormalRGBA8,
    Depth,
    DepthStencil,
    Depth16,
    Depth24,
    FloatDepth32,
}

impl SizedComponent {
    fn api_size(self) -> gl::types::GLint {
        (match self {
            SizedComponent::R8 => gl::R8,
            SizedComponent::R16 => gl::R16,
            SizedComponent::RG8 => gl::RG8,
            SizedComponent::RG16 => gl::RG16,
            SizedComponent::RGB332 => gl::R3_G3_B2,
            SizedComponent::RGB4 => gl::RGB4,
            SizedComponent::RGB5 => gl::RGB5,
            SizedComponent::RGB8 => gl::RGB8,
            SizedComponent::RGB10 => gl::RGB10,
            SizedComponent::RGB12 => gl::RGB12,
            SizedComponent::RGBA2 => gl::RGBA2,
            SizedComponent::RGBA4 => gl::RGBA4,
            SizedComponent::RGB5A1 => gl::RGB5_A1,
            SizedComponent::RGBA8 => gl::RGBA8,
            SizedComponent::RGB10A2 => gl::RGB10_A2,
            SizedComponent::UnsignedIntRGB10A2 => gl::RGB10_A2UI,
            SizedComponent::RGBA12 => gl::RGBA12,
            SizedComponent::RGBA16 => gl::RGBA16,
            SizedComponent::SRGB8 => gl::SRGB8,
            SizedComponent::SRGB8A8 => gl::SRGB8_ALPHA8,
            SizedComponent::FloatR16 => gl::R16F,
            SizedComponent::FloatRG16 => gl::RG16F,
            SizedComponent::FloatRGB16 => gl::RGB16F,
            SizedComponent::FloatRGBA16 => gl::RGBA16F,
            SizedComponent::FloatR32 => gl::R32F,
            SizedComponent::FloatRG32 => gl::RG32F,
            SizedComponent::FloatRGB32 => gl::RGB32F,
            SizedComponent::FloatRGBA32 => gl::RGBA32F,
            SizedComponent::FloatR11G11B10 => gl::R11F_G11F_B10F,
            SizedComponent::IntR8 => gl::R8I,
            SizedComponent::UnsignedIntR8 => gl::R8UI,
            SizedComponent::IntR16 => gl::R16I,
            SizedComponent::UnsignedIntR16 => gl::R16UI,
            SizedComponent::IntR32 => gl::R32I,
            SizedComponent::UnsignedIntR32 => gl::R32UI,
            SizedComponent::IntRG8 => gl::RG8I,
            SizedComponent::UnsignedIntRG8 => gl::RG8UI,
            SizedComponent::IntRG16 => gl::RG16I,
            SizedComponent::UnsignedIntRG16 => gl::RG16UI,
            SizedComponent::IntRG32 => gl::RG32I,
            SizedComponent::UnsignedIntRG32 => gl::RG32UI,
            SizedComponent::IntRGB8 => gl::RGB8I,
            SizedComponent::UnsignedIntRGB8 => gl::RGB8UI,
            SizedComponent::IntRGB16 => gl::RGB16I,
            SizedComponent::UnsignedIntRGB16 => gl::RGB16UI,
            SizedComponent::IntRGB32 => gl::RGB32I,
            SizedComponent::UnsignedIntRGB32 => gl::RGB32UI,
            SizedComponent::IntRGBA8 => gl::RGBA8I,
            SizedComponent::UnsignedIntRGBA8 => gl::RGBA8UI,
            SizedComponent::IntRGBA16 => gl::RGBA16I,
            SizedComponent::UnsignedIntRGBA16 => gl::RGBA16UI,
            SizedComponent::IntRGBA32 => gl::RGBA32I,
            SizedComponent::UnsignedIntRGBA32 => gl::RGBA32UI,
            SizedComponent::NormalR8 => gl::R8_SNORM,
            SizedComponent::NormalR16 => gl::R16_SNORM,
            SizedComponent::NormalRG8 => gl::RG8_SNORM,
            SizedComponent::NormalRG16 => gl::RG16_SNORM,
            SizedComponent::NormalRGB8 => gl::RGB8_SNORM,
            SizedComponent::NormalRGB16 => gl::RGB16_SNORM,
            SizedComponent::NormalRGBA8 => gl::RGBA8_SNORM,
            SizedComponent::Depth => gl::DEPTH_COMPONENT,
            SizedComponent::DepthStencil => gl::DEPTH_STENCIL,
            SizedComponent::Depth16 => gl::DEPTH_COMPONENT16,
            SizedComponent::Depth24 => gl::DEPTH_COMPONENT24,
            SizedComponent::FloatDepth32 => gl::DEPTH_COMPONENT32,
        }) as gl::types::GLint
    }
}

pub enum Mipmap {
    None,
    Inbuilt{ count: u8 },
    Custom{ count: u8, shader: shader::Program }
}

pub struct Arguments {
    dimensions: IVec2,
    pub(super) internal_components: Component,
    internal_size: SizedComponent,
    mipmap_type: Mipmap,
    data: Option<Data>
}

impl Arguments {
    fn verify(&self) {
        match self.internal_components {
            Component::R => {
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
            Component::RG => {
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
            Component::RGB => {
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
            Component::RGBA => {
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

pub struct Texture2d {
    handle: gl::types::GLuint
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
                arguments.internal_size.api_size(),
                arguments.dimensions.x,
                arguments.dimensions.y,
                0,
                data_format,
                data_type,
                data
            );
        }

        Texture2d {
            handle
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

pub struct TextureBuilder2d {
    meta_builder: texture_2d::Texture2dBuilder,
    size: SizedComponent,
    mipmap_type: Mipmap
}

impl TextureBuilder2d {
    pub(super) fn new(meta_builder: texture_2d::Texture2dBuilder) -> TextureBuilder2d {
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
