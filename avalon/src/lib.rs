extern crate gl;

mod engine;
pub mod event;
pub mod shader;

use engine::Engine;

pub fn engine() -> Engine {
    Engine::new()
}
