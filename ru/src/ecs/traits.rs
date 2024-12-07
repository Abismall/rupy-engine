use std::ops::Range;

use crate::{
    core::{cache::ComponentCacheKey, error::AppError},
    ecs::components::{mesh::model::Mesh, model::model::Model},
};

use super::{components::mesh::manager::MeshManager, systems::render::BufferManager};

pub trait BufferCreator {
    fn create_buffer<T: bytemuck::Pod>(
        device: &wgpu::Device,
        data: &[T],
        usage: wgpu::BufferUsages,
        label: &str,
    ) -> wgpu::Buffer;
}

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
pub trait RenderPassDraw<'a> {
    fn draw_model(
        &mut self,
        model: &Model,
        bind_groups: &[&'a wgpu::BindGroup],
        instances: Option<Range<u32>>,
        buffer_manager: &BufferManager,
        mesh_manager: &MeshManager,
    );

    fn draw_mesh(
        &mut self,
        vertex_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        mesh: &Mesh,
        bind_groups: &[&wgpu::BindGroup],
        instances: Option<Range<u32>>,
    );

    fn draw_vertices(
        &mut self,
        bind_groups: &[&'a wgpu::BindGroup],
        vertices: Range<u32>,
        instances: Option<Range<u32>>,
    );
}
pub trait Cache<R> {
    fn get(&self, id: ComponentCacheKey) -> Option<&R>;
    fn contains(&self, id: ComponentCacheKey) -> bool;
    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut R>;
    fn get_or_create<F>(&mut self, id: ComponentCacheKey, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>;
    fn put(&mut self, id: ComponentCacheKey, resource: R) -> Result<(), AppError>;
    fn remove(&mut self, id: ComponentCacheKey);
}
