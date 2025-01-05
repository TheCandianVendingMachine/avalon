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
        arguments.internal_size.verify(arguments.internal_components);
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
            gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
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

    pub fn vram_data(mut self, size: SizedComponent) -> Self {
        self.size = size;
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
