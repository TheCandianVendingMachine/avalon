#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused)]

use nalgebra_glm::{ Mat4, Mat3, Vec3, TVec3, Vec2, IVec2, vec2, vec3 };

pub mod voxel;
pub mod render;

use avalon;
use avalon::input;
use avalon::debug::GpuAnnotation;
use avalon::gpu_buffer;
use avalon::viewport;
use avalon::model;
use avalon::shader::{ self, Source, Program, };
use avalon::texture::algorithms;
use avalon::texture::data;
use avalon::texture::{ Component, GpuTexture3d, GpuTexture2d };
use avalon::texture::gpu::{ self, Arguments2d, UniqueTexture, Access, Sampler, Image };

fn main() {
    let mut engine = avalon::engine();

    let asset_library = avalon::asset_library::Library::new_with_scan("./assets/bins/");

    let mut camera = render::Camera::new(vec2(1920, 1080));
    camera.transform.set_position(vec3(0.0, 5.0, -5.0));
    camera.transform.set_euler_angles(avalon::transform::Euler {
        pitch: -5.0_f32.to_radians(),
        yaw: -45.0_f32.to_radians(),
        roll: 0.0_f32.to_radians()
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
        .map("look")
            .mouse(input::action::Mouse::Move)
            .finish()
        .build();
    let mut inputs = input::Engine::new(&mut engine, action_map);
    inputs.push_layer("test_layer");
    let mut context = inputs.active_layer_mut().unwrap().context_handler()
        .name("flycamera")
        .action("move_forward")
        .action("move_backward")
        .action("strafe_left")
        .action("strafe_right")
        .action("look")
        .build();

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

        let mut move_direction = vec3(0.0, 0.0, 0.0);
        while let Some(action) = context.pop() {
            match action.id.name.as_str() {
                "move_forward" => move_direction += camera.transform.forward(),
                "move_backward" => move_direction -= camera.transform.forward(),
                "strafe_left" => move_direction += camera.transform.left(),
                "strafe_right" => move_direction -= camera.transform.left(),
                "look" => {},
                _ => {},
            }
        }

        while accumulator > update_rate {
            if move_direction.magnitude_squared() > 0.0 {
                move_direction = move_direction.normalize();
                camera.transform.translate(move_direction * 10.0 * update_rate.as_secs_f32());
            }
            accumulator -= update_rate;
        }
        /*camera.transform.set_euler_angles(avalon::transform::Euler {
            pitch: dt.sin() * 0.2,
            yaw: dt.cos(),
            roll: 0.0_f32.to_radians()
        });*/
        engine.render();
        render_pass.execute(&asset_library, &camera, &grid);
        debug_render_pass.execute(&asset_library, &camera, &render_pass.lights);
        engine.swap();
        engine.end_frame();
    }
}
