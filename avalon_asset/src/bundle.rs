use crate::asset;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Bundle {
    pub group: Vec<asset::Metadata>,
    pub name: String,
}

impl Bundle {
    pub fn asset(&self, tag: impl Into<String>) -> Option<asset::Metadata> {
        let tag = tag.into();
        for asset in self.group.iter() {
            if asset.tag == tag {
                return Some(asset.clone());
            }
        }
        return None;
    }
}

