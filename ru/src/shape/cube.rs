use nalgebra::{Matrix4, Vector3};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{
    ecs::components::vertex::Vertex,
    math::spatial::{GetValue, Height, Size3D, Width},
    traits::rendering::Renderable,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cube {
    pub model_matrix: [[f32; 4]; 4],
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub size: Size3D,
    pub position: [f32; 3],
}

impl Cube {
    pub fn new(width: u32, height: u32, depth: u32, position: [f32; 3], scale_factor: f32) -> Self {
        let size = Size3D::new(width, height, depth);
        let half_width = scale_factor / 2.0;
        let half_height = scale_factor / 2.0;
        let half_depth = scale_factor / 2.0;

        let vertices = vec![
            // Front face
            Vertex {
                position: [-half_width, -half_height, half_depth],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [half_width, -half_height, half_depth],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [half_width, half_height, half_depth],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [-half_width, half_height, half_depth],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Back face
            Vertex {
                position: [-half_width, -half_height, -half_depth],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [half_width, -half_height, -half_depth],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [half_width, half_height, -half_depth],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [-half_width, half_height, -half_depth],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Left face
            Vertex {
                position: [-half_width, -half_height, -half_depth],
                color: [0.0, 0.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [-half_width, -half_height, half_depth],
                color: [0.0, 0.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [-half_width, half_height, half_depth],
                color: [0.0, 0.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [-half_width, half_height, -half_depth],
                color: [0.0, 0.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Right face
            Vertex {
                position: [half_width, -half_height, -half_depth],
                color: [1.0, 1.0, 0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [half_width, -half_height, half_depth],
                color: [1.0, 1.0, 0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [half_width, half_height, half_depth],
                color: [1.0, 1.0, 0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [half_width, half_height, -half_depth],
                color: [1.0, 1.0, 0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Top face
            Vertex {
                position: [-half_width, half_height, half_depth],
                color: [0.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [half_width, half_height, half_depth],
                color: [0.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [half_width, half_height, -half_depth],
                color: [0.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [-half_width, half_height, -half_depth],
                color: [0.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Bottom face
            Vertex {
                position: [-half_width, -half_height, half_depth],
                color: [1.0, 0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex {
                position: [half_width, -half_height, half_depth],
                color: [1.0, 0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex {
                position: [half_width, -half_height, -half_depth],
                color: [1.0, 0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex {
                position: [-half_width, -half_height, -half_depth],
                color: [1.0, 0.0, 1.0, 1.0],
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

        let model_matrix =
            (Matrix4::new_translation(&Vector3::<f32>::new(position[0], position[1], position[2]))
                * Matrix4::new_nonuniform_scaling(&Vector3::new(
                    scale_factor,
                    scale_factor,
                    scale_factor,
                )))
            .into();

        Self {
            model_matrix,
            vertices,
            indices,
            size,
            position,
        }
    }

    pub fn set_size(&mut self, width: Width, height: Height) {
        self.size.size_2d.width = width.clone();
        self.size.size_2d.height = height.clone();
        let half_width = width.get() as f32 / 2.0;

        let scale_x = half_width;
        let scale_z = half_width;
        let scale_y = height;

        self.model_matrix =
            (Matrix4::new_translation(&Vector3::<f32>::new(
                self.position[0],
                self.position[1],
                self.position[2],
            )) * Matrix4::new_nonuniform_scaling(&Vector3::new(scale_x, scale_y.into(), scale_z)))
            .into();
    }

    pub fn set_position(&mut self, new_position: [f32; 3], scale: [f32; 3]) {
        self.position = new_position;
        let scale_x = scale[0] / 2.0;
        let scale_z = scale[0] / 2.0;
        let scale_y = scale[1];
        self.model_matrix =
            (Matrix4::new_translation(&Vector3::<f32>::new(
                self.position[0],
                self.position[1],
                self.position[2],
            )) * Matrix4::new_nonuniform_scaling(&Vector3::new(scale_x, scale_y, scale_z)))
            .into();
    }

    pub fn get_size(&self) -> &Size3D {
        &self.size
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix.into()
    }
}

impl Renderable for Cube {
    type VertexType = Vertex;

    fn update(&mut self) {}

    fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix.into()
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
            self.size.size_2d.width,
            self.size.size_2d.height,
            self.position[0],
            self.position[1],
            self.position[2]
        )
    }
}
