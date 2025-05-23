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
    combiner: Program,
    combine_viewport: viewport::Viewport,
    bloom: Program,
    bloom_viewport: viewport::Viewport,
    tone_mapping: Program,
    gamma_correction: Program,
    pub viewport: viewport::Viewport,
    options: PassOptions,
}

impl PassPostProcess {
    pub fn new(options: PassOptions) -> PassPostProcess {
        PassPostProcess {
            combiner: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/pp_combiner.frag").unwrap())
                .build()
                .unwrap(),
            bloom: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/bloom.frag").unwrap())
                .build()
                .unwrap(),
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
            combine_viewport: viewport::Viewport::new(options.final_size)
                .colour_attachment()
                    .format(gpu::SizedComponent::FloatRGBA32)
                .build(),
            bloom_viewport: viewport::Viewport::new(options.final_size)
                .colour_attachment()
                    .format(gpu::SizedComponent::FloatRGBA32)
                .build(),
            viewport: viewport::Viewport::new(options.final_size)
                .colour_attachment()
                    .format(gpu::SizedComponent::RGBA8)
                .build(),
            options
        }
    }

    pub fn execute(
        &self,
        bloom_map: &GpuTexture2d,
        skydome: &GpuTexture2d,
        pre_processed_scene: &GpuTexture2d
    ) {
        {
            let _annotation = GpuAnnotation::push("Bloom");
            let viewport = self.bloom_viewport.bind();
            let mut bind = self.bloom.activate();
            bind.uniform("uScreenSize").unwrap().set_ivec2(self.options.final_size);
            bind.sampler("scene", skydome).unwrap();
            bind.sampler("bloom", bloom_map).unwrap();

            gpu_buffer::State::degenerate().bind().draw(&bind);
        }
        {
            let _annotation = GpuAnnotation::push("Combine Renders");
            let viewport = self.combine_viewport.bind();
            let mut bind = self.combiner.activate();
            bind.uniform("uScreenSize").unwrap().set_ivec2(self.options.final_size);
            bind.sampler("colour", &self.bloom_viewport.colour_attachment(0).unwrap().colour).unwrap();
            gpu_buffer::State::degenerate().bind().draw(&bind);

            bind.sampler("colour", pre_processed_scene).unwrap();
            gpu_buffer::State::degenerate().bind().draw(&bind);
        }
        {
            let _annotation = GpuAnnotation::push("Tone Mapping");
            let viewport = self.viewport.bind();
            let mut bind = self.tone_mapping.activate();
            bind.sampler("texture", &self.combine_viewport.colour_attachment(0).unwrap().colour).unwrap();
            bind.uniform("white").unwrap().set_vec3(vec3(4.0, 4.0, 4.0));

            gpu_buffer::State::degenerate().bind().draw(&bind);
        }

        let _annotation = GpuAnnotation::push("Gamma Correction");
        let mut bind = self.gamma_correction.activate();
        bind.sampler("texture", &self.viewport.colour_attachment(0).unwrap().colour).unwrap();

        gpu_buffer::State::degenerate().bind().draw(&bind);
    }
}

