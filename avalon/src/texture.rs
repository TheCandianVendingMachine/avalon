pub mod gpu;
pub mod cpu;
pub mod data;
pub mod texture_2d;

pub use gpu::Texture2d as GpuTexture2d;
pub use cpu::Texture as CpuTexture;

pub use texture_2d::Texture2d as Texture2d;

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
    fn as_api(self) -> gl::types::GLenum {
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
}
