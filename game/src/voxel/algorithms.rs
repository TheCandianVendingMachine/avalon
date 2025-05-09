use avalon::shader::{ self, Source };
use avalon::texture::{ Component, data::Data };
use avalon::texture::gpu::{ UniqueTexture, Access, SizedComponent, Arguments3d, Texture3d, Mipmap };
use crate::voxel::Grid;
use nalgebra_glm::vec3;

impl<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32> Grid<SIDE_LENGTH, VOXELS_PER_METER> where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
    pub fn bake(&mut self) {
        #[allow(clippy::erasing_op)]
        if self.gpu_grid.is_none() {
            self.gpu_grid = Some(Texture3d::generate(Arguments3d {
                dimensions: vec3(SIDE_LENGTH, SIDE_LENGTH, SIDE_LENGTH).cast(),
                internal_components: Component::IntR,
                internal_size: SizedComponent::UnsignedIntR32,
                mipmap_type: Mipmap::None,
                data: None
            }));
        }
        if !self.dirty {
            return;
        }
        let grid_texture = self.gpu_grid.as_ref().unwrap();

        let oven = shader::Program::new()
            .compute(shader::Compute::load_from_path("assets/shaders/voxel/bake_distance_field.comp").unwrap())
            .build()
            .unwrap();

        let distance_buffer = {
            let data = {
                let mut data = Data::empty_u8(Component::IntRGBA, SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH);
                for (idx, voxel) in self.cells.iter().enumerate() {
                    if !voxel.is_empty() {
                        let position = self.index_to_vec(idx);
                        let idx = 4 * idx;
                        data.set(idx + 0, position.x);
                        data.set(idx + 1, position.y);
                        data.set(idx + 2, position.z);
                        data.set(idx + 3, 1_u8);
                    }
                }
                data
            };
            let temp_parent_buffers = Texture3d::generate_many::<2>(Arguments3d {
                dimensions: vec3(SIDE_LENGTH, SIDE_LENGTH, SIDE_LENGTH).cast(),
                internal_components: Component::IntRGBA,
                internal_size: SizedComponent::UnsignedIntRGBA8,
                mipmap_type: Mipmap::None,
                data: Some(data)
            }).map(|t| t.as_managed());

            let mut distance_buffer = Texture3d::generate(Arguments3d {
                dimensions: vec3(SIDE_LENGTH, SIDE_LENGTH, SIDE_LENGTH).cast(),
                internal_components: Component::IntR,
                internal_size: SizedComponent::UnsignedIntR16,
                mipmap_type: Mipmap::None,
                data: None
            }).as_managed();

            {
                let mut bind = oven.activate();
                bind.image("distanceBuffer", &distance_buffer, Access::ReadWrite(0));

                let (dispatch_x, dispatch_y, dispatch_z) = oven.dispatch_counts(
                    SIDE_LENGTH,
                    SIDE_LENGTH,
                    SIDE_LENGTH
                );

                let step_count = (SIDE_LENGTH as f64).log2().ceil() as usize;
                for jump in 0..=step_count {
                    let new_idx = jump % 2;
                    let old_idx = (jump + 1) % 2;

                    // the first iteration should initialise all cells; we get less artifacts
                    if jump == 0 {
                        bind.uniform("jump").unwrap().set_i32(1);
                    } else {
                        bind.uniform("jump").unwrap().set_i32(2_i32.pow((step_count - jump) as u32));
                    }
                    bind.image("newParentBuffer", &temp_parent_buffers[new_idx], Access::Write(0)).unwrap();
                    bind.image("oldParentBuffer", &temp_parent_buffers[old_idx], Access::Read(0)).unwrap();
                    bind.barrier();
                    bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
                }

            }

            let buffer = distance_buffer.bind().fetch_pixels(0);
            buffer
        };

        for (idx, cell) in self.cells.iter_mut().enumerate() {
            if cell.is_empty() {
                let distance = distance_buffer.get(idx).try_into().unwrap();
                cell.set_safe_step(distance);
            }
        }

        let mut cell_data = Data::empty_u32(Component::IntR, self.cells.len());
        for (idx, cell) in self.cells.iter().enumerate() {
            cell_data.set(idx, cell.0);
        }
        self.gpu_grid.as_mut().unwrap().bind().write_pixels(0, cell_data);
    }
}
