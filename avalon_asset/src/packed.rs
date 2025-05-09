use std::collections::HashMap;
use crate::{ bundle, asset };

#[derive(Debug)]
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
    use crate::model::packed::PackedModel;
    use crate::model::ngon;
    use crate::{ error, asset };
    use std::io::Write;
    use miniserde::json;
    use obj;
    use zip;

    struct Packer {
        original_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Packer {
        fn new<R: std::io::Read>(mut reader: R) -> Packer {
            let mut original_data = vec![];
            reader.read_to_end(&mut original_data).expect("data stream needs to be readable");
            Packer {
                original_data,
                write_data: Vec::new()
            }
        }

        fn cursor(self) -> std::io::Cursor<Vec<u8>> {
            std::io::Cursor::new(self.write_data)
        }

        fn pack_model_data(mut self) -> Result<std::io::Cursor<Vec<u8>>, error::ModelUnpackError> {
            let cursor = std::io::Cursor::new(&self.original_data);
            let raw_model = obj::raw::parse_obj(cursor).map_err(error::ModelUnpackError::ObjError)?;
            let vertices: Vec<_> = raw_model.positions.iter()
                .map(|v| [v.0, v.1, v.2])
                .collect();

            let mut triangles = Vec::new();
            for polygon in raw_model.polygons {
                let obj::raw::object::Polygon::PTN(polygon) = polygon else {
                    return Err(error::ModelUnpackError::InvalidFormat);
                };
                if polygon.len() > 4 {
                    return Err(error::ModelUnpackError::TooManyVertices);
                } else if polygon.len() == 3 {
                    let (index0, uv_idx0, normal_idx0) = polygon[0];
                    let (index1, uv_idx1, normal_idx1) = polygon[1];
                    let (index2, uv_idx2, normal_idx2) = polygon[2];

                    let normal_0 = raw_model.normals[normal_idx0];
                    let normal_1 = raw_model.normals[normal_idx1];
                    let normal_2 = raw_model.normals[normal_idx2];

                    let uv_0 = raw_model.tex_coords[uv_idx0]; let uv_0 = (uv_0.0, uv_0.1);
                    let uv_1 = raw_model.tex_coords[uv_idx1]; let uv_1 = (uv_1.0, uv_1.1);
                    let uv_2 = raw_model.tex_coords[uv_idx2]; let uv_2 = (uv_2.0, uv_2.1);

                    triangles.push(ngon::Triangle::new(
                        &vertices,
                        [
                            (index0 as u32, normal_0.into(), uv_0.into()),
                            (index1 as u32, normal_1.into(), uv_1.into()),
                            (index2 as u32, normal_2.into(), uv_2.into()),
                        ]
                    ));
                } else {
                    let (index0, uv_idx0, normal_idx0) = polygon[0];
                    let (index1, uv_idx1, normal_idx1) = polygon[1];
                    let (index2, uv_idx2, normal_idx2) = polygon[2];
                    let (index3, uv_idx3, normal_idx3) = polygon[3];

                    let normal_0 = raw_model.normals[normal_idx0];
                    let normal_1 = raw_model.normals[normal_idx1];
                    let normal_2 = raw_model.normals[normal_idx2];
                    let normal_3 = raw_model.normals[normal_idx3];

                    let uv_0 = raw_model.tex_coords[uv_idx0]; let uv_0 = (uv_0.0, uv_0.1);
                    let uv_1 = raw_model.tex_coords[uv_idx1]; let uv_1 = (uv_1.0, uv_1.1);
                    let uv_2 = raw_model.tex_coords[uv_idx2]; let uv_2 = (uv_2.0, uv_2.1);
                    let uv_3 = raw_model.tex_coords[uv_idx3]; let uv_3 = (uv_3.0, uv_3.1);

                    let quad = ngon::Quad::new([
                        (index0 as u32, normal_0.into(), uv_0.into()),
                        (index1 as u32, normal_1.into(), uv_1.into()),
                        (index2 as u32, normal_2.into(), uv_2.into()),
                        (index3 as u32, normal_3.into(), uv_3.into()),
                    ]);
                    let (t0, t1) = quad.to_triangles(&vertices)?;
                    triangles.push(t0);
                    triangles.push(t1);
                }
            }

            let (min_bounds, max_bounds) = {
                let mut min_bounds = [f32::INFINITY, f32::INFINITY, f32::INFINITY];
                let mut max_bounds = [f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY];

                for v in vertices.iter() {
                    min_bounds[0] = min_bounds[0].min(v[0]);
                    min_bounds[1] = min_bounds[1].min(v[1]);
                    min_bounds[2] = min_bounds[2].min(v[2]);

                    max_bounds[0] = max_bounds[0].max(v[0]);
                    max_bounds[1] = max_bounds[1].max(v[1]);
                    max_bounds[2] = max_bounds[2].max(v[2]);
                }

                (min_bounds, max_bounds)
            };

            let packed = PackedModel {
                index_count: 3 * triangles.len(),
                min_bounds,
                max_bounds,
                triangles,
                positions: raw_model.positions.iter()
                    .map(|v| [v.0, v.1, v.2])
                    .collect(),
            };

            self.write_data = packed.to_buffer();
            Ok(self.cursor())
        }

        fn pack_texture_data(mut self) -> std::io::Cursor<Vec<u8>> {
            self.write_data = self.original_data.clone();
            self.cursor()
        }

        fn pack_text_data(mut self) -> std::io::Cursor<Vec<u8>> {
            self.write_data = self.original_data.clone();
            self.cursor()
        }

        fn pack_shader_data(mut self) -> std::io::Cursor<Vec<u8>> {
            self.write_data = self.original_data.clone();
            self.cursor()
        }
    }

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
                archive.write_all(meta_json.as_bytes())?;

                archive.start_file_from_path(
                    directory.join(".stored"),
                    zip::write::SimpleFileOptions::default()
                )?;

                let mut packer = Packer::new(
                    std::fs::OpenOptions::new()
                        .read(true)
                        .open(asset.filepath.as_ref().expect("Need to have filepath to bundle"))?
                );
                let mut cursor = match asset::Type::from(asset.unit) {
                    asset::Type::Shader => packer.pack_shader_data(),
                    asset::Type::Texture => packer.pack_texture_data(),
                    asset::Type::Model => packer.pack_model_data()?,
                    asset::Type::Text => packer.pack_text_data(),
                };
                std::io::copy(&mut cursor, &mut archive)?;
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
                                let read_bytes = file.read_to_end(stored)?;
                                if file.size() as usize != read_bytes {
                                    return Err(error::UnpackError::SizeMismatch(read_bytes, file.size() as usize));
                                }
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
