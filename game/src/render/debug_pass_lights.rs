use avalon::shader::{ self, Source, Program, };
use avalon::viewport;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::gpu_buffer;
use avalon::debug::GpuAnnotation;
use crate::voxel;
use crate::render::{ Camera, PassOptions, Light, DebugLight };

use nalgebra_glm::vec3;

pub struct DebugPassLights {
    shader: Program,
    light_buffer: gpu_buffer::storage::Storage,
}

impl DebugPassLights {
    pub fn new() -> DebugPassLights {
        DebugPassLights {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/dev/light.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/dev/light.frag").unwrap())
                .build()
                .unwrap(),
            light_buffer: gpu_buffer::storage::Storage::new()
        }
    }

    pub fn execute(
        &mut self,
        icon_bundle: avalon::asset_library::BundleView,
        camera: &Camera,
        lights: &Vec<Light>
    ) {
        let _annotation = GpuAnnotation::push("Light Icons");
        viewport::Viewport::screen_viewport()
            .bind_mut()
            .depth_test()
            .enable(true)
            .function(viewport::depth_options::Function::Less)
            .finish();

        let icon_pointlight = icon_bundle.tag::<GpuTexture2d>("pointlight").unwrap();
        let spotlight_on = icon_bundle.tag::<GpuTexture2d>("spotlight-off").unwrap();
        let spotlight_off = icon_bundle.tag::<GpuTexture2d>("spotlight-off").unwrap();

        let point_lights = lights.iter().filter(|light| light.is_point());
        let spot_lights = lights.iter().filter(|light| light.is_spotlight());

        let mut debug_lights = Vec::new();
        for light in point_lights {
            let Light::Point { colour, position, .. } = light else { panic!() };
            debug_lights.push(DebugLight {
                colour: (colour.x, colour.y, colour.z, 1.0),
                position: (position.x, position.y, position.z),
            });
        }

        let usage = gpu_buffer::storage::Usage::Dynamic(gpu_buffer::storage::Access::CpuWrite);
        {
            let mut light_shader = self.shader.activate();
            light_shader.uniform("view").unwrap().set_mat4(camera.transform.matrix());
            light_shader.uniform("projection").unwrap().set_mat4(camera.projection);
            light_shader.sampler("icon", &*icon_pointlight).unwrap();

            self.light_buffer.bind_mut().write_structs(
                &debug_lights,
                usage
            );
            let storage_bind = self.light_buffer.bind();
            light_shader.storage(0, &storage_bind, usage);
            light_shader.barrier();
            gpu_buffer::State::degenerate().bind().draw_instanced(&light_shader, debug_lights.len());
        }

        debug_lights.clear();
        for light in spot_lights {
            let Light::Spotlight { colour, position, .. } = light else { panic!() };
            debug_lights.push(DebugLight {
                position: (position.x, position.y, position.z),
                colour: (colour.x, colour.y, colour.z, 1.0),
            });
        }
        {
            let mut light_shader = self.shader.activate();
            light_shader.uniform("view").unwrap().set_mat4(camera.transform.matrix());
            light_shader.uniform("projection").unwrap().set_mat4(camera.projection);
            light_shader.sampler("icon", &*spotlight_on).unwrap();

            self.light_buffer.bind_mut().write_structs(
                &debug_lights,
                usage
            );
            let storage_bind = self.light_buffer.bind();
            light_shader.storage(0, &storage_bind, usage);
            light_shader.barrier();
            gpu_buffer::State::degenerate().bind().draw_instanced(&light_shader, debug_lights.len());
        }
    }
}

