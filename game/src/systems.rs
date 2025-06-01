use crate::components::{ Camera, Particle, Transform };
use avalon::ecs::component::{ Query, Group };
use avalon::ecs::system::System;
use nalgebra_glm::{ vec3, vec2 };

pub struct ParticleSystem {

}

impl System for ParticleSystem {
    fn query() -> Query {
        Query::new()
            .select::<Particle>()
            .select::<Transform>()
    }
}

impl ParticleSystem {
    pub fn new() -> ParticleSystem {
        ParticleSystem {

        }
    }

    pub fn tick(&mut self, dt: f32, entities: &mut [Group]) {
        for entity in entities.iter_mut() {
            let mut position = entity.get::<Transform>().transform.position();

            let particle = entity.get::<Particle>();
            position += particle.velocity * dt;

            entity.get_mut::<Transform>().transform.set_position(position);
        }
    }
}

pub struct CameraSystem {
    pub camera: crate::render::Camera
}

impl System for CameraSystem {
    fn query() -> Query {
        Query::new()
            .select::<Camera>()
            .select::<Transform>()
    }
}

impl CameraSystem {
    pub fn new() -> CameraSystem {
        CameraSystem {
            camera: crate::render::Camera::new(vec2(1920, 1080))
        }
    }

    pub fn tick(&mut self, entities: &[Group]) {
        for entity in entities.iter() {
            self.camera.transform = entity.get::<Transform>().transform;
        }
    }
}
