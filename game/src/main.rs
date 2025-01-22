#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused)]

use nalgebra_glm::{ ortho, Mat4, Mat3, Vec3, TVec3, Vec2, IVec2, vec2, vec3 };

pub mod voxel;

use avalon;
use avalon::debug::GpuAnnotation;
use avalon::gpu_buffer;
use avalon::viewport;
use avalon::shader::{ self, Source, Program, };
use avalon::texture::algorithms;
use avalon::texture::data;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };

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
    perspective: Mat4,
    projection: Mat3,
}

impl Camera {
    fn new(dimensions: IVec2) -> Camera {
        let dimensions: Vec2 = dimensions.cast();
        Camera {
            transform: avalon::transform::Transform::new(),
            perspective: ortho(
                -dimensions.x,
                dimensions.x,
                -dimensions.y,
                dimensions.y,
                0.01,
                10000.0,
            ),
            projection: Mat3::new(
                1.0, 0.0, 0.0,
                0.0, dimensions.y / dimensions.x, 0.0,
                0.0, 0.0, 1.0
            ),
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
    raytrace_size: IVec2,
    lighting_halves: u32,
    ao_halves: u32,
}

impl PassOptions {
    fn lighting_resolution(&self) -> IVec2 {
        self.raytrace_size / 2_i32.pow(self.lighting_halves)
    }

    fn ao_resolution(&self) -> IVec2 {
        self.raytrace_size / 2_i32.pow(self.ao_halves)
    }
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
            viewport: viewport::Viewport::new(options.raytrace_size)
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
                .build(),
            options
        }
    }

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self,
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
        bind.uniform("inverseView").unwrap().set_mat4(camera.transform.matrix().try_inverse().unwrap());
        bind.uniform("projection").unwrap().set_mat3(camera.projection);
        bind.uniform("inverseProjection").unwrap().set_mat3(camera.projection.try_inverse().unwrap());
        bind.uniform("cameraPos").unwrap().set_vec3(camera.transform.position());

        // draw command
        let viewport_bind = self.viewport.bind();
        gpu_buffer::State::degenerate().bind().draw(&bind);
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
        let _lighting_annotation = GpuAnnotation::push("Deferred Lighting");
        let grid_texture: &GpuTexture3d = grid.try_into().unwrap();
        let mut bind = self.shader.activate();

        bind.sampler("grid", grid_texture).unwrap();
        bind.sampler("normalBuffer", normals).unwrap();
        bind.sampler("positionBuffer", positions).unwrap();
        bind.image("lightingBuffer", &self.lighting_buffer, Access::ReadWrite(0)).unwrap();

        bind.uniform("halveCount").unwrap().set_i32(self.options.lighting_halves as i32);

        let (dispatch_x, dispatch_y, dispatch_z) = self.shader.dispatch_counts(
            self.options.lighting_resolution().x as usize,
            self.options.lighting_resolution().y as usize,
            1
        );

        let point_lights = lights.iter().filter(|light| light.is_point());
        let directional_lights = lights.iter().filter(|light| light.is_directional());
        let spot_lights = lights.iter().filter(|light| light.is_spotlight());

        {
            let _point_annotation = GpuAnnotation::push("Point Lights");
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
        }

        {
            let _point_annotation = GpuAnnotation::push("Spot Lights");
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
        }

        {
            let _directional_annotation = GpuAnnotation::push("Directional Lights");
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
}

struct PassLightingCombine {
    shader: Program,
    viewport: viewport::Viewport,
    options: PassOptions,
}

impl PassLightingCombine {
    fn new(options: PassOptions) -> PassLightingCombine {
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

    fn execute(
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

struct PassLightingAo {
    shader_voxelize_light: Program,
    shader_mipmap_light_voxels: Program,
    shader_conetrace: Program,
    viewport: viewport::Viewport,
    light_voxels: GpuTexture3d,
    options: PassOptions,
}

impl PassLightingAo {
    fn new(options: PassOptions, side_length: usize) -> PassLightingAo {
        PassLightingAo {
            shader_voxelize_light: Program::new()
                .compute(shader::Compute::load_from_path("assets/shaders/voxel/voxelize_light.comp").unwrap())
                .build()
                .unwrap(),
            shader_mipmap_light_voxels: Program::new()
                .compute(shader::Compute::load_from_path("assets/shaders/voxel/mipmap_light.comp").unwrap())
                .build()
                .unwrap(),
            shader_conetrace: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/voxel/cone_indirect.frag").unwrap())
                .build()
                .unwrap(),
            viewport: viewport::Viewport::new(options.ao_resolution())
                .colour_attachment()
                    .format(gpu::SizedComponent::FloatRGBA32)
                .build(),
            light_voxels: GpuTexture3d::generate_storage(gpu::Arguments3d {
                dimensions: vec3(side_length as i32, side_length as i32, side_length as i32),
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::FloatRGBA32,
                mipmap_type: gpu::Mipmap::None,
                data: None,
            }, 5),
            options
        }
    }

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self,
        lighted_scene: &GpuTexture2d,
        grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>,
        positions: &GpuTexture2d,
        normals: &GpuTexture2d,
        tangents: &GpuTexture2d,
        delta_time: f32,
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        let _annotation = GpuAnnotation::push("Cone Trace AO");
        {
            let _voxelize_annotation = GpuAnnotation::push("Voxelize Light");
            let mut bind = self.shader_voxelize_light.activate();
            bind.sampler("lightedScene", lighted_scene).unwrap();
            bind.sampler("positions", positions).unwrap();
            bind.image("lightVoxel", &self.light_voxels, Access::ReadWrite(0)).unwrap();
            bind.uniform("halvedCount").unwrap().set_i32(self.options.lighting_halves as i32);
            bind.uniform("deltaTime").unwrap().set_f32(delta_time);

            let (dispatch_x, dispatch_y, dispatch_z) = self.shader_voxelize_light.dispatch_counts(
                self.options.lighting_resolution().x as usize,
                self.options.lighting_resolution().y as usize,
                1
            );
            bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
            bind.barrier();
        }

        {
            let _voxelize_annotation = GpuAnnotation::push("Mipmap Light Voxels");
            let mut bind = self.shader_mipmap_light_voxels.activate();
            bind.sampler("lightVoxels", &self.light_voxels).unwrap();

            for level in 1..self.light_voxels.levels() {
                bind.image("mipmap", &self.light_voxels, Access::ReadWrite(level)).unwrap();
                bind.uniform("level").unwrap().set_i32(level as i32);

                let dimension = SIDE_LENGTH / 2_usize.pow(level);
                let (dispatch_x, dispatch_y, dispatch_z) = self.shader_mipmap_light_voxels.dispatch_counts(
                    dimension,
                    dimension,
                    dimension
                );
                bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
            }
        }

        let grid_texture: &GpuTexture3d = grid.try_into().unwrap();

        let viewport = self.viewport.bind();
        let mut bind = self.shader_conetrace.activate();
        bind.sampler("positionBuffer", positions).unwrap();
        bind.sampler("normalBuffer", normals).unwrap();
        bind.sampler("tangentBuffer", tangents).unwrap();
        bind.sampler("lightGrid", &self.light_voxels).unwrap();
        bind.sampler("grid", grid_texture).unwrap();
        bind.uniform("resolution").unwrap().set_i32(1);
        bind.uniform("halveCount").unwrap().set_i32(self.options.ao_halves as i32);

        gpu_buffer::State::degenerate().bind().draw(&bind);
    }
}

struct PassLightingAoCombine {
    shader: Program,
    viewport: viewport::Viewport,
    options: PassOptions,
}

impl PassLightingAoCombine {
    fn new(options: PassOptions) -> PassLightingAoCombine {
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

    fn execute(
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

struct PassPostProcess {
    tone_mapping: Program,
    gamma_correction: Program,
    viewport: viewport::Viewport,
    options: PassOptions,
}

impl PassPostProcess {
    fn new(options: PassOptions) -> PassPostProcess {
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

    fn execute(
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
        bind.sampler("texture", &self.viewport.colour_attachment(0).colour).unwrap();

        gpu_buffer::State::degenerate().bind().draw(&bind);
    }
}

struct RenderPass {
    options: PassOptions,
    pass_raytrace: PassRaytrace,
    pass_lighting: PassLighting,
    pass_lighting_combine: PassLightingCombine,
    pass_ao: PassLightingAo,
    pass_ao_combine: PassLightingAoCombine,
    pass_post_process: PassPostProcess,
    rescaler: algorithms::Rescaler,
    lights: Vec<Light>
}

impl RenderPass {
    fn new() -> RenderPass {
        let mut lights = Vec::new();
        lights.push(
            Light::Directional {
                colour: vec3(1.0, 0.50, 0.55),
                direction: vec3(1.0, -0.4, 0.2).normalize(),
                intensity: 0.9
            }
        );

        lights.push(
            Light::Directional {
                colour: vec3(1.0, 1.0, 1.0),
                direction: vec3(0.3, -0.9, 0.2).normalize(),
                intensity: 0.1
            }
        );

        lights.push(
            Light::Point {
                colour: vec3(0.6, 0.6, 0.6),
                position: vec3(4.0, 9.0, 25.0),
                intensity: 160.0
            }
        );

        lights.push(
            Light::Point {
                colour: vec3(0.6, 0.6, 0.6),
                position: vec3(4.0, 2.5, -2.0),
                intensity: 15.0
            }
        );

        lights.push(
            Light::Point {
                colour: vec3(0.6, 0.1, 0.3),
                position: vec3(3.0, 3.0, 1.0),
                intensity: 3.0
            }
        );

        lights.push(
            Light::Spotlight {
                colour: vec3(0.6, 0.6, 1.0),
                position: vec3(30.5, 8.0, -5.0),
                direction: vec3(-0.8, -0.3, 1.0).normalize(),
                angle: 5.0_f32.to_radians(),
                intensity: 170.0
            }
        );

        let final_size = vec2(1920, 1080);
        let options = PassOptions {
            final_size,
            raytrace_size: final_size,
            lighting_halves: 2,
            ao_halves: 2
        };

        let pass_raytrace = PassRaytrace::new(options);
        let pass_lighting = PassLighting::new(options);
        let pass_lighting_combine = PassLightingCombine::new(options);
        let pass_ao = PassLightingAo::new(options, 32);
        let pass_ao_combine = PassLightingAoCombine::new(options);
        let pass_post_process = PassPostProcess::new(options);
        RenderPass {
            options,
            pass_raytrace,
            pass_lighting,
            pass_lighting_combine,
            pass_ao,
            pass_ao_combine,
            pass_post_process,
            rescaler: algorithms::Rescaler::new(),
            lights
        }
    }

    fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &self,
        assets: &avalon::asset_library::Library,
        camera: &Camera,
        grid: &voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>
    ) where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
        let _annotation = GpuAnnotation::push("Game Render Pass");
        let albedo_raw = assets.bundle("voxel-textures").unwrap().tag("albedo").unwrap();
        let normal_raw = assets.bundle("voxel-textures").unwrap().tag("normal").unwrap();
        self.pass_raytrace.execute(
            camera,
            grid,
            *albedo_raw,
            *normal_raw,
        );

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

        let lighting = self.pass_lighting.lighting_buffer;
        let lighting_upscaled = self.rescaler.upscale_doubling(&lighting, self.options.lighting_halves).as_managed();
        self.pass_lighting_combine.execute(
            &albedo,
            &lighting_upscaled
        );

        let lighted_scene = self.pass_lighting_combine.viewport.colour_attachment(0).colour;
        self.pass_ao.execute(
            &lighted_scene,
            &grid,
            &positions,
            &normals,
            &tangents,
            1.0 / 60.0
        );

        let ao_scene = self.pass_ao.viewport.colour_attachment(0).colour;
        let ao_scene_upscaled = self.rescaler.upscale_doubling(&ao_scene, self.options.ao_halves);
        self.pass_ao_combine.execute(
            &ao_scene_upscaled,
            &lighting_upscaled,
            &albedo,
        );

        let finished_scene = self.pass_ao_combine.viewport.colour_attachment(0).colour;
        let finished_scene_upscaled = self.rescaler.upscale(
            &finished_scene,
            self.options.final_size
        );
        self.pass_post_process.execute(
            &finished_scene_upscaled
        );
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(align(16))]
struct DebugLight {
    colour: (f32, f32, f32, f32),
    position: (f32, f32, f32),
}

unsafe impl avalon::Pod for DebugLight {}

struct DebugPassLights {
    shader: Program,
    light_buffer: gpu_buffer::storage::Storage,
}

impl DebugPassLights {
    fn new() -> DebugPassLights {
        DebugPassLights {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/dev/light.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/dev/light.frag").unwrap())
                .build()
                .unwrap(),
            light_buffer: gpu_buffer::storage::Storage::new()
        }
    }

    fn execute(
        &mut self,
        icon_bundle: avalon::asset_library::BundleView,
        camera: &Camera,
        lights: &Vec<Light>
    ) {
        let _annotation = GpuAnnotation::push("Light Icons");
        let icon_pointlight = icon_bundle.tag::<GpuTexture2d>("pointlight").unwrap();
        let spotlight_on = icon_bundle.tag::<GpuTexture2d>("spotlight-off").unwrap();
        let spotlight_off = icon_bundle.tag::<GpuTexture2d>("spotlight-off").unwrap();

        let point_lights = lights.iter().filter(|light| light.is_point());
        let spot_lights = lights.iter().filter(|light| light.is_spotlight());

        let mut light_shader = self.shader.activate();
        light_shader.uniform("view").unwrap().set_mat4(camera.transform.matrix());
        //light_shader.uniform("projection").unwrap().set_mat4(camera.perspective);
        //light_shader.uniform("projectionTick").unwrap().set_mat3(camera.projection);

        let mut debug_lights = Vec::new();
        light_shader.sampler("icon", &*icon_pointlight).unwrap();
        for light in point_lights {
            let Light::Point { colour, position, .. } = light else { panic!() };
            debug_lights.push(DebugLight {
                colour: (colour.x, colour.y, colour.z, 1.0),
                position: (position.x, position.y, position.z),
            });
        }

        let usage = gpu_buffer::storage::Usage::Dynamic(gpu_buffer::storage::Access::CpuWrite);
        self.light_buffer.bind_mut().write_structs(
            &debug_lights,
            usage
        );
        let storage_bind = self.light_buffer.bind();
        light_shader.storage(0, &storage_bind, usage);
        light_shader.barrier();
        gpu_buffer::State::degenerate().bind().draw_instanced(&light_shader, debug_lights.len());

        debug_lights.clear();
        light_shader.sampler("icon", &*spotlight_on).unwrap();
        for light in spot_lights {
            let Light::Spotlight { colour, position, .. } = light else { panic!() };
            debug_lights.push(DebugLight {
                position: (position.x, position.y, position.z),
                colour: (colour.x, colour.y, colour.z, 1.0),
            });
        }
        let storage_bind = self.light_buffer.bind();
        light_shader.storage(0, &storage_bind, usage);
        light_shader.barrier();
        gpu_buffer::State::degenerate().bind().draw_instanced(&light_shader, debug_lights.len());
    }
}

struct DebugRenderPass {
    debug_lights: DebugPassLights
}

impl DebugRenderPass {
    fn new() -> DebugRenderPass {
        let debug_lights = DebugPassLights::new();
        DebugRenderPass {
            debug_lights
        }
    }
    fn execute(
        &mut self,
        asset_library: &avalon::asset_library::Library,
        camera: &Camera,
        lights: &Vec<Light>
    ) {
        let _annotation = GpuAnnotation::push("Debug Render Pass");
        let dev_icons = asset_library.bundle("dev-icons").unwrap();
        self.debug_lights.execute(dev_icons, camera, lights);
    }
}

fn main() {
    let mut engine = avalon::engine();

    let asset_library = avalon::asset_library::Library::new_with_scan("./assets/bins/");

    let mut camera = Camera::new(vec2(1920, 1080));
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

    for x in 1..=5 {
        for y in 1..=5 {
            set_cell(vec3(x, y, 3));
            set_cell(vec3(x, y, 8));
        }

        for z in 3..=8 {
            set_cell(vec3(x, 6, z));
        }
    }
    grid.bake();

    let render_pass = RenderPass::new();
    let mut debug_render_pass = DebugRenderPass::new();

    let start = std::time::Instant::now();
    while engine.is_open() {
        engine.start_frame();
        engine.poll_events();
        let dt = start.elapsed().as_secs_f32();
        camera.transform.set_position(
            vec3(0.0, 5.0, -5.0) + vec3(5.0 * dt.cos(), 0.0, 0.0 * dt.cos() * dt.sin())
        );
        engine.render();
        render_pass.execute(&asset_library, &camera, &grid);
        debug_render_pass.execute(&asset_library, &camera, &render_pass.lights);
        engine.swap();
        engine.end_frame();
    }
}
