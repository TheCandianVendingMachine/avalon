use avalon::shader::{ self, Source, Program, };
use avalon::viewport;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::gpu_buffer;
use avalon::debug::GpuAnnotation;
use crate::voxel;
use crate::render::{ Camera, PassOptions, Light };

use nalgebra_glm::vec3;

pub struct PassLighting {
    shader: Program,
    pub lighting_buffer: GpuTexture2d,
    options: PassOptions,
}

impl PassLighting {
    pub fn new(options: PassOptions) -> PassLighting {
        PassLighting {
            shader: Program::new()
                .compute(shader::Compute::load_from_path("assets/shaders/voxel/lighting.comp").unwrap())
                .build()
                .unwrap(),
            lighting_buffer: GpuTexture2d::generate(Arguments2d {
                data: None,
                dimensions: options.lighting_resolution(),
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::FloatRGBA32,
                mipmap_type: gpu::Mipmap::None
            }),
            options
        }
    }

    pub fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self,
        camera: &Camera,
        grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>,
        normals: &GpuTexture2d,
        positions: &GpuTexture2d,
        lights: &[Light]
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        let _lighting_annotation = GpuAnnotation::push("Deferred Lighting");
        let grid_texture: &GpuTexture3d = grid.try_into().unwrap();
        let mut bind = self.shader.activate();

        bind.sampler("grid", grid_texture).unwrap();
        bind.sampler("normalBuffer", normals).unwrap();
        bind.sampler("positionBuffer", positions).unwrap();
        bind.image("lightingBuffer", &self.lighting_buffer, Access::ReadWrite(0)).unwrap();

        bind.uniform("halveCount").unwrap().set_i32(self.options.lighting_halves as i32);
        bind.uniform("gridSideLength").unwrap().set_i32(SIDE_LENGTH as i32);

        let (dispatch_x, dispatch_y, dispatch_z) = self.shader.dispatch_counts(
            self.options.lighting_resolution().x as usize,
            self.options.lighting_resolution().y as usize,
            1
        );

        let point_lights = lights.iter().filter(|light| light.is_point());
        let directional_lights = lights.iter().filter(|light| light.is_directional());
        let spot_lights = lights.iter().filter(|light| light.is_spotlight());

        {
            let _point_annotation = GpuAnnotation::push("Point Lights");
            bind.uniform("firstPass").unwrap().set_bool(true);
            bind.uniform("lightType").unwrap().set_i32(2);
            for (idx, light) in point_lights.enumerate() {
                if let Light::Point { colour, position, intensity } = *light {
                    bind.uniform("lightColour").unwrap().set_vec3(colour);
                    bind.uniform("lightPosition").unwrap().set_vec3(position);
                    bind.uniform("intensity").unwrap().set_f32(intensity);
                }

                bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
                bind.barrier();

                bind.uniform("firstPass").unwrap().set_bool(false);
            }
        }

        {
            let _point_annotation = GpuAnnotation::push("Spot Lights");
            bind.uniform("lightType").unwrap().set_i32(3);
            for (idx, light) in spot_lights.enumerate() {
                if let Light::Spotlight { colour, position, direction, angle, intensity } = *light {
                    bind.uniform("lightColour").unwrap().set_vec3(colour);
                    bind.uniform("lightPosition").unwrap().set_vec3(position);
                    bind.uniform("lightDirection").unwrap().set_vec3(direction);
                    bind.uniform("lightConeAngle").unwrap().set_f32(angle);
                    bind.uniform("intensity").unwrap().set_f32(intensity);
                }

                bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
                bind.barrier();

                bind.uniform("firstPass").unwrap().set_bool(false);
            }
        }

        {
            let _directional_annotation = GpuAnnotation::push("Directional Lights");
            bind.uniform("lightType").unwrap().set_i32(1);
            for (idx, light) in directional_lights.enumerate() {
                if let Light::Directional { colour, direction, intensity } = *light {
                    bind.uniform("lightColour").unwrap().set_vec3(colour);
                    bind.uniform("lightDirection").unwrap().set_vec3(direction);
                    bind.uniform("intensity").unwrap().set_f32(intensity);
                }

                bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
                bind.barrier();

                bind.uniform("firstPass").unwrap().set_bool(false);
            }
        }
    }
}

