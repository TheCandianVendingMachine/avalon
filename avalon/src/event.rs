use std::collections::HashMap;

mod derives;
mod error {
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
}

#[derive(Debug, Copy, Clone)]
pub enum Entry {
    Bool(bool),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

pub struct Library<TId: std::fmt::Debug + Copy + Eq + std::hash::Hash> {
    entries: HashMap<TId, Entry>
}

pub struct Event<TEid: std::fmt::Debug, TId: std::fmt::Debug + Copy + Eq + std::hash::Hash> {
    pub data: Library<TId>,
    pub id: TEid
}

impl<TId: std::fmt::Debug + Copy + Eq + std::hash::Hash> Library<TId> {
    pub fn retrieve<T: TryFrom<Entry, Error=error::Entry>>(&self, id: TId) -> Result<T, error::Library<TId>> {
        if !self.entries.contains_key(&id) {
            return Err(error::Library::KeyNotPresent(id));
        }

       T::try_from(
            *self.entries.get(&id).unwrap()
        ).map_err(|e| error::Library::ConversionError(e))
    }

    pub fn store<T: Into<Entry>>(&mut self, key: TId, value: T) -> Result<(), error::Library<TId>> {
        if self.entries.contains_key(&key) {
            return Err(error::Library::KeyPresent(key))
        }
        self.entries.insert(key, value.into());
        Ok(())
    }
}
