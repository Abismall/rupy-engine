use std::{any::TypeId, collections::HashMap};

use crate::core::error::AppError;

use super::{
    components::{Component, ComponentStorage, ComponentVec, IntoComponentCacheKey},
    entity::{Entity, EntityManager},
    scene::Scene,
    traits::Cache,
};

pub struct World {
    entities: EntityManager,
    scenes: HashMap<u64, Scene>,
    component_storage: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntityManager::new(),
            component_storage: HashMap::new(),
            scenes: HashMap::new(),
        }
    }

    pub fn add_component<T: Component + IntoComponentCacheKey>(
        &mut self,
        entity: Entity,
        component: T,
        manager: &mut impl Cache<T>,
    ) -> Result<(), AppError> {
        let component_key = component.into_cache_key();

        let cached_component = manager.get(component_key);

        if cached_component.is_none() {
            manager.put(component_key, component)?;
        }

        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.component_storage.get_mut(&type_id) {
            let component_vec = storage
                .as_any_mut()
                .downcast_mut::<ComponentVec<T>>()
                .expect("ComponentVec<T> downcast failed");
            let _ = component_vec.insert(entity, *manager.get(component_key).unwrap());
        } else {
            let mut component_vec = ComponentVec::<T>::new();
            let _ = component_vec.insert(entity, *manager.get(component_key).unwrap());
            self.component_storage
                .insert(type_id, Box::new(component_vec));
        }

        Ok(())
    }
}
impl World {
    pub fn query<T: Component>(&self, mut f: impl FnMut(Entity, &T)) -> Result<(), AppError> {
        let storage_t = self.component_storage.get(&TypeId::of::<T>());

        if let Some(storage_t) = storage_t {
            let component_vec_t = storage_t
                .as_any()
                .downcast_ref::<ComponentVec<T>>()
                .ok_or_else(|| {
                    AppError::ComponentError(format!("Failed to downcast ComponentVec<T>"))
                })?;

            let data_t = component_vec_t.data.read().map_err(|e| {
                AppError::LockAcquisitionFailure(format!("Failed to acquire read lock: {:?}", e))
            })?;

            for (&entity, component_t) in data_t.iter() {
                f(entity, component_t);
            }

            Ok(())
        } else {
            Err(AppError::ComponentError(
                "Component storage for the requested type not found.".to_string(),
            ))
        }
    }

    pub fn query_mut<T: Component>(
        &self,
        mut f: impl FnMut(Entity, &mut T),
    ) -> Result<(), AppError> {
        let storage_t = self.component_storage.get(&TypeId::of::<T>());

        if let Some(storage_t) = storage_t {
            let component_vec_t = storage_t
                .as_any()
                .downcast_ref::<ComponentVec<T>>()
                .ok_or_else(|| {
                    AppError::ComponentError(format!("Failed to downcast ComponentVec<T>"))
                })?;

            let mut data_t = component_vec_t.data.write().map_err(|e| {
                AppError::LockAcquisitionFailure(format!("Failed to acquire write lock: {:?}", e))
            })?;

            for (&entity, component_t) in data_t.iter_mut() {
                f(entity, component_t);
            }

            Ok(())
        } else {
            Err(AppError::ComponentError(
                "Component storage for the requested type not found.".to_string(),
            ))
        }
    }
    pub fn get_entities(self) -> std::vec::IntoIter<Entity> {
        self.entities.into_iter()
    }
}

impl World {
    pub fn new_scene(&mut self, id: u64, name: &str) {
        if !self.scenes.contains_key(&id) {
            self.scenes.insert(id, Scene::new(name, id));
        }
    }
    pub fn load_scene(&mut self, id: u64) -> Option<&mut Scene> {
        self.scenes.get_mut(&id)
    }

    pub fn get_scene(&self, id: u64) -> Option<&Scene> {
        self.scenes.get(&id)
    }
    pub fn get_scene_mut(&mut self, id: u64) -> Option<&mut Scene> {
        self.scenes.get_mut(&id)
    }
}
