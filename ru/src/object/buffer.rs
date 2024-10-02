use std::collections::HashMap;

use wgpu::util::DeviceExt;

use super::Mesh;

use crate::math::Mat4;

pub struct BufferManager {
    vertex_buffers: HashMap<u64, wgpu::Buffer>,
    index_buffers: HashMap<u64, wgpu::Buffer>,
}

impl BufferManager {
    pub fn new() -> Self {
        Self {
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
        }
    }

    pub fn get_or_create_vertex_buffer(
        &mut self,
        device: &wgpu::Device,
        mesh: &Mesh,
        key: u64,
    ) -> &wgpu::Buffer {
        self.vertex_buffers.entry(key).or_insert_with(|| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&mesh.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        })
    }

    pub fn get_or_create_index_buffer(
        &mut self,
        device: &wgpu::Device,
        mesh: &Mesh,
        key: u64,
    ) -> &wgpu::Buffer {
        self.index_buffers.entry(key).or_insert_with(|| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            })
        })
    }
}
