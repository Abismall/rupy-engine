use wgpu::{Buffer, VertexBufferLayout};

pub trait IndexBuffer: bytemuck::Pod + bytemuck::Zeroable {
    fn index_buffer<T: bytemuck::Pod>(device: &wgpu::Device, indices: &[T]) -> Buffer;
}

pub trait UniformBuffer: bytemuck::Pod + bytemuck::Zeroable {
    fn uniform_buffer(&self, device: &wgpu::Device) -> Buffer;
    fn sized_uniform_buffer(&self, buffer_size: u64) -> Buffer
    where
        Self: Sized;
}
pub trait VertexBuffer: bytemuck::Pod + bytemuck::Zeroable {
    fn vertex_buffer_layout(&self) -> VertexBufferLayout;
    fn vertex_buffer(device: &wgpu::Device, vertices: &[Self]) -> Buffer
    where
        Self: Sized;
}
