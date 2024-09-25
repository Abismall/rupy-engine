use super::{
    spatial::{Height, Size2D, Width},
    traits::Renderable,
};
use crate::material::vertex::{TexturedVertex, Vertex};
use nalgebra::{Matrix4, Vector3};
use std::fmt;

#[derive(Debug)]
pub struct ShadedTriangleStructure {
    pub model_matrix: Matrix4<f32>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub size: Size2D,
    pub position: Vector3<f32>,
}

impl ShadedTriangleStructure {
    pub fn new(width: u32, height: u32, position: Vector3<f32>) -> Self {
        let half_width = width as f32 / 2.0;
        let half_height = height as f32; // height represents the vertical height of the pyramid

        // Define the vertices for the pyramid with a square base
        let vertices: Vec<Vertex> = vec![
            // Base of the pyramid (square)
            Vertex {
                position: [-half_width, 0.0, -half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            }, // Bottom left
            Vertex {
                position: [half_width, 0.0, -half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            }, // Bottom right
            Vertex {
                position: [half_width, 0.0, half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            }, // Top right
            Vertex {
                position: [-half_width, 0.0, half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            }, // Top left
            // Apex of the pyramid
            Vertex {
                position: [0.0, half_height, 0.0],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            }, // Apex
        ];

        // Define indices for the triangles (3 indices per triangle)
        let indices = vec![
            // Base (square made from two triangles)
            0, 1, 2, // First triangle for the base
            0, 2, 3, // Second triangle for the base
            // Sides (triangles connecting the base to the apex)
            0, 1, 4, // Side 1
            1, 2, 4, // Side 2
            2, 3, 4, // Side 3
            3, 0, 4, // Side 4
        ];

        Self {
            model_matrix: Matrix4::new_translation(&position)
                * Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, 1.0, 1.0)),
            vertices,
            indices,
            size: Size2D::new(width, height),
            position,
        }
    }

    pub fn set_size(&mut self, width: Width, height: Height) {
        let width: f32 = width.into();
        let height: f32 = height.into();
        self.size.set_width(width as u32);
        self.size.set_height(height as u32);

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

    pub fn get_size(&self) -> Size2D {
        self.size.clone()
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }
}

impl Renderable for ShadedTriangleStructure {
    type VertexType = Vertex;

    fn update(&mut self) {}

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

impl fmt::Display for ShadedTriangleStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ShadedTriangle with size: (Width: {}, Height: {}), Position: ({}, {}, {})",
            self.size.width(),
            self.size.height(),
            self.position.x,
            self.position.y,
            self.position.z
        )
    }
}
