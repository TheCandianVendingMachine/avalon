use std::vec;

use avalon::shader::{ self, Source };
use avalon::texture::{ Component, data::Data };
use avalon::texture::gpu::{ UniqueTexture, Access, SizedComponent, Arguments3d, Texture3d, Mipmap };
use crate::voxel::{ cells, Cell, Grid };
use nalgebra_glm::{ self as glm, TVec3, Vec3, vec3 };

#[derive(Debug, Clone, Copy)]
pub struct RaycastResult {
    pub start_position: Vec3,
    pub end_position: Vec3,
    pub distance: f32,
    pub cell: Option<Cell>
}

pub enum Condition {
    Cell(Box<dyn Fn(&Cell) -> bool>),
    Distance(Box<dyn Fn(&Cell, f32) -> bool>),
}

impl Condition {
    pub fn cell<F: Fn(&Cell) -> bool + 'static>(func: F) -> Condition {
        Condition::Cell(Box::new(func))
    }

    pub fn distance<F: Fn(&Cell, f32) -> bool + 'static>(func: F) -> Condition {
        Condition::Distance(Box::new(func))
    }
}

impl<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32> Grid<SIDE_LENGTH, VOXELS_PER_METER> where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
    #[allow(clippy::identity_op)]
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

    pub fn ray_while_condition(&self, start: TVec3<f32>, direction: TVec3<f32>, condition: Condition) -> RaycastResult {
        let position = start;

        let intersect_plane = |position: Vec3, direction: Vec3, center: Vec3, normal: Vec3| -> f32 {
            let denom = glm::dot(&normal, &direction);
            if (denom.abs() > 0.0) {
                let t = glm::dot(&(center.map(|c| c as f32) - start), &normal) / denom;
                if (t >= 0.0) {
                    return t;
                }
            }
            f32::INFINITY
        };

        let mut map_pos = position.map(|c| c.floor() as i16);
        let delta_dist = direction.map(|c| c.abs().recip()).map(|c| if c.is_finite() { c } else { f32::MAX });
        let ray_step = direction.map(|c| c.signum() as i16);
        let ray_dir_sign = direction.map(|c| c.signum());
        let mut t_max = delta_dist.component_mul(&(
            ray_dir_sign.component_mul(&(map_pos.cast() - position))
            + (ray_dir_sign * 0.5)
            + vec3(0.5, 0.5, 0.5)
        ));
        let mut mask = TVec3::default();

        let ray_intersect = |mask: TVec3<bool>, map_pos: TVec3<i16>| -> Vec3{
            let normal = if (mask.x) {
                -vec3(1, 0, 0).component_mul(&ray_step)
            } else if (mask.y) {
                -vec3(0, 1, 0).component_mul(&ray_step)
            } else if (mask.z) {
                -vec3(0, 0, 1).component_mul(&ray_step)
            } else {
                vec3(0, 0, 0)
            };
            let back_step = -(0.5 * -ray_step.component_mul(&mask.map(|c| c as i16)).map(|c| c as f32) + vec3(0.5, 0.5, 0.5));
            let center = map_pos - (back_step + glm::not(&mask).map(|c| c as u16 as f32) * 0.5).map(|c| c as i16);
            let t = intersect_plane(position, direction, center.map(|c| c as f32), normal.map(|c| c as f32));
            start + direction * (t + 0.00001 * glm::dot(&-back_step, &vec3(1.0, 1.0, 1.0)))
        };

        while {
            map_pos.iter().all(|&c| c >= 0 && c < SIDE_LENGTH as i16) &&
            {
                let null_cell = cells::Cell::Void.into();
                let cell = self.cell(map_pos.cast()).unwrap_or(&null_cell);
                match condition {
                    Condition::Distance(ref func) => {
                        let t = {
                            let t = (ray_intersect(mask, map_pos) - start).magnitude();
                            if t.is_nan() {
                                0.0
                            } else {
                                t
                            }
                        };
                        func(cell, t)
                    },
                    Condition::Cell(ref func) => func(cell)
                }
            }
        } {
            mask = glm::less_than_equal(&t_max.xyz(), &glm::min2(&t_max.yzx(), &t_max.zxy()));
            t_max += mask.map(|c| c as u16 as f32).component_mul(&delta_dist);
            map_pos += (mask.map(|c| c as i16).component_mul(&ray_step)).map(|c| c as i16);
        }

        let intersect = ray_intersect(mask, map_pos);
        let final_cell = self.cell(map_pos.cast());
        RaycastResult {
            start_position: start,
            end_position: intersect,
            distance: (intersect - start).magnitude(),
            cell: final_cell.copied()
        }
    }

    pub fn ray_until_nonempty(&self, start: TVec3<f32>, direction: TVec3<f32>) -> RaycastResult {
        self.ray_while_condition(start, direction, Condition::cell(|cell: &Cell| cell.is_empty()))
    }

    pub fn ray_until_distance(&self, start: TVec3<f32>, direction: TVec3<f32>, distance: f32) -> RaycastResult {
        self.ray_while_condition(start, direction, Condition::distance(move |cell: &Cell, voxel_distance: f32| cell.is_empty() && voxel_distance < distance))
    }
}
