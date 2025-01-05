use avalon;
fn main() {
    let mut engine = avalon::engine();
    while engine.is_open() {
        engine.poll_events();
        engine.render();
    }
}
