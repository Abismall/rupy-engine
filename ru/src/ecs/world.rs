use crate::core::cache::CacheId;
use crate::ecs::components::instance::model::Instance;
use crate::{core::error::AppError, log_debug};
use cgmath::Rotation3;
use pollster::block_on;
use std::{any::TypeId, collections::HashMap};

use super::components::model::model::Model;
use super::components::{ComponentStorage, ComponentVec};
use super::resources::{load_model, ResourceManager};
use super::traits::Cache;
use super::{
    entity::{Component, Entity},
    scene::Scene,
};

pub struct World {
    pub next_entity_id: u32,
    pub generations: Vec<u32>,
    pub entities: Vec<Entity>,
    pub scenes: HashMap<String, Scene>,
    pub scene: Option<String>,
    pub components: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            scene: None,
            generations: Vec::new(),
            entities: Vec::new(),
            scenes: HashMap::new(),
            components: HashMap::new(),
        }
    }
    pub async fn initialize_test_scene_components(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        resources: &mut ResourceManager,
    ) -> Result<(), AppError> {
        const NUM_INSTANCES_PER_ROW: u32 = 15;
        const NUM_INSTANCES_PER_COLUMN: u32 = 15;
        const SPACE_BETWEEN: f32 = 2.0;

        let instances = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|x| {
                (0..NUM_INSTANCES_PER_COLUMN).map(move |z| {
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                    let z = SPACE_BETWEEN * (z as f32);

                    let position = cgmath::Vector3 { x, y: 0.0, z };
                    let rotation = cgmath::Quaternion::from_axis_angle(
                        cgmath::Vector3::unit_z(),
                        cgmath::Deg(0.0),
                    );

                    Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();
        const NUM_WALL_ROWS: u32 = 15;
        const NUM_WALL_COLUMNS: u32 = 15;
        const WALL_SPACING: f32 = 2.0;
        let wall_instances = (0..NUM_WALL_ROWS)
            .flat_map(|x| {
                (0..NUM_WALL_COLUMNS).map(move |y| {
                    let x = WALL_SPACING * (x as f32 - NUM_WALL_ROWS as f32 / 2.0);
                    let y = WALL_SPACING * (y as f32);

                    let position = cgmath::Vector3 {
                        x: x,
                        y: y,
                        z: SPACE_BETWEEN * (NUM_INSTANCES_PER_COLUMN as f32 / 2.0 + 1.0),
                    };

                    let rotation = cgmath::Quaternion::from_axis_angle(
                        cgmath::Vector3::unit_y(),
                        cgmath::Deg(90.0),
                    );

                    Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();

        let combined_instances = wall_instances
            .into_iter()
            .chain(instances.into_iter())
            .collect::<Vec<_>>();

        let cube_entity = self.new_entity();
        let cube_entity_cache_id = &CacheId::from(cube_entity);
        resources.buffer_manager.create_instance_buffer(
            device,
            &combined_instances,
            cube_entity_cache_id.value(),
        );

        let (.., model) = self
            .load_model(
                cube_entity_cache_id.value(),
                "cube.obj",
                device,
                queue,
                layout,
                resources,
            )
            .await?;
        if let Ok(_) = resources
            .model_manager
            .put(cube_entity_cache_id.value(), model.clone())
        {
            self.add_component(cube_entity, model);
            self.add_component(cube_entity, combined_instances);
        }

        Ok(())
    }
}

impl World {
    pub fn new_entity(&mut self) -> Entity {
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        if id as usize >= self.generations.len() {
            self.generations.push(0);
        }

        let generation = self.generations[id as usize];
        let entity = Entity { id, generation };
        self.entities.push(entity);
        entity
    }

    pub fn add_component<T: Component + std::fmt::Debug>(&mut self, entity: Entity, component: T) {
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
}
impl World {
    pub async fn load_model<'a>(
        &'a mut self,
        cache_id: u64,
        file_name: &str,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        layout: &'a wgpu::BindGroupLayout,
        resources: &'a mut ResourceManager,
    ) -> Result<(u64, Model), AppError> {
        let mesh_manager = &mut resources.mesh_manager;
        let material_manager = &mut resources.material_manager;
        let buffer_manager = &mut resources.buffer_manager;
        let (.., model) = resources
            .model_manager
            .load_model_from_files(
                file_name,
                cache_id,
                move |name| {
                    let model = block_on(load_model(
                        name,
                        &cache_id,
                        device,
                        queue,
                        layout,
                        buffer_manager,
                    ))?;
                    Ok(model)
                },
                mesh_manager,
                material_manager,
            )
            .await?;

        Ok((cache_id, model))
    }
}

impl World {
    pub fn query<T: Component>(&self, mut f: impl FnMut(Entity, &T)) {
        let storage_t = self.components.get(&TypeId::of::<T>());

        if let Some(storage_t) = storage_t {
            let component_vec_t = match storage_t.as_any().downcast_ref::<ComponentVec<T>>() {
                Some(vec) => vec,
                None => {
                    log_debug!("Failed to downcast ComponentVec<T>.");
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

            for (&entity, component_t) in data_t.iter() {
                f(entity, component_t);
            }
        } else {
            log_debug!("One or more components missing. Query aborted.");
        }
    }
    pub fn query_two<T: Component, U: Component>(&self, mut f: impl FnMut(Entity, &T, &U)) {
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
                    log_debug!("Entity {:?} is missing component U", entity);
                }
            }
        } else {
            log_debug!("One or more components missing. Query aborted.");
        }
    }
    pub fn query_mut<T: Component, U: Component, M: Component>(
        &self,
        mut f: impl FnMut(Entity, &mut T, &mut U, &mut M),
    ) {
        let storage_t = self
            .components
            .get(&TypeId::of::<T>())
            .and_then(|storage| storage.as_any().downcast_ref::<ComponentVec<T>>());

        let storage_u = self
            .components
            .get(&TypeId::of::<U>())
            .and_then(|storage| storage.as_any().downcast_ref::<ComponentVec<U>>());

        let storage_m = self
            .components
            .get(&TypeId::of::<M>())
            .and_then(|storage| storage.as_any().downcast_ref::<ComponentVec<M>>());

        if let (Some(component_vec_t), Some(component_vec_u), Some(component_vec_m)) =
            (storage_t, storage_u, storage_m)
        {
            let mut data_t = component_vec_t.data.write().unwrap();
            let mut data_u = component_vec_u.data.write().unwrap();
            let mut data_m = component_vec_m.data.write().unwrap();

            for (&entity, component_t) in data_t.iter_mut() {
                if let (Some(component_u), Some(component_m)) =
                    (data_u.get_mut(&entity), data_m.get_mut(&entity))
                {
                    f(entity, component_t, component_u, component_m);
                }
            }
        }
    }
    pub fn query_scene<T: Component, U: Component, M: Component>(
        &self,
        scene_name: &str,
        mut f: impl FnMut(Entity, &T, &U, &M),
    ) -> Result<(), AppError> {
        let scene = self
            .scenes
            .get(scene_name)
            .ok_or(AppError::SceneNotFoundError(format!(
                "Scene '{}' not found.",
                scene_name
            )))?;

        let scene_entities = scene.get_entities();

        let storage_t = self.components.get(&TypeId::of::<T>());
        let storage_u = self.components.get(&TypeId::of::<U>());
        let storage_m = self.components.get(&TypeId::of::<M>());

        if let (Some(storage_t), Some(storage_u), Some(storage_m)) =
            (storage_t, storage_u, storage_m)
        {
            let component_vec_t = storage_t
                .as_any()
                .downcast_ref::<ComponentVec<T>>()
                .unwrap();
            let component_vec_u = storage_u
                .as_any()
                .downcast_ref::<ComponentVec<U>>()
                .unwrap();
            let component_vec_m = storage_m
                .as_any()
                .downcast_ref::<ComponentVec<M>>()
                .unwrap();

            let data_t = component_vec_t.data.read().unwrap();
            let data_u = component_vec_u.data.read().unwrap();
            let data_m = component_vec_m.data.read().unwrap();

            for &entity in scene_entities {
                if let (Some(component_t), Some(component_u), Some(component_m)) = (
                    data_t.get(&entity),
                    data_u.get(&entity),
                    data_m.get(&entity),
                ) {
                    f(entity, component_t, component_u, component_m);
                }
            }
        }

        Ok(())
    }
    pub fn query_current_scene<T: Component, U: Component, M: Component>(
        &self,
        f: impl FnMut(Entity, &T, &U, &M),
    ) -> Result<(), AppError> {
        if let Some(current_scene_name) = &self.scene {
            self.query_scene(current_scene_name, f)
        } else {
            Err(AppError::SceneNotFoundError(
                "No active scene set.".to_string(),
            ))
        }
    }
    pub fn query_current_scene_mut<T: Component, U: Component, M: Component>(
        &self,
        f: impl FnMut(Entity, &mut T, &mut U, &mut M),
    ) -> Result<(), AppError> {
        if let Some(current_scene_name) = &self.scene {
            self.query_scene_mut(current_scene_name, f)
        } else {
            Err(AppError::SceneNotFoundError(
                "No active scene set.".to_string(),
            ))
        }
    }
    pub fn query_scene_mut<T: Component, U: Component, M: Component>(
        &self,
        scene_name: &str,
        mut f: impl FnMut(Entity, &mut T, &mut U, &mut M),
    ) -> Result<(), AppError> {
        if let Some(scene) = self.get_scene(scene_name) {
            let scene_entities = scene.get_entities();

            let storage_t = self.components.get(&TypeId::of::<T>());
            let storage_u = self.components.get(&TypeId::of::<U>());
            let storage_m = self.components.get(&TypeId::of::<M>());

            if let (Some(storage_t), Some(storage_u), Some(storage_m)) =
                (storage_t, storage_u, storage_m)
            {
                let component_vec_t = storage_t
                    .as_any()
                    .downcast_ref::<ComponentVec<T>>()
                    .unwrap();
                let component_vec_u = storage_u
                    .as_any()
                    .downcast_ref::<ComponentVec<U>>()
                    .unwrap();
                let component_vec_m = storage_m
                    .as_any()
                    .downcast_ref::<ComponentVec<M>>()
                    .unwrap();

                let mut data_t = component_vec_t.data.write().unwrap();
                let mut data_u = component_vec_u.data.write().unwrap();
                let mut data_m = component_vec_m.data.write().unwrap();

                for &entity in scene_entities {
                    if let (Some(component_t), Some(component_u), Some(component_m)) = (
                        data_t.get_mut(&entity),
                        data_u.get_mut(&entity),
                        data_m.get_mut(&entity),
                    ) {
                        f(entity, component_t, component_u, component_m);
                    }
                }
            }
        }

        Ok(())
    }
    pub fn update_scene_component<T: Component>(
        &mut self,
        scene_name: &str,
        mut update_fn: impl FnMut(&mut T),
    ) -> Result<(), AppError> {
        let scene = self
            .scenes
            .get(scene_name)
            .ok_or(AppError::SceneNotFoundError(format!(
                "Scene '{}' not found.",
                scene_name
            )))?;

        let scene_entities = scene.get_entities();

        if let Some(storage) = self.components.get_mut(&TypeId::of::<T>()) {
            if let Some(component_vec) = storage.as_any_mut().downcast_mut::<ComponentVec<T>>() {
                let mut data = component_vec.data.write().unwrap();

                for &entity in scene_entities {
                    if let Some(component) = data.get_mut(&entity) {
                        update_fn(component);
                    }
                }
            }
        }

        Ok(())
    }
}

impl World {
    pub fn add_scene(&mut self, name: &str) {
        if !self.scenes.contains_key(name) {
            self.scenes.insert(name.to_string(), Scene::new(name));
        }
    }
    pub fn load_scene(&mut self, name: &str) -> Option<&mut Scene> {
        self.scenes.get_mut(name)
    }
    pub fn set_scene(&mut self, name: &str) -> Result<(), AppError> {
        if !self.scenes.contains_key(name) {
            Err(AppError::SceneNotFoundError(format!(
                "Scene '{}' not found.",
                name
            )))
        } else {
            self.scene = Some(name.to_string());
            Ok(())
        }
    }
    pub fn get_scene(&self, scene_name: &str) -> Option<&Scene> {
        self.scenes.get(scene_name)
    }
    pub fn get_scene_mut(&mut self, scene_name: &str) -> Option<&mut Scene> {
        self.scenes.get_mut(scene_name)
    }
    pub fn get_current_scene(&self) -> Option<&Scene> {
        self.scene.as_ref().and_then(|name| self.scenes.get(name))
    }
    pub fn get_current_scene_mut(&mut self) -> Option<&mut Scene> {
        self.scene
            .as_ref()
            .and_then(|name| self.scenes.get_mut(name))
    }
}
