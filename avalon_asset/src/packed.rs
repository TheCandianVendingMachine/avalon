use crate::bundle;

pub struct Packed {
    bundle: bundle::Bundle
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
                let uuid_path = asset.metadata.uuid.to_string();
                let directory = std::path::Path::new(&uuid_path);
                archive.add_directory_from_path(
                    directory,
                    options
                )?;

                archive.start_file_from_path(
                    directory.join("metadata.json"),
                    options
                )?;
                let meta_json = json::to_string(&asset.metadata);
                archive.write(meta_json.as_bytes())?;

                archive.start_file_from_path(
                    directory.join(".stored"),
                    zip::write::SimpleFileOptions::default()
                )?;

                let mut data_file = std::fs::OpenOptions::new()
                    .read(true)
                    .open(asset.metadata.filepath.as_ref().expect("Need to have filepath to bundle"))?;
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
    use super::Packed;
    use miniserde::json;
    use zip;

    impl Packed {
        pub fn read_from_file(path: impl AsRef<std::path::Path>) -> Packed {
            let path = path.as_ref();
            let packed_file = std::fs::File::options()
                .read(true)
                .open(path.to_path_buf())
                .unwrap();

            let mut reader = zip::ZipArchive::new(packed_file).unwrap();
            for i in 0..reader.len() {
                let file = reader.by_index(i).unwrap();
                dbg!(file.name());
            };

            panic!();
        }
    }
}

impl From<bundle::Bundle> for Packed {
    fn from(bundle: bundle::Bundle) -> Packed {
        Packed {
            bundle
        }
    }
}

impl From<Packed> for bundle::Bundle {
    fn from(packed: Packed) -> bundle::Bundle {
        packed.bundle
    }
}
