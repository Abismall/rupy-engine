use crate::scene::{
    model::{Components, EntityData, SceneData},
    Scene,
};
use crate::{
    camera::{Camera, Frustum},
    core::error::AppError,
    ecs::components::model::{Vertex2D, Vertex3D},
    log_debug, log_error,
    texture::TextureFile,
};
use crate::{
    gpu::{
        binding::{texture_bind_group, uniform_bind_group, BindGroupLayouts},
        buffer::{create_index_buffer, create_uniform_buffer, VertexBuffer},
    },
    scene::model::create_detailed_cube_scene,
};
use nalgebra::Vector3;
use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    sync::Arc,
};
use wgpu::{DepthStencilState, RenderPass, RenderPipeline, TextureViewDescriptor};

use super::{
    components::{
        model::{ComponentVec, Entity, Transform, Uniforms},
        Component,
    },
    materials::MaterialManager,
    pipelines::PipelineManager,
    shaders::ShaderManager,
    textures::TextureManager,
};

pub struct World {
    pub next_entity_id: u32,
    pub generations: Vec<u32>,
    pub entities: Vec<Entity>,
    pub entity_data: HashMap<Entity, EntityData>,
    pub components: HashMap<TypeId, Box<dyn Any>>,
    pub scenes: HashMap<String, Scene>,
    pub current_scene: Option<String>,
    pub bind_group_layouts: BindGroupLayouts,
    pub camera: Camera,
    _dst: wgpu::DepthStencilState,
    _textures: TextureManager,
    _shaders: ShaderManager,
    _pipelines: PipelineManager,
    _materials: MaterialManager,
}

impl World {
    pub fn new(
        _textures: TextureManager,
        shader_manager: ShaderManager,
        pipeline_manager: PipelineManager,
        material_manager: MaterialManager,
        bind_group_layouts: BindGroupLayouts,
        _dst: DepthStencilState,
    ) -> Self {
        let camera = Camera::default();

        Self {
            next_entity_id: 0,
            generations: Vec::new(),
            entities: Vec::new(),
            components: HashMap::new(),
            entity_data: HashMap::new(),

            scenes: HashMap::new(),
            current_scene: None,
            _dst,
            bind_group_layouts,

            camera,
            _textures,
            _shaders: shader_manager,
            _pipelines: pipeline_manager,
            _materials: material_manager,
        }
    }
    pub fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut RenderPass<'a>,
        pipeline_label: String,
    ) -> Result<(), AppError> {
        let vp_matrix = self.camera.view_projection_matrix();
        let frustum = Frustum::from_view_projection_matrix(&vp_matrix);

        if let Some(scene_name) = &self.current_scene {
            if let Some(scene) = self.scenes.get(scene_name) {
                let pipeline = match self.get_pipeline(&pipeline_label).ok_or_else(|| {
                    AppError::PipelineNotFoundError(format!(
                        "Pipeline '{}' not found",
                        pipeline_label
                    ))
                }) {
                    Ok(pipeline) => pipeline,
                    Err(e) => return Err(e),
                };
                pass.set_pipeline(&pipeline);
                for &entity in scene.get_entities() {
                    if let Some(entity_data) = self.get_entity_data(entity) {
                        if let Some(transform) = match &entity_data.components {
                            Components::Components2D { transform, .. }
                            | Components::Components3D { transform, .. } => transform,
                        } {
                            let position = Vector3::new(
                                transform.position[0],
                                transform.position[1],
                                transform.position[2],
                            );
                            let radius = 25.0;
                            if !frustum.contains_sphere(position, radius) {
                                continue;
                            }
                            if let Some(uniform_buffer) = &entity_data.uniform_buffer {
                                let model_matrix = transform.to_model_matrix();
                                let uniform_data = Uniforms::new(
                                    vp_matrix.into(),
                                    model_matrix.into(),
                                    [1.0, 1.0, 1.0, 1.0],
                                    [
                                        self.camera.position[0],
                                        self.camera.position[1],
                                        self.camera.position[2],
                                        1.0,
                                    ],
                                    [5.0, 5.0, 1.0, 1.0],
                                );
                                queue.write_buffer(
                                    uniform_buffer,
                                    0,
                                    bytemuck::bytes_of(&uniform_data),
                                );
                            }

                            if let Some(uniform_bind_group) = &entity_data.uniform_bind_group {
                                pass.set_bind_group(0, uniform_bind_group, &[]);
                            }

                            if let Some(texture_bind_group) = &entity_data.texture_bind_group {
                                pass.set_bind_group(1, texture_bind_group, &[]);
                            }

                            match &entity_data.components {
                                Components::Components2D {
                                    vertices, indices, ..
                                } => {
                                    let vertex_buffer =
                                        Vertex2D::create_vertex_buffer(device, vertices);
                                    let index_buffer = create_index_buffer(device, indices);
                                    pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                                    pass.set_index_buffer(
                                        index_buffer.slice(..),
                                        wgpu::IndexFormat::Uint16,
                                    );
                                    pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
                                }
                                Components::Components3D {
                                    vertices, indices, ..
                                } => {
                                    let vertex_buffer =
                                        Vertex3D::create_vertex_buffer(device, vertices);
                                    let index_buffer = create_index_buffer(device, indices);
                                    pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                                    pass.set_index_buffer(
                                        index_buffer.slice(..),
                                        wgpu::IndexFormat::Uint16,
                                    );
                                    pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
                                }
                            };
                        };
                    };
                }
            } else {
                log_error!("Current scene has not been set!");
            }
        }

        Ok(())
    }

    pub fn create_entity_bind_groups(
        &self,
        device: &wgpu::Device,
        bind_group_layouts: &BindGroupLayouts,
        uniform_buffer: &wgpu::Buffer,
        texture_file: Option<&TextureFile>,
    ) -> (wgpu::BindGroup, Option<wgpu::BindGroup>) {
        let uniform_bind_group =
            uniform_bind_group(device, uniform_buffer, &bind_group_layouts.uniform_layout);

        let texture_bind_group = texture_file.map(|texture| {
            texture_bind_group(
                device,
                &texture
                    .texture
                    .create_view(&TextureViewDescriptor::default()),
                &texture.sampler,
            )
        });

        (uniform_bind_group, texture_bind_group)
    }
    pub fn load_components(
        &mut self,
        device: &wgpu::Device,
        data: SceneData,
    ) -> Result<(), AppError> {
        let mut new_entities = Vec::new();

        for entity_data in data.entities.into_iter() {
            log_debug!("Entity data: {:?}", entity_data);
            let entity = self.create_entity();
            let mut entity_data_struct = entity_data;

            match &entity_data_struct.components {
                Components::Components2D {
                    transform,
                    material,
                    ..
                }
                | Components::Components3D {
                    transform,
                    material,
                    ..
                } => {
                    if let Some(transform) = transform {
                        let model_matrix = transform.to_model_matrix();
                        let uniform_data = Uniforms::new(
                            self.camera.view_projection_matrix().into(),
                            model_matrix.into(),
                            material.as_ref().map_or([1.0, 1.0, 1.0, 1.0], |m| m.color),
                            [
                                self.camera.position[0],
                                self.camera.position[0],
                                self.camera.position[0],
                                1.0,
                            ],
                            [5.0, 5.0, 5.0, 1.0],
                        );

                        let uniform_buffer = create_uniform_buffer(device, &uniform_data);
                        let texture_file = material
                            .as_ref()
                            .and_then(|mat| mat.texture_id)
                            .and_then(|texture_id| self.get_texture(texture_id));

                        let (uniform_bind_group, texture_bind_group) = self
                            .create_entity_bind_groups(
                                device,
                                &self.bind_group_layouts,
                                &uniform_buffer,
                                texture_file,
                            );

                        entity_data_struct.uniform_buffer = Some(uniform_buffer);
                        entity_data_struct.uniform_bind_group = Some(uniform_bind_group);
                        entity_data_struct.texture_bind_group = texture_bind_group;
                    }
                }
            }

            self.add_entity_data(entity, entity_data_struct);
            new_entities.push(entity);
        }

        if let Some(scene_name) = &self.current_scene {
            if let Some(scene) = self.scenes.get_mut(scene_name) {
                scene.add_entities(new_entities);
            } else {
                return Err(AppError::SceneNotFoundError(format!(
                    "Scene '{}' not found.",
                    scene_name
                )));
            }
        }

        Ok(())
    }

    pub fn load_scene(&mut self, device: &wgpu::Device, scene_name: &str) -> Result<(), AppError> {
        if let Err(e) = self.add_scene(scene_name) {
            Err(e)
        } else {
            let scene_data = create_detailed_cube_scene();
            let _ = self.set_current_scene(scene_name);
            self.load_components(device, scene_data)?;

            Ok(())
        }
    }

    pub fn update_uniforms_for_camera(&mut self) {
        let vp_matrix = self.camera.view_projection_matrix();

        let mut updated_uniforms = Vec::new();

        if let Some(scene_name) = &self.current_scene {
            if let Some(scene) = self.scenes.get(scene_name) {
                for &entity in scene.get_entities() {
                    if let Some(uniform) = self.get_component::<Uniforms>(entity) {
                        let mut new_uniform = *uniform;
                        new_uniform.view_proj = vp_matrix.into();
                        updated_uniforms.push((entity, new_uniform));
                    }
                }
            }
        }

        for (entity, updated_uniform) in updated_uniforms {
            self.add_uniform_component(entity, updated_uniform);
        }
    }
    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();

        if let Some(storage) = self.components.get_mut(&type_id) {
            let component_vec = storage
                .downcast_mut::<ComponentVec<T>>()
                .expect("ComponentVec<T> downcast failed");
            component_vec.insert(entity, component);
        } else {
            let mut component_vec = ComponentVec::<T>::new();
            component_vec.insert(entity, component);
            self.components.insert(type_id, Box::new(component_vec));
        }
    }
    pub fn add_entity_data(&mut self, entity: Entity, data: EntityData) {
        self.entity_data.insert(entity, data);
    }

    pub fn get_entity_data(&self, entity: Entity) -> Option<&EntityData> {
        self.entity_data.get(&entity)
    }

    pub fn get_entity_data_mut(&mut self, entity: Entity) -> Option<&mut EntityData> {
        self.entity_data.get_mut(&entity)
    }
    pub fn get_pipeline(&self, name: &str) -> Option<Arc<RenderPipeline>> {
        self._pipelines.get_pipeline(name)
    }
    pub fn get_texture(&self, key: u64) -> std::option::Option<&TextureFile> {
        self._textures.get(key)
    }
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .and_then(|storage| storage.downcast_ref::<ComponentVec<T>>())
            .and_then(|component_vec| component_vec.get(entity))
    }
    pub fn add_uniform_component(&mut self, entity: Entity, uniform: Uniforms) {
        let type_id = TypeId::of::<Uniforms>();

        if let Some(storage) = self.components.get_mut(&type_id) {
            let component_vec = storage
                .downcast_mut::<ComponentVec<Uniforms>>()
                .expect("ComponentVec<Uniforms> downcast failed");
            component_vec.insert(entity, uniform);
        } else {
            let mut component_vec = ComponentVec::<Uniforms>::new();
            component_vec.insert(entity, uniform);
            self.components.insert(type_id, Box::new(component_vec));
        }
    }
    pub fn update_uniforms(&mut self, entity: Entity, new_transform: Transform) {
        if let Some(uniform) = self.get_component::<Uniforms>(entity) {
            let mut updated_uniform = *uniform;
            updated_uniform.model = new_transform.to_model_matrix().into();
            self.add_uniform_component(entity, updated_uniform);
        }
    }
    pub fn query<T: Component>(&self, mut f: impl FnMut(Entity, &T)) {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.components.get(&type_id) {
            if let Some(component_vec) = storage.downcast_ref::<ComponentVec<T>>() {
                for (entity, component) in component_vec.iter() {
                    f(entity, component);
                }
            }
        }
    }

    pub fn query_two<T: Component, U: Component>(&self, mut f: impl FnMut(Entity, &T, &U)) {
        let type_id_t = TypeId::of::<T>();
        let type_id_u = TypeId::of::<U>();

        if let (Some(storage_t), Some(storage_u)) = (
            self.components.get(&type_id_t),
            self.components.get(&type_id_u),
        ) {
            if let (Some(component_vec_t), Some(component_vec_u)) = (
                storage_t.downcast_ref::<ComponentVec<T>>(),
                storage_u.downcast_ref::<ComponentVec<U>>(),
            ) {
                for (entity, component_t) in component_vec_t.iter() {
                    if let Some(component_u) = component_vec_u.get(entity) {
                        f(entity, component_t, component_u);
                    }
                }
            }
        }
    }
    pub fn query_four<T: Component, U: Component, M: Component, R: Component>(
        &self,
        mut f: impl FnMut(Entity, &T, &U, &M, &R),
    ) {
        let type_id_t = TypeId::of::<T>();
        let type_id_u = TypeId::of::<U>();
        let type_id_m = TypeId::of::<M>();
        let type_id_r = TypeId::of::<R>();

        if let (Some(storage_t), Some(storage_u), Some(storage_m), Some(storage_r)) = (
            self.components.get(&type_id_t),
            self.components.get(&type_id_u),
            self.components.get(&type_id_m),
            self.components.get(&type_id_r),
        ) {
            if let (
                Some(component_vec_t),
                Some(component_vec_u),
                Some(component_vec_m),
                Some(component_vec_r),
            ) = (
                storage_t.downcast_ref::<ComponentVec<T>>(),
                storage_u.downcast_ref::<ComponentVec<U>>(),
                storage_m.downcast_ref::<ComponentVec<M>>(),
                storage_r.downcast_ref::<ComponentVec<R>>(),
            ) {
                for (entity, component_t) in component_vec_t.iter() {
                    if let Some(component_u) = component_vec_u.get(entity) {
                        if let Some(component_m) = component_vec_m.get(entity) {
                            if let Some(component_r) = component_vec_r.get(entity) {
                                f(entity, component_t, component_u, component_m, component_r);
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn query_three<T: Component, U: Component, M: Component>(
        &self,
        mut f: impl FnMut(Entity, &T, &U, &M),
    ) {
        let type_id_t = TypeId::of::<T>();
        let type_id_u = TypeId::of::<U>();
        let type_id_m = TypeId::of::<M>();
        if let (Some(storage_t), Some(storage_u), Some(storage_m)) = (
            self.components.get(&type_id_t),
            self.components.get(&type_id_u),
            self.components.get(&type_id_m),
        ) {
            if let (Some(component_vec_t), Some(component_vec_u), Some(component_vec_m)) = (
                storage_t.downcast_ref::<ComponentVec<T>>(),
                storage_u.downcast_ref::<ComponentVec<U>>(),
                storage_m.downcast_ref::<ComponentVec<M>>(),
            ) {
                for (entity, component_t) in component_vec_t.iter() {
                    if let Some(component_u) = component_vec_u.get(entity) {
                        if let Some(component_m) = component_vec_m.get(entity) {
                            f(entity, component_t, component_u, component_m);
                        }
                    }
                }
            }
        }
    }
    pub fn add_scene(&mut self, scene_name: &str) -> Result<(), AppError> {
        if self.scenes.contains_key(scene_name) {
            return Err(AppError::CreateSceneError(format!(
                "Scene '{}' already exists.",
                scene_name
            )));
        }
        self.scenes
            .insert(scene_name.to_string(), Scene::new(scene_name));
        Ok(())
    }

    pub fn set_current_scene(&mut self, scene_name: &str) -> Result<(), AppError> {
        if self.scenes.contains_key(scene_name) {
            self.current_scene = Some(scene_name.to_string());
            Ok(())
        } else {
            Err(AppError::SceneNotFoundError(format!(
                "Scene '{}' not found.",
                scene_name
            )))
        }
    }

    pub fn get_current_scene(&self) -> Option<&Scene> {
        self.current_scene
            .as_ref()
            .and_then(|name| self.scenes.get(name))
    }

    pub fn get_current_scene_entities(&self) -> Option<&HashSet<Entity>> {
        self.get_current_scene().map(|scene| scene.get_entities())
    }

    pub fn add_entity_to_scene(&mut self, scene_name: &str, entity: Entity) -> Result<(), String> {
        if let Some(scene) = self.scenes.get_mut(scene_name) {
            scene.add_entity(entity);
            Ok(())
        } else {
            Err(format!("Scene '{}' not found.", scene_name))
        }
    }

    pub fn remove_entity_from_scene(
        &mut self,
        scene_name: &str,
        entity: Entity,
    ) -> Result<(), String> {
        if let Some(scene) = self.scenes.get_mut(scene_name) {
            scene.remove_entity(&entity);
            Ok(())
        } else {
            Err(format!("Scene '{}' not found.", scene_name))
        }
    }
    pub fn create_entity(&mut self) -> Entity {
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        if id as usize >= self.generations.len() {
            self.generations.push(0);
        }

        let generation = self.generations[id as usize];
        Entity { id, generation }
    }

    pub fn create_entity_in_current_scene(&mut self) -> Result<Entity, String> {
        let entity = self.create_entity();
        if let Some(scene_name) = self.current_scene.clone() {
            self.add_entity_to_scene(&scene_name, entity)?;
            Ok(entity)
        } else {
            Err("No active scene set.".to_string())
        }
    }

    pub fn query_in_current_scene<T: Component>(&self, mut f: impl FnMut(Entity, &T)) {
        if let Some(scene) = self.get_current_scene() {
            let type_id = TypeId::of::<T>();
            if let Some(storage) = self.components.get(&type_id) {
                if let Some(component_vec) = storage.downcast_ref::<ComponentVec<T>>() {
                    for (entity, component) in component_vec.iter() {
                        if scene.has_entity(&entity) {
                            f(entity, component);
                        }
                    }
                }
            }
        }
    }
}
