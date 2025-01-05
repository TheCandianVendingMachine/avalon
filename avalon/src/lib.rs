extern crate gl;

mod engine;
pub mod event;
pub mod shader;
pub mod input;

use engine::Engine;

pub fn engine() -> Engine {
    Engine::new()
}
