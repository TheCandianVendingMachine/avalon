use avalon;
use avalon::shader::Source;

fn main() {
    let mut engine = avalon::engine();

    let shader = avalon::shader::Program::new()
        .vertex(avalon::shader::Vertex::load_from_path("assets/shaders/voxel/world.vert").unwrap())
        .fragment(avalon::shader::Fragment::load_from_path("assets/shaders/voxel/world.frag").unwrap())
        .build()
    .unwrap();

    let texture = avalon::texture::Texture2d::new(nalgebra_glm::vec2(1920, 1080))
        .gpu()
            .vram_data(avalon::texture::gpu::SizedComponent::RGBA8)
            .finish()
        .components(avalon::texture::Component::RGBA)
        .build();

    while engine.is_open() {
        engine.start_frame();
        engine.poll_events();
        engine.render();
        engine.end_frame();
    }
}
