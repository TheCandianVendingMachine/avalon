use avalon;

fn main() {
    let mut engine = avalon::engine();

    while engine.is_open() {
        engine.start_frame();
        engine.poll_events();
        engine.render();
        engine.end_frame();
    }
}
