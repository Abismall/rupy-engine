use crate::{
    core::{
        cache::{ComponentCacheKey, HashCache},
        error::AppError,
    },
    ecs::{components::IntoComponentCacheKey, systems::render::BufferManager, traits::Cache},
    graphics::model::VertexType,
};

use super::model::Mesh;

pub struct MeshManager {
    meshes: HashCache<Mesh>,
}

impl MeshManager {
    pub fn new() -> Self {
        Self {
            meshes: HashCache::new(),
        }
    }

    pub fn create_mesh(
        &mut self,
        device: &wgpu::Device,
        buffer_manager: &mut BufferManager,
        vertices: Vec<VertexType>,
        indices: Vec<u16>,
        material: usize,
    ) -> Result<ComponentCacheKey, AppError> {
        let mesh = Mesh {
            num_elements: indices.len() as u32,
            material,
        };
        let mesh_cache_id = mesh.into_cache_key();
        buffer_manager.create_vertex_buffer(device, &vertices, mesh_cache_id);
        buffer_manager.create_index_buffer(device, &indices, mesh_cache_id);
        self.put(mesh_cache_id, mesh)?;

        Ok(mesh_cache_id)
    }
}

impl Cache<Mesh> for MeshManager {
    fn get(&self, id: ComponentCacheKey) -> Option<&Mesh> {
        self.meshes.get(id)
    }

    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.meshes.contains(id)
    }

    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut Mesh> {
        self.meshes.get_mut(id)
    }

    fn get_or_create<F>(
        &mut self,
        id: ComponentCacheKey,
        create_fn: F,
    ) -> Result<&mut Mesh, AppError>
    where
        F: FnOnce() -> Result<Mesh, AppError>,
    {
        self.meshes.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: ComponentCacheKey, resource: Mesh) -> Result<(), AppError> {
        self.meshes.put(id, resource)
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.meshes.remove(id);
    }
}
