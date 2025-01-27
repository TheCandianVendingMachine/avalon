use crate::error;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub index: u32,
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub uv: [f32; 2]
}

impl Vertex {
    pub const BYTE_N: usize = (1 * 4) + (3 * 4) + (3 * 4) + (2 * 4);
    pub fn unpack(bytes: &[u8; Vertex::BYTE_N]) -> Vertex {
        let mut offset = 0;
        let index = u32::from_be_bytes([
            bytes[offset + 0],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;
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

        bytes.extend_from_slice(&self.index.to_be_bytes());
        bytes.extend_from_slice(self.normal.map(|n| n.to_be_bytes()).as_flattened());
        bytes.extend_from_slice(self.tangent.map(|t| t.to_be_bytes()).as_flattened());
        bytes.extend_from_slice(self.uv.map(|uv| uv.to_be_bytes()).as_flattened());

        bytes
    }

    pub fn reduced_pack(&self) -> Vec<u8> {
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
    pub fn new(vertices: [(u32, [f32; 3], [f32; 2]); 4]) -> Quad {
        Quad {
            vertices: [
                Vertex {
                    index: vertices[0].0,
                    normal: vertices[0].1,
                    uv: vertices[0].2,
                    tangent: [0.0; 3]
                },
                Vertex {
                    index: vertices[1].0,
                    normal: vertices[1].1,
                    uv: vertices[1].2,
                    tangent: [0.0; 3]
                },
                Vertex {
                    index: vertices[2].0,
                    normal: vertices[2].1,
                    uv: vertices[2].2,
                    tangent: [0.0; 3]
                },
                Vertex {
                    index: vertices[3].0,
                    normal: vertices[3].1,
                    uv: vertices[3].2,
                    tangent: [0.0; 3]
                },
            ],
            edges: [
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0)
            ]
        }
    }
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
            let dz = p0[2] - p1[2];

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
        let mut idx = 0;
        for (iv0, iv1) in edges {
            let v0 = self.vertices[iv0];
            let v1 = self.vertices[iv1];
            if v0.index == split_v0.index {
                neighbours[idx] = v1;
                idx += 1;
            } else if v1.index == split_v0.index {
                neighbours[idx] = v0;
                idx += 1;
            }
        }

        Ok((
            Triangle::new(positions, [
                (split_v0.index, split_v0.normal, split_v0.uv),
                (split_v1.index, split_v1.normal, split_v1.uv),
                (neighbours[0].index, neighbours[0].normal, neighbours[0].uv),
            ]),
            Triangle::new(positions, [
                (split_v0.index, split_v0.normal, split_v0.uv),
                (split_v1.index, split_v1.normal, split_v1.uv),
                (neighbours[1].index, neighbours[1].normal, neighbours[1].uv),
            ]),
        ))
    }
}

#[derive(Debug, Copy, Clone)]
struct Point3 {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Debug, Copy, Clone)]
struct Point2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Triangle {
    pub fn new(positions: &Vec<[f32; 3]>, vertices: [(u32, [f32; 3], [f32; 2]); 3]) -> Triangle {
        let p1 = Point3::from(positions[vertices[0].0 as usize]);
        let p2 = Point3::from(positions[vertices[1].0 as usize]);
        let p3 = Point3::from(positions[vertices[2].0 as usize]);
        let uv1 = Point2::from(vertices[0].2);
        let uv2 = Point2::from(vertices[1].2);
        let uv3 = Point2::from(vertices[2].2);

        let calc_tangent = |edge1: Point3, edge2: Point3, delta_uv1: Point2, delta_uv2: Point2| -> Point3 {
            let f = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
            Point3 {
                x: f * (delta_uv2.y * edge1.x - delta_uv1.y * edge2.x),
                y: f * (delta_uv2.y * edge1.y - delta_uv1.y * edge2.y),
                z: f * (delta_uv2.y * edge1.z - delta_uv1.y * edge2.z),
            }.normalize()
        };

        let tangent1 = {
            let edge1 = p2 - p1;
            let edge2 = p3 - p1;

            let delta_uv1 = uv2 - uv1;
            let delta_uv2 = uv3 - uv1;

            calc_tangent(edge1, edge2, delta_uv1, delta_uv2)
        };

        let tangent2 = {
            let edge1 = p1 - p2;
            let edge2 = p3 - p2;

            let delta_uv1 = uv1 - uv2;
            let delta_uv2 = uv3 - uv2;

            calc_tangent(edge1, edge2, delta_uv1, delta_uv2)
        };

        let tangent3 = {
            let edge1 = p2 - p3;
            let edge2 = p1 - p3;

            let delta_uv1 = uv2 - uv3;
            let delta_uv2 = uv1 - uv3;

            calc_tangent(edge1, edge2, delta_uv1, delta_uv2)
        };
        Triangle {
            vertices: [
                Vertex {
                    index: vertices[0].0,
                    normal: vertices[0].1,
                    uv: vertices[0].2,
                    tangent: tangent1.into()
                },
                Vertex {
                    index: vertices[1].0,
                    normal: vertices[1].1,
                    uv: vertices[1].2,
                    tangent: tangent2.into()
                },
                Vertex {
                    index: vertices[2].0,
                    normal: vertices[2].1,
                    uv: vertices[2].2,
                    tangent: tangent3.into()
                },
            ]
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(self.vertices[0].pack());
        buffer.extend(self.vertices[1].pack());
        buffer.extend(self.vertices[2].pack());
        buffer
    }

    pub fn from_buffer(buffer: &[u8; Vertex::BYTE_N * 3]) -> Triangle {
        Triangle {
            vertices: [
                Vertex::unpack(&buffer[(0 * Vertex::BYTE_N)..(1 * Vertex::BYTE_N)].as_array().unwrap()),
                Vertex::unpack(&buffer[(1 * Vertex::BYTE_N)..(2 * Vertex::BYTE_N)].as_array().unwrap()),
                Vertex::unpack(&buffer[(2 * Vertex::BYTE_N)..(3 * Vertex::BYTE_N)].as_array().unwrap()),
            ],
        }
    }
}

impl Point3 {
    fn normalize(self) -> Point3 {
        let d = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Point3 {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }
}

impl From<[f32; 3]> for Point3 {
    fn from(p: [f32; 3]) -> Point3 {
        Point3 {
            x: p[0],
            y: p[1],
            z: p[2],
        }
    }
}

impl From<[f32; 2]> for Point2 {
    fn from(p: [f32; 2]) -> Point2 {
        Point2 {
            x: p[0],
            y: p[1],
        }
    }
}

impl From<Point3> for [f32; 3] {
    fn from(p: Point3) -> [f32; 3] {
        [p.x, p.y, p.z]
    }
}

impl From<Point2> for [f32; 2] {
    fn from(p: Point2) -> [f32; 2] {
        [p.x, p.y]
    }
}


impl std::ops::Sub for Point3 {
    type Output = Point3;
    fn sub(self, other: Point3) -> Point3 {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub for Point2 {
    type Output = Point2;
    fn sub(self, other: Point2) -> Point2 {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod vertex_tests {
    use super::Vertex;

    #[test]
    fn test_pack_equal_unpack() {
        let vertex = Vertex {
            index: 32,
            normal: [0.0, 3.14, 0.2],
            tangent: [1.0, 2.0, 3.0],
            uv: [6.0, 9.0]
        };

        let buffer = vertex.pack();
        let unpacked = Vertex::unpack(&buffer[0..Vertex::BYTE_N].as_array().unwrap());

        assert_eq!(vertex.index, unpacked.index);
        assert_eq!(vertex.normal, unpacked.normal);
        assert_eq!(vertex.tangent, unpacked.tangent);
        assert_eq!(vertex.uv, unpacked.uv);
    }

    #[test]
    fn test_pack_many_equal_unpack() {
        let mut buffer = Vec::new();
        for _ in 0..10 {
            buffer.extend(Vertex {
                index: 36325,
                normal: [10.0, 13.14, 10.2],
                tangent: [11.0, 12.0, 13.0],
                uv: [16.0, 19.0]
            }.pack());
        }

        let vertex = Vertex {
            index: 32,
            normal: [0.0, 3.14, 0.2],
            tangent: [1.0, 2.0, 3.0],
            uv: [6.0, 9.0]
        };
        buffer.extend(vertex.pack());

        for _ in 0..10 {
            buffer.extend(Vertex {
                index: 636325,
                normal: [610.0, 613.14, 610.2],
                tangent: [611.0, 612.0, 613.0],
                uv: [616.0, 619.0]
            }.pack());
        }

        let unpacked = Vertex::unpack(&buffer[(10 * Vertex::BYTE_N)..(11 * Vertex::BYTE_N)].as_array().unwrap());

        assert_eq!(vertex.index, unpacked.index);
        assert_eq!(vertex.normal, unpacked.normal);
        assert_eq!(vertex.tangent, unpacked.tangent);
        assert_eq!(vertex.uv, unpacked.uv);
    }
}

#[cfg(test)]
mod triangle_tests {
    use super::{ Vertex, Triangle };

    #[test]
    fn test_pack_equal_unpack() {
        let triangle = Triangle {
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
        };

        let buffer = triangle.to_buffer();
        let unpacked = Triangle::from_buffer(&buffer[0..(3 * Vertex::BYTE_N)].as_array().unwrap());

        assert_eq!(triangle.vertices[0].index, unpacked.vertices[0].index);
        assert_eq!(triangle.vertices[0].normal, unpacked.vertices[0].normal);
        assert_eq!(triangle.vertices[0].tangent, unpacked.vertices[0].tangent);
        assert_eq!(triangle.vertices[0].uv, unpacked.vertices[0].uv);

        assert_eq!(triangle.vertices[1].index, unpacked.vertices[1].index);
        assert_eq!(triangle.vertices[1].normal, unpacked.vertices[1].normal);
        assert_eq!(triangle.vertices[1].tangent, unpacked.vertices[1].tangent);
        assert_eq!(triangle.vertices[1].uv, unpacked.vertices[1].uv);

        assert_eq!(triangle.vertices[2].index, unpacked.vertices[2].index);
        assert_eq!(triangle.vertices[2].normal, unpacked.vertices[2].normal);
        assert_eq!(triangle.vertices[2].tangent, unpacked.vertices[2].tangent);
        assert_eq!(triangle.vertices[2].uv, unpacked.vertices[2].uv);
    }
}
