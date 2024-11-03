use nalgebra::{Matrix4, Vector3};
use std::fmt;

use crate::{
    ecs::components::vertex::Vertex,
    math::{
        self,
        spatial::{Height, Size2D, Width},
    },
    traits::rendering::Renderable,
};

#[derive(Debug, Clone)]
pub struct Cube {
    pub model_matrix: Matrix4<f32>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub size: Size2D,
    pub position: Vector3<f32>,
}

impl Cube {
    pub fn new(width: u32, height: u32, depth: u32, position: Vector3<f32>) -> Self {
        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;
        let half_depth = depth as f32 / 2.0;

        let vertices = vec![
            // Front face
            Vertex {
                position: [-half_width, -half_height, half_depth],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [half_width, -half_height, half_depth],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [half_width, half_height, half_depth],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [-half_width, half_height, half_depth],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Back face
            Vertex {
                position: [-half_width, -half_height, -half_depth],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [half_width, -half_height, -half_depth],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [half_width, half_height, -half_depth],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            Vertex {
                position: [-half_width, half_height, -half_depth],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            // Left face
            Vertex {
                position: [-half_width, -half_height, -half_depth],
                color: [0.0, 0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [-half_width, -half_height, half_depth],
                color: [0.0, 0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [-half_width, half_height, half_depth],
                color: [0.0, 0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [-half_width, half_height, -half_depth],
                color: [0.0, 0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Right face
            Vertex {
                position: [half_width, -half_height, -half_depth],
                color: [1.0, 1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [half_width, -half_height, half_depth],
                color: [1.0, 1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [half_width, half_height, half_depth],
                color: [1.0, 1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            Vertex {
                position: [half_width, half_height, -half_depth],
                color: [1.0, 1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            // Top face
            Vertex {
                position: [-half_width, half_height, half_depth],
                color: [0.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            Vertex {
                position: [half_width, half_height, half_depth],
                color: [0.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [half_width, half_height, -half_depth],
                color: [0.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [-half_width, half_height, -half_depth],
                color: [0.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            // Bottom face
            Vertex {
                position: [-half_width, -half_height, half_depth],
                color: [1.0, 0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [half_width, -half_height, half_depth],
                color: [1.0, 0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [half_width, -half_height, -half_depth],
                color: [1.0, 0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [-half_width, -half_height, -half_depth],
                color: [1.0, 0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
        ];
        let indices = vec![
            0, 1, 2, 2, 3, 0, // Front
            4, 5, 6, 6, 7, 4, // Back
            8, 9, 10, 10, 11, 8, // Left
            12, 13, 14, 14, 15, 12, // Right
            16, 17, 18, 18, 19, 16, // Top
            20, 21, 22, 22, 23, 20, // Bottom
        ];

        let model_matrix = Matrix4::new_translation(&position)
            * Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, 1.0, 1.0));

        Self {
            model_matrix,
            vertices,
            indices,
            size: Size2D::new(width, height),
            position,
        }
    }

    pub fn set_size(&mut self, width: Width, height: Height) {
        let width: f32 = width.into();
        let height: f32 = height.into();
        self.size.width = math::spatial::Width(width as u32);
        self.size.height = math::spatial::Height(height as u32);
        let scale =
            Matrix4::new_nonuniform_scaling(&Vector3::new(width / 2.0, height / 2.0, width / 2.0));
        self.model_matrix = Matrix4::new_translation(&self.position) * scale;
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) {
        self.position = new_position;
        self.model_matrix = Matrix4::new_translation(&self.position)
            * Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, 1.0, 1.0));
    }

    pub fn get_size(&self) -> Size2D {
        self.size.clone()
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }
}

impl Renderable for Cube {
    type VertexType = Vertex;

    fn update(&mut self) {}

    fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }

    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[u16] {
        &self.indices
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ShadedCube with size: (Width: {}, Height: {}), Position: ({}, {}, {})",
            self.size.width, self.size.height, self.position.x, self.position.y, self.position.z
        )
    }
}
