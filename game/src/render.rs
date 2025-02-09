pub mod pass_raytrace;
pub mod pass_geometry;
pub mod pass_lighting;
pub mod pass_lighting_combine;
pub mod pass_lighting_ao;
pub mod pass_lighting_ao_combine;
pub mod pass_skybox;
pub mod pass_post_process;
pub mod debug_pass_lights;

use nalgebra_glm::{ Mat4, Vec3, Vec2, IVec2, vec3, vec2 };

use avalon;
use avalon::debug::GpuAnnotation;
use avalon::gpu_buffer;
use avalon::viewport;
use avalon::model;
use avalon::shader::{ self, Source, Program, };
use avalon::texture::algorithms;
use avalon::texture::data;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };

#[derive(Debug, Copy, Clone)]
pub enum Light {
    Directional { colour: Vec3, direction: Vec3, intensity: f32 },
    Point { colour: Vec3, position: Vec3, intensity: f32 },
    Spotlight { colour: Vec3, position: Vec3, direction: Vec3, angle: f32, intensity: f32 },
}

impl Light {
    pub fn is_directional(&self) -> bool {
        if let Light::Directional {..} = self {
            true
        } else {
            false
        }
    }

    pub fn is_point(&self) -> bool {
        if let Light::Point {..} = self {
            true
        } else {
            false
        }
    }

    pub fn is_spotlight(&self) -> bool {
        if let Light::Spotlight {..} = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct DebugLight {
    colour: (f32, f32, f32, f32),
    position: (f32, f32, f32),
}

unsafe impl avalon::Pod for DebugLight {}

pub struct Camera {
    pub transform: avalon::transform::Transform,
    focal: f32,
    near: f32,
    far: f32,
    projection: Mat4,
    dimensions: IVec2,
}

impl Camera {
    pub fn new(dimensions: IVec2) -> Camera {
        let focal = 1.0;
        let int_dimensions = dimensions;
        let dimensions: Vec2 = dimensions.cast();
        let aspect = dimensions.y / dimensions.x;
        let near = 0.01;
        let far = std::f32::INFINITY;

        let projection = if far == std::f32::INFINITY {
            Mat4::new(
                focal,  0.0,            0.0,                        0.0,
                0.0,   -focal / aspect, 0.0,                        0.0,
                0.0,    0.0,           -1.0,                       -2.0 * near,
                0.0,    0.0,           -1.0,                        0.0
            )
        } else {
            Mat4::new(
                focal,  0.0,            0.0,                        0.0,
                0.0,   -focal / aspect, 0.0,                        0.0,
                0.0,    0.0,          -(far + near) / (far - near),-2.0 * far * near / (far - near),
                0.0,    0.0,           -1.0,                        0.0
            )
        };

        Camera {
            transform: avalon::transform::Transform::new(),
            dimensions: int_dimensions,
            projection,
            focal,
            near,
            far
        }
    }
}

struct GeometryBuffers {
    position: GpuTexture2d,
    normal: GpuTexture2d,
    tangent: GpuTexture2d,
}

#[derive(Debug, Copy, Clone)]
pub struct PassOptions {
    final_size: IVec2,
    raytrace_size: IVec2,
    lighting_halves: u32,
    ao_halves: u32,
}

impl PassOptions {
    pub fn lighting_resolution(&self) -> IVec2 {
        self.raytrace_size / 2_i32.pow(self.lighting_halves)
    }

    pub fn ao_resolution(&self) -> IVec2 {
        self.raytrace_size / 2_i32.pow(self.ao_halves)
    }
}

pub struct RenderPass {
    options: PassOptions,
    pass_raytrace: pass_raytrace::PassRaytrace,
    pass_geometry: pass_geometry::PassGeometry,
    pass_lighting: pass_lighting::PassLighting,
    pass_lighting_combine: pass_lighting_combine::PassLightingCombine,
    pass_ao: pass_lighting_ao::PassLightingAo,
    pass_ao_combine: pass_lighting_ao_combine::PassLightingAoCombine,
    pass_skybox: pass_skybox::PassSkybox,
    pass_post_process: pass_post_process::PassPostProcess,
    rescaler: algorithms::Rescaler,
    pub lights: Vec<Light>
}

impl RenderPass {
    pub fn new() -> RenderPass {
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
                position: vec3(4.0, 7.0, 25.0),
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
                position: vec3(3.0, 1.5, 1.0),
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
            lighting_halves: 0,
            ao_halves: 2
        };

        let pass_raytrace = pass_raytrace::PassRaytrace::new(options);
        let pass_geometry = pass_geometry::PassGeometry::new(options);
        let pass_lighting = pass_lighting::PassLighting::new(options);
        let pass_lighting_combine = pass_lighting_combine::PassLightingCombine::new(options);
        let pass_ao = pass_lighting_ao::PassLightingAo::new(options, 32);
        let pass_ao_combine = pass_lighting_ao_combine::PassLightingAoCombine::new(options);
        let pass_skybox = pass_skybox::PassSkybox::new(options);
        let pass_post_process = pass_post_process::PassPostProcess::new(options);
        RenderPass {
            options,
            pass_raytrace,
            pass_geometry,
            pass_lighting,
            pass_lighting_combine,
            pass_ao,
            pass_ao_combine,
            pass_skybox,
            pass_post_process,
            rescaler: algorithms::Rescaler::new(),
            lights
        }
    }

    pub fn execute<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>(
        &mut self,
        assets: &avalon::asset_library::Library,
        camera: &Camera,
        grid: &crate::voxel::Grid<SIDE_LENGTH, VOXELS_PER_METER>
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

        /*self.pass_geometry.execute(
            assets,
            &mut self.pass_raytrace.viewport,
            camera,
        );*/

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

        let lighted_scene = self.pass_lighting_combine.viewport.colour_attachment(0).unwrap().colour;
        self.pass_ao.execute(
            &lighted_scene,
            &grid,
            &positions,
            &normals,
            &tangents,
            1.0 / 60.0
        );

        let ao_scene = self.pass_ao.viewport.colour_attachment(0).unwrap().colour;
        let ao_scene_upscaled = self.rescaler.upscale_doubling(&ao_scene, self.options.ao_halves);
        self.pass_ao_combine.execute(
            &ao_scene_upscaled,
            &lighting_upscaled,
            &albedo,
        );

        let finished_scene = self.pass_ao_combine.viewport.colour_attachment(0).unwrap().colour;
        let finished_scene_upscaled = self.rescaler.upscale(
            &finished_scene,
            self.options.final_size
        );

        self.pass_skybox.execute(
            assets,
            camera
        );

        let scene_bloom = self.pass_skybox.viewport.tagged_colour("bloom").unwrap().colour;
        let bloom_downscaled = self.rescaler.downsample(&scene_bloom, scene_bloom.dimensions() / 4);
        let bloom_upscaled = self.rescaler.upscale(&bloom_downscaled, scene_bloom.dimensions());

        self.pass_post_process.execute(
            &bloom_upscaled,
            &self.pass_skybox.viewport.tagged_colour("albedo").unwrap().colour,
            &finished_scene_upscaled
        );
        self.pass_raytrace.viewport.blit_attachment(
            viewport::Attachment::DepthStencil,
            viewport::BlitTarget::Screen(viewport::Attachment::DepthStencil)
        );
    }
}

pub struct DebugRenderPass {
    debug_lights: debug_pass_lights::DebugPassLights
}

impl DebugRenderPass {
    pub fn new() -> DebugRenderPass {
        let debug_lights = debug_pass_lights::DebugPassLights::new();
        DebugRenderPass {
            debug_lights
        }
    }
    pub fn execute(
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
