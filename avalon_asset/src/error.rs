use thiserror::Error;
use miniserde;

#[derive(Debug, Copy, Clone, Error)]
pub enum UnitConversionError {
    #[error("Asset Unit is not of type Shader")]
    UnitIsNotShader,
    #[error("Asset Unit is not of type Texture")]
    UnitIsNotTexture,
    #[error("Asset Unit is not of type Model")]
    UnitIsNotModel,
    #[error("Asset Unit is not of type Config")]
    UnitIsNotConfig,
    #[error("Asset Unit is not of type Text")]
    UnitIsNotText,
}

#[derive(Debug, Error)]
pub enum PackError {
    #[error("Invalid pack directory: {0}")]
    InvalidDirectory(#[from] std::io::Error),
    #[error("Error while modifying archive: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Error with model: {0}")]
    ModelError(#[from] ModelUnpackError)
}

#[derive(Debug, Error)]
pub enum ModelUnpackError {
    #[error("Cannot read model from .obj: {0}")]
    ObjError(#[from] obj::ObjError),
    #[error("Obj file needs to have Position/Texture/Normal data defined")]
    InvalidFormat,
    #[error("Exported model has a face with > 4 vertices")]
    TooManyVertices,
    #[error("Exported model could not be triangulated: {0}")]
    TriangulationError(#[from] NgonError)
}

#[derive(Debug, Error)]
pub enum UnpackError {
    #[error("Invalid unpack directory: {0}")]
    InvalidDirectory(#[from] std::io::Error),
    #[error("Error while reading archive: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Error while parsing Json: {0}")]
    JsonError(#[from] miniserde::Error),
    #[error("File structure is not as expected")]
    UnexpectedFileStructure,
    #[error("Stored data not correct size. Read: {0} | Expected: {1}")]
    SizeMismatch(usize, usize),
}

#[derive(Debug, Copy, Clone, Error)]
pub enum NgonError {
    #[error("Cannot triangulate given quad")]
    CannotTriangulateQuad,
    #[error("Cannot triangulate models with faces that have more than 4 vertices")]
    NgonIsNotQuadOrTriangle
}
