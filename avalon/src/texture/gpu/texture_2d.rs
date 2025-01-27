use nalgebra_glm::IVec2;
use crate::texture::{ texture_2d, data, data::Data, Component };
use crate::texture::gpu::{ TextureDimension, ImageAttachment, TextureAttachment, UniqueTexture, Image, Sampler, Mipmap, SizedComponent, Access };
use crate::asset_library;

pub struct TextureBind2d<'t> {
    texture: &'t mut Texture2d
}

impl TextureBind2d<'_> {
    pub fn fetch_pixels(&self, mip_level: u32) -> Data {
        let pixels = unsafe {
            let area = self.texture.dimensions.x * self.texture.dimensions.y;
            let count = self.texture.internal_size.component_count() * area as usize;
            let mut pixels = data::Pixels::from_api(self.texture.internal_size.map_to_cpu_types(), count);

            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
            gl::GetTexImage(
                gl::TEXTURE_3D,
                mip_level as i32,
                self.texture.internal_components.as_api(),
                self.texture.internal_size.map_to_cpu_types(),
                pixels.as_mut()
            );
            pixels
        };
        Data {
            data: pixels,
            components: self.texture.internal_components,
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

    pub fn set_name(&self, name: impl Into<String>) {
        let name = name.into();
        unsafe {
            gl::ObjectLabel(
                gl::TEXTURE,
                self.texture.handle,
                name.len() as i32,
                name.as_ptr() as *const i8
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

#[derive(Debug, Copy, Clone)]
pub struct Texture2d {
    handle: gl::types::GLuint,
    pub(crate) internal_components: Component,
    pub(crate) internal_size: SizedComponent,
    dimensions: IVec2,
    mip_levels: u32,
}

impl Texture2d {
    pub(crate) fn from_handle(
        handle: gl::types::GLuint,
        internal_components: Component,
        internal_size: SizedComponent,
        dimensions: IVec2
    ) -> Texture2d {
        Texture2d {
            handle,
            internal_components,
            internal_size,
            dimensions,
            mip_levels: 0
        }
    }

    pub fn generate(arguments: Arguments) -> Texture2d {
        Texture2d::generate_many::<1>(arguments)[0]
    }

    pub fn generate_many<const COUNT: usize>(arguments: Arguments) -> [Texture2d; COUNT] {
        arguments.internal_size.verify(arguments.internal_components);
        let handles = unsafe {
            let mut textures = [0; COUNT];
            gl::GenTextures(COUNT as i32, textures.as_mut_ptr());
            textures
        };

        let (data_format, data_type, data) = if let Some(data) = arguments.data.as_ref() {
            (
                data.components.as_api(),
                data.data.as_api(),
                data.data.as_ptr()
            )
        } else {
            (
                arguments.internal_components.as_api(),
                arguments.internal_size.map_to_cpu_types(),
                std::ptr::null()
            )
        };

        for handle in handles.iter() {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, *handle);
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
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }

        core::array::from_fn(|idx| Texture2d {
            handle: handles[idx],
            internal_components: arguments.internal_components,
            internal_size: arguments.internal_size,
            dimensions: arguments.dimensions,
            mip_levels: 0
        })
    }

    pub fn bind(&mut self) -> TextureBind2d {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
        TextureBind2d {
            texture: self
        }
    }

    pub fn dimensions(&self) -> IVec2 {
        self.dimensions
    }
}

impl asset_library::Asset for Texture2d {}

impl UniqueTexture for Texture2d {
    fn levels(&self) -> u32 {
        0
    }
    fn handle(&self) -> u32 {
        self.handle
    }
}

impl Sampler for Texture2d {
    fn sampler<'t>(&'t self, unit: gl::types::GLenum) -> TextureAttachment<'t> {
        unsafe {
            gl::ActiveTexture(unit);
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
        TextureAttachment {
            _lifetime: &std::marker::PhantomData,
            dimension: TextureDimension::Dimension2d,
            unit
        }
    }
}

impl Image for Texture2d {
    fn image<'t>(&'t self, idx: gl::types::GLuint, access: Access) -> ImageAttachment<'t> {
        let level = match access {
            Access::Read(level) => level,
            Access::Write(level) => level,
            Access::ReadWrite(level) => level,
        };
        unsafe {
            gl::BindImageTexture(
                idx,
                self.handle,
                level as i32,
                gl::FALSE,
                0,
                match access {
                    Access::Read(_) => gl::READ_ONLY,
                    Access::Write(_) => gl::WRITE_ONLY,
                    Access::ReadWrite(_) => gl::READ_WRITE,
                },
                self.internal_size.as_api() as u32
            );
        }
        ImageAttachment {
            _lifetime: &std::marker::PhantomData,
            dimension: TextureDimension::Dimension2d,
            unit: idx
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
