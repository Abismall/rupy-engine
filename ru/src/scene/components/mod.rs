pub mod material;
pub mod mesh;
pub mod resources;
pub mod traits;
pub mod transform;
pub mod uniform;
pub mod vertex;
use std::{any::Any, collections::HashMap, sync::RwLock};

use traits::{Component, ComponentStorage};

use super::entities::models::Entity;

pub struct ComponentVec<T: Component> {
    pub data: RwLock<HashMap<Entity, T>>,
}

impl<T: Component> ComponentVec<T> {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
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

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
