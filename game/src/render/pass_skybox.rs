use avalon::shader::{ self, Source, Program, };
use avalon::viewport;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::gpu_buffer;
use avalon::debug::GpuAnnotation;
use avalon::model;
use crate::voxel;
use crate::render::{ Camera, PassOptions, Light };

use nalgebra_glm::vec3;

pub struct PassSkybox {
    skybox_shader: Program,
    star_shader: Program,
    pub viewport: viewport::Viewport,
    options: PassOptions
}

impl PassSkybox {
    pub fn new(options: PassOptions) -> PassSkybox {
        PassSkybox {
            skybox_shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/geo/skybox.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/geo/skybox.frag").unwrap())
                .build()
                .unwrap(),
            star_shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/geo/star.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/geo/star.frag").unwrap())
                .build()
                .unwrap(),
            viewport: viewport::Viewport::new(options.final_size)
                .colour_attachment()
                    .tag("albedo")
                    .format(gpu::SizedComponent::RGB8)
                .colour_attachment()
                    .tag("bloom")
                    .format(gpu::SizedComponent::R8)
                .build(),
            options
        }
    }

    pub fn execute(
        &self,
        assets: &avalon::asset_library::Library,
        camera: &Camera,
    ) {
        let _annotation = GpuAnnotation::push("Skydome");
        let _bind = self.viewport.bind();
        {
            let dome = assets.bundle("default").unwrap().tag::<gpu::ManagedTexture<GpuTexture2d>>("default").unwrap();

            let mut shader = self.skybox_shader.activate();
            shader.uniform("uScreenSize").unwrap().set_ivec2(self.options.final_size);
            shader.uniform("view").unwrap().set_mat4(camera.transform.matrix());
            shader.uniform("projection").unwrap().set_mat4(camera.projection);

            shader.sampler("skydome", &*dome);

            gpu_buffer::State::degenerate().bind().draw(&shader);
        }

        {
            let star = assets.bundle("ambient-visuals").unwrap().tag::<model::Model>("star-model").unwrap();

            let mut shader = self.star_shader.activate();
            shader.uniform("view").unwrap().set_mat4(camera.transform.matrix());
            shader.uniform("projection").unwrap().set_mat4(camera.projection);

            star.bind().draw_instanced(&shader, 20000);
        }
    }
}
