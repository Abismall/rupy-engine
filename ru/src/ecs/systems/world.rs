use std::{any::TypeId, collections::HashMap};

use crate::{
    core::error::AppError,
    ecs::{
        components::{
            storage::{ComponentStorage, ComponentVec},
            Component,
        },
        entities::models::Entity,
    },
    log_debug,
    scene::{load_scene_from_file_path, Scene},
};

use super::materials::MaterialManager;

pub struct World {
    pub next_entity: Entity,
    pub entities: Vec<Entity>,
    pub components: HashMap<TypeId, Box<dyn ComponentStorage>>,
    pub scenes: HashMap<String, Scene>,
    pub current_scene: Option<String>,
}
impl World {
    pub fn new() -> Self {
        Self {
            next_entity: 0,
            entities: Vec::new(),
            components: HashMap::new(),
            scenes: HashMap::new(),
            current_scene: None,
        }
    }

    pub fn create_scene(&mut self, scene_name: &str) {
        let scene = Scene::new(scene_name);
        self.scenes.insert(scene_name.to_string(), scene);
    }

    pub fn set_current_scene(&mut self, scene_name: &str) {
        if self.scenes.contains_key(scene_name) {
            self.current_scene = Some(scene_name.to_string());
        }
    }

    pub fn get_current_scene_entities(&self) -> Option<&Vec<Entity>> {
        self.current_scene
            .as_ref()
            .and_then(|name| self.scenes.get(name).map(|scene| &scene.entities))
    }
}
impl World {
    pub fn create_entity(&mut self, scene_name: &str) -> Entity {
        let entity = self.next_entity;
        self.next_entity += 1;
        self.entities.push(entity);

        if let Some(scene) = self.scenes.get_mut(scene_name) {
            scene.entities.push(entity);
        }
        entity
    }
    pub fn load_scene(
        &mut self,
        file_path: &str,
        material_manager: &mut MaterialManager,
    ) -> Result<(), AppError> {
        let scene = load_scene_from_file_path(self, material_manager, file_path)?;
        self.scenes.insert(file_path.into(), scene);
        self.set_current_scene(file_path);
        Ok(())
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();

        if let Some(storage) = self.components.get_mut(&type_id) {
            let component_vec = storage
                .as_any_mut()
                .downcast_mut::<ComponentVec<T>>()
                .expect("ComponentVec<T> downcast failed");
            component_vec
                .insert(entity, component)
                .expect("Insert failed");
        } else {
            let component_vec = ComponentVec::<T>::new();
            component_vec
                .insert(entity, component)
                .expect("Insert failed");
            self.components.insert(type_id, Box::new(component_vec));
        }
    }
    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        if let Some(storage) = self.components.get_mut(&TypeId::of::<T>()) {
            let component_vec = storage
                .as_any_mut()
                .downcast_mut::<ComponentVec<T>>()
                .expect("ComponentVec<T> downcast failed");
            component_vec.remove(entity);
        }
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
        if let Some(scene_name) = &self.current_scene {
            if let Some(scene) = self.scenes.get(scene_name) {
                if let Some(storage) = self.components.get(&TypeId::of::<T>()) {
                    let component_vec = storage
                        .as_any()
                        .downcast_ref::<ComponentVec<T>>()
                        .expect("ComponentVec downcast failed");

                    let data = component_vec.data.read().unwrap();
                    for &entity in &scene.entities {
                        if let Some(component) = data.get(&entity) {
                            f(entity, component);
                        }
                    }
                }
            }
        }
    }
    pub fn query2<T: Component, U: Component>(&self, mut f: impl FnMut(Entity, &T, &U)) {
        let storage_t = self.components.get(&TypeId::of::<T>());
        let storage_u = self.components.get(&TypeId::of::<U>());

        if let (Some(storage_t), Some(storage_u)) = (storage_t, storage_u) {
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

            for (&entity, component_t) in data_t.iter() {
                if let Some(component_u) = data_u.get(&entity) {
                    f(entity, component_t, component_u);
                } else {
                    log_debug!("Entity {} is missing component U", entity);
                }
            }
        } else {
            log_debug!("One or more components missing. Query4 aborted.");
        }
    }
    pub fn query3<T: Component, U: Component, M: Component>(
        &self,
        mut f: impl FnMut(Entity, &T, &U, &M),
    ) {
        let storage_t = self.components.get(&TypeId::of::<T>());
        let storage_u = self.components.get(&TypeId::of::<U>());
        let storage_m = self.components.get(&TypeId::of::<M>());

        if let (Some(storage_t), Some(storage_u), Some(storage_m)) =
            (storage_t, storage_u, storage_m)
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

            for (&entity, component_t) in data_t.iter() {
                if let Some(component_u) = data_u.get(&entity) {
                    if let Some(component_m) = data_m.get(&entity) {
                        f(entity, component_t, component_u, component_m);
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

    pub fn for_each<T: Component, U: Component, M: Component>(
        &self,
        mut f: impl FnMut(Entity, &T, &U, &M),
    ) {
        if let (Some(storage_t), Some(storage_u), Some(storage_m)) = (
            self.components.get(&TypeId::of::<T>()),
            self.components.get(&TypeId::of::<U>()),
            self.components.get(&TypeId::of::<M>()),
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

            let data_t = component_vec_t.data.read().unwrap();
            let data_u = component_vec_u.data.read().unwrap();
            let data_m = component_vec_m.data.read().unwrap();
            for (&entity, component_t) in data_t.iter() {
                if let Some(component_u) = data_u.get(&entity) {
                    if let Some(component_m) = data_m.get(&entity) {
                        f(entity, component_t, component_u, component_m);
                    }
                }
            }
        }
    }
}
