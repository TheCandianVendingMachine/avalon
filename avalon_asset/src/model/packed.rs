use crate::model::ngon::{ Vertex, Triangle };

#[derive(Debug, Clone)]
pub struct PackedModel {
    pub index_count: usize,
    pub min_bounds: [f32; 3],
    pub max_bounds: [f32; 3],
    pub positions: Vec<[f32; 3]>,
    pub triangles: Vec<Triangle>
}

impl PackedModel {
    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&(self.positions.len() as u32).to_be_bytes());
        buffer.extend_from_slice(&(self.index_count as u32).to_be_bytes());
        buffer.extend_from_slice(&(self.triangles.len() as u32).to_be_bytes());
        buffer.extend_from_slice(self.min_bounds.map(|p| p.to_be_bytes()).as_flattened());
        buffer.extend_from_slice(self.max_bounds.map(|p| p.to_be_bytes()).as_flattened());
        for v in self.positions.iter() {
            buffer.extend_from_slice(
                v.map(|p|
                    p.to_be_bytes()
                )
                .as_flattened()
            );
        }
        for t in self.triangles.iter() {
            buffer.extend(t.to_buffer());
        }
        buffer
    }

    #[allow(clippy::erasing_op)]
    #[allow(clippy::identity_op)]
    pub fn from_buffer(buffer: &[u8]) -> PackedModel {
        let mut offset = 0;
        let vertex_count = u32::from_be_bytes([
            buffer[offset + 0],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ]);
        offset += 4;

        let index_count = u32::from_be_bytes([
            buffer[offset + 0],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ]);
        offset += 4;

        let triangle_count = u32::from_be_bytes([
            buffer[offset + 0],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ]);
        offset += 4;

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

        let mut positions = Vec::new();
        for _ in 0..vertex_count {
            positions.push([
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
            ]);
            offset += 12;
        }

        let mut triangles = Vec::new();
        for _ in 0..triangle_count {
            triangles.push(
                Triangle::from_buffer(buffer[offset..(offset + 3 * Vertex::BYTE_N)].as_array().unwrap())
            );
            offset += 3 * Vertex::BYTE_N;
        }

        PackedModel {
            index_count: index_count as usize,
            min_bounds,
            max_bounds,
            positions,
            triangles
        }
    }
}

#[cfg(test)]
mod packed_model_tests {
    use super::PackedModel;
    use crate::model::ngon::{ Vertex, Triangle };

    #[test]
    fn test_pack_equal_unpack() {
        let packed_model = PackedModel {
            index_count: 10,
            min_bounds: [-1.0, 0.0, 1.0],
            max_bounds: [-5.0, 5.0, 5.0],
            positions: vec![
                [1.0, 2.0, 3.0],
                [11.0, 12.0, 13.0],
                [111.0, 112.0, 113.0],
                [1111.0, 1112.0, 1113.0],
            ],
            triangles: vec![
                Triangle {
                    vertices: [
                        Vertex {
                            index: 10,
                            normal: [10.0, 0.0, 0.0],
                            tangent: [0.0, 11.0, 0.0],
                            uv: [0.0, 11.0]
                        },
                        Vertex {
                            index: 13,
                            normal: [0.0, 11.0, 0.0],
                            tangent: [0.0, 0.0, 11.0],
                            uv: [11.0, 0.0]
                        },
                        Vertex {
                            index: 15,
                            normal: [0.0, 0.0, 1.0],
                            tangent: [1.0, 0.0, 0.0],
                            uv: [0.5, 0.5]
                        },
                    ]
                },
                Triangle {
                    vertices: [
                        Vertex {
                            index: 1,
                            normal: [1.0, 0.0, 0.0],
                            tangent: [0.0, 1.0, 0.0],
                            uv: [0.0, 1.0]
                        },
                        Vertex {
                            index: 3,
                            normal: [0.0, 1.0, 0.0],
                            tangent: [0.0, 0.0, 1.0],
                            uv: [1.0, 0.0]
                        },
                        Vertex {
                            index: 5,
                            normal: [0.0, 0.0, 1.0],
                            tangent: [1.0, 0.0, 0.0],
                            uv: [0.5, 0.5]
                        },
                    ]
                },
            ],
        };

        let buffer = packed_model.to_buffer();
        let unpacked = PackedModel::from_buffer(&buffer);

        assert_eq!(packed_model.index_count, unpacked.index_count);
        assert_eq!(packed_model.min_bounds, unpacked.min_bounds);
        assert_eq!(packed_model.max_bounds, unpacked.max_bounds);
        assert_eq!(packed_model.positions, unpacked.positions);

        assert_eq!(unpacked.triangles.len(), 2);
        assert_eq!(packed_model.triangles[0].vertices[0].index, unpacked.triangles[0].vertices[0].index);
        assert_eq!(packed_model.triangles[0].vertices[0].normal, unpacked.triangles[0].vertices[0].normal);
        assert_eq!(packed_model.triangles[0].vertices[0].tangent, unpacked.triangles[0].vertices[0].tangent);
        assert_eq!(packed_model.triangles[0].vertices[0].uv,  unpacked.triangles[0].vertices[0].uv);

        assert_eq!(packed_model.triangles[0].vertices[1].index, unpacked.triangles[0].vertices[1].index);
        assert_eq!(packed_model.triangles[0].vertices[1].normal, unpacked.triangles[0].vertices[1].normal);
        assert_eq!(packed_model.triangles[0].vertices[1].tangent, unpacked.triangles[0].vertices[1].tangent);
        assert_eq!(packed_model.triangles[0].vertices[1].uv,  unpacked.triangles[0].vertices[1].uv);

        assert_eq!(packed_model.triangles[0].vertices[2].index, unpacked.triangles[0].vertices[2].index);
        assert_eq!(packed_model.triangles[0].vertices[2].normal, unpacked.triangles[0].vertices[2].normal);
        assert_eq!(packed_model.triangles[0].vertices[2].tangent, unpacked.triangles[0].vertices[2].tangent);
        assert_eq!(packed_model.triangles[0].vertices[2].uv,  unpacked.triangles[0].vertices[2].uv);


        assert_eq!(packed_model.triangles[1].vertices[0].index, unpacked.triangles[1].vertices[0].index);
        assert_eq!(packed_model.triangles[1].vertices[0].normal, unpacked.triangles[1].vertices[0].normal);
        assert_eq!(packed_model.triangles[1].vertices[0].tangent, unpacked.triangles[1].vertices[0].tangent);
        assert_eq!(packed_model.triangles[1].vertices[0].uv,  unpacked.triangles[1].vertices[0].uv);

        assert_eq!(packed_model.triangles[1].vertices[1].index, unpacked.triangles[1].vertices[1].index);
        assert_eq!(packed_model.triangles[1].vertices[1].normal, unpacked.triangles[1].vertices[1].normal);
        assert_eq!(packed_model.triangles[1].vertices[1].tangent, unpacked.triangles[1].vertices[1].tangent);
        assert_eq!(packed_model.triangles[1].vertices[1].uv,  unpacked.triangles[1].vertices[1].uv);

        assert_eq!(packed_model.triangles[1].vertices[2].index, unpacked.triangles[1].vertices[2].index);
        assert_eq!(packed_model.triangles[1].vertices[2].normal, unpacked.triangles[1].vertices[2].normal);
        assert_eq!(packed_model.triangles[1].vertices[2].tangent, unpacked.triangles[1].vertices[2].tangent);
        assert_eq!(packed_model.triangles[1].vertices[2].uv,  unpacked.triangles[1].vertices[2].uv);
    }
}
