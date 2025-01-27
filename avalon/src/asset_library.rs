mod load;

use std::collections::{ HashMap, HashSet };
use avalon_asset::{ asset, bundle, packed };
use glob;

pub trait Asset: std::fmt::Debug {}

#[derive(Debug, Clone)]
pub struct AssetView<'v, T: Asset> {
    // we never refer to _asset, it just exists as a reference counter
    // i guess we could use the metadata if needed, but generally nah
    _asset: asset::AssetReference<'v>,
    resource: &'v T
}

impl<'v, T: Asset> std::ops::Deref for AssetView<'v, T> {
    type Target = T;
    fn deref(&self) -> &'v T {
        self.resource
    }
}

#[derive(Clone)]
pub struct BundleView<'v> {
    library: &'v Library,
    bundle: &'v bundle::Bundle
}

impl std::fmt::Debug for BundleView<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Bundle View")
            .field("bundle", self.bundle)
            .finish_non_exhaustive()
    }
}

impl<'r, 'v: 'r> BundleView<'v> {
    pub fn tag<T: Asset>(&self, asset_tag: impl Into<String>) -> Option<AssetView<'r, T>> {
        let asset_meta = self.bundle.asset(asset_tag)?;
        let (asset_reference, asset) = self.library.asset_library.get_key_value(&asset_meta.into())?;
        Some(AssetView {
            _asset: asset_reference.refer(),
            resource: {
                let ref_asset = asset.as_ref();
                let asset_ptr = &*ref_asset as *const dyn Asset;
                let asset_cvoid = asset_ptr as *const std::ffi::c_void;
                /* Proof this fuckery is safe:
                 *  We assume the programmer is competent, and that the asset_tag refers
                 *  to the type we are casting to. We have no guarantees that this will
                 *  work, but it will crash at runtime
                 */
                unsafe {
                    let ptr_asset: *const T = std::mem::transmute(asset_cvoid);
                    &*ptr_asset
                }
            }
        })
    }
}

#[derive(Debug)]
pub struct Library {
    asset_library: HashMap<asset::Asset, Box<dyn Asset>>,
    bundle_library: HashSet<bundle::Bundle>
}

impl Library {
    pub fn bundle(&self, tag: impl Into<String>) -> Option<BundleView> {
        Some(BundleView {
            library: &self,
            bundle: self.bundle_library.get(&bundle::Bundle {
                name: tag.into(),
                group: Vec::new()
            })?
        })
    }

    pub fn new_with_scan(scan_directory: impl AsRef<std::path::Path>) -> Library {
        let scan_directory = std::path::Path::canonicalize(scan_directory.as_ref()).unwrap();
        let bundles = glob::glob(scan_directory.join("*.bundle").to_str().unwrap())
            .unwrap()
            .filter_map(|f| f.ok())
            .map(|path| packed::Packed::read_from_file(path));

        let mut library = Library {
            asset_library: HashMap::new(),
            bundle_library: HashSet::new()
        };

        // todo! multithread this
        for packed in bundles {
            let Ok(packed) = packed else {
                println!("Error loading bundle: {}", packed.err().unwrap());
                continue;
            };

            let bundle = packed.bundle;
            for asset in bundle.group.iter() {
                let data = packed.data_map.get(asset).unwrap();
                match asset.unit {
                    asset::Unit::Model(model) => {
                        let model = library.load_model(model, data);
                        library.asset_library.insert(asset::Asset::from(asset.clone()), Box::new(model));
                    },
                    asset::Unit::Text(text) => todo!(),
                    asset::Unit::Shader(shader) => todo!(),
                    asset::Unit::Texture(texture) => {
                        let texture = library.load_texture(asset, texture, data);
                        library.asset_library.insert(asset::Asset::from(asset.clone()), Box::new(texture));
                    },
                }
            }

            library.bundle_library.insert(bundle);
        }

        library
    }
}
