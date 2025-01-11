pub mod gpu;
pub mod cpu;
pub mod data;
pub mod texture_2d;
pub mod texture_3d;

pub use gpu::Texture2d as GpuTexture2d;
pub use cpu::Texture2d as CpuTexture2d;
pub use texture_2d::Texture2d as Texture2d;

pub use gpu::Texture3d as GpuTexture3d;
pub use cpu::Texture3d as CpuTexture3d;
pub use texture_3d::Texture3d as Texture3d;

#[derive(Debug, Copy, Clone)]
pub enum Component {
    Depth,
    DepthStencil,
    R,
    RG,
    RGB,
    RGBA,
    IntR,
    IntRG,
    IntRGB,
    IntRGBA,
}

impl Component {
    pub(super) fn as_api(self) -> gl::types::GLenum {
        match self {
            Component::R => gl::RED,
            Component::RG => gl::RG,
            Component::RGB => gl::RGB,
            Component::RGBA => gl::RGBA,
            Component::IntR => gl::RED_INTEGER,
            Component::IntRG => gl::RG_INTEGER,
            Component::IntRGB => gl::RGB_INTEGER,
            Component::IntRGBA => gl::RGBA_INTEGER,
            Component::Depth => gl::DEPTH_COMPONENT,
            Component::DepthStencil => gl::DEPTH_STENCIL,
        }
    }

    pub fn component_count(&self) -> usize {
        match self {
            Component::R => 1,
            Component::RG => 2,
            Component::RGB => 3,
            Component::RGBA => 4,
            Component::IntR => 1,
            Component::IntRG => 2,
            Component::IntRGB => 3,
            Component::IntRGBA => 4,
            Component::Depth => 1,
            Component::DepthStencil => 2,
        }
    }
}

impl From<gpu::SizedComponent> for Component {
    fn from(sized: gpu::SizedComponent) -> Component {
        match sized {
            gpu::SizedComponent::R8 => Component::R,
            gpu::SizedComponent::R16 => Component::R,
            gpu::SizedComponent::FloatR16 => Component::R,
            gpu::SizedComponent::FloatR32 => Component::R,
            gpu::SizedComponent::IntR8 => Component::R,
            gpu::SizedComponent::UnsignedIntR8 => Component::R,
            gpu::SizedComponent::IntR16 => Component::R,
            gpu::SizedComponent::UnsignedIntR16 => Component::R,
            gpu::SizedComponent::IntR32 => Component::R,
            gpu::SizedComponent::UnsignedIntR32 => Component::R,
            gpu::SizedComponent::NormalR8 => Component::R,
            gpu::SizedComponent::NormalR16 => Component::R,
            gpu::SizedComponent::RG8 => Component::RG,
            gpu::SizedComponent::RG16 => Component::RG,
            gpu::SizedComponent::FloatRG16 => Component::RG,
            gpu::SizedComponent::FloatRG32 => Component::RG,
            gpu::SizedComponent::IntRG8 => Component::RG,
            gpu::SizedComponent::UnsignedIntRG8 => Component::RG,
            gpu::SizedComponent::IntRG16 => Component::RG,
            gpu::SizedComponent::UnsignedIntRG16 => Component::RG,
            gpu::SizedComponent::IntRG32 => Component::RG,
            gpu::SizedComponent::UnsignedIntRG32 => Component::RG,
            gpu::SizedComponent::NormalRG8 => Component::RG,
            gpu::SizedComponent::NormalRG16 => Component::RG,
            gpu::SizedComponent::RGB332 => Component::RGB,
            gpu::SizedComponent::RGB4 => Component::RGB,
            gpu::SizedComponent::RGB5 => Component::RGB,
            gpu::SizedComponent::RGB8 => Component::RGB,
            gpu::SizedComponent::RGB10 => Component::RGB,
            gpu::SizedComponent::RGB12 => Component::RGB,
            gpu::SizedComponent::SRGB8 => Component::RGB,
            gpu::SizedComponent::FloatRGB16 => Component::RGB,
            gpu::SizedComponent::FloatRGB32 => Component::RGB,
            gpu::SizedComponent::FloatR11G11B10 => Component::RGB,
            gpu::SizedComponent::IntRGB8 => Component::RGB,
            gpu::SizedComponent::UnsignedIntRGB8 => Component::RGB,
            gpu::SizedComponent::IntRGB16 => Component::RGB,
            gpu::SizedComponent::UnsignedIntRGB16 => Component::RGB,
            gpu::SizedComponent::IntRGB32 => Component::RGB,
            gpu::SizedComponent::UnsignedIntRGB32 => Component::RGB,
            gpu::SizedComponent::NormalRGB8 => Component::RGB,
            gpu::SizedComponent::NormalRGB16 => Component::RGB,
            gpu::SizedComponent::RGBA2 => Component::RGBA,
            gpu::SizedComponent::RGBA4 => Component::RGBA,
            gpu::SizedComponent::RGB5A1 => Component::RGBA,
            gpu::SizedComponent::RGBA8 => Component::RGBA,
            gpu::SizedComponent::RGB10A2 => Component::RGBA,
            gpu::SizedComponent::UnsignedIntRGB10A2 => Component::RGBA,
            gpu::SizedComponent::RGBA12 => Component::RGBA,
            gpu::SizedComponent::RGBA16 => Component::RGBA,
            gpu::SizedComponent::SRGB8A8 => Component::RGBA,
            gpu::SizedComponent::FloatRGBA16 => Component::RGBA,
            gpu::SizedComponent::FloatRGBA32 => Component::RGBA,
            gpu::SizedComponent::IntRGBA8 => Component::RGBA,
            gpu::SizedComponent::UnsignedIntRGBA8 => Component::RGBA,
            gpu::SizedComponent::IntRGBA16 => Component::RGBA,
            gpu::SizedComponent::UnsignedIntRGBA16 => Component::RGBA,
            gpu::SizedComponent::IntRGBA32 => Component::RGBA,
            gpu::SizedComponent::UnsignedIntRGBA32 => Component::RGBA,
            gpu::SizedComponent::NormalRGBA8 => Component::RGBA,
            gpu::SizedComponent::Depth => Component::Depth,
            gpu::SizedComponent::DepthStencil => Component::DepthStencil,
            gpu::SizedComponent::Depth16 => Component::Depth,
            gpu::SizedComponent::Depth24 => Component::Depth,
            gpu::SizedComponent::FloatDepth32 => Component::Depth,
        }
    }
}
