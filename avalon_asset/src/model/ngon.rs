use crate::error;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub index: u32,
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub uv: [f32; 2]
}

impl Vertex {
    const BYTE_N: usize = 3 * 4 + 3 * 4 + 3 * 4 + 2 * 4;
    pub fn unpack(index: u32, bytes: &[u8; Vertex::BYTE_N]) -> Vertex {
        let mut offset = 0;
        let normal = [
            f32::from_be_bytes([bytes[offset + 0], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]),
            f32::from_be_bytes([bytes[offset + 4], bytes[offset + 5], bytes[offset + 6], bytes[offset + 7]]),
            f32::from_be_bytes([bytes[offset + 8], bytes[offset + 9], bytes[offset + 10], bytes[offset + 11]]),
        ];
        offset += 12;
        let tangent = [
            f32::from_be_bytes([bytes[offset + 0], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]),
            f32::from_be_bytes([bytes[offset + 4], bytes[offset + 5], bytes[offset + 6], bytes[offset + 7]]),
            f32::from_be_bytes([bytes[offset + 8], bytes[offset + 9], bytes[offset + 10], bytes[offset + 11]]),
        ];
        offset += 12;
        let uv = [
            f32::from_be_bytes([bytes[offset + 0], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]),
            f32::from_be_bytes([bytes[offset + 4], bytes[offset + 5], bytes[offset + 6], bytes[offset + 7]]),
        ];
        offset += 8;
        assert_eq!(offset, Vertex::BYTE_N);
        Vertex {
            index,
            normal,
            tangent,
            uv
        }
    }
    pub fn pack(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(self.normal.map(|n| n.to_be_bytes()).as_flattened());
        bytes.extend_from_slice(self.tangent.map(|t| t.to_be_bytes()).as_flattened());
        bytes.extend_from_slice(self.uv.map(|uv| uv.to_be_bytes()).as_flattened());

        bytes
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Quad {
    pub vertices: [Vertex; 4],
    pub edges: [(usize, usize); 4]
}

impl Quad {
    pub fn to_triangles(self, positions: &Vec<[f32; 3]>) -> Result<(Triangle, Triangle), error::NgonError> {
        // Triangulate quad by splitting on shortest possible non-existing edge
        let edges = self.edges.map(|(v0, v1)| (v0.min(v1), v1.max(v0)));
        let mut non_connected_edges = vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
            (2, 3),
        ];

        // Remove edges that exist in the polygon
        for edge in edges {
            let mut remove_idx: Option<usize> = None;
            for (idx, test_edge) in non_connected_edges.iter().enumerate() {
                if *test_edge == edge {
                    remove_idx = Some(idx);
                    break;
                }
            }

            if let Some(idx) = remove_idx {
                non_connected_edges.remove(idx);
            }
        }

        if non_connected_edges.is_empty() {
            return Err(error::NgonError::CannotTriangulateQuad);
        }

        // Find smallest edge
        let mut smallest_distance = std::f32::INFINITY;
        let mut smallest_edge = None;
        for edge in non_connected_edges {
            let v0 = self.vertices[edge.0];
            let v1 = self.vertices[edge.1];

            let p0 = positions[v0.index as usize];
            let p1 = positions[v1.index as usize];

            let dx = p0[0] - p1[0];
            let dy = p0[1] - p1[1];
            let dz = p0[2] - p1[3];

            let distance = dx * dx + dy * dy + dz * dz;
            if distance < smallest_distance {
                smallest_distance = distance;
                smallest_edge = Some(edge);
            }
        }

        let Some(smallest_edge) = smallest_edge else {
            return Err(error::NgonError::CannotTriangulateQuad);
        };

        // Because the vertices will not be neighbours, each vertex will have the same
        // 2 neighbours. We construct two triangles with vertices (v0, v1, n0) and
        // (v0, v1, n1).
        let split_v0 = self.vertices[smallest_edge.0];
        let split_v1 = self.vertices[smallest_edge.1];

        let mut neighbours = [split_v0, split_v1];
        for (iv0, iv1) in edges {
            let v0 = self.vertices[iv0];
            let v1 = self.vertices[iv1];
            if v0.index == split_v0.index {
                neighbours[0] = v0;
            } else if v1.index == split_v0.index {
                neighbours[1] = v1;
            }
        }

        Ok((
            Triangle {
                vertices: [
                    split_v0,
                    split_v1,
                    neighbours[0]
                ]
            },
            Triangle {
                vertices: [
                    split_v0,
                    split_v1,
                    neighbours[1]
                ]
            },
        ))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Triangle {
    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(self.vertices[0].pack());
        buffer.extend(self.vertices[1].pack());
        buffer.extend(self.vertices[2].pack());
        buffer
    }
}

