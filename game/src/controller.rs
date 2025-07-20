use crate::components::{ PlayerState, PlayerController, Particle, Transform, Camera };
use crate::voxel::{ self, Grid, algorithms };
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

            if move_direction.magnitude_squared() == 0.0 {
                let particle = entity.get_mut::<Particle>();
                particle.velocity = vec3(0.0, 0.0, 0.0);
                continue;
            }
            let mut move_direction = move_direction.normalize();

            let cell = grid.cell_at_position(transform.transform.position());
            if let Some(cell) = cell {
                if !cell.is_empty() && cell.cell_id() == voxel::CellType::SpaceTimeFus.into() {
                    let forward = transform.transform.forward();
                    let direction_2d = vec3(
                        move_direction.x,
                        0.0,
                        move_direction.z
                    ).normalize();
                    let raycast = grid.ray_while_condition(
                        transform.transform.position(),
                        direction_2d,
                        |cell| {
                            !cell.is_empty() && cell.cell_id() == voxel::CellType::SpaceTimeFus.into()
                        }
                    );
                    dbg!(&raycast);
                    if raycast.distance.is_finite() {
                        let scale = 0.5_f32.powf((1.0 + raycast.distance).log2());
                        move_direction.z *= scale;
                    }
                }
            }

            let particle = entity.get_mut::<Particle>();
            particle.velocity = move_direction * controller.max_speed;
        }
    }
}
