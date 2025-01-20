use miniserde::{ Serialize, Deserialize };

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ColourSpace {
    RGB,
    SRGB
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Texture {
    pub colour_space: ColourSpace,
}

impl std::fmt::Display for ColourSpace {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColourSpace::RGB => "RGB".fmt(formatter),
            ColourSpace::SRGB => "sRGB".fmt(formatter),
        }
    }
}
