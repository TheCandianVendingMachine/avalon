use avalon::shader::{ self, Source, Program, };
use avalon::viewport;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::gpu_buffer;
use avalon::debug::GpuAnnotation;
use crate::voxel;
use crate::render::{ Camera, PassOptions, Light };

use nalgebra_glm::vec3;

pub struct PassLightingAo {
    shader_voxelize_light: Program,
    shader_mipmap_light_voxels: Program,
    shader_conetrace: Program,
    pub viewport: viewport::Viewport,
    light_voxels: GpuTexture3d,
    options: PassOptions,
}

impl PassLightingAo {
    pub fn new(options: PassOptions, side_length: usize) -> PassLightingAo {
        PassLightingAo {
            shader_voxelize_light: Program::new()
                .compute(shader::Compute::load_from_path("assets/shaders/voxel/voxelize_light.comp").unwrap())
                .build()
                .unwrap(),
            shader_mipmap_light_voxels: Program::new()
                .compute(shader::Compute::load_from_path("assets/shaders/voxel/mipmap_light.comp").unwrap())
                .build()
                .unwrap(),
            shader_conetrace: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/voxel/cone_indirect.frag").unwrap())
                .build()
                .unwrap(),
            viewport: viewport::Viewport::new(options.ao_resolution())
                .colour_attachment()
                    .format(gpu::SizedComponent::FloatRGBA32)
                .build(),
            light_voxels: GpuTexture3d::generate_storage(gpu::Arguments3d {
                dimensions: vec3(side_length as i32, side_length as i32, side_length as i32),
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::FloatRGBA32,
                mipmap_type: gpu::Mipmap::None,
                data: None,
            }, 5),
            options
        }
    }

    pub fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self,
        lighted_scene: &GpuTexture2d,
        grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>,
        positions: &GpuTexture2d,
        normals: &GpuTexture2d,
        tangents: &GpuTexture2d,
        delta_time: f32,
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        let _annotation = GpuAnnotation::push("Cone Trace AO");
        {
            let _voxelize_annotation = GpuAnnotation::push("Voxelize Light");
            let mut bind = self.shader_voxelize_light.activate();
            bind.sampler("lightedScene", lighted_scene).unwrap();
            bind.sampler("positions", positions).unwrap();
            bind.image("lightVoxel", &self.light_voxels, Access::ReadWrite(0)).unwrap();
            //bind.uniform("halvedCount").unwrap().set_i32(self.options.lighting_halves as i32);
            bind.uniform("deltaTime").unwrap().set_f32(delta_time);

            let (dispatch_x, dispatch_y, dispatch_z) = self.shader_voxelize_light.dispatch_counts(
                self.options.lighting_resolution().x as usize,
                self.options.lighting_resolution().y as usize,
                1
            );
            bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
            bind.barrier();
        }

        {
            let _voxelize_annotation = GpuAnnotation::push("Mipmap Light Voxels");
            let mut bind = self.shader_mipmap_light_voxels.activate();
            bind.sampler("lightVoxels", &self.light_voxels).unwrap();

            for level in 1..self.light_voxels.levels() {
                bind.image("mipmap", &self.light_voxels, Access::ReadWrite(level)).unwrap();
                bind.uniform("level").unwrap().set_i32(level as i32);

                let dimension = SIDE_LENGTH / 2_usize.pow(level);
                let (dispatch_x, dispatch_y, dispatch_z) = self.shader_mipmap_light_voxels.dispatch_counts(
                    dimension,
                    dimension,
                    dimension
                );
                bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
            }
        }

        let grid_texture: &GpuTexture3d = grid.try_into().unwrap();

        let viewport = self.viewport.bind();
        let mut bind = self.shader_conetrace.activate();
        bind.sampler("positionBuffer", positions).unwrap();
        bind.sampler("normalBuffer", normals).unwrap();
        bind.sampler("tangentBuffer", tangents).unwrap();
        bind.sampler("lightGrid", &self.light_voxels).unwrap();
        bind.sampler("grid", grid_texture).unwrap();
        bind.uniform("resolution").unwrap().set_i32(1);
        bind.uniform("halveCount").unwrap().set_i32(self.options.ao_halves as i32);

        gpu_buffer::State::degenerate().bind().draw(&bind);
    }
}
