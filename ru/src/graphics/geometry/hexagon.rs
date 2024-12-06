use nalgebra::{Matrix4, Vector3};

use crate::graphics::data::VertexStruct;

pub struct Hexagon {
    pub vertices: Vec<VertexStruct>,
    pub indices: Vec<u16>,
    pub model_matrix: Matrix4<f32>,
}

impl Hexagon {
    pub fn new(radius: f32, position: Vector3<f32>) -> Self {
        let angle_step = 2.0 * std::f32::consts::PI / 6.0;
        let mut vertices = vec![VertexStruct {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.5, 0.5],
        }];
        let mut indices = Vec::new();

        for i in 0..6 {
            let angle = i as f32 * angle_step;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            vertices.push(VertexStruct {
                position: [x, y, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [(x / (2.0 * radius) + 0.5), (y / (2.0 * radius) + 0.5)],
            });
        }

        for i in 1..=6 {
            indices.push(0);
            indices.push(i);
            indices.push(if i == 6 { 1 } else { i + 1 });
        }

        Self {
            vertices,
            indices,
            model_matrix: Matrix4::new_translation(&position),
        }
    }
}
