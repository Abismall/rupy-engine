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
pub struct Triangle {
    pub model_matrix: Matrix4<f32>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub size: Size2D,
    pub position: Vector3<f32>,
}

impl Triangle {
    pub fn new(width: u32, height: u32, position: Vector3<f32>) -> Self {
        let half_width = width as f32 / 2.0;
        let half_height = height as f32;

        let vertices = vec![
            Vertex {
                position: [-half_width, 0.0, -half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [half_width, 0.0, -half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [half_width, 0.0, half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-half_width, 0.0, half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.0, half_height, 0.0],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
        ];

        let indices = vec![0, 1, 2, 0, 2, 3, 0, 1, 4, 1, 2, 4, 2, 3, 4, 3, 0, 4];

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

        let scale_x = width / 2.0;
        let scale_z = width / 2.0;
        let scale_y = height;

        self.model_matrix = Matrix4::new_translation(&self.position)
            * Matrix4::new_nonuniform_scaling(&Vector3::new(scale_x, scale_y, scale_z));
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) {
        self.position = new_position;
        self.model_matrix = Matrix4::new_translation(&self.position)
            * Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, 1.0, 1.0));
    }

    pub fn get_size(&self) -> &Size2D {
        &self.size
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

impl Renderable for Triangle {
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

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ShadedTriangle with size: (Width: {}, Height: {}), Position: ({}, {}, {})",
            self.size.width, self.size.height, self.position.x, self.position.y, self.position.z
        )
    }
}
