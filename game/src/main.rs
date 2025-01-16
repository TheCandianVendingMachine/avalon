#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused)]

use nalgebra_glm::{ TVec3, IVec2, vec2, vec3 };

pub mod voxel;

use avalon;
use avalon::viewport;
use avalon::shader::{ self, Source, Program, };
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, Access, Sampler, Image };

struct GeometryBuffers {
    position: GpuTexture2d,
    normal: GpuTexture2d,
    tangent: GpuTexture2d,
}

#[derive(Debug, Copy, Clone)]
struct PassOptions {
    final_size: IVec2
}

struct PassRaytrace {
    shader: Program,
    viewport: viewport::Viewport,
    options: PassOptions,
}

impl PassRaytrace {
    fn new(options: PassOptions) -> PassRaytrace {
        PassRaytrace {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/voxel/world.frag").unwrap())
                .build()
                .unwrap(),
            viewport: viewport::Viewport::new(options.final_size)
                .depth_stencil(viewport::DepthStencil::Depth)
                .build(),
            options
        }
    }

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self,
        grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>,
        buffers: &GeometryBuffers,
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        let grid_texture: &GpuTexture3d = grid.try_into().unwrap();
        let mut bind = self.shader.activate();
        bind.uniform("uScreenSize").unwrap().set_ivec2(self.options.final_size);

        bind.sampler("grid", grid_texture).unwrap();
        //bind.sampler("albedo", grid_texture).unwrap();
        //bind.sampler("normal", grid_texture).unwrap();
        //bind.sampler("bump", grid_texture).unwrap();

        //bind.uniform("view").unwrap().set_mat4();
        //bind.uniform("inverseView").unwrap().set_mat4();
        //bind.uniform("projection").unwrap().set_mat3();
        //bind.uniform("inverseProjection").unwrap().set_mat3();
        bind.uniform("cameraPos").unwrap().set_vec3(vec3(3.0, 7.0, 2.0));

        bind.image("positionBuffer", &buffers.position, Access::Write).unwrap();
        bind.image("normalBuffer", &buffers.normal, Access::Write).unwrap();
        bind.image("tangentBuffer", &buffers.tangent, Access::Write).unwrap();

        // draw command
    }
}

struct RenderPass {
    options: PassOptions,
    geometry_buffers: GeometryBuffers,
    pass_raytrace: PassRaytrace,
}

impl RenderPass {
    fn new() -> RenderPass {
        let options = PassOptions {
            final_size: vec2(1280, 720)
        };

        let geometry_buffers = {
            let textures = GpuTexture2d::generate_many::<3>(Arguments2d {
                dimensions: options.final_size,
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::RGBA8,
                mipmap_type: gpu::Mipmap::None,
                data: None
            });
            GeometryBuffers {
                position: textures[0],
                normal: textures[1],
                tangent: textures[2],
            }
        };

        let pass_raytrace = PassRaytrace::new(options);
        RenderPass {
            options,
            geometry_buffers,
            pass_raytrace
        }
    }

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self, grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        self.pass_raytrace.execute(grid, &self.geometry_buffers);
    }
}

fn main() {
    let mut engine = avalon::engine();

    let mut grid: voxel::Grid<32, 1> = voxel::Grid::new();
    let mut set_cell = |position: TVec3<u8>| {
        let mut cell = grid.cell_mut(position);
        cell.set_empty(0);
        cell.set_opaque(1);
        cell.set_cell_id(1);
    };

    for x in 0..20 {
        for z in 0..32 {
            set_cell(vec3(x, 0, z));
        }
    }

    for x in 13..18 {
        for y in 1..10 {
            set_cell(vec3(x, y, 15));
        }
    }

    for x in 5..10 {
        for y in 1..10 {
            set_cell(vec3(x, y, 15));
        }
    }

    for x in 0..10 {
        for y in 1..=5 {
            set_cell(vec3(x, y, 3));
            set_cell(vec3(x, y, 8));
        }

        for z in 3..=8 {
            set_cell(vec3(x, 5, z));
        }
    }
    grid.bake();

    let render_pass = RenderPass::new();

    while engine.is_open() {
        engine.start_frame();
        engine.poll_events();
        engine.render();
        render_pass.execute(&grid);
        engine.end_frame();
    }
}
