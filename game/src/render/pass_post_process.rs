use avalon::shader::{ self, Source, Program, };
use avalon::viewport;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::gpu_buffer;
use avalon::debug::GpuAnnotation;
use crate::voxel;
use crate::render::{ Camera, PassOptions, Light };

use nalgebra_glm::vec3;

pub struct PassPostProcess {
    tone_mapping: Program,
    gamma_correction: Program,
    pub viewport: viewport::Viewport,
    options: PassOptions,
}

impl PassPostProcess {
    pub fn new(options: PassOptions) -> PassPostProcess {
        PassPostProcess {
            tone_mapping: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/reinhard_tonemap.frag").unwrap())
                .build()
                .unwrap(),
            gamma_correction: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/gamma_correction.frag").unwrap())
                .build()
                .unwrap(),
            viewport: viewport::Viewport::new(options.final_size)
                .colour_attachment()
                    .format(gpu::SizedComponent::RGBA8)
                .build(),
            options
        }
    }

    pub fn execute(
        &self,
        pre_processed_scene: &GpuTexture2d
    ) {
        {
            let _annotation = GpuAnnotation::push("Tone Mapping");
            let viewport = self.viewport.bind();
            let mut bind = self.tone_mapping.activate();
            bind.sampler("texture", pre_processed_scene).unwrap();
            bind.uniform("white").unwrap().set_vec3(vec3(4.0, 4.0, 4.0));

            gpu_buffer::State::degenerate().bind().draw(&bind);
        }

        let _annotation = GpuAnnotation::push("Gamma Correction");
        let mut bind = self.gamma_correction.activate();
        bind.sampler("texture", &self.viewport.colour_attachment(0).unwrap().colour).unwrap();

        gpu_buffer::State::degenerate().bind().draw(&bind);
    }
}

