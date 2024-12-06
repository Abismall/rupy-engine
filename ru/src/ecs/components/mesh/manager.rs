use crate::{
    core::{cache::HashCache, error::AppError},
    ecs::{systems::render::BufferManager, traits::Cache},
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
        name: &str,
        cache_id: u64,
        device: &wgpu::Device,
        buffer_manager: &mut BufferManager,
        vertices: Vec<VertexType>,
        indices: Vec<u16>,
        material: usize,
    ) -> Result<u64, AppError> {
        buffer_manager.create_vertex_buffer(device, &vertices, cache_id);
        buffer_manager.create_index_buffer(device, &indices, cache_id);

        let mesh = Mesh {
            name: name.to_string(),
            num_elements: indices.len() as u32,
            material,
        };

        self.put(cache_id, mesh)?;

        Ok(cache_id)
    }
}
impl Cache<Mesh> for MeshManager {
    fn get(&self, id: u64) -> Option<&Mesh> {
        self.meshes.get(id)
    }

    fn contains(&self, id: u64) -> bool {
        self.meshes.contains(id)
    }

    fn get_mut(&mut self, id: u64) -> Option<&mut Mesh> {
        self.meshes.get_mut(id)
    }

    fn get_or_create<F>(&mut self, id: u64, create_fn: F) -> Result<&mut Mesh, AppError>
    where
        F: FnOnce() -> Result<Mesh, AppError>,
    {
        self.meshes.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: u64, resource: Mesh) -> Result<(), AppError> {
        self.meshes.put(id, resource)
    }

    fn remove(&mut self, id: u64) {
        self.meshes.remove(id);
    }
}
