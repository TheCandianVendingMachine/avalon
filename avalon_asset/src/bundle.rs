use crate::asset;

#[derive(Debug, Eq)]
pub struct Bundle {
    pub group: Vec<asset::Metadata>,
    pub name: String,
}

impl PartialEq for Bundle {
    fn eq(&self, rhs: &Bundle) -> bool {
        self.name.eq(&rhs.name)
    }
}

impl std::hash::Hash for Bundle {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.name.hash(hasher);
    }
}

impl Bundle {
    pub fn asset(&self, tag: impl Into<String>) -> Option<asset::Metadata> {
        let tag = tag.into();
        for asset in self.group.iter() {
            if asset.tag == tag {
                return Some(asset.clone());
            }
        }
        None
    }
}

