use thiserror::Error;
#[derive(Error, Debug)]
pub enum Creation {
    #[error("Failed to compile shader: {reason}")]
    FailedToCompile { reason: String },
    #[error("Failed to link shader: {reason}")]
    FailedToLink { reason: String },
    #[error("Failed to load shader file `{path}`")]
    ShaderNotFound { path: String },
    #[error("Failed to read shader file")]
    FileReadError(#[from] std::io::Error),
    #[error("Failed to parse shader file")]
    FileParseError(#[from] std::string::FromUtf8Error),
}

#[derive(Error, Debug)]
pub enum Program {
    #[error("Cannot find uniform `{name}` in shader")]
    UniformNotFound {
        name: String
    }
}
