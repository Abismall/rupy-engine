use super::{
    spatial::{Height, Size2D, Width},
    Renderable,
};
use crate::material::vertex::{TexturedVertex, Vertex};
use nalgebra::{Matrix4, Vector3};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TriangleStructure {
    pub model_matrix: Matrix4<f32>,
    pub vertices: Vec<Vertex>,
    pub textured_vertices: Vec<TexturedVertex>,
    pub indices: Vec<u32>,
    pub size: Size2D,
    pub position: Vector3<f32>,
    pub rotation: f32,
    pub scale: Vector3<f32>,
    pub is_textured: bool,
}

impl TriangleStructure {
    pub fn new(width: u32, height: u32, position: Vector3<f32>, is_textured: bool) -> Self {
        let half_width = width as f32 / 2.0;
        let half_height = height as f32;

        let vertices: Vec<Vertex> = vec![
            Vertex {
                position: [-half_width, 0.0, -half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [half_width, 0.0, -half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [half_width, 0.0, half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-half_width, 0.0, half_width],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.0, half_height, 0.0],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
        ];

        let textured_vertices: Vec<TexturedVertex> = vec![
            TexturedVertex {
                position: [-half_width, 0.0, -half_width],
                uv: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            TexturedVertex {
                position: [half_width, 0.0, -half_width],
                uv: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            TexturedVertex {
                position: [half_width, 0.0, half_width],
                uv: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            TexturedVertex {
                position: [-half_width, 0.0, half_width],
                uv: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            TexturedVertex {
                position: [0.0, half_height, 0.0],
                uv: [0.5, 0.5],
                normal: [0.0, 1.0, 0.0],
            },
        ];

        let indices = vec![
            0, 1, 2, // First triangle for the base
            0, 2, 3, // Second triangle for the base
            0, 1, 4, // Side 1
            1, 2, 4, // Side 2
            2, 3, 4, // Side 3
            3, 0, 4, // Side 4
        ];

        Self {
            model_matrix: Matrix4::new_translation(&position)
                * Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, 1.0, 1.0)),
            vertices,
            textured_vertices,
            indices,
            size: Size2D::new(width, height),
            position,
            rotation: 0.0,
            scale: Vector3::new(1.0, 1.0, 1.0),
            is_textured,
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

        self.scale = Vector3::new(scale_x, scale_y, scale_z);
        self.update_model_matrix();
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) {
        self.position = new_position;
        self.update_model_matrix();
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.update_model_matrix();
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
        self.update_model_matrix();
    }

    fn update_model_matrix(&mut self) {
        let translation = Matrix4::new_translation(&self.position);
        let rotation = Matrix4::new_rotation(Vector3::new(0.0, self.rotation, 0.0));
        let scaling = Matrix4::new_nonuniform_scaling(&self.scale);

        self.model_matrix = translation * rotation * scaling;
    }

    pub fn get_size(&self) -> Size2D {
        self.size.clone()
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }
}

impl Renderable for TriangleStructure {
    type VertexType = Vertex;

    fn update(&mut self) {}

    fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }

    fn vertices(&self) -> &[Self::VertexType] {
        if self.is_textured {
            panic!("Textured vertices require a different type!");
        } else {
            &self.vertices
        }
    }

    fn indices(&self) -> &[u32] {
        &self.indices
    }

    fn update_texture(&self, _queue: &wgpu::Queue) {
        if self.is_textured {}
    }

    fn is_textured(&self) -> bool {
        self.is_textured
    }
}

impl fmt::Display for TriangleStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Triangle with size: (Width: {}, Height: {}), Position: ({}, {}, {})",
            self.size.width(),
            self.size.height(),
            self.position.x,
            self.position.y,
            self.position.z
        )
    }
}

impl Default for TriangleStructure {
    fn default() -> Self {
        Self::new(1, 1, Vector3::new(0.0, 0.0, 0.0), false)
    }
}
