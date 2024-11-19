use wgpu::RenderPass;

use crate::gpu::buffer::setup::BufferSetup;
use crate::gpu::{InstanceData, RenderBatch};
use crate::log_warning;
use crate::scene::model::create_detailed_cube_scene;

use crate::scene::{model::SceneData, Scene};
use crate::{camera::frustum::Frustum, core::error::AppError, texture::manager::TextureManager};

use std::sync::Arc;
use std::{collections::HashMap, time::Instant};

use super::buffer::BufferManager;

use super::model::{Material, Mesh};
use super::storage::ComponentManager;
use super::{
    components::{Component, Components},
    materials::manager::MaterialManager,
    model::{ComponentVec, Entity, Transform, Vertex3D},
};
pub struct World {
    pub next_entity_id: u32,
    pub generations: Vec<u32>,
    pub entities: Vec<Entity>,

    pub scenes: HashMap<String, Scene>,
    pub current_scene: Option<String>,
    _start_time: Instant,
    texture_manager: TextureManager,
    material_manager: MaterialManager,
    buffer_manager: BufferManager,
    component_manager: ComponentManager,
}

impl World {
    pub fn new(
        texture_manager: TextureManager,
        material_manager: MaterialManager,
        component_manager: ComponentManager,
        buffer_manager: BufferManager,
    ) -> Self {
        Self {
            next_entity_id: 0,
            generations: Vec::new(),
            entities: Vec::new(),
            component_manager,
            buffer_manager,
            scenes: HashMap::new(),
            current_scene: None,
            texture_manager,
            material_manager,
            _start_time: Instant::now(),
        }
    }
    pub fn create_entity(&mut self) -> Entity {
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        if id as usize >= self.generations.len() {
            self.generations.push(id);
        }

        let generation = self.generations[id as usize];
        let entity = Entity { id, generation };
        self.entities.push(entity);
        entity
    }

    pub fn destroy_entity<C: Component>(&mut self, entity: Entity) {
        if let Some(index) = self.entities.iter().position(|&e| e == entity) {
            self.entities.swap_remove(index);
        }
        self.generations[entity.id as usize] += 1;
        self.component_manager
            .iter_mut::<C>()
            .for_each(|(_, storage)| {
                if let Some(storage) = storage.downcast_mut::<ComponentVec<C>>() {
                    storage.remove(entity);
                }
            });
    }

    pub fn insert_component<C: Component + std::fmt::Debug>(
        &mut self,
        entity: Entity,
        component: C,
    ) {
        self.component_manager.insert_component(entity, component);
    }

    pub fn remove_component<C: Component>(&mut self, entity: Entity) {
        self.component_manager.remove_component::<C>(entity);
    }

    pub fn query_mut<C: Component>(&mut self, f: impl FnMut(Entity, &mut C)) {
        self.component_manager.query_mut(f);
    }

    pub fn query_two<C1: Component, C2: Component>(&self, f: impl FnMut(Entity, &C1, &C2)) {
        self.component_manager.query_two(f);
    }

    pub fn render<'pass>(
        &'pass mut self,
        pass: &mut wgpu::RenderPass<'pass>,
        device: &wgpu::Device,
        frustum: &Frustum,
    ) -> Result<(), AppError> {
        let scene_entities = match self.get_current_scene() {
            Some(scene) => scene.get_entities_as_vec_ref(),
            None => return Ok(()),
        };

        // Step 1: Collect all data that requires immutable access
        let mut meshes_to_render = Vec::new();
        for &entity in scene_entities {
            if let (Some(transform), Some(material), Some(mesh)) = (
                self.component_manager
                    .get_component_from_map::<Transform>(entity),
                self.component_manager
                    .get_component_from_map::<Material>(entity),
                self.component_manager
                    .get_component_from_map::<Mesh<Vertex3D>>(entity),
            ) {
                let bounding_sphere = mesh.calculate_bounding_sphere();
                let world_center = [
                    transform.position[0] + bounding_sphere.center[0],
                    transform.position[1] + bounding_sphere.center[1],
                    transform.position[2] + bounding_sphere.center[2],
                ];

                if frustum.contains_sphere(world_center.into(), bounding_sphere.radius) {
                    meshes_to_render.push((mesh, material, transform));
                }
            }
        }

        // Step 2: Perform all mutable operations
        for (mesh, material, transform) in meshes_to_render {
            self.buffer_manager
                .ensure_buffers(&mesh.id, &mesh.vertices, &mesh.indices, device);

            let vertex_buffer = match self.buffer_manager.get_vertex_buffer(mesh.id) {
                Some(buffer) => buffer,
                None => {
                    log_warning!("Vertex buffer not found for mesh ID: {}", mesh.id);
                    continue;
                }
            };

            let index_buffer = match self.buffer_manager.get_index_buffer(mesh.id) {
                Some(buffer) => buffer,
                None => {
                    log_warning!("Index buffer not found for mesh ID: {}", mesh.id);
                    continue;
                }
            };

            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            if let Some(texture_id) = material.texture_id {
                if let Some(bind_group) = self.texture_manager.get_bind_group(texture_id) {
                    pass.set_bind_group(1, &bind_group, &[]);
                } else {
                    log_warning!("Bind group not found for texture ID: {}", texture_id);
                }
            }

            let index_count = index_buffer.size() as u32 / std::mem::size_of::<u16>() as u32;
            pass.draw_indexed(0..index_count, 0, 0..1);
        }

        Ok(())
    }

    pub fn update_components(&mut self, delta_time: f32) {
        self.query_mut::<Transform>(|_, transform| {
            transform.update(delta_time);
        });
    }

    pub fn load_scene(&mut self, device: &wgpu::Device, name: &str) -> Result<(), AppError> {
        if let Err(e) = self.add_scene(name) {
            return Err(e);
        }

        if let Err(e) = self.set_current_scene(name) {
            return Err(e);
        }

        if let Err(e) = self.load_components(device, create_detailed_cube_scene()) {
            return Err(e);
        }

        Ok(())
    }

    pub fn add_scene(&mut self, name: &str) -> Result<(), AppError> {
        if self.scenes.contains_key(name) {
            return Err(AppError::CreateSceneError(format!(
                "Scene '{}' already exists.",
                name
            )));
        }
        self.scenes.insert(name.to_string(), Scene::new(name));
        Ok(())
    }

    pub fn set_current_scene(&mut self, name: &str) -> Result<(), AppError> {
        if self.scenes.contains_key(name) {
            self.current_scene = Some(name.to_string());
            Ok(())
        } else {
            Err(AppError::SceneNotFoundError(format!(
                "Scene '{}' not found.",
                name
            )))
        }
    }

    pub fn get_current_scene(&self) -> Option<&Scene> {
        self.current_scene
            .as_ref()
            .and_then(|name| self.scenes.get(name))
    }
    pub fn get_current_scene_mut(&mut self) -> Option<&Scene> {
        self.current_scene
            .as_mut()
            .and_then(|name| self.scenes.get(name))
    }

    pub fn load_components(
        &mut self,
        device: &wgpu::Device,
        data: SceneData,
    ) -> Result<(), AppError> {
        let mut new_entities = Vec::with_capacity(data.entities.len());
        for entity_data in data.entities.into_iter() {
            let entity = self.create_entity();
            new_entities.push(entity);
            match entity_data.components {
                Components::Components2D {
                    transform,
                    material,

                    mesh,
                } => {
                    let mesh = Mesh {
                        id: mesh.id,
                        vertices: mesh.vertices.into_iter().map(Vertex3D::from).collect(),
                        indices: mesh.indices,
                    };
                    self.process_components(device, entity, transform, material, Some(mesh))?;
                }
                Components::Components3D {
                    transform,
                    material,

                    mesh,
                } => {
                    self.process_components(device, entity, transform, material, Some(mesh))?;
                }
            }
        }

        if let Some(name) = &self.current_scene {
            if let Some(scene) = self.scenes.get_mut(name) {
                new_entities.into_iter().for_each(|e| scene.add_entity(e));
            } else {
                return Err(AppError::SceneNotFoundError(format!(
                    "Scene '{}' not found.",
                    name
                )));
            }
        }

        Ok(())
    }
    fn process_components(
        &mut self,
        device: &wgpu::Device,
        entity: Entity,
        transform: Option<Transform>,
        material: Option<Material>,
        mesh: Option<Mesh<Vertex3D>>,
    ) -> Result<(), AppError> {
        if let Some(transform) = transform {
            self.insert_component(entity, transform);
        }
        if let Some(mesh) = mesh {
            self.insert_component(entity, mesh);
        }

        if let Some(material) = material {
            self.insert_component(entity, material);
            if let Some(texture_id) = material.texture_id {
                self.texture_manager
                    .get_or_create_bind_group(device, texture_id)
                    .map_err(|_| AppError::MissingTexture)?;
            }
        }

        Ok(())
    }
    pub fn get_vertex_buffer(&self, id: u64) -> Option<&wgpu::Buffer> {
        self.buffer_manager.get_vertex_buffer(id)
    }

    pub fn get_index_buffer(&self, id: u64) -> Option<&wgpu::Buffer> {
        self.buffer_manager.get_index_buffer(id)
    }
}
