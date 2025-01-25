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
    #[error("Cannot read model from .obj: {0}")]
    ObjError(#[from] obj::ObjError)
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
}

#[derive(Debug, Copy, Clone, Error)]
pub enum NgonError {
    #[error("Cannot triangulate given quad")]
    CannotTriangulateQuad,
    #[error("Cannot triangulate models with faces that have more than 4 vertices")]
    NgonIsNotQuadOrTriangle
}
