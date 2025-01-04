use thiserror::Error;
#[derive(Error, Debug)]
pub enum Entry {
    #[error("Type in entry does not match conversion type")]
    TypeMismatch
}

#[derive(Error, Debug)]
pub enum Library<TId: std::fmt::Debug> {
    #[error("Library does not contain key `{0:?}` while trying to retrieve")]
    KeyNotPresent(TId),
    #[error("Library already contains key `{0:?}` while trying to insert")]
    KeyPresent(TId),
    #[error("Error while converting entry")]
    ConversionError(#[from] Entry),
}
