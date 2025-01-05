use avalon;
use avalon::shader::{ Program, Source, Vertex, Fragment, Compute };

fn main() {
    let mut engine = avalon::engine();
    let test_program = Program::new()
        .compute(Compute::load_from_path("assets/shaders/voxel/lighting.comp").unwrap())
        .build().unwrap();

    dbg!(test_program.info_log());
    test_program.attach().uniform("halveCount").unwrap().set_i32(0);
    test_program.attach().location(1).unwrap().set_i32(0);

    while engine.is_open() {
        engine.poll_events();
        engine.render();
    }
}
