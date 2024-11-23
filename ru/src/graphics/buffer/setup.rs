use wgpu::{util::DeviceExt, Buffer, BufferUsages};

use crate::graphics::data::InstanceData;

pub struct BufferSetup;

impl BufferSetup {
    pub fn index_buffer(device: &wgpu::Device, indices: &[u16]) -> Buffer {
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
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
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
}
