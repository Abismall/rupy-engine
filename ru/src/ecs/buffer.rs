use std::collections::HashMap;

use crate::gpu::buffer::setup::BufferSetup;

use super::model::Vertex3D;
#[derive(Debug)]
pub struct BufferManager {
    vertex_buffers: HashMap<u64, wgpu::Buffer>,
    index_buffers: HashMap<u64, wgpu::Buffer>,
}

impl BufferManager {
    pub const CAPACITY: usize = 100;
    pub fn new() -> Self {
        Self {
            vertex_buffers: HashMap::with_capacity(Self::CAPACITY),
            index_buffers: HashMap::with_capacity(Self::CAPACITY),
        }
    }
    pub fn get_vertex_buffer(&self, id: u64) -> Option<&wgpu::Buffer> {
        self.vertex_buffers.get(&id)
    }

    pub fn get_index_buffer(&self, id: u64) -> Option<&wgpu::Buffer> {
        self.index_buffers.get(&id)
    }

    pub fn add_buffers(
        &mut self,
        id: u64,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
    ) {
        self.vertex_buffers.insert(id, vertex_buffer);
        self.index_buffers.insert(id, index_buffer);
    }
    pub fn ensure_buffers(
        &mut self,
        id: &u64,
        vertices: &[Vertex3D],
        indices: &[u16],
        device: &wgpu::Device,
    ) {
        if !self.vertex_buffers.contains_key(&id) {
            let vertex_buffer = BufferSetup::vertex_buffer(device, vertices);
            self.vertex_buffers.insert(*id, vertex_buffer);
        }

        if !self.index_buffers.contains_key(&id) {
            let index_buffer = BufferSetup::index_buffer(device, indices);
            self.index_buffers.insert(*id, index_buffer);
        }
    }
}
