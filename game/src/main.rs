#![feature(generic_const_exprs)]
#![feature(default_field_values)]
#![allow(incomplete_features, unused)]

use nalgebra_glm::{ Mat4, Mat3, Vec3, TVec3, Vec2, IVec2, vec2, vec3 };

pub mod voxel;
pub mod render;
pub mod stores;
pub mod components;
pub mod controller;
pub mod systems;

use avalon::input;
use avalon::debug::GpuAnnotation;
use avalon::gpu_buffer;
use avalon::viewport;
use avalon::model;
use avalon::shader::{ self, Source, Program, };
use avalon::texture::algorithms;
use avalon::texture::data;
use avalon::texture::{ GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };
use avalon::ecs::Poolable;
use avalon::ecs::component::{ self, Query, Component, Store as ComponentStore, Mutability };
use avalon::ecs::system::System;

use crate::stores::Store;

struct Stores {
    transform_store: Store<components::Transform>,
    collider_store: Store<components::Collider>,
    particle_store: Store<components::Particle>,
    player_controller_store: Store<components::PlayerController>,
    camera_store: Store<components::Camera>,
}

impl Stores {
    fn new() -> Self {
        Stores {
            transform_store: Store::new(),
            collider_store: Store::new(),
            particle_store: Store::new(),
            player_controller_store: Store::new(),
            camera_store: Store::new(),
        }
    }

    fn tick_system<F>(&mut self, entities: &component::Bag, dt: f32, query: component::Query, system_tick: F) where
        F: FnOnce(f32, &mut [component::Group])
    {
        let mut relevant_entities = entities.entities_with_components(query);
        relevant_entities.sort_unstable();

        let transforms = self.transform_store.components_matching_entities(&relevant_entities);
        let colliders = self.collider_store.components_matching_entities(&relevant_entities);
        let particles = self.particle_store.components_matching_entities(&relevant_entities);
        let player_controllers = self.player_controller_store.components_matching_entities(&relevant_entities);
        let cameras = self.camera_store.components_matching_entities(&relevant_entities);

        let mut groups = Vec::new();
        for (idx, entity) in relevant_entities.into_iter().enumerate() {
            let mut component_group = component::Group::<64>::new(entity);
            if !transforms.is_empty() {
                assert_eq!(entity, transforms[idx].0);
                component_group.assign(transforms[idx].1, Mutability::Constant);
            }
            if !colliders.is_empty() {
                assert_eq!(entity, colliders[idx].0);
                component_group.assign(colliders[idx].1, Mutability::Constant);
            }
            if !particles.is_empty() {
                assert_eq!(entity, particles[idx].0);
                component_group.assign(particles[idx].1, Mutability::Constant);
            }
            if !player_controllers.is_empty() {
                assert_eq!(entity, player_controllers[idx].0);
                component_group.assign(player_controllers[idx].1, Mutability::Constant);
            }
            if !cameras.is_empty() {
                assert_eq!(entity, cameras[idx].0);
                component_group.assign(cameras[idx].1, Mutability::Constant);
            }
            groups.push(component_group)
        }
        system_tick(dt, &mut groups);

        let mut transforms = Vec::new();
        let mut colliders = Vec::new();
        let mut particles = Vec::new();
        let mut player_controllers = Vec::new();
        let mut cameras = Vec::new();
        for group in groups.into_iter() {
            transforms.push(*group.get());
            //colliders.push(*group.get());
            particles.push(*group.get());
            player_controllers.push(*group.get());
            cameras.push(*group.get());
        }

        self.transform_store.update_components(&transforms);
        self.collider_store.update_components(&colliders);
        self.particle_store.update_components(&particles);
        self.player_controller_store.update_components(&player_controllers);
        self.camera_store.update_components(&cameras);
    }
}

fn main() {
    let mut engine = avalon::engine();

    let asset_library = avalon::asset_library::Library::new_with_scan("./assets/bins/");

    let mut grid: voxel::Grid<32, 1> = voxel::Grid::new();
    for x in 0..20 {
        for z in 0..32 {
            *grid.cell_mut(vec3(x, 0, z)) = voxel::CellType::Floor.into();
        }
    }

    for x in 13..20 {
        for y in 1..8 {
            *grid.cell_mut(vec3(x, y, 15)) = voxel::CellType::Floor.into();
        }
    }

    for x in 5..10 {
        for y in 1..10 {
            *grid.cell_mut(vec3(x, y, 15)) = voxel::CellType::Floor.into();
        }
    }

    for x in 1..=5 {
        for y in 1..=5 {
            *grid.cell_mut(vec3(x, y, 3)) = voxel::CellType::Floor.into();
            *grid.cell_mut(vec3(x, y, 8)) = voxel::CellType::Floor.into();
        }

        for z in 3..=8 {
            *grid.cell_mut(vec3(x, 6, z)) = voxel::CellType::Floor.into();
        }
    }

    for x in 15..=18 {
        for z in 20..30 {
            *grid.cell_mut(vec3(x, 1, z)) = voxel::CellType::SpaceTimeFus.into();
            *grid.cell_mut(vec3(x, 1, z)) = voxel::CellType::SpaceTimeFus.into();
            *grid.cell_mut(vec3(x, 2, z)) = voxel::CellType::SpaceTimeFus.into();
            *grid.cell_mut(vec3(x, 2, z)) = voxel::CellType::SpaceTimeFus.into();
            *grid.cell_mut(vec3(x, 3, z)) = voxel::CellType::SpaceTimeFus.into();
            *grid.cell_mut(vec3(x, 3, z)) = voxel::CellType::SpaceTimeFus.into();
        }
    }
    grid.bake();

    let mut action_map = input::action::Map::new()
        .map("move_forward")
            .key(input::action::Keyboard::Hold(input::action::Key::Scancode(input::keyboard::Scancode::W)))
            .finish()
        .map("move_backward")
            .key(input::action::Keyboard::Hold(input::action::Key::Scancode(input::keyboard::Scancode::S)))
            .finish()
        .map("strafe_left")
            .key(input::action::Keyboard::Hold(input::action::Key::Scancode(input::keyboard::Scancode::A)))
            .finish()
        .map("strafe_right")
            .key(input::action::Keyboard::Hold(input::action::Key::Scancode(input::keyboard::Scancode::D)))
            .finish()
        .map("jump")
            .key(input::action::Keyboard::Hold(input::action::Key::Scancode(input::keyboard::Scancode::Space)))
            .finish()
        .map("look")
            .mouse(input::action::Mouse::Move)
            .finish()
        .build();
    let mut inputs = input::Engine::new(&mut engine, action_map);
    inputs.push_layer("test_layer");


    let mut particle_system = systems::ParticleSystem::new();
    let mut controller_system = controller::PlayerControllerSystem::new(inputs.active_layer_mut().unwrap());
    let mut camera_system = systems::CameraSystem::new();
    camera_system.camera.transform.set_position(vec3(0.0, 5.0, -5.0));
    camera_system.camera.transform.set_euler_angles(avalon::transform::Euler {
        pitch: -5.0_f32.to_radians(),
        yaw: 45.0_f32.to_radians(),
        roll: 0.0_f32.to_radians()
    });

    let mut entities = component::Bag::new();
    let mut stores = Stores::new();

    let player = entities.create(
        Query::new()
            .select::<components::Transform>()
            .select::<components::Particle>()
            .select::<components::PlayerController>()
            .select::<components::Camera>()
    );
    stores.transform_store.allocate_mut(player).transform.set_position(vec3(12.0, 2.0 + 1.7, 2.0));
    stores.particle_store.allocate(player);
    stores.player_controller_store.allocate(player);
    stores.camera_store.allocate(player);


    let mut render_pass = render::RenderPass::new();
    let mut debug_render_pass = render::DebugRenderPass::new();

    let mut accumulator = std::time::Duration::ZERO;
    let update_rate: std::time::Duration = std::time::Duration::from_secs_f64(1.0 / 60.0);

    let start = std::time::Instant::now();
    let mut frame_start = std::time::Instant::now();
    while engine.is_open() {
        engine.start_frame();
        engine.poll_events();
        inputs.poll();
        inputs.dispatch();

        accumulator += frame_start.elapsed();
        frame_start = std::time::Instant::now();

        while accumulator > update_rate {
            let dt = update_rate.as_secs_f32();
            let query = controller::PlayerControllerSystem::query();
            stores.tick_system(&entities, dt, query, |dt, entities| {
                controller_system.tick(&grid, dt, entities);
            });

            let query = systems::ParticleSystem::query();
            stores.tick_system(&entities, dt, query, |dt, entities| {
                particle_system.tick(dt, entities);
            });

            let query = systems::CameraSystem::query();
            stores.tick_system(&entities, dt, query, |dt, entities| {
                camera_system.tick(entities);
            });
            accumulator -= update_rate;
        }

        //camera_system.tick();
        engine.render();
        render_pass.execute(&asset_library, &camera_system.camera, &grid);
        debug_render_pass.execute(&asset_library, &camera_system.camera, &render_pass.lights);
        engine.swap();
        engine.end_frame();
    }
}
