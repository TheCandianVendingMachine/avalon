#[derive(Debug, Clone)]
pub struct PackedModelBuffer {
    pub indices: Vec<u32>,
    pub positions: Vec<u8>,
    pub extra: Vec<u8>,
    pub min_bounds: [f32; 3],
    pub max_bounds: [f32; 3]
}

impl PackedModelBuffer {
    pub fn to_buffer(self) -> Vec<u8> {
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

    pub fn from_buffer(buffer: impl AsRef<[u8]>) -> PackedModelBuffer {
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
