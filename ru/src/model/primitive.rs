use crate::prelude::Vec3;
use crate::{
    math::{Mat4, Vec2, Vec4},
    traits::{
        buffers::{IndexBuffer, UniformBuffer, VertexBuffer},
        rendering::{Color, Position, TextureMapping},
    },
};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct Primitive {
    pub position: Vec3, // 2D position
    pub color: Vec4,    // RGBA color
    pub uv: Vec2,       // UV coordinates (default [0.0, 0.0] for non-textured quads)
}

impl Position for Primitive {
    fn position(&self) -> Vec3 {
        self.position
    }
}

impl Color for Primitive {
    fn color(&self) -> Vec4 {
        self.color
    }
}

impl TextureMapping for Primitive {
    fn uv(&self) -> Vec2 {
        self.uv
    }
}

impl UniformBuffer for Primitive {}
impl IndexBuffer for Primitive {
    type IndexType = u16;
}

impl VertexBuffer for Primitive {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: (std::mem::size_of::<Vec3>()
                + std::mem::size_of::<Vec4>()
                + std::mem::size_of::<Vec2>()) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<Vec3>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<Vec3>() + std::mem::size_of::<Vec4>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
