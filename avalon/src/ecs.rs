use bitfield;
use std::collections::HashMap;

bitfield::bitfield!{
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Handle(u64);
    impl Debug;
    idx, set_idx: 16, 0;
    pool_id, set_pool_id: 64, 11;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entity {
    handle: Handle,
}

pub mod component {
    use std::collections::{ HashMap, BinaryHeap };
    use bit_set::BitSet;
    use aligned_vec::{ AVec, ConstAlign };
    use image::imageops::FilterType::Triangle;
    use crate::ecs::Entity;

    pub trait Tag {
        fn uid(&self) -> u32;
    }

    pub trait Component: Sized + Copy {
        fn tag() -> impl Tag;
        fn id(&self) -> u32;

        fn tag_from(&self) -> impl Tag { Self::tag() }
        fn uid(&self) -> u64 { ((self.id() as u64) << 32) | (Self::tag().uid() as u64) }
    }

    pub trait Store<T: Component> {
        fn components_matching_entities(&self, entities: &[Entity]) -> Vec<(Entity, T)>;
    }

    #[derive(Debug, Copy, Clone)]
    struct Metadata {
        stride: usize,
        component: usize,
        tag: u32,
    }

    pub struct Group<const BlockSize: usize = 64> {
        entity: Entity,
        item_map: HashMap<u32, (usize, usize)>,
        blocks: AVec<[u8; BlockSize], ConstAlign<128>>,
        available_block: usize,
        available_idx: usize,
        last_block: usize
    }

    impl<const BlockSize: usize> Group<BlockSize> {
        const DEFAULT_BLOCK: [u8; BlockSize] = [0xFC; BlockSize];
        pub fn new(entity: Entity) -> Group {
            Group {
                entity,
                item_map: HashMap::new(),
                blocks: AVec::with_capacity(128, BlockSize),
                available_idx: 0,
                available_block: 0,
                last_block: 0
            }
        }

        pub fn get<T: Component>(&self) -> &T {
            std::debug_assert_eq!(self.item_map.contains_key(&T::tag().uid()), true);
            let (block, idx) = self.item_map.get(&T::tag().uid()).unwrap();
            let ptr = &self.blocks[*block][*idx] as *const u8;
            unsafe {
                let t_ptr: *const T = std::mem::transmute(ptr);
                &*t_ptr
            }
        }

        pub fn get_mut<T: Component>(&mut self) -> &mut T {
            std::debug_assert_eq!(self.item_map.contains_key(&T::tag().uid()), true);
            let (block, idx) = self.item_map.get(&T::tag().uid()).unwrap();
            let ptr = &mut self.blocks[*block][*idx] as *mut u8;
            unsafe {
                let t_ptr: *mut T = std::mem::transmute(ptr);
                &mut *t_ptr
            }
        }

        pub fn assign<T: Component>(&mut self, component: T) {
            // only one of each component can be in a block
            std::debug_assert_eq!(self.item_map.contains_key(&T::tag().uid()), false);

            let component_alignment: usize = std::mem::align_of::<T>();
            let metadata_alignment: usize = std::mem::align_of::<Metadata>();

            std::debug_assert_eq!(128 % component_alignment, 0);
            std::debug_assert_eq!(128 % metadata_alignment, 0);

            let metadata_stride: usize = std::mem::size_of::<Metadata>();
            let component_stride: usize = std::mem::size_of::<T>();

            let alloc_size = metadata_stride + component_stride;
            let block_count = alloc_size.div_ceil(BlockSize);

            // since we are aligned on the 128 line, all elements can be allocated at the
            // start of a block. this means that we can be greedy with allocating blocks,
            // and can guarantee enough blocks to allocate within
            if self.available_block == self.last_block {
                for _ in 0..block_count {
                    self.blocks.push(Self::DEFAULT_BLOCK);
                }
                self.last_block += block_count;
            } else if block_count >= 2 {
                for _ in 0..(block_count - 1) {
                    self.blocks.push(Self::DEFAULT_BLOCK);
                }
                self.last_block += block_count - 1;
            } else if self.available_idx != 0 {
                self.blocks.push(Self::DEFAULT_BLOCK);
                self.last_block += 1;
            }

            // 1) push metadata
            // axiom: the available index of the available block is _always_ aligned with
            // the metadata, and has space to store it

            let mut alignment_error = ((self.available_idx + metadata_stride) % component_alignment);
            if alignment_error != 0 {
                alignment_error = component_alignment - alignment_error;
            }

            let metadata = Metadata {
                stride: metadata_stride + alignment_error + component_stride,
                component: metadata_stride + alignment_error,
                tag: T::tag().uid()
            };

            // Proof of Safety:
            // We know from axiom (1) that there is enough space to treat this area of
            // memory as contigious, and with enough size to store the metadata. So we can
            // naively copy bits to the element at the index
            std::debug_assert!(metadata_stride.div_ceil(BlockSize) + self.available_block < self.blocks.len());
            std::debug_assert_eq!(self.available_idx % metadata_alignment, 0);
            {
                let mut ptr = &mut self.blocks[self.available_block][self.available_idx] as *mut u8;
                let mut type_ptr = ptr as *mut Metadata;
                unsafe {
                    *type_ptr = metadata;
                };
            };

            // 2) push component
            std::debug_assert!(component_stride.div_ceil(BlockSize) + self.available_block < self.blocks.len());

            self.available_idx += metadata_stride + alignment_error;
            if self.available_idx >= BlockSize {
                self.available_block += 1;
                self.available_idx -= BlockSize;
            }
            std::debug_assert_eq!(self.available_idx % component_alignment, 0);

            // Proof of Safety:
            // We have incremented the index to a valid alignment, and have proved through
            // assertion that the current index can support the alignment of the type
            {
                let mut ptr = &mut self.blocks[self.available_block][self.available_idx] as *mut u8;
                let mut type_ptr = ptr as *mut T;
                unsafe {
                    *type_ptr = component;
                };
            };

            self.item_map.insert(T::tag().uid(), (self.available_block, self.available_idx));
            self.available_idx += component_stride;
            if self.available_idx >= BlockSize {
                self.available_block += 1;
                self.available_idx -= BlockSize;
            }

            if self.available_idx % metadata_alignment != 0 {
                self.available_idx += metadata_alignment - (self.available_idx % metadata_alignment);
            }
        }
    }

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

        pub fn entities_with_components(&self, components: BitSet) -> Vec<Entity> {
            self.iter()
                .filter(|(_, c)| c.is_subset(&components))
                .map(|(e, _)| e)
                .collect()
        }
    }
}

trait Poolable: Sized + Copy {
    fn with_handle(handle: Handle) -> Self;
    fn handle(&self) -> Handle;
}


struct Pool<T: Poolable> {
    objects: Vec<Option<T>>,
    free_indices: Vec<usize>
}

impl<T: Poolable> Pool<T> {
    const POOL_SIZE: usize = 2_usize.pow(16);
    fn new() -> Pool<T> {
        Pool {
            objects: vec![None; Self::POOL_SIZE],
            free_indices: Vec::from_iter(0..Self::POOL_SIZE)
        }
    }

    fn allocate(&mut self, pool_idx: usize) -> Option<T> {
        let free_index = self.free_indices.pop()?;
        let mut handle = Handle(0);
        handle.set_idx(free_index as u64);
        handle.set_pool_id(pool_idx as u64);
        self.objects[free_index] = Some(T::with_handle(handle));
        self.objects[free_index]
    }

    fn deallocate(&mut self, handle: Handle) {
        self.objects[handle.idx() as usize] = None;
        self.free_indices.push(handle.idx() as usize);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.objects.iter().filter_map(|e| e.as_ref())
    }
}

pub struct GrowablePool<T: Poolable> {
    pools: Vec<Pool<T>>
}

impl<T: Poolable> GrowablePool<T> {
    pub fn new() -> GrowablePool<T> {
        GrowablePool {
            pools: Vec::with_capacity(4)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.pools.iter()
            .map(|p| p.iter())
            .flatten()
    }

    pub fn allocate(&mut self) -> T {
        for (idx, pool) in self.pools.iter_mut().enumerate() {
            if let Some(object) = pool.allocate(idx) {
                return object
            }
        }

        // If we are here, we were unable to allocate from any existing pool so we create
        // a new one
        self.pools.push(Pool::new());
        let pool_idx = self.pools.len() - 1;
        self.pools.last_mut().unwrap().allocate(pool_idx).unwrap()
    }

    pub fn deallocate(&mut self, object: T) {
        self.deallocate_handle(object.handle())
    }

    pub fn deallocate_handle(&mut self, handle: Handle) {
        self.pools[handle.pool_id() as usize].deallocate(handle);
    }
}

/*
 * For each system,
 *  prefetch components and store groups of them associated with entity
 *  pass system this list
 *  system interacts with list, modfying as needed
 *  update original components with modified ones
 */
