#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entity {
    handle: u64,
}

pub mod component {
    use std::collections::BinaryHeap;
    use bit_set::BitSet;
    use crate::ecs::Entity;

    struct EntityPair {
        entity: Entity,
        components: BitSet
    }

    impl PartialEq for EntityPair {
        fn eq(&self, other: &EntityPair) -> bool {
            self.entity.eq(&other.entity)
        }
    }
    impl Eq for EntityPair {}

    impl PartialOrd for EntityPair {
        fn partial_cmp(&self, other: &EntityPair) -> Option<std::cmp::Ordering> {
            self.entity.partial_cmp(&other.entity)
        }
    }

    impl Ord for EntityPair {
        fn cmp(&self, other: &EntityPair) -> std::cmp::Ordering {
            self.entity.cmp(&other.entity)
        }
    }

    pub struct Bag {
        entity_map: BinaryHeap<EntityPair>
    }

    impl Bag {
        pub fn new() -> Bag {
            Bag {
                entity_map: BinaryHeap::with_capacity(2_usize.pow(16))
            }
        }

        pub fn iter(&self) -> impl Iterator<Item = (Entity, &BitSet)> {
            self.entity_map.iter().map(|pair| (pair.entity, &pair.components))
        }
    }
}
