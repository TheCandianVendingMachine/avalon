#[derive(Debug, Copy, Clone)]
pub enum ColourSpace {
    RGB,
    SRGB
}

#[derive(Debug, Copy, Clone)]
pub enum Components {
    Red,
    RG,
    RGB,
    RGBA
}

#[derive(Debug, Copy, Clone)]
pub struct Texture {
    colour_space: ColourSpace,
    components: Components,
    dimensions: (u32, u32)
}
