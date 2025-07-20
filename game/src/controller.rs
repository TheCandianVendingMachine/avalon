use crate::components::{ PlayerState, PlayerController, Particle, Transform, Camera, MoveState };
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
                .action("jump")
                .action("look")
                .build()
        }
    }

    pub fn tick(&mut self, grid: &Grid, dt: f32, entities: &mut [Group]) {
        for entity in entities.iter_mut() {
            let controller = *entity.get::<PlayerController>();
            let transform = entity.get_mut::<Transform>();

            let mut move_direction = vec3(0.0, 0.0, 0.0);
            let mut jump = 0.0;
            let mut camera_euler = transform.transform.euler_angles();
            let position = transform.transform.position();

            let forward_2d = vec3(
                transform.transform.forward().x,
                0.0,
                transform.transform.forward().z
            ).normalize();
            let left_2d = vec3(
                transform.transform.left().x,
                0.0,
                transform.transform.left().z
            ).normalize();

            while let Some(action) = self.input.pop() {
                match action.id.name.as_str() {
                    "move_forward" => move_direction += forward_2d,
                    "move_backward" => move_direction -= forward_2d,
                    "strafe_left" => move_direction += left_2d,
                    "strafe_right" => move_direction -= left_2d,
                    "jump" => jump += 1.0,
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

            let mut next_state = controller.state.state;
            match controller.state.state {
                MoveState::Idle => {
                    if jump > 0.0 {
                        next_state = MoveState::JumpStart;
                    } else if move_direction.magnitude_squared() > 0.0 {
                        next_state = MoveState::Walk;
                    }
                },
                MoveState::Walk => {
                    if jump > 0.0 {
                        next_state = MoveState::JumpStart;
                    } else if move_direction.magnitude_squared() == 0.0 {
                        next_state = MoveState::Idle;
                    }
                },
                MoveState::Sprint => {},
                MoveState::Slide => {},
                MoveState::JumpStart => {
                    if jump == 0.0 {
                        next_state = MoveState::JumpFree;
                    }
                },
                MoveState::JumpFree => {
                    next_state = MoveState::Fall;
                },
                MoveState::Fall => {
                },
            }

            let foot_collision = grid.ray_until_distance(
            transform.transform.position(),
            vec3(0.0, -1.0, 0.0),
                5.0 * controller.height
            );
            if foot_collision.distance.is_finite() {
                if foot_collision.distance < controller.height {
                    transform.transform.set_position(
                        foot_collision.end_position + vec3(0.0, controller.height, 0.0)
                    );
                } else if foot_collision.distance > controller.height {
                    next_state = MoveState::Fall;
                }
            }

            {
                let controller = entity.get_mut::<PlayerController>();
                if controller.state.state != next_state {
                    controller.state.enter_time = std::time::Instant::now();
                }
                controller.state.state = next_state;
            }

            if move_direction.magnitude_squared() == 0.0 {
                let particle = entity.get_mut::<Particle>();
                particle.velocity = vec3(0.0, 0.0, 0.0);
                continue;
            }
            let mut move_direction = move_direction.normalize();

            let cell = grid.cell_at_position(position);
            if let Some(cell) = cell {
                if !cell.is_empty() && cell.cell_id() == voxel::CellType::SpaceTimeFus.into() {
                    let move_direction_2d = vec3(
                        move_direction.x,
                        0.0,
                        move_direction.z
                    ).normalize();
                    let raycast = grid.ray_while_condition(
                        position,
                        move_direction_2d,
                        algorithms::Condition::cell(|cell| {
                            !cell.is_empty() && cell.cell_id() == voxel::CellType::SpaceTimeFus.into()
                        })
                    );
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
