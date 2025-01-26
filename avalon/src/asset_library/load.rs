use avalon_asset as assets;
use crate::asset_library::Library;
use crate::texture;
use crate::texture::data;
use crate::texture::gpu::{ self, UniqueTexture, Arguments2d, Texture2d, ManagedTexture };

impl Library {
    pub fn load_texture(&self, texture_info: assets::texture::Texture, data: &Vec<u8>) -> ManagedTexture<Texture2d> {
        let (image, dimensions) = data::Data::from_buffer(data.to_vec());
        let (components, size) = match texture_info.colour_space {
            assets::texture::ColourSpace::RGBA => (
                texture::Component::RGBA,
                gpu::SizedComponent::RGBA8,
            ),
            assets::texture::ColourSpace::SRGBA => (
                texture::Component::RGBA,
                gpu::SizedComponent::SRGB8A8,
            ),
            assets::texture::ColourSpace::RGB => (
                texture::Component::RGB,
                gpu::SizedComponent::RGB8,
            ),
            assets::texture::ColourSpace::SRGB => (
                texture::Component::RGB,
                gpu::SizedComponent::SRGB8,
            ),
        };

        let arguments = Arguments2d {
            dimensions,
            internal_components: components,
            internal_size: size,
            mipmap_type: gpu::Mipmap::None,
            data: Some(image)
        };

        Texture2d::generate(arguments).as_managed()
    }

}
