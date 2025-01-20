#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused)]

use nalgebra_glm::{ Mat3, TVec3, Vec2, IVec2, vec2, vec3 };

pub mod voxel;

use avalon;
use avalon::viewport;
use avalon::shader::{ self, Source, Program, };
use avalon::texture::data;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, Access, Sampler, Image };

struct Camera {
    transform: avalon::transform::Transform,
    projection: Mat3
}

impl Camera {
    fn new(dimensions: IVec2) -> Camera {
        let dimensions: Vec2 = dimensions.cast();
        Camera {
            transform: avalon::transform::Transform::new(),
            projection: Mat3::new(
                1.0, 0.0, 0.0,
                0.0, dimensions.y / dimensions.x, 0.0,
                0.0, 0.0, 1.0
            )
        }
    }
}

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
    albedo: GpuTexture2d,
    normal: GpuTexture2d,
    options: PassOptions,
}

impl PassRaytrace {
    fn new(options: PassOptions) -> PassRaytrace {
        let albedo_data = data::Data::from_file("assets/bins/textures/wall_texture_full.png");
        let normal_data = data::Data::from_file("assets/bins/textures/wall_texture_full_normal.png");
        PassRaytrace {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/voxel/world.frag").unwrap())
                .build()
                .unwrap(),
            viewport: viewport::Viewport::new(options.final_size)
                .colour_attachment()
                    .format(gpu::SizedComponent::RGB8)
                .colour_attachment()
                    .format(gpu::SizedComponent::NormalRGB8)
                .colour_attachment()
                    .format(gpu::SizedComponent::NormalRGB8)
                .colour_attachment()
                    .format(gpu::SizedComponent::FloatRGB32)
                .depth_stencil(viewport::DepthStencil::Depth)
                .build(),
            albedo: GpuTexture2d::generate(Arguments2d {
                data: Some(albedo_data),
                dimensions: vec2(96, 224),
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::RGBA8,
                mipmap_type: gpu::Mipmap::None,
            }),
            normal: GpuTexture2d::generate(Arguments2d {
                data: Some(normal_data),
                dimensions: vec2(96, 224),
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::RGBA8,
                mipmap_type: gpu::Mipmap::None,
            }),
            options
        }
    }

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self,
        camera: &Camera,
        grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>,
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        let grid_texture: &GpuTexture3d = grid.try_into().unwrap();
        let mut bind = self.shader.activate();
        bind.uniform("uScreenSize").unwrap().set_ivec2(self.options.final_size);

        bind.sampler("grid", grid_texture).unwrap();
        bind.sampler("albedo", &self.albedo).unwrap();
        bind.sampler("tNormal", &self.normal).unwrap();
        //bind.sampler("bump", grid_texture).unwrap();

        bind.uniform("view").unwrap().set_mat4(camera.transform.matrix());
        bind.uniform("inverseView").unwrap().set_mat4(camera.transform.matrix().try_inverse().unwrap());
        bind.uniform("projection").unwrap().set_mat3(camera.projection);
        bind.uniform("inverseProjection").unwrap().set_mat3(camera.projection.try_inverse().unwrap());
        bind.uniform("cameraPos").unwrap().set_vec3(camera.transform.position());

        // draw command
        let viewport_bind = self.viewport.bind();
        bind.temp_render();
    }
}

struct PassLighting {
    shader: Program,
    viewport: viewport::Viewport,
    albedo: GpuTexture2d,
    normal: GpuTexture2d,
    options: PassOptions,
}

impl PassLighting {
    fn new(options: PassOptions) -> PassRaytrace {
        let albedo_data = data::Data::from_file("assets/bins/wall_texture_full.png");
        let normal_data = data::Data::from_file("assets/bins/wall_texture_full_normal.png");
        PassRaytrace {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/voxel/world.frag").unwrap())
                .build()
                .unwrap(),
            viewport: viewport::Viewport::new(options.final_size)
                .depth_stencil(viewport::DepthStencil::Depth)
                .build(),
            albedo: GpuTexture2d::generate(Arguments2d {
                data: Some(albedo_data),
                dimensions: vec2(96, 224),
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::RGBA8,
                mipmap_type: gpu::Mipmap::None,
            }),
            normal: GpuTexture2d::generate(Arguments2d {
                data: Some(normal_data),
                dimensions: vec2(96, 224),
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::RGBA8,
                mipmap_type: gpu::Mipmap::None,
            }),
            options
        }
    }

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self,
        camera: &Camera,
        grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>,
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        let grid_texture: &GpuTexture3d = grid.try_into().unwrap();
        let mut bind = self.shader.activate();
        bind.uniform("uScreenSize").unwrap().set_ivec2(self.options.final_size);

        bind.sampler("grid", grid_texture).unwrap();
        bind.sampler("albedo", &self.albedo).unwrap();
        bind.sampler("tNormal", &self.normal).unwrap();
        //bind.sampler("bump", grid_texture).unwrap();

        bind.uniform("view").unwrap().set_mat4(camera.transform.matrix());
        bind.uniform("inverseView").unwrap().set_mat4(camera.transform.matrix().try_inverse().unwrap());
        bind.uniform("projection").unwrap().set_mat3(camera.projection);
        bind.uniform("inverseProjection").unwrap().set_mat3(camera.projection.try_inverse().unwrap());
        bind.uniform("cameraPos").unwrap().set_vec3(camera.transform.position());

        // draw command
        bind.temp_render();
    }
}

struct RenderPass {
    options: PassOptions,
    pass_raytrace: PassRaytrace,
}

impl RenderPass {
    fn new() -> RenderPass {
        let options = PassOptions {
            final_size: vec2(1280, 720)
        };

        let pass_raytrace = PassRaytrace::new(options);
        RenderPass {
            options,
            pass_raytrace
        }
    }

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self, camera: &Camera, grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        self.pass_raytrace.execute(camera, grid);
    }
}

fn main() {
    let mut engine = avalon::engine();

    let mut camera = Camera::new(vec2(1280, 720));
    camera.transform.set_position(vec3(0.0, 5.0, -5.0));
    camera.transform.set_euler_angles(avalon::transform::Euler {
        pitch: 0.0,
        yaw: std::f32::consts::FRAC_PI_8,
        roll: 0.0
    });

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

    for x in 13..20 {
        for y in 1..8 {
            set_cell(vec3(x, y, 15));
        }
    }

    for x in 5..10 {
        for y in 1..10 {
            set_cell(vec3(x, y, 15));
        }
    }

    for y in 1..=5 {
        for x in 1..=5 {
            set_cell(vec3(x, y, 3));
            set_cell(vec3(x, y, 8));
        }

        for z in 3..=8 {
            set_cell(vec3(5, y, z));
        }
    }
    grid.bake();

    let render_pass = RenderPass::new();

    while engine.is_open() {
        engine.start_frame();
        engine.poll_events();
        engine.render();
        render_pass.execute(&camera, &grid);
        engine.swap();
        engine.end_frame();
    }
}
