use std::{any::TypeId, collections::HashMap};

use crate::{
    ecs::{
        components::{
            storage::{ComponentStorage, ComponentVec},
            Component,
        },
        entities::models::Entity,
    },
    log_debug,
};

pub struct World {
    pub next_entity: Entity,
    pub entities: Vec<Entity>,
    pub components: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity: 0,
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }
}

impl World {
    pub fn create_entity(&mut self) -> Entity {
        let entity = self.next_entity;
        self.next_entity += 1;
        self.entities.push(entity);
        log_debug!("Created entity: {}", entity);
        entity
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        log_debug!("Deleting entity: {}", entity);
        self.entities.retain(|&e| e != entity);
        for storage in self.components.values_mut() {
            storage.remove(entity);
        }
    }

    pub fn add_component<T: Component + std::fmt::Debug>(&mut self, entity: Entity, component: T) {
        log_debug!("Adding component to entity {}: {:?}", entity, component);
        let type_id = TypeId::of::<T>();

        if let Some(storage) = self.components.get_mut(&type_id) {
            let component_vec = storage
                .as_any_mut()
                .downcast_mut::<ComponentVec<T>>()
                .expect("ComponentVec<T> downcast failed");
            let _ = component_vec.insert(entity, component);
        } else {
            let component_vec = ComponentVec::<T>::new();
            let _ = component_vec.insert(entity, component);
            self.components.insert(type_id, Box::new(component_vec));
        }

        log_debug!("Component added to entity {}", entity);
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<T>
    where
        T: Clone,
    {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<ComponentVec<T>>())
            .and_then(|component_vec| component_vec.get(entity))
    }

    pub fn query<T: Component>(&self, mut f: impl FnMut(Entity, &T)) {
        if let Some(storage) = self.components.get(&TypeId::of::<T>()) {
            let component_vec = storage
                .as_any()
                .downcast_ref::<ComponentVec<T>>()
                .expect("ComponentVec downcast failed");

            let data = component_vec.data.read().unwrap();
            for (&entity, component) in data.iter() {
                f(entity, component);
            }
        }
    }

    pub fn query4<T: Component, U: Component, M: Component, R: Component>(
        &self,
        mut f: impl FnMut(Entity, &T, &U, &M, &R),
    ) {
        let storage_t = self.components.get(&TypeId::of::<T>());
        let storage_u = self.components.get(&TypeId::of::<U>());
        let storage_m = self.components.get(&TypeId::of::<M>());
        let storage_r = self.components.get(&TypeId::of::<R>());

        if let (Some(storage_t), Some(storage_u), Some(storage_m), Some(storage_r)) =
            (storage_t, storage_u, storage_m, storage_r)
        {
            let component_vec_t = match storage_t.as_any().downcast_ref::<ComponentVec<T>>() {
                Some(vec) => vec,
                None => {
                    log_debug!("Failed to downcast ComponentVec<T>.");
                    return;
                }
            };

            let component_vec_u = match storage_u.as_any().downcast_ref::<ComponentVec<U>>() {
                Some(vec) => vec,
                None => {
                    log_debug!("Failed to downcast ComponentVec<U>.");
                    return;
                }
            };

            let component_vec_m = match storage_m.as_any().downcast_ref::<ComponentVec<M>>() {
                Some(vec) => vec,
                None => {
                    log_debug!("Failed to downcast ComponentVec<M>.");
                    return;
                }
            };

            let component_vec_r = match storage_r.as_any().downcast_ref::<ComponentVec<R>>() {
                Some(vec) => vec,
                None => {
                    log_debug!("Failed to downcast ComponentVec<R>.");
                    return;
                }
            };

            let data_t = match component_vec_t.data.read() {
                Ok(data) => data,
                Err(e) => {
                    log_debug!("Failed to acquire read lock for ComponentVec<T>: {:?}", e);
                    return;
                }
            };

            let data_u = match component_vec_u.data.read() {
                Ok(data) => data,
                Err(e) => {
                    log_debug!("Failed to acquire read lock for ComponentVec<U>: {:?}", e);
                    return;
                }
            };

            let data_m = match component_vec_m.data.read() {
                Ok(data) => data,
                Err(e) => {
                    log_debug!("Failed to acquire read lock for ComponentVec<M>: {:?}", e);
                    return;
                }
            };

            let data_r = match component_vec_r.data.read() {
                Ok(data) => data,
                Err(e) => {
                    log_debug!("Failed to acquire read lock for ComponentVec<R>: {:?}", e);
                    return;
                }
            };

            for (&entity, component_t) in data_t.iter() {
                if let Some(component_u) = data_u.get(&entity) {
                    if let Some(component_m) = data_m.get(&entity) {
                        if let Some(component_r) = data_r.get(&entity) {
                            f(entity, component_t, component_u, component_m, component_r);
                        } else {
                            log_debug!("Entity {} is missing component R", entity);
                        }
                    } else {
                        log_debug!("Entity {} is missing component M", entity);
                    }
                } else {
                    log_debug!("Entity {} is missing component U", entity);
                }
            }
        } else {
            log_debug!("One or more components missing. Query4 aborted.");
        }
    }
    pub fn for_each<T: Component, U: Component, M: Component, R: Component>(
        &self,
        mut f: impl FnMut(Entity, &T, &U, &M, &R),
    ) {
        if let (Some(storage_t), Some(storage_u), Some(storage_m), Some(storage_r)) = (
            self.components.get(&TypeId::of::<U>()),
            self.components.get(&TypeId::of::<T>()),
            self.components.get(&TypeId::of::<M>()),
            self.components.get(&TypeId::of::<R>()),
        ) {
            let component_vec_t = storage_t
                .as_any()
                .downcast_ref::<ComponentVec<T>>()
                .expect("ComponentVec downcast failed");
            let component_vec_u = storage_u
                .as_any()
                .downcast_ref::<ComponentVec<U>>()
                .expect("ComponentVec downcast failed");
            let component_vec_m = storage_m
                .as_any()
                .downcast_ref::<ComponentVec<M>>()
                .expect("ComponentVec downcast failed");
            let component_vec_r = storage_r
                .as_any()
                .downcast_ref::<ComponentVec<R>>()
                .expect("ComponentVec downcast failed");

            let data_t = component_vec_t.data.read().unwrap();
            let data_u = component_vec_u.data.read().unwrap();
            let data_m = component_vec_m.data.read().unwrap();
            let data_r = component_vec_r.data.read().unwrap();
            for (&entity, component_t) in data_t.iter() {
                log_debug!("Fund entity: {:?}", entity);
                if let Some(component_u) = data_u.get(&entity) {
                    if let Some(component_m) = data_m.get(&entity) {
                        if let Some(component_r) = data_r.get(&entity) {
                            f(entity, component_t, component_u, component_m, component_r);
                        }
                    }
                }
            }
        }
    }
}
