use nalgebra_glm::IVec2;
use crate::texture::data::Data;

#[derive(Clone)]
pub struct Texture2d {
    data: Data,
    dimensions: IVec2,
}

impl Texture2d {
    pub(super) fn generate(dimensions: IVec2, data: Data) -> Texture2d {
        Texture2d {
            data,
            dimensions
        }
    }
}
