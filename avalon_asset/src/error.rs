use thiserror::Error;

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
}

