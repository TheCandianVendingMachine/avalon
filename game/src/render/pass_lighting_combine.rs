use avalon::shader::{ self, Source, Program, };
use avalon::viewport;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::gpu_buffer;
use avalon::debug::GpuAnnotation;
use crate::voxel;
use crate::render::{ Camera, PassOptions, Light };

use nalgebra_glm::vec3;

pub struct PassLightingCombine {
    shader: Program,
    pub viewport: viewport::Viewport,
    options: PassOptions,
}

impl PassLightingCombine {
    pub fn new(options: PassOptions) -> PassLightingCombine {
        PassLightingCombine {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/voxel/combine.frag").unwrap())
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
        albedo: &GpuTexture2d,
        light: &GpuTexture2d,
    ) {
        let _annotation = GpuAnnotation::push("Combine Light and Texture");
        let mut bind = self.shader.activate();

        bind.sampler("albedo", albedo).unwrap();
        bind.sampler("light", light).unwrap();

        let viewport = self.viewport.bind();
        gpu_buffer::State::degenerate().bind().draw(&bind);
    }
}

