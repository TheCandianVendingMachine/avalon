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
}
