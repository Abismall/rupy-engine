pub mod instance;
pub mod material;
pub mod mesh;
pub mod model;
pub mod transform;
use instance::manager::InstanceManager;
use material::manager::MaterialManager;
use mesh::manager::MeshManager;
use model::manager::ModelManager;

use super::{entity::Entity, systems::render::BufferManager};
use crate::{
    core::error::AppError,
    graphics::{
        binding::BindGroupManager, pipelines::manager::PipelineManager,
        shaders::manager::ShaderManager, textures::manager::TextureManager,
    },
};
use std::{any::Any, collections::HashMap, sync::RwLock};
pub struct ResourceContext {
    pub material_manager: MaterialManager,
    pub texture_manager: TextureManager,
    pub buffer_manager: BufferManager,
    pub model_manager: ModelManager,
    pub pipeline_manager: PipelineManager,
    pub mesh_manager: MeshManager,
    pub instance_manager: InstanceManager,
    pub bind_group_manager: BindGroupManager,
    pub shader_manager: ShaderManager,
}
pub trait VertexData {
    fn vertices(&self) -> Vec<crate::graphics::vertex::VertexType>;
}

pub trait IndexData {
    fn indices(&self) -> Vec<u32>;
}

pub trait Component: Any + Send + Sync + Clone {}

impl<T> Component for T where T: Any + Send + Sync + Clone {}
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
