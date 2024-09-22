use nalgebra::Matrix4;

use super::{
    traits::Renderable,
    vertex::{TexturedVertex, Vertex},
};

pub struct ShadedTriangle {
    model_matrix: Matrix4<f32>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl ShadedTriangle {
    pub fn new() -> Self {
        Self {
            model_matrix: Matrix4::identity(),
            vertices: vec![
                Vertex {
                    position: [0.0, 1.0, 0.0],
                    color: [1.0, 0.0, 0.0],  // Red color
                    normal: [0.0, 0.0, 1.0], // Normal facing out
                },
                Vertex {
                    position: [-1.0, -1.0, 0.0],
                    color: [0.0, 1.0, 0.0],  // Green color
                    normal: [0.0, 0.0, 1.0], // Normal facing out
                },
                Vertex {
                    position: [1.0, -1.0, 0.0],
                    color: [0.0, 0.0, 1.0],  // Blue color
                    normal: [0.0, 0.0, 1.0], // Normal facing out
                },
            ],
            indices: vec![0, 1, 2],
        }
    }

    // Additional methods for shading and lighting
}

impl Renderable<Vertex> for ShadedTriangle {
    fn update(&mut self) {
        // Apply transformation or updates (if needed)
    }

    fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }

    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[u32] {
        &self.indices
    }
}
pub struct TexturedTriangle {
    model: Matrix4<f32>,
    vertices: Vec<TexturedVertex>, // Simpler Vertex for texture mapping
    indices: Vec<u32>,
}

impl TexturedTriangle {
    pub fn new() -> Self {
        Self {
            model: Matrix4::identity(),
            vertices: vec![
                TexturedVertex {
                    position: [0.0, 1.0, 0.0],
                    tex_coords: [0.5, 0.0],
                },
                TexturedVertex {
                    position: [-1.0, -1.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                TexturedVertex {
                    position: [1.0, -1.0, 0.0],
                    tex_coords: [1.0, 1.0],
                },
            ],
            indices: vec![0, 1, 2],
        }
    }

    // Methods for textured rendering
}

impl Renderable<TexturedVertex> for TexturedTriangle {
    fn update(&mut self) {
        // Transformation or logic to update the object
    }

    fn model_matrix(&self) -> Matrix4<f32> {
        self.model
    }

    fn vertices(&self) -> &[TexturedVertex] {
        &self.vertices
    }

    fn indices(&self) -> &[u32] {
        &self.indices
    }
}
