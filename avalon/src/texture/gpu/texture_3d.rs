use nalgebra_glm::IVec3;
use crate::texture::{ texture_3d, data, data::Data, Component };
use crate::texture::gpu::{ UniqueTexture, Sampler, Image, ImageAttachment, TextureAttachment, TextureDimension, Access, Mipmap, SizedComponent };

pub struct TextureBind3d<'t> {
    texture: &'t mut Texture3d
}

impl TextureBind3d<'_> {
    pub fn fetch_pixels(&self, mip_level: u32) -> Data {
        let pixels = unsafe {
            let volume = self.texture.dimensions.x * self.texture.dimensions.y * self.texture.dimensions.z;
            let count = self.texture.internal_size.component_count() * volume as usize;
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
            components: self.texture.internal_components
        }
    }

    pub fn write_pixels(&self, mip_level: u32, data: Data) {
        unsafe {
            gl::MemoryBarrier(gl::TEXTURE_UPDATE_BARRIER_BIT);
            gl::TexImage3D(
                gl::TEXTURE_3D,
                mip_level as i32,
                self.texture.internal_size.as_api(),
                self.texture.dimensions.x,
                self.texture.dimensions.y,
                self.texture.dimensions.z,
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
            gl::TexImage3D(
                gl::TEXTURE_3D,
                mip_level as i32,
                self.texture.internal_size.as_api(),
                self.texture.dimensions.x,
                self.texture.dimensions.y,
                self.texture.dimensions.z,
                0,
                self.texture.internal_size.map_to_cpu_types(),
                gl::RGBA,
                pixels.as_ptr()
            );
        }
    }
}

impl Drop for TextureBind3d<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_3D, 0);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Texture3d {
    handle: gl::types::GLuint,
    internal_components: Component,
    internal_size: SizedComponent,
    dimensions: IVec3,
    mip_levels: u32,
}

impl Texture3d {
    pub fn generate(arguments: Arguments) -> Texture3d {
        Texture3d::generate_many::<1>(arguments)[0]
    }

    pub fn generate_many<const COUNT: usize>(arguments: Arguments) -> [Texture3d; COUNT] {
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
                gl::BindTexture(gl::TEXTURE_3D, *handle);
                gl::TexImage3D(
                    gl::TEXTURE_3D,
                    0,
                    arguments.internal_size.as_api(),
                    arguments.dimensions.x,
                    arguments.dimensions.y,
                    arguments.dimensions.z,
                    0,
                    data_format,
                    data_type,
                    data
                );
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                gl::BindTexture(gl::TEXTURE_3D, 0);
            }
        }

        core::array::from_fn(|idx| Texture3d {
            handle: handles[idx],
            internal_components: arguments.internal_components,
            internal_size: arguments.internal_size,
            dimensions: arguments.dimensions,
            mip_levels: 0
        })
    }

    pub fn generate_storage(arguments: Arguments, levels: u32) -> Texture3d {
        Texture3d::generate_many_storage::<1>(arguments, levels)[0]
    }

    pub fn generate_many_storage<const COUNT: usize>(arguments: Arguments, levels: u32) -> [Texture3d; COUNT] {
        arguments.internal_size.verify(arguments.internal_components);
        let handles = unsafe {
            let mut textures = [0; COUNT];
            gl::GenTextures(COUNT as i32, textures.as_mut_ptr());
            textures
        };

        for handle in handles.iter() {
            unsafe {
                gl::BindTexture(gl::TEXTURE_3D, *handle);
                gl::TexStorage3D(
                    gl::TEXTURE_3D,
                    levels as i32,
                    arguments.internal_size.as_api() as u32,
                    arguments.dimensions.x,
                    arguments.dimensions.y,
                    arguments.dimensions.z,
                );
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                gl::BindTexture(gl::TEXTURE_3D, 0);
            }
        }

        core::array::from_fn(|idx| Texture3d {
            handle: handles[idx],
            internal_components: arguments.internal_components,
            internal_size: arguments.internal_size,
            dimensions: arguments.dimensions,
            mip_levels: levels
        })
    }

    pub fn bind(&mut self) -> TextureBind3d {
        unsafe {
            gl::BindTexture(gl::TEXTURE_3D, self.handle);
        }
        TextureBind3d {
            texture: self
        }
    }
}

impl UniqueTexture for Texture3d {
    fn levels(&self) -> u32 {
        self.mip_levels
    }
    fn handle(&self) -> u32 {
        self.handle
    }
}

impl Sampler for Texture3d {
    fn sampler<'t>(&'t self, unit: gl::types::GLenum) -> TextureAttachment<'t> {
        unsafe {
            gl::ActiveTexture(unit);
            gl::BindTexture(gl::TEXTURE_3D, self.handle);
        }
        TextureAttachment {
            _lifetime: &std::marker::PhantomData,
            dimension: TextureDimension::Dimension3d,
            unit
        }
    }
}

impl Image for Texture3d {
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
                gl::TRUE,
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
            dimension: TextureDimension::Dimension3d,
            unit: idx
        }
    }
}

#[derive(Clone)]
pub struct Arguments {
    pub dimensions: IVec3,
    pub internal_components: Component,
    pub internal_size: SizedComponent,
    pub mipmap_type: Mipmap,
    pub data: Option<Data>
}

pub struct TextureBuilder3d {
    meta_builder: texture_3d::Texture3dBuilder,
    size: SizedComponent,
    mipmap_type: Mipmap
}

impl TextureBuilder3d {
    pub fn new(meta_builder: texture_3d::Texture3dBuilder) -> TextureBuilder3d {
        TextureBuilder3d {
            meta_builder,
            size: SizedComponent::RGBA8,
            mipmap_type: Mipmap::None,
        }
    }
    pub fn mipmap(mut self, mipmap: Mipmap) -> Self {
        self.mipmap_type = mipmap;
        self
    }

    pub fn finish(mut self) -> texture_3d::Texture3dBuilder {
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
