use avalon::shader::{ self, Source, Program, };
use avalon::viewport;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::gpu_buffer;
use avalon::debug::GpuAnnotation;
use crate::voxel;
use crate::render::{ Camera, PassOptions, Light };

use nalgebra_glm::vec3;

pub struct PassLightingAoCombine {
    shader: Program,
    pub viewport: viewport::Viewport,
    options: PassOptions,
}

impl PassLightingAoCombine {
    pub fn new(options: PassOptions) -> PassLightingAoCombine {
        PassLightingAoCombine {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/voxel/ao_combine.frag").unwrap())
                .build()
                .unwrap(),
            viewport: viewport::Viewport::new(options.raytrace_size)
                .colour_attachment()
                    .format(gpu::SizedComponent::FloatRGBA32)
                .build(),
            options
        }
    }

    pub fn execute(
        &self,
        scene_ao: &GpuTexture2d,
        scene_lighting: &GpuTexture2d,
        albedo: &GpuTexture2d,
    ) {
        let _annotation = GpuAnnotation::push("Combine With AO");
        let viewport = self.viewport.bind();
        let mut bind = self.shader.activate();
        bind.sampler("lightBuffer", scene_lighting).unwrap();
        bind.sampler("aoBuffer", scene_ao).unwrap();
        bind.sampler("albedoBuffer", albedo).unwrap();

        gpu_buffer::State::degenerate().bind().draw(&bind);
    }
}
