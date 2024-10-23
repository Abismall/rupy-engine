use std::sync::Arc;

use wgpu::util::DeviceExt;

use super::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertex_buffer: Arc<wgpu::Buffer>,
    pub index_buffer: Arc<wgpu::Buffer>,
    pub index_count: u32,
}
impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer: vertex_buffer.into(),
            index_buffer: index_buffer.into(),
            index_count: indices.len() as u32,
        }
    }
    pub fn vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout<'static> {
        Vertex::vertex_buffer_layout()
    }
}
