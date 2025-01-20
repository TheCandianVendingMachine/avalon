use crate::asset;

#[derive(Debug, Hash)]
pub struct Bundle {
    pub group: Vec<asset::Asset>,
    pub name: String,
}

impl Bundle {
    pub fn asset(&self, tag: impl Into<String>) -> Option<&asset::Asset> {
        let tag = tag.into();
        for asset in self.group.iter() {
            if asset.metadata.tag == tag {
                return Some(asset);
            }
        }
        return None;
    }
}

impl std::ops::Index<String> for Bundle {
    type Output = asset::Asset;
    fn index(&self, index: String) -> &asset::Asset {
        self.asset(index).expect("Tag needs to exist within bundle!")
    }
}
