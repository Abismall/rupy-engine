use nalgebra::{Matrix4, Vector3};

use crate::ecs::model::Vertex2D;

pub struct Rectangle {
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<u16>,
    pub model_matrix: Matrix4<f32>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, position: Vector3<f32>) -> Self {
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let vertices = vec![
            Vertex2D {
                position: [-half_width, -half_height],
                color: [1.0, 0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex2D {
                position: [half_width, -half_height],
                color: [1.0, 0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex2D {
                position: [half_width, half_height],
                color: [1.0, 0.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex2D {
                position: [-half_width, half_height],
                color: [1.0, 0.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self {
            vertices,
            indices,
            model_matrix: Matrix4::new_translation(&position),
        }
    }
}
