use crate::asset;

#[derive(Debug, Hash)]
pub struct Bundle {
    pub group: Vec<asset::Asset>,
    pub name: String,
}
