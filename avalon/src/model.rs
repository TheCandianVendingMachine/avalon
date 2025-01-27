pub(crate) use crate::asset_library;
use crate::gpu_buffer::State;
use avalon_asset::model::packed::PackedModel;
use nalgebra_glm::Vec3;

#[derive(Debug)]
struct Gpu {
    vertex_array_object: State,
    position_vbo: gl::types::GLuint,
    extra_vbo: gl::types::GLuint,
    index_buffer: gl::types::GLuint,
}

#[derive(Debug)]
pub struct Model {
    gpu: Gpu,
    min_bounds: Vec3,
    max_bounds: Vec3,
}

impl Model {
    pub fn center(&self) -> Vec3 {
        (self.max_bounds + self.max_bounds) / 2.0
    }

    pub fn extents(&self) -> Vec3 {
        self.max_bounds - self.min_bounds
    }
}

impl asset_library::Asset for Model {}
impl std::ops::Deref for Model {
    type Target = State;
    fn deref(&self) -> &State {
        &self.gpu.vertex_array_object
    }
}

impl From<&PackedModel> for Model {
    fn from(model: &PackedModel) -> Model {
        let mut gpu = Gpu::new();

        let mut indices = Vec::new();
        let mut packed_additional = Vec::new();
        for triangle in model.triangles.iter() {
            for vertex in triangle.vertices {
                packed_additional.extend(vertex.reduced_pack());
                indices.push(vertex.index);
            }
        }
        unsafe {
            let vao_bind = gpu.vertex_array_object.bind();
            gl::BindBuffer(gl::ARRAY_BUFFER, gpu.position_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (3 * std::mem::size_of::<f32>() * model.positions.len()) as isize,
                model.positions.iter()
                    .flat_map(|p| *p)
                    .collect::<Vec<f32>>()
                    .as_slice()
                    .as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                std::ptr::null()
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, gpu.extra_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                packed_additional.len() as isize,
                packed_additional.as_slice().as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW
            );
            // vertex normal
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * std::mem::size_of::<f32>() as i32,
                std::mem::transmute(0 * std::mem::size_of::<f32>())
            );
            // vertex tangent
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * std::mem::size_of::<f32>() as i32,
                std::mem::transmute(3 * std::mem::size_of::<f32>())
            );
            // vertex uv
            gl::VertexAttribPointer(
                3,
                2,
                gl::FLOAT,
                gl::FALSE,
                8 * std::mem::size_of::<f32>() as i32,
                std::mem::transmute(6 * std::mem::size_of::<f32>())
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, gpu.index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (std::mem::size_of::<u32>() * indices.len()) as isize,
                indices.as_slice().as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW
            );

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);
            gl::EnableVertexAttribArray(3);

            drop(vao_bind);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        gpu.vertex_array_object.count = crate::gpu_buffer::Kind::Indexed {
            index_count: model.index_count
        };
        Model {
            gpu,
            min_bounds: model.min_bounds.into(),
            max_bounds: model.max_bounds.into(),
        }
    }
}

impl From<&mut PackedModel> for Model {
    fn from(packed: &mut PackedModel) -> Model {
        Model::from(&*packed)
    }
}

impl From<PackedModel> for Model {
    fn from(packed: PackedModel) -> Model {
        Model::from(&packed)
    }
}

impl Gpu {
    fn new() -> Gpu {
        let (position_vbo, extra_vbo, index_buffer) = unsafe {
            let mut buffers = [0; 3];
            gl::GenBuffers(3, buffers.as_mut_ptr());
            (buffers[0], buffers[1], buffers[2])
        };

        Gpu {
            vertex_array_object: State::new(),
            position_vbo,
            extra_vbo,
            index_buffer
        }
    }
}

impl Drop for Gpu {
    fn drop(&mut self) {
        unsafe {
            let buffers = [
                self.position_vbo,
                self.extra_vbo,
                self.index_buffer
            ];
            gl::DeleteBuffers(buffers.len() as i32, buffers.as_ptr());
        }
    }
}
