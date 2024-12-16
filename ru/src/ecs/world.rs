use super::{
    components::{Component, ComponentStorage, ComponentVec},
    entity::{Entity, EntityManager},
    scene::Scene,
};
use crate::core::{cache::HasCacheKey, error::AppError};
use std::{any::TypeId, collections::HashMap};
pub trait Query<'a> {
    type Output;

    fn fetch(
        storages: &'a HashMap<TypeId, Box<dyn ComponentStorage>>,
        entities: &'a Vec<Entity>,
    ) -> Vec<(Entity, Self::Output)>;
}

impl<'a, A: Component + 'static, B: Component + 'static> Query<'a> for (A, B) {
    type Output = (A, B);

    fn fetch(
        storages: &'a HashMap<TypeId, Box<dyn ComponentStorage>>,
        entities: &'a Vec<Entity>,
    ) -> Vec<(Entity, Self::Output)> {
        let storage_a = storages
            .get(&TypeId::of::<A>())
            .and_then(|s| s.as_any().downcast_ref::<ComponentVec<A>>())
            .expect("Storage for component A not found!");

        let storage_b = storages
            .get(&TypeId::of::<B>())
            .and_then(|s| s.as_any().downcast_ref::<ComponentVec<B>>())
            .expect("Storage for component B not found!");

        let data_a = storage_a.data.read().unwrap().clone();
        let data_b = storage_b.data.read().unwrap().clone();

        entities
            .iter()
            .filter_map(|&entity| {
                let a = data_a.get(&entity)?.clone();
                let b = data_b.get(&entity)?.clone();
                Some((entity, (a, b)))
            })
            .collect()
    }
}

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
    pub fn create_entity(&mut self) -> Entity {
        self.entities.new_entity()
    }
    pub fn add_component<T: Component + HasCacheKey>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Result<(), AppError> {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.component_storage.get_mut(&type_id) {
            let component_vec = storage
                .as_any_mut()
                .downcast_mut::<ComponentVec<T>>()
                .expect("ComponentVec<T> downcast failed");
            let _ = component_vec.insert(entity, component);
        } else {
            let component_vec = ComponentVec::<T>::new();
            let _ = component_vec.insert(entity, component);
            self.component_storage
                .insert(type_id, Box::new(component_vec));
        }

        Ok(())
    }
}

impl World {
    pub fn query<T: Component>(&self, mut f: impl FnMut(Entity, &T)) -> Result<(), AppError> {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.component_storage.get(&type_id) {
            match storage.as_any().downcast_ref::<ComponentVec<T>>() {
                Some(c_vec) => {
                    let data_t = c_vec.data.read().map_err(|e| {
                        AppError::LockAcquisitionFailure(format!(
                            "Error acquiring read lock for the requested type: {:?} {:?}",
                            type_id, e
                        ))
                    })?;

                    for (&entity, component_t) in data_t.iter() {
                        f(entity, component_t);
                    }
                }
                None => {
                    return Err(AppError::WorldQueryError(format!(
                        "Component vector downcast_ref returned None for the requested type: {:?}",
                        type_id
                    )))
                }
            };
        } else {
            return Err(AppError::ResourceNotFound(format!(
                "No component storage found for the requested type: {:?}",
                type_id
            )));
        }

        Ok(())
    }
    pub fn query_multi<'a, Q: Query<'a>>(
        &'a self,
        mut f: impl FnMut(Entity, Q::Output),
    ) -> Result<(), AppError> {
        let entities = self.entities.get_entities();

        let results = Q::fetch(&self.component_storage, entities);

        for (entity, data) in results {
            f(entity, data);
        }

        Ok(())
    }

    pub fn query_mut<T: Component>(
        &self,
        mut f: impl FnMut(Entity, &mut T),
    ) -> Result<(), AppError> {
        if let Some(storage_t) = self.component_storage.get(&TypeId::of::<T>()) {
            let component_vec_t = storage_t
                .as_any()
                .downcast_ref::<ComponentVec<T>>()
                .ok_or_else(|| {
                    AppError::WorldQueryError(format!("Failed to downcast ComponentVec<T>"))
                })?;

            let mut data_t = component_vec_t.data.write().map_err(|e| {
                AppError::WorldQueryError(format!("Failed to acquire write lock: {:?}", e))
            })?;

            for (&entity, component_t) in data_t.iter_mut() {
                f(entity, component_t);
            }

            Ok(())
        } else {
            Err(AppError::WorldQueryError(format!(
                "Component storage for the requested type not found."
            )))
        }
    }
    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities.get_entities()
    }
    pub fn get_entities_mut(&mut self) -> Vec<Entity> {
        self.entities.get_entities().to_vec()
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
