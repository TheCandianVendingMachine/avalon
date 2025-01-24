use std::collections::HashMap;
use crate::{ bundle, asset };

#[derive(Debug, Clone)]
pub struct PackedModelBuffer {
    pub indices: Vec<u32>,
    pub positions: Vec<u8>,
    pub extra: Vec<u8>,
    pub min_bounds: [f32; 3],
    pub max_bounds: [f32; 3]
}

impl PackedModelBuffer {
    fn to_buffer(self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&(self.indices.len() as u32).to_be_bytes());
        buffer.extend(self.indices.iter().map(|i| i.to_be_bytes()).flatten());
        buffer.extend(self.min_bounds.iter().map(|f| f.to_be_bytes()).flatten());
        buffer.extend(self.max_bounds.iter().map(|f| f.to_be_bytes()).flatten());
        buffer.extend_from_slice(&(self.positions.len() as u32 / 4).to_be_bytes());
        buffer.extend(self.positions);
        buffer.extend(self.extra);
        buffer
    }

    fn from_buffer(buffer: impl AsRef<[u8]>) -> PackedModelBuffer {
        let buffer = buffer.as_ref();
        let mut offset = 0;
        let index_count = u32::from_be_bytes([
            buffer[offset + 0],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3]
        ]);
        offset += 4;
        let mut indices = vec![0; index_count as usize];
        for idx in 0..index_count {
            indices[idx as usize] = u32::from_be_bytes([
                buffer[offset + (4 * idx + 0) as usize],
                buffer[offset + (4 * idx + 1) as usize],
                buffer[offset + (4 * idx + 2) as usize],
                buffer[offset + (4 * idx + 3) as usize]
            ]);
        }
        offset += 4 * index_count as usize;

        let min_bounds = [
            f32::from_be_bytes([
                buffer[offset + 0],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]),
            f32::from_be_bytes([
                buffer[offset + 4],
                buffer[offset + 5],
                buffer[offset + 6],
                buffer[offset + 7],
            ]),
            f32::from_be_bytes([
                buffer[offset + 8],
                buffer[offset + 9],
                buffer[offset + 10],
                buffer[offset + 11],
            ]),
        ];
        offset += 12;
        let max_bounds = [
            f32::from_be_bytes([
                buffer[offset + 0],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]),
            f32::from_be_bytes([
                buffer[offset + 4],
                buffer[offset + 5],
                buffer[offset + 6],
                buffer[offset + 7],
            ]),
            f32::from_be_bytes([
                buffer[offset + 8],
                buffer[offset + 9],
                buffer[offset + 10],
                buffer[offset + 11],
            ]),
        ];
        offset += 12;

        let vertex_count = u32::from_be_bytes([
            buffer[offset + 0],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3]
        ]);
        offset += 4;
        let mut positions = vec![0; 4 * vertex_count as usize];
        for idx in 0..vertex_count * 4 {
            positions[idx as usize] = u8::from_be_bytes([buffer[offset + idx as usize]]);
        }
        offset += 4 * vertex_count as usize;

        let mut extra = vec![0; 12 * vertex_count as usize];
        for idx in 0..vertex_count * 12 {
            extra[idx as usize] = u8::from_be_bytes([buffer[offset + idx as usize]]);
        }
        //offset += 12 * vertex_count as usize;

        PackedModelBuffer {
            indices,
            positions,
            extra,
            min_bounds,
            max_bounds
        }
    }
}

impl From<PackedModel> for PackedModelBuffer {
    fn from(model: PackedModel) -> PackedModelBuffer {
        let positions: Vec<u8> = model.positions
            .as_flattened()
            .iter()
            .map(|p| p.to_ne_bytes())
            .flatten()
            .collect();
        let indices: Vec<u32> = model.indices;

        let byte_arrays: Vec<Vec<u8>> = model.texture_coordinates.iter().zip(model.normals.iter().zip(model.tangents.iter()))
        .map(|(uv, (n, t))| (*uv, *n, *t))
        .map(|(uv, n, t)| {
            let mut bytes = Vec::new();
            // 2 * 2 + 4 + 4
            // = 4 + 4 + 4
            // = 12 bytes
            bytes.extend_from_slice(uv.map(|f| f.to_be_bytes()).as_flattened());
            bytes.extend_from_slice(&n.to_be_bytes());
            bytes.extend_from_slice(&t.to_be_bytes());
            bytes
        })
        .collect();

        let mut extra = Vec::new();
        for byte_array in byte_arrays {
            extra.extend(byte_array.iter());
        }

        PackedModelBuffer {
            indices,
            positions,
            extra,
            min_bounds: model.min_bounds,
            max_bounds: model.max_bounds
        }
    }
}

impl From<PackedModelBuffer> for PackedModel {
    fn from(model_buffer: PackedModelBuffer) -> PackedModel {
        let vertex_count = model_buffer.positions.len() / 4;
        let index_count = model_buffer.indices.len();
        let min_bounds = model_buffer.min_bounds;
        let max_bounds = model_buffer.max_bounds;
        let indices = model_buffer.indices;

        let mut positions: Vec<[f32; 3]> = Vec::new();
        for idx in 0..vertex_count {
            let b0 = model_buffer.positions[idx * 12 + 0];
            let b1 = model_buffer.positions[idx * 12 + 1];
            let b2 = model_buffer.positions[idx * 12 + 2];
            let b3 = model_buffer.positions[idx * 12 + 3];
            let p0 = f32::from_be_bytes([b0, b1, b2, b3]);

            let b0 = model_buffer.positions[idx * 12 + 4];
            let b1 = model_buffer.positions[idx * 12 + 5];
            let b2 = model_buffer.positions[idx * 12 + 6];
            let b3 = model_buffer.positions[idx * 12 + 7];
            let p1 = f32::from_be_bytes([b0, b1, b2, b3]);

            let b0 = model_buffer.positions[idx * 12 + 8];
            let b1 = model_buffer.positions[idx * 12 + 9];
            let b2 = model_buffer.positions[idx * 12 + 10];
            let b3 = model_buffer.positions[idx * 12 + 11];
            let p2 = f32::from_be_bytes([b0, b1, b2, b3]);

            positions.push([p0, p1, p2]);
        }

        let mut texture_coordinates: Vec<[u16; 2]> = Vec::new();
        let mut normals: Vec<i32> = Vec::new();
        let mut tangents: Vec<i32> = Vec::new();
        for idx in 0..vertex_count {
            let offset = idx * 16;
            let b = &model_buffer.extra;

            {
                let uv0 = u16::from_be_bytes([b[offset + 0], b[offset + 1]]);
                let uv1 = u16::from_be_bytes([b[offset + 2], b[offset + 3]]);
                texture_coordinates.push([uv0, uv1]);
            }

            {
                let normal = i32::from_be_bytes([
                    b[offset + 4],
                    b[offset + 5],
                    b[offset + 6],
                    b[offset + 7],
                ]);
                normals.push(normal);
            }

            {
                let tangent = i32::from_be_bytes([
                    b[offset + 8],
                    b[offset + 9],
                    b[offset + 10],
                    b[offset + 11],
                ]);
                tangents.push(tangent);
            }
        }

        PackedModel {
            vertex_count,
            index_count,
            min_bounds,
            max_bounds,
            indices,
            positions,
            normals,
            tangents,
            texture_coordinates
        }
    }
}

#[derive(Debug, Clone)]
pub struct PackedModel {
    pub vertex_count: usize,
    pub index_count: usize,
    pub min_bounds: [f32; 3],
    pub max_bounds: [f32; 3],
    pub positions: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<i32>,
    pub tangents: Vec<i32>,
    pub texture_coordinates: Vec<[u16; 2]>,
}

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
            let mut original_data = vec![0xFF_u8; 10];
            reader.read_to_end(&mut original_data).expect("data stream needs to be readable");
            Packer {
                original_data,
                write_data: Vec::new()
            }
        }

        fn cursor(self) -> std::io::Cursor<Vec<u8>> {
            std::io::Cursor::new(self.write_data)
        }

        fn pack_model_data(mut self) -> std::io::Cursor<Vec<u8>> {
            let cursor = std::io::Cursor::new(&self.original_data);
            let reader = std::io::BufReader::new(cursor);
            let model: obj::Obj<obj::TexturedVertex, u32> = obj::Obj::new(
                obj::raw::parse_obj(reader).expect("obj format needs to be valid")
            ).expect("Issues wrapping raw obj");

            let mut min_bounds = [0.0_f32, 0.0, 0.0];
            let mut max_bounds = [0.0_f32, 0.0, 0.0];

            for vertex in model.vertices.iter() {
                min_bounds[0] = min_bounds[0].min(vertex.position[0]);
                min_bounds[1] = min_bounds[1].min(vertex.position[1]);
                min_bounds[2] = min_bounds[2].min(vertex.position[2]);

                max_bounds[0] = max_bounds[0].max(vertex.position[0]);
                max_bounds[1] = max_bounds[1].max(vertex.position[1]);
                max_bounds[2] = max_bounds[2].max(vertex.position[2]);
            }

            let min_bounds = min_bounds;
            let max_bounds = max_bounds;

            let mut positions = Vec::new();
            for vertex in model.vertices.iter() {
                let scaled_position = [
                    (2.0 * vertex.position[0] - max_bounds[0] - min_bounds[0]) / (max_bounds[0] - min_bounds[0]),
                    (2.0 * vertex.position[1] - max_bounds[1] - min_bounds[1]) / (max_bounds[1] - min_bounds[1]),
                    (2.0 * vertex.position[2] - max_bounds[2] - min_bounds[2]) / (max_bounds[2] - min_bounds[2]),
                ];
                positions.push(scaled_position);
            }
            let positions = positions;
            let indices = model.indices.clone();

            let mut normals = Vec::new();
            let mut tangents = Vec::new();
            let mut texture_coordinates = Vec::new();
            for vertex in model.vertices.iter() {
                let n = vertex.normal;
                let t = [0.0_f32; 3];
                let uv = [vertex.texture[0], vertex.texture[1]];

                let adj_nx = (n[0] / (((1 << 9) - 1) as f32)) as i32;
                let adj_ny = (n[1] / (((1 << 9) - 1) as f32)) as i32;
                let adj_nz = (n[1] / (((1 << 9) - 1) as f32)) as i32;
                let packed_n =
                    (adj_nx) |
                    (adj_ny << 10) |
                    (adj_ny << 20);

                let adj_tx = (t[0] / (((1 << 9) - 1) as f32)) as i32;
                let adj_ty = (t[1] / (((1 << 9) - 1) as f32)) as i32;
                let adj_tz = (t[1] / (((1 << 9) - 1) as f32)) as i32;
                let packed_t =
                    (adj_tx) |
                    (adj_ty << 10) |
                    (adj_ty << 20);

                let adj_u = (uv[0] / ((1 << 16) - 1) as f32) as u16;
                let adj_v = (uv[1] / ((1 << 16) - 1) as f32) as u16;

                normals.push(packed_n);
                tangents.push(packed_t);
                texture_coordinates.push([adj_u, adj_v]);
            }

            let buffers = super::PackedModelBuffer::from(super::PackedModel {
                vertex_count: model.vertices.len(),
                index_count: model.indices.len(),
                positions,
                indices,
                min_bounds,
                max_bounds,
                normals,
                tangents,
                texture_coordinates
            });

            self.write_data = buffers.to_buffer();
            self.cursor()
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
                archive.write(meta_json.as_bytes())?;

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
                    asset::Type::Model => packer.pack_model_data(),
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
