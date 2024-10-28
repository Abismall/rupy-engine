use super::vertex::Vertex;

use crate::traits::buffers::VertexBuffer;
use crate::traits::rendering::Renderable;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, Device, IndexFormat, VertexBufferLayout};

pub const MESH_VERTEX_BUFFER_LABEL: &str = "Mesh Vertex Buffer";

pub const MESH_INDEX_BUFFER_LABEL: &str = "Mesh Index Buffer";

#[derive(Debug)]
pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn new(device: &Device, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(MESH_VERTEX_BUFFER_LABEL),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(MESH_INDEX_BUFFER_LABEL),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
        }
    }
    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }
}

impl Renderable for Mesh {
    fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    fn vertex_buffer_layout(&self) -> VertexBufferLayout<'static> {
        Vertex::vertex_buffer_layout()
    }

    fn index_format(&self) -> IndexFormat {
        IndexFormat::Uint16
    }

    fn vertex_count(&self) -> u32 {
        self.vertices.len() as u32
    }

    fn index_count(&self) -> u32 {
        self.indices.len() as u32
    }
}
