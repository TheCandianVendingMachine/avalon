pub mod serialization;
#[cfg(feature = "write")]
pub use serialization::write;
#[cfg(feature = "read")]
pub use serialization::read;

use crate::{ error, shader, texture, text };

use uuid;
use std::path::PathBuf;
use std::sync::atomic;
use std::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Shader,
    Texture,
    Model,
    Text,
}

#[derive(Debug, Copy, Clone)]
pub enum Unit {
    Shader(shader::Shader),
    Texture(texture::Texture),
    Model,
    Text(text::Text),
}

#[derive(Debug, Hash)]
pub struct AssetReference<'asset> {
    asset: &'asset Asset
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub uuid: uuid::Uuid,
    pub tag: String,
    pub filepath: Option<PathBuf>,
    pub unit: Unit,
}

#[derive(Debug)]
pub struct Asset {
    pub(crate) metadata: Metadata,
    references: atomic::AtomicUsize
}

impl Asset {
    fn increment_reference(&self) {
        self.references.fetch_add(1, atomic::Ordering::Acquire);
    }

    fn decrement_reference(&self) {
        self.references.fetch_sub(1, atomic::Ordering::Acquire);
    }

    pub fn refer(&self) -> AssetReference {
        self.into()
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
}

impl Metadata {
    pub fn new(tag: impl Into<String>, filepath: impl Into<PathBuf>, unit: impl Into<Unit>) -> Metadata {
        Metadata {
            uuid: uuid::Uuid::new_v4(),
            tag: tag.into(),
            filepath: Some(filepath.into()),
            unit: unit.into()
        }
    }
}

#[cfg(feature = "write")]
impl From<Metadata> for Asset {
    fn from(metadata: Metadata) -> Asset {
        Asset {
            metadata,
            references: atomic::AtomicUsize::new(0)
        }
    }
}

impl std::hash::Hash for Metadata {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.uuid.hash(hasher);
    }
}

impl PartialEq for Metadata {
    fn eq(&self, rhs: &Metadata) -> bool {
        self.uuid == rhs.uuid
    }
}
impl Eq for Metadata {}

impl std::hash::Hash for Asset {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.metadata.hash(hasher);
    }
}

impl PartialEq for Asset {
    fn eq(&self, rhs: &Asset) -> bool {
        self.metadata == rhs.metadata
    }
}
impl Eq for Asset {}

impl<'old> Clone for AssetReference<'old> {
    fn clone(&self) -> AssetReference<'old> {
        self.asset.increment_reference();
        AssetReference {
            asset: self.asset
        }
    }
}

impl Drop for AssetReference<'_> {
    fn drop(&mut self) {
        self.asset.decrement_reference();
    }
}

impl AsRef<Asset> for AssetReference<'_> {
    fn as_ref(&self) -> &Asset {
        self.asset
    }
}

impl<'reference, 'asset: 'reference> From<&'asset Asset> for AssetReference<'reference> {
    fn from(asset: &'asset Asset) -> AssetReference<'reference> {
        asset.increment_reference();
        AssetReference {
            asset: &asset
        }
    }
}

impl Deref for AssetReference<'_> {
    type Target = Asset;
    fn deref(&self) -> &Asset {
        self.as_ref()
    }
}

impl TryFrom<Unit> for shader::Shader {
    type Error = error::UnitConversionError;
    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        if let Unit::Shader(shader) = unit {
            return Ok(shader);
        }
        Err(error::UnitConversionError::UnitIsNotShader)
    }
}

impl TryFrom<Unit> for texture::Texture {
    type Error = error::UnitConversionError;
    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        if let Unit::Texture(texture) = unit {
            return Ok(texture);
        }
        Err(error::UnitConversionError::UnitIsNotTexture)
    }
}

impl TryFrom<Unit> for text::Text {
    type Error = error::UnitConversionError;
    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        if let Unit::Text(text) = unit {
            return Ok(text);
        }
        Err(error::UnitConversionError::UnitIsNotText)
    }
}

impl Type {
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Option<Type> {
        let path = path.as_ref();
        if !path.is_file() {
            return None;
        }
        let extension = path.extension()?;
        match extension.to_ascii_lowercase().to_str()? {
            "png" | "jpg" | "jpeg" => Some(Type::Texture),
            "comp" | "vert" | "frag" => Some(Type::Shader),
            _ => None,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Shader => "Shader".fmt(formatter),
            Type::Texture => "Texture".fmt(formatter),
            Type::Model => "Model".fmt(formatter),
            Type::Text => "Text".fmt(formatter),
        }
    }
}

impl From<Unit> for Type {
    fn from(unit: Unit) -> Type {
        match unit {
            Unit::Model => Type::Model,
            Unit::Shader(_) => Type::Shader,
            Unit::Texture(_) => Type::Texture,
            Unit::Text(_) => Type::Text,
        }
    }
}
