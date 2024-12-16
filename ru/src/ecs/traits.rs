use std::ops::Range;

use crate::{
    core::cache::CacheKey,
    core::error::AppError,
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

pub trait RenderPassDraw<'a> {
    fn draw_model(
        &mut self,
        model: &Model,
        bind_groups: &[&'a wgpu::BindGroup],
        instances: &Option<Range<u32>>,
        buffer_manager: &BufferManager,
        mesh_manager: &MeshManager,
    );
    fn draw_models(
        &mut self,
        models: &Vec<Model>,
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
    fn get(&self, id: &CacheKey) -> Option<&R>;
    fn contains(&self, id: &CacheKey) -> bool;
    fn get_mut(&mut self, id: &CacheKey) -> Option<&mut R>;
    fn get_or_create<F>(&mut self, id: CacheKey, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>;
    fn put(&mut self, id: CacheKey, resource: R);
    fn remove(&mut self, id: &CacheKey);
}
