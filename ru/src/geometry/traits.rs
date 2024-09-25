use nalgebra::Matrix4;

use crate::material::vertex::{TexturedVertex, Vertex, VertexType};

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

pub trait Renderable {
    type VertexType; // Associated type for vertex type

    fn update(&mut self);

    fn model_matrix(&self) -> Matrix4<f32>;

    fn vertices(&self) -> &[Self::VertexType]; // Use associated type for vertices

    fn indices(&self) -> &[u32];
}
