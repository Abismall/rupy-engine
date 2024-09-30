use bytemuck::{Pod, Zeroable};

use crate::render::traits::Renderable;

use super::{HasTexture, VertexData};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct VertexTextured {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl VertexData for VertexTextured {
    fn position(&self) -> [f32; 3] {
        self.position
    }

    fn normal(&self) -> [f32; 3] {
        self.normal
    }

    fn color(&self) -> [f32; 3] {
        self.color
    }
}

impl HasTexture for VertexTextured {
    fn uv(&self) -> [f32; 2] {
        self.uv
    }
}

pub struct Mesh<V: VertexData + Pod> {
    vertices: Vec<V>,
    indices: Vec<u32>,
}

impl<V: VertexData + Pod> Mesh<V> {
    pub fn new(vertices: Vec<V>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn draw(&self) {
        println!("Drawing mesh with {} vertices.", self.vertices.len());
    }
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

impl VertexData for Vertex {
    fn position(&self) -> [f32; 3] {
        self.position
    }

    fn normal(&self) -> [f32; 3] {
        self.normal
    }

    fn color(&self) -> [f32; 3] {
        self.color
    }
}
