use wgpu::{util::DeviceExt, Buffer, BufferUsages};

use crate::gpu::InstanceData;

use super::VertexBuffer;

pub struct BufferSetup;

impl BufferSetup {
    pub fn index_buffer<T: bytemuck::Pod>(device: &wgpu::Device, indices: &[T]) -> Buffer {
        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("IndexBufferInitDescriptor"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        };
        device.create_buffer_init(&desc)
    }
    pub fn instance_buffer(device: &wgpu::Device, instances: &[InstanceData]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
    }
    pub fn uniform_buffer(device: &wgpu::Device, buffer_size: u64) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("GlobalUniformBuffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn vertex_buffer<T: VertexBuffer>(device: &wgpu::Device, vertices: &[T]) -> Buffer {
        T::vertex_buffer(device, vertices)
    }
    pub fn vertex_buffer_description<'a, V: VertexBuffer>(
        vertices: &'a [V],
    ) -> wgpu::util::BufferInitDescriptor<'a> {
        wgpu::util::BufferInitDescriptor {
            label: Some("VertexBufferInitDescriptor"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        }
    }
}
