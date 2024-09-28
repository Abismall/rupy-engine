use bytemuck::{Pod, Zeroable};
use nalgebra::Vector3;
use std::{hash::Hasher, mem};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TexturedVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

impl TexturedVertex {
    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TexturedVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 2]>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
pub trait VertexType {
    fn position(&self) -> [f32; 3];
    fn normal(&self) -> [f32; 3];
    fn color(&self) -> [f32; 3];
    fn uv(&self) -> [f32; 2];
}

impl VertexType for Vertex {
    fn position(&self) -> [f32; 3] {
        self.position
    }

    fn normal(&self) -> [f32; 3] {
        self.normal
    }

    fn color(&self) -> [f32; 3] {
        self.color
    }

    fn uv(&self) -> [f32; 2] {
        [0.0, 0.0]
    }
}

impl VertexType for TexturedVertex {
    fn position(&self) -> [f32; 3] {
        self.position
    }

    fn normal(&self) -> [f32; 3] {
        self.normal
    }

    fn color(&self) -> [f32; 3] {
        [1.0, 1.0, 1.0]
    }

    fn uv(&self) -> [f32; 2] {
        self.uv
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Vector3Wrapper(Vector3<f32>);

impl PartialEq for Vector3Wrapper {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).norm() < 1e-6
    }
}

impl Eq for Vector3Wrapper {}

impl std::hash::Hash for Vector3Wrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x = (self.0.x * 1e6) as i32;
        let y = (self.0.y * 1e6) as i32;
        let z = (self.0.z * 1e6) as i32;
        x.hash(state);
        y.hash(state);
        z.hash(state);
    }
}
