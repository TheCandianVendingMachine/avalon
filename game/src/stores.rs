use avalon::ecs::component::{ Component, Store as ComponentStore };
use avalon::ecs::{ Handle, Entity, GrowablePool, Poolable };

pub struct Store<T: Component + Poolable> {
    pool: GrowablePool<T>,
}

impl<T: Component + Poolable> Store<T> {
    pub fn new() -> Store<T> {
        Store {
            pool: GrowablePool::new()
        }
    }
}

impl<T: Component + Poolable> ComponentStore<T> for Store<T> {
    fn components_matching_entities(&self, entities: &[Entity]) -> Vec<(Entity, T)> {
        let mut pairs = Vec::new();
        for component in self.pool.iter() {
        }
        pairs
    }

    fn update_components(&mut self, components: &[T]) {
        
    }
}
