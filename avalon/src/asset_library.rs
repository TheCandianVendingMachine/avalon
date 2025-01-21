use std::collections::{ HashMap, HashSet };
use avalon_asset::{ self as assets, asset, bundle, packed };
use crate::texture;
use crate::texture::data;
use crate::texture::gpu::{ self, UniqueTexture, Arguments2d, Texture2d, ManagedTexture };
use glob;

pub trait Asset: std::fmt::Debug {}

#[derive(Debug)]
pub struct Library {
    asset_library: HashMap<asset::Asset, Box<dyn Asset>>,
    bundle_library: HashSet<bundle::Bundle>
}

impl Library {
    fn load_texture(&self, texture_info: assets::texture::Texture, data: &Vec<u8>) -> ManagedTexture<Texture2d> {
        let (image, dimensions) = data::Data::from_buffer(data.to_vec());
        let (components, size) = match texture_info.colour_space {
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

    pub fn new_with_scan(scan_directory: impl AsRef<std::path::Path>) -> Library {
        let scan_directory = std::path::Path::canonicalize(scan_directory.as_ref()).unwrap();
        let bundles = glob::glob(scan_directory.join("*.bundle").to_str().unwrap())
            .unwrap()
            .filter_map(|f| f.ok())
            .filter_map(|path| packed::Packed::read_from_file(path).ok());

        let mut library = Library {
            asset_library: HashMap::new(),
            bundle_library: HashSet::new()
        };

        // todo! multithread this
        for packed in bundles {
            let bundle = packed.bundle;
            for asset in bundle.group.iter() {
                let data = packed.data_map.get(asset).unwrap();
                match asset.unit {
                    asset::Unit::Model => todo!(),
                    asset::Unit::Text(text) => todo!(),
                    asset::Unit::Shader(shader) => todo!(),
                    asset::Unit::Texture(texture) => {
                        let texture = library.load_texture(texture, data);
                        library.asset_library.insert(asset::Asset::from(asset.clone()), Box::new(texture));
                    },
                }
            }

            library.bundle_library.insert(bundle);
        }

        library
    }
}
