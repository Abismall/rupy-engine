use wgpu::{util::DeviceExt, Buffer, BufferUsages, Device};

use super::vertex::Vertex;

#[repr(C)]
#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, device: &Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        Mesh {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn get_vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }
}
