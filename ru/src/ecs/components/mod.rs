pub mod instance;
pub mod material;
pub mod mesh;
pub mod model;
pub mod transform;
pub mod utils;
use super::entity::Entity;
use crate::core::{cache::ComponentCacheKey, error::AppError};
use std::{any::Any, collections::HashMap, sync::RwLock};

pub trait IntoComponentCacheKey {
    fn into_cache_key(&self) -> ComponentCacheKey;
}
pub trait Component: Any + Send + Sync {}

impl<T> Component for T where T: Any + Send + Sync {}
pub trait ComponentStorage {
    fn remove(&self, entity: Entity);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: Component> ComponentStorage for ComponentVec<T> {
    fn remove(&self, entity: Entity) {
        self.remove(entity);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
pub struct ComponentVec<T: Component> {
    pub data: RwLock<HashMap<Entity, T>>,
}

impl<T: Component> ComponentVec<T> {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, entity: Entity, component: T) -> Result<(), AppError> {
        self.data
            .write()
            .map(|mut data| {
                data.insert(entity, component);
            })
            .map_err(|e| AppError::LockAcquisitionFailure(e.to_string()))
    }

    pub fn get(&self, entity: Entity) -> Option<T>
    where
        T: Clone,
    {
        self.data.read().ok()?.get(&entity).cloned()
    }

    pub fn remove(&self, entity: Entity) {
        if let Ok(mut data) = self.data.write() {
            data.remove(&entity);
        }
    }
}
