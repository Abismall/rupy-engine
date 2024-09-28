use wgpu::util::DeviceExt;

use std::sync::Arc;
use wgpu::{BindGroup, Buffer, RenderPipeline};

pub struct RenderCommand {
    pub pipeline: Arc<RenderPipeline>,
    pub uniform_data: Arc<BindGroup>,
    pub vertex_buffer: Arc<Buffer>,
    pub index_buffer: Arc<Buffer>,
    pub index_count: u32,
}

pub struct RenderCommandBuffer {
    pub commands: Vec<RenderCommand>,
}

impl RenderCommandBuffer {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn push(&mut self, command: RenderCommand) {
        self.commands.push(command);
    }
}

pub fn create_vertex_buffer<T: bytemuck::Pod>(
    device: &wgpu::Device,
    vertices: &[T],
) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn create_index_buffer(device: &wgpu::Device, indices: &[u32]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
    })
}
