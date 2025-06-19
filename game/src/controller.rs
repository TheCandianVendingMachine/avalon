use crate::components::{ PlayerState, PlayerController, Particle, Transform, Camera };
use crate::voxel::Grid;
use avalon::ecs::component::{ Query, Group };
use avalon::ecs::system::System;
use avalon::input::layer::Layer;
use avalon::input::action;
use avalon::event::Channel;
use nalgebra_glm::{ vec3, vec2 };

pub struct PlayerControllerSystem {
    input: Channel<action::Action, &'static str>
}


impl System for PlayerControllerSystem {
    fn query() -> Query {
        Query::new()
            .select::<PlayerController>()
            .select::<Transform>()
            .select::<Particle>()
    }
}

impl PlayerControllerSystem {
    pub fn new(layer: &mut Layer) -> PlayerControllerSystem {
        PlayerControllerSystem {
            input: layer.context_handler()
                .name("flycamera")
                .action("move_forward")
                .action("move_backward")
                .action("strafe_left")
                .action("strafe_right")
                .action("look")
                .build()
        }
    }

    pub fn tick(&mut self, grid: &Grid, dt: f32, entities: &mut [Group]) {
        for entity in entities.iter_mut() {
            let controller = *entity.get::<PlayerController>();
            let transform = entity.get_mut::<Transform>();

            //dbg!(&controller, &transform);

            let mut move_direction = vec3(0.0, 0.0, 0.0);
            let mut camera_euler = transform.transform.euler_angles();
            while let Some(action) = self.input.pop() {
                match action.id.name.as_str() {
                    "move_forward" => move_direction += transform.transform.forward(),
                    "move_backward" => move_direction -= transform.transform.forward(),
                    "strafe_left" => move_direction += transform.transform.left(),
                    "strafe_right" => move_direction -= transform.transform.left(),
                    "look" => {
                        let direction = vec2(
                            action.data.retrieve::<f32>("axis_x").unwrap(),
                            action.data.retrieve::<f32>("axis_y").unwrap()
                        );
                        camera_euler.pitch += direction.y * 0.05;
                        camera_euler.yaw += -direction.x * 0.05;
                    },
                    _ => {},
                }
            }
            camera_euler.pitch = camera_euler.pitch.clamp(-80.0_f32.to_radians(), 80.0_f32.to_radians());
            transform.transform.set_euler_angles(camera_euler);

            let particle = entity.get_mut::<Particle>();
            if move_direction.magnitude_squared() == 0.0 {
                particle.velocity = vec3(0.0, 0.0, 0.0);
            } else {
                particle.velocity = move_direction.normalize() * controller.max_speed;
            }
        }
    }
}
