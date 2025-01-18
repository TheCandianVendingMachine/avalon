#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused)]

use nalgebra_glm::{ Mat3, Vec3, TVec3, Vec2, IVec2, vec2, vec3 };

pub mod voxel;

use avalon;
use avalon::viewport;
use avalon::shader::{ self, Source, Program, };
use avalon::texture::data;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, Access, Sampler, Image };

#[derive(Debug, Copy, Clone)]
enum Light {
    Directional { colour: Vec3, direction: Vec3, intensity: f32 },
    Point { colour: Vec3, position: Vec3, intensity: f32 },
    Spotlight { colour: Vec3, position: Vec3, direction: Vec3, angle: f32, intensity: f32 },
}

impl Light {
    fn is_directional(&self) -> bool {
        if let Light::Directional {..} = self {
            true
        } else {
            false
        }
    }

    fn is_point(&self) -> bool {
        if let Light::Point {..} = self {
            true
        } else {
            false
        }
    }

    fn is_spotlight(&self) -> bool {
        if let Light::Spotlight {..} = self {
            true
        } else {
            false
        }
    }
}

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
    final_size: IVec2,
    lighting_halves: u32,
    ao_halves: u32,
}

impl PassOptions {
    fn lighting_resolution(&self) -> IVec2 {
        self.final_size / 2_i32.pow(self.lighting_halves)
    }

    fn ao_resolution(&self) -> IVec2 {
        self.final_size / 2_i32.pow(self.lighting_halves)
    }
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
        let albedo_data = data::Data::from_file("assets/bins/wall_texture_full.png");
        let normal_data = data::Data::from_file("assets/bins/wall_texture_full_normal.png");
        PassRaytrace {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/voxel/world.frag").unwrap())
                .build()
                .unwrap(),
            viewport: viewport::Viewport::new(options.final_size)
                .colour_attachment()
                    .tag("albedo")
                    .format(gpu::SizedComponent::RGB8)
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
    lighting_buffer: GpuTexture2d,
    options: PassOptions,
}

impl PassLighting {
    fn new(options: PassOptions) -> PassLighting {
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

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self,
        camera: &Camera,
        grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>,
        normals: &GpuTexture2d,
        positions: &GpuTexture2d,
        lights: &Vec<Light>
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        let grid_texture: &GpuTexture3d = grid.try_into().unwrap();
        let mut bind = self.shader.activate();

        bind.sampler("grid", grid_texture).unwrap();
        bind.sampler("normalBuffer", normals).unwrap();
        bind.sampler("positionBuffer", positions).unwrap();
        bind.image("lightingBuffer", &self.lighting_buffer, Access::ReadWrite).unwrap();

        bind.uniform("halveCount").unwrap().set_i32(self.options.lighting_halves as i32);

        let (dispatch_x, dispatch_y, dispatch_z) = self.shader.dispatch_counts(
            self.options.lighting_resolution().x as usize,
            self.options.lighting_resolution().y as usize,
            1
        );

        let point_lights = lights.iter().filter(|light| light.is_point());
        let directional_lights = lights.iter().filter(|light| light.is_directional());
        let spot_lights = lights.iter().filter(|light| light.is_spotlight());

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

struct RenderPass {
    options: PassOptions,
    pass_raytrace: PassRaytrace,
    pass_lighting: PassLighting,
    lights: Vec<Light>
}

impl RenderPass {
    fn new() -> RenderPass {
        let mut lights = Vec::new();
        lights.push(
            Light::Directional {
                colour: vec3(1.0, 0.90, 0.95),
                direction: vec3(1.0, -0.4, 0.2).normalize(),
                intensity: 0.3
            }
        );

        lights.push(
            Light::Point {
                colour: vec3(0.6, 0.6, 0.6),
                position: vec3(4.0, 9.0, 25.0),
                intensity: 60.0
            }
        );

        lights.push(
            Light::Spotlight {
                colour: vec3(0.6, 0.6, 1.0),
                position: vec3(30.5, 8.0, -5.0),
                direction: vec3(-0.8, -0.3, 1.0).normalize(),
                angle: 5.0_f32.to_radians(),
                intensity: 70.0
            }
        );

        let options = PassOptions {
            final_size: vec2(1280, 720),
            lighting_halves: 0,
            ao_halves: 0
        };

        let pass_raytrace = PassRaytrace::new(options);
        let pass_lighting = PassLighting::new(options);
        RenderPass {
            options,
            pass_raytrace,
            pass_lighting,
            lights
        }
    }

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self, camera: &Camera, grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        self.pass_raytrace.execute(camera, grid);
        let albedo = self.pass_raytrace.viewport.tagged_colour("albedo").unwrap().colour;
        let normals = self.pass_raytrace.viewport.tagged_colour("normal").unwrap().colour;
        let tangents = self.pass_raytrace.viewport.tagged_colour("tangent").unwrap().colour;
        let positions = self.pass_raytrace.viewport.tagged_colour("position").unwrap().colour;
        self.pass_lighting.execute(
            camera,
            grid,
            &normals,
            &positions,
            &self.lights
        );
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
