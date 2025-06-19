use avalon::ecs::component::{ Component, Store as ComponentStore };
use avalon::ecs::{ Handle, Entity, GrowablePool, Poolable };
use std::collections::HashMap;

pub struct Store<T: Component + Poolable> {
    pool: GrowablePool<T>,
    entity_component_map: HashMap<Entity, Handle>
}

impl<T: Component + Poolable> Store<T> {
    pub fn new() -> Store<T> {
        Store {
            pool: GrowablePool::new(),
            entity_component_map: HashMap::new()
        }
    }
}

impl<T: Component + Poolable> ComponentStore<T> for Store<T> {
    fn allocate(&mut self, entity: Entity) {
        let handle = self.pool.allocate().handle();
        self.entity_component_map.insert(entity, handle);
    }

    fn components_matching_entities(&self, entities: &[Entity]) -> Vec<(Entity, T)> {
        let mut pairs = Vec::with_capacity(entities.len());
        for entity in entities {
            if let Some(idx) = self.entity_component_map.get(entity) {
                pairs.push((*entity, *self.pool.get(*idx).unwrap()))
            }
        }
        pairs
    }

    fn update_components(&mut self, components: &[T]) {
        for component in components {
            *self.pool.get_mut(component.id().into()).unwrap() = *component;
        }
    }
}
