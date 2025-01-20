use crate::{ error, shader, texture };

use uuid;
use filetime;
use std::path::PathBuf;
use std::sync::atomic;
use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub enum Unit {
    Shader(shader::Shader),
    Texture(texture::Texture),
    Model,
    Config,
    Text,
}

#[derive(Debug, Hash)]
pub struct AssetReference<'asset> {
    asset: &'asset Asset
}

#[derive(Debug)]
pub struct Asset {
    uuid: uuid::Uuid,
    tag: String,
    filepath: PathBuf,
    filetime: filetime::FileTime,
    is_human_readable: bool,
    unit: Unit,
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
}

impl std::hash::Hash for Asset {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.uuid.hash(hasher);
    }
}

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
