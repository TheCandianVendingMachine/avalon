use avalon;
use nalgebra_glm::vec2;

fn main() {
    let mut engine = avalon::engine();

    while engine.is_open() {
        engine.start_frame();
        engine.poll_events();
        engine.render();
        engine.end_frame();
    }
}
