use nalgebra::{Matrix4, Vector3};
use serde::{Deserialize, Serialize};

use crate::graphics::data::VertexStruct;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Triangle3D {
    pub model_matrix: [[f32; 4]; 4],
    pub vertices: Vec<VertexStruct>,
    pub indices: Vec<u16>,
    pub size: [f32; 3],
    pub position: [f32; 3],
}

impl Triangle3D {
    pub fn new(scale: f32, position: Vector3<f32>, tex_coords: Option<[[f32; 2]; 24]>) -> Self {
        let size = [1.0, 1.0, 1.0];
        let half_width = size[0] / 2.0;
        let half_height = size[1];

        let mut vertices = vec![
            VertexStruct {
                position: [-half_width, 0.0, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            VertexStruct {
                position: [half_width, 0.0, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            VertexStruct {
                position: [half_width, 0.0, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            VertexStruct {
                position: [-half_width, 0.0, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            VertexStruct {
                position: [0.0, half_height, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
        ];

        if let Some(coords) = tex_coords {
            for (i, vertex) in vertices.iter_mut().enumerate() {
                vertex.tex_coords = coords[i];
            }
        }
        let indices = vec![0, 1, 2, 0, 2, 3, 0, 1, 4, 1, 2, 4, 2, 3, 4, 3, 0, 4];

        Self {
            model_matrix: (Matrix4::new_translation(&position)
                * Matrix4::new_nonuniform_scaling(&Vector3::new(scale, scale, scale)))
            .into(),
            vertices,
            indices,
            size,
            position: position.into(),
        }
    }
}
