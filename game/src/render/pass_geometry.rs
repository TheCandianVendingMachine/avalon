use avalon::shader::{ self, Source, Program, };
use avalon::viewport;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::gpu_buffer;
use avalon::debug::GpuAnnotation;
use avalon::model;
use crate::voxel;
use crate::render::{ Camera, PassOptions, Light };

use nalgebra_glm::vec3;

pub struct PassGeometry {
    shader: Program,
    options: PassOptions
}

impl PassGeometry {
    pub fn new(options: PassOptions) -> PassGeometry {
        PassGeometry {
            shader: Program::new()
                .vertex(shader::Vertex::load_from_path("assets/shaders/geo/star.vert").unwrap())
                .fragment(shader::Fragment::load_from_path("assets/shaders/geo/star.frag").unwrap())
                .build()
                .unwrap(),
            options
        }
    }

    pub fn execute(
        &self,
        assets: &avalon::asset_library::Library,
        world_viewport: &mut viewport::Viewport,
        camera: &Camera,
    ) {
        let _raytrace_annotation = GpuAnnotation::push("Draw Geometry");
        let model = assets.bundle("ambient-visuals").unwrap().tag::<model::Model>("star-model").unwrap();
        let mut transform = avalon::transform::Transform::new();
        transform.set_position(vec3(16.0, 2.7, 12.0));
        transform.set_euler_angles(avalon::transform::Euler {
            pitch: 0.0_f32.to_radians(),
            yaw: 0.0_f32.to_radians(),
            roll: 0.0_f32.to_radians()
        });

        {
            let bind = world_viewport.bind_mut();
            bind.depth_test()
                .function(viewport::depth_options::Function::Less)
                .finish();
        }

        let shader = self.shader.activate();
        shader.uniform("model").unwrap().set_mat4(transform.matrix());
        shader.uniform("view").unwrap().set_mat4(camera.transform.matrix());
        shader.uniform("projection").unwrap().set_mat4(camera.projection);

        let bind = world_viewport.bind();
        //model.bind().draw(&shader);
    }
}


