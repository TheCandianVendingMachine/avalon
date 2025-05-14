use avalon::shader::{ self, Source, Program, };
use avalon::viewport;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::gpu_buffer;
use avalon::debug::GpuAnnotation;
use crate::voxel;
use crate::render::{ Camera, PassOptions };

use nalgebra_glm::vec3;

pub struct PassRaytrace {
    shader: Program,
    pub viewport: viewport::Viewport,
    options: PassOptions,
}

impl PassRaytrace {
    pub fn new(options: PassOptions) -> PassRaytrace {
        let mut viewport = viewport::Viewport::new(options.raytrace_size)
            .colour_attachment()
                .tag("albedo")
                .format(gpu::SizedComponent::SRGB8A8)
            .colour_attachment()
                .tag("normal")
                .format(gpu::SizedComponent::NormalRGB8)
            .colour_attachment()
                .tag("tangent")
                .format(gpu::SizedComponent::NormalRGB8)
            .colour_attachment()
                .tag("position")
                .format(gpu::SizedComponent::FloatRGB32)
            .depth_stencil(viewport::DepthStencil::Depth)
            .build();
        viewport.bind_mut()
            .depth_test()
            .enable(true)
            .function(viewport::depth_options::Function::Always)
            .clear_value(1.0)
            .finish()
        .set_clear_colour(vec3(0.0, 0.0, 0.0));
        PassRaytrace {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/voxel/world.frag").unwrap())
                .build()
                .unwrap(),
            viewport,
            options
        }
    }

    pub fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &mut self,
        camera: &Camera,
        grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>,
        albedo: GpuTexture2d,
        normal: GpuTexture2d
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        let _raytrace_annotation = GpuAnnotation::push("Raytrace Grid");
        let grid_texture: &GpuTexture3d = grid.try_into().unwrap();
        let mut bind = self.shader.activate();
        bind.uniform("uScreenSize").unwrap().set_ivec2(self.options.raytrace_size);

        bind.sampler("grid", grid_texture).unwrap();
        bind.sampler("albedo", &albedo).unwrap();
        bind.sampler("tNormal", &normal).unwrap();
        //bind.sampler("bump", grid_texture).unwrap();

        bind.uniform("view").unwrap().set_mat4(camera.transform.matrix());
        //bind.uniform("inverseView").unwrap().set_mat4(camera.transform.matrix().try_inverse().unwrap());
        bind.uniform("projection").unwrap().set_mat4(camera.projection);
        //bind.uniform("inverseProjection").unwrap().set_mat4(camera.projection.try_inverse().unwrap());
        bind.uniform("cameraPos").unwrap().set_vec3(camera.transform.position());
        bind.uniform("gridSideLength").unwrap().set_i32(SIDE_LENGTH as i32);

        {
            let bind = self.viewport.bind_mut();
            bind.depth_test()
                .function(viewport::depth_options::Function::Always)
                .finish();
        }

        // draw command
        let viewport_bind = self.viewport.bind();
        viewport_bind.clear();
        gpu_buffer::State::degenerate().bind().draw(&bind);
    }
}

