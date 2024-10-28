use bytemuck::{Pod, Zeroable};
use wgpu::{Buffer, Device, VertexBufferLayout};

use crate::traits::buffers::VertexBuffer;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn create_buffer(device: &Device, vertices: &[Self]) -> Buffer {
        Self::create_static_vertex_buffer(device, vertices)
    }

    pub fn layout<'a>() -> VertexBufferLayout<'a> {
        Vertex::vertex_buffer_layout()
    }
}

impl VertexBuffer for Vertex {
    fn vertex_buffer_layout<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 28,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
