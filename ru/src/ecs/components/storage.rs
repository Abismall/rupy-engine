use crate::ecs::entities::models::Entity;

use super::Component;

pub trait ComponentStorage {
    fn remove(&self, entity: Entity);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct ComponentVec<T: Component> {
    pub data: std::sync::RwLock<std::collections::HashMap<Entity, T>>,
}

impl<T: Component> ComponentVec<T> {
    pub fn new() -> Self {
        Self {
            data: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub fn insert(&self, entity: Entity, component: T) -> Result<(), String> {
        match self.data.write() {
            Ok(mut data) => {
                data.insert(entity, component);
                Ok(())
            }
            Err(e) => Err(format!("Failed to acquire write lock: {}", e)),
        }
    }

    pub fn get(&self, entity: Entity) -> Option<T>
    where
        T: Clone,
    {
        let data = self.data.read().unwrap();
        data.get(&entity).cloned()
    }
}

impl<T: Component> ComponentStorage for ComponentVec<T> {
    fn remove(&self, entity: Entity) {
        let mut data = self.data.write().unwrap();
        data.remove(&entity);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
