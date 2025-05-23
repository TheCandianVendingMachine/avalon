#![feature(trait_alias)]
#![feature(slice_split_once)]
extern crate gl;

mod engine;
mod render_engine;
pub mod asset_library;
pub mod debug;
pub mod ecs;
pub mod event;
pub mod gpu_buffer;
pub mod shader;
pub mod input;
pub mod model;
pub mod texture;
pub mod transform;
pub mod viewport;

use engine::Engine;

pub use bytemuck::NoUninit as Pod;

pub fn engine() -> Engine {
    Engine::new()
}
