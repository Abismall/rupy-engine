use super::model::Mesh;
use crate::{
    core::{
        cache::{CacheKey, HasCacheKey, HashCache},
        error::AppError,
    },
    ecs::systems::render::BufferManager,
    graphics::vertex::VertexType,
};

pub struct MeshManager {
    pub meshes: HashCache<Mesh>,
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
        indices: Vec<u32>,
        material: Option<usize>,
        name: String,
    ) -> Result<CacheKey, AppError> {
        create_cached_mesh_with_buffers(
            device,
            &mut self.meshes,
            &mut buffer_manager.buffers,
            vertices,
            indices,
            material,
            name,
        )
    }
}

pub fn create_cached_mesh_with_buffers(
    device: &wgpu::Device,
    meshes: &mut HashCache<Mesh>,
    buffers: &mut HashCache<wgpu::Buffer>,
    vertices: Vec<VertexType>,
    indices: Vec<u32>,
    material: Option<usize>,
    name: String,
) -> Result<CacheKey, AppError> {
    let cache_key = Mesh::key(vec![&name, &material.unwrap_or(0).to_string()]);
    let mesh = Mesh::cache(Mesh::new(cache_key, material, vertices, indices), meshes);

    Mesh::cache_index_buffer(&mesh, device, buffers);
    Mesh::cache_vertex_buffer(&mesh, device, buffers);

    Ok(cache_key)
}
