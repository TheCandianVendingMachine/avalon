extern crate gl;

mod engine;
mod render_engine;
pub mod event;
pub mod shader;
pub mod input;
pub mod texture;

use engine::Engine;

pub fn engine() -> Engine {
    Engine::new()
}
