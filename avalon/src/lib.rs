extern crate gl;

mod engine;
mod render_engine;
pub mod asset_library;
pub mod event;
pub mod shader;
pub mod input;
pub mod texture;
pub mod transform;
pub mod viewport;

use engine::Engine;

pub fn engine() -> Engine {
    Engine::new()
}
