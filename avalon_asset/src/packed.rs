use std::collections::HashMap;
use crate::{ bundle, asset };

pub struct Packed {
    pub bundle: bundle::Bundle,
    pub data_map: HashMap<asset::Metadata, Vec<u8>>
}

impl Packed {
    pub const EXTENSION: &str = "bundle";
}

#[cfg(feature = "write")]
pub mod write {
    use super::Packed;
    use crate::error;
    use std::io::Write;
    use miniserde::json;
    use zip;

    impl Packed {
        pub fn pack_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), error::PackError> {
            let path = path.as_ref();
            let mut file = std::fs::File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(&if path.extension().is_some() || path.is_file() {
                    path.to_path_buf()
                } else {
                    path.join(self.bundle.name.clone() + "." + Self::EXTENSION)
                })?;

            let mut archive = zip::ZipWriter::new(&file);
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Bzip2);

            archive.set_flush_on_finish_file(true);
            archive.set_comment(format!("Bundled asset generated on {}", chrono::Utc::now()));

            for asset in self.bundle.group.iter() {
                let uuid_path = asset.uuid.to_string();
                let directory = std::path::Path::new(&uuid_path);
                archive.add_directory_from_path(
                    directory,
                    options
                )?;

                archive.start_file_from_path(
                    directory.join("metadata.json"),
                    options
                )?;
                let meta_json = json::to_string(&asset);
                archive.write(meta_json.as_bytes())?;

                archive.start_file_from_path(
                    directory.join(".stored"),
                    zip::write::SimpleFileOptions::default()
                )?;

                let mut data_file = std::fs::OpenOptions::new()
                    .read(true)
                    .open(asset.filepath.as_ref().expect("Need to have filepath to bundle"))?;
                std::io::copy(&mut data_file, &mut archive)?;
            }
            archive.finish()?;
            file.flush()?;
            Ok(())
        }
    }
}

#[cfg(feature = "read")]
pub mod read {
    use crate::asset;
    use crate::error;
    use crate::packed::Packed;
    use crate::bundle::Bundle;
    use std::collections::HashMap;
    use std::io::Read;
    use miniserde::json;
    use zip;

    impl Packed {
        pub fn read_from_file(path: impl AsRef<std::path::Path>) -> Result<Packed, error::UnpackError> {
            let path = path.as_ref();
            let packed_file = std::fs::File::options()
                .read(true)
                .open(path.to_path_buf())?;

            let mut data_map: HashMap<asset::Metadata, Vec<u8>> = HashMap::new();
            let mut bundle = Bundle {
                group: Vec::new(),
                name: path.file_stem().unwrap().to_string_lossy().to_string()
            };

            let mut reader = zip::ZipArchive::new(packed_file)?;
            for i in 0..reader.len() {
                let mut file = reader.by_index(i)?;
                if file.is_file() {
                    let split_path = file.name().split('/');
                    let path: Vec<_> = split_path.collect();
                    if path.len() != 2 {
                        return Err(error::UnpackError::UnexpectedFileStructure);
                    }
                    let directory = path[0];
                    let filename = path[1];
                    match filename {
                        "metadata.json" => {
                            let mut buffer = String::new();
                            file.read_to_string(&mut buffer)?;
                            let metadata: asset::Metadata = json::from_str(buffer.as_str())?;
                            data_map.insert(metadata.clone(), Vec::new());
                            bundle.group.push(metadata.into());
                        },
                        ".stored" => {
                            if let Some((_, stored)) = data_map.iter_mut().find(|(metadata, _)| metadata.uuid.to_string() == directory) {
                                stored.resize(file.size() as usize, 0xFF);
                                file.read(stored)?;
                            } else {
                                todo!("zip file not reading in order; we need to store this data for later reads");
                            }
                        },
                        _ => {}
                    }
                }
            };

            Ok(Packed {
                bundle,
                data_map: data_map.iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect()
            })
        }
    }
}

impl From<bundle::Bundle> for Packed {
    fn from(bundle: bundle::Bundle) -> Packed {
        Packed {
            bundle,
            data_map: HashMap::new()
        }
    }
}

impl From<Packed> for bundle::Bundle {
    fn from(packed: Packed) -> bundle::Bundle {
        packed.bundle
    }
}
