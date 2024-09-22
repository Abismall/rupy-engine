use bytemuck::{Pod, Zeroable};
use nalgebra::Matrix4;

use crate::graphics::vertex::{TexturedVertex, Vertex};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
}

pub enum RenderableObject {
    Shaded {
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    Textured {
        vertices: Vec<TexturedVertex>,
        indices: Vec<u32>,
    },
}

impl RenderableObject {
    pub fn model_matrix(&self) -> Matrix4<f32> {
        Matrix4::identity()
    }

    pub fn render(&self) {
        match self {
            RenderableObject::Shaded { vertices, indices } => {
                println!(
                    "Rendering a shaded object with {} vertices.",
                    vertices.len()
                );
            }
            RenderableObject::Textured { vertices, indices } => {
                println!(
                    "Rendering a textured object with {} vertices.",
                    vertices.len()
                );
            }
        }
    }
}

pub trait Renderable<V> {
    fn update(&mut self);
    fn model_matrix(&self) -> Matrix4<f32>;
    fn vertices(&self) -> &[V];
    fn indices(&self) -> &[u32];
}
