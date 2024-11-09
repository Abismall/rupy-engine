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
}
impl<T: Component> ComponentVec<T> {
    pub fn insert(&self, entity: Entity, component: T) -> Result<(), String> {
        let mut data = self.data.write().map_err(|e| e.to_string())?;
        data.insert(entity, component);
        Ok(())
    }

    pub fn get(&self, entity: Entity) -> Option<T>
    where
        T: Clone,
    {
        let data = self.data.read().ok()?;
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
