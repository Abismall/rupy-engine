pub mod buffer;
pub mod color;
pub mod manager;
pub mod object;
pub mod shape;
pub mod vertex;
use crate::{
    math::{Mat4, Vec3},
    render::traits::Renderable,
};
use bytemuck::Pod;
use vertex::VertexTextured;
pub struct ObjectHasher;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Debug)]
pub struct Mesh<V: VertexData> {
    vertices: Vec<V>,
    indices: Vec<u32>,
}

impl<V: VertexData + Pod> Renderable for Mesh<V> {
    fn vertex_buffer_data(&self) -> &[u8] {
        bytemuck::cast_slice(&self.vertices)
    }

    fn index_buffer_data(&self) -> &[u32] {
        bytemuck::cast_slice(&self.indices)
    }

    fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }

    fn is_textured(&self) -> bool {
        std::any::TypeId::of::<V>() == std::any::TypeId::of::<VertexTextured>()
    }

    fn update(&mut self) {}
}

pub trait VertexData {
    fn position(&self) -> [f32; 3];
    fn normal(&self) -> [f32; 3];
    fn color(&self) -> [f32; 3];
}

pub trait HasTexture: VertexData {
    fn uv(&self) -> [f32; 2];
}
