use avalon;
use nalgebra_glm::vec2;

fn main() {
    let mut engine = avalon::engine();

    let framebuffer = avalon::viewport::Viewport::new(vec2(1920, 1080))
        .colour_attachment().format(avalon::texture::gpu::SizedComponent::RGBA8)
        .colour_attachment().format(avalon::texture::gpu::SizedComponent::R8)
        .colour_attachment().format(avalon::texture::gpu::SizedComponent::FloatRGBA32)
        .depth_stencil(avalon::viewport::DepthStencil::Depth)
        .build();

    while engine.is_open() {
        engine.start_frame();
        engine.poll_events();
        engine.render();
        engine.end_frame();
    }
}
