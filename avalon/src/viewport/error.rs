use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Viewport {
    #[error("Viewport does not have colour attachment with name \"{0}\"")]
    NoColourWithName(String),
    #[error("Viewport does not have colour attachment at index {0}")]
    NoColourAtIndex(usize),
    #[error("Viewport does not have depth-stencil attachment")]
    NoDepthStencilAttachment,
}

#[derive(Debug, Clone, Error)]
pub enum DepthFunction {
    #[error("Unknown function value")]
    InvalidFunction,
}
