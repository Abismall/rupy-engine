use std::ops::Range;

use crate::{
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

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
pub trait RenderPassDraw<'a> {
    fn draw_model(
        &mut self,
        cache_id: u64,
        model: &Model,
        bind_groups: &[&'a wgpu::BindGroup],
        instances: Option<Range<u32>>,
        buffer_manager: &BufferManager,
        mesh_manager: &MeshManager,
    );

    fn draw_mesh(
        &mut self,
        cache_id: u64,
        mesh: &'a Mesh,
        bind_groups: &[&'a wgpu::BindGroup],
        instances: Option<Range<u32>>,
        buffer_manager: BufferManager,
    );

    fn draw_vertices(
        &mut self,
        bind_groups: &[&'a wgpu::BindGroup],
        vertices: Range<u32>,
        instances: Option<Range<u32>>,
    );
}
pub trait Cache<R> {
    fn get(&self, id: u64) -> Option<&R>;
    fn contains(&self, id: u64) -> bool;
    fn get_mut(&mut self, id: u64) -> Option<&mut R>;
    fn get_or_create<F>(&mut self, id: u64, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>;
    fn put(&mut self, id: u64, resource: R) -> Result<(), AppError>;
    fn remove(&mut self, id: u64);
}
