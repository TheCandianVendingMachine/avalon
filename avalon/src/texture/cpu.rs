use nalgebra_glm::{ IVec3, IVec2 };
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

#[derive(Clone)]
pub struct Texture3d {
    data: Data,
    dimensions: IVec3,
}

impl Texture3d {
    pub(super) fn generate(dimensions: IVec3, data: Data) -> Texture3d {
        Texture3d {
            data,
            dimensions
        }
    }
}
