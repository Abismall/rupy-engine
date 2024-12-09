use nalgebra::{Matrix4, Vector3};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{
    ecs::components::model::Vertex2D,
    math::{
        self,
        spatial::{Height, Size2D, Width},
    },
    traits::rendering::Renderable,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Triangle {
    pub model_matrix: [[f32; 4]; 4],
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<u16>,
    pub size: Size2D,
    pub position: [f32; 3],
}

impl Triangle {
    pub fn new(width: u32, height: u32, position: Vector3<f32>) -> Self {
        let size = Size2D::new(width, height);
        let half_width = f32::from(size.width) / 2.0;
        let half_height = f32::from(size.height);

        let vertices = vec![
            Vertex2D {
                position: [-half_width, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex2D {
                position: [half_width, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex2D {
                position: [half_width, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex2D {
                position: [-half_width, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex2D {
                position: [0.0, half_height],
                color: [1.0, 0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
        ];

        let indices = vec![0, 1, 2, 0, 2, 3, 0, 1, 4, 1, 2, 4, 2, 3, 4, 3, 0, 4];

        Self {
            model_matrix: (Matrix4::new_translation(&position)
                * Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, 1.0, 1.0)))
            .into(),
            vertices,
            indices,
            size: Size2D::new(width, height),
            position: position.into(),
        }
    }
    pub fn set_tex_coords(&mut self, tex_coords: &[[f32; 2]; 5]) {
        for (i, vertex) in self.vertices.iter_mut().enumerate() {
            if i < tex_coords.len() {
                vertex.tex_coords = tex_coords[i];
            }
        }
    }

    pub fn reset_tex_coords_default(&mut self) {
        let default_tex_coords = [
            [0.0, 1.0], // Bottom-left
            [1.0, 1.0], // Bottom-right
            [1.0, 0.0], // Top-right
            [0.0, 0.0], // Top-left
            [0.5, 0.5], // Apex of the triangle
        ];
        self.set_tex_coords(&default_tex_coords);
    }
    pub fn set_size(&mut self, width: Width, height: Height) {
        let width: f32 = width.into();
        let height: f32 = height.into();
        self.size.width = math::spatial::Width(width as u32);
        self.size.height = math::spatial::Height(height as u32);

        let scale_x = width / 2.0;
        let scale_z = width / 2.0;
        let scale_y = height;

        self.model_matrix =
            (Matrix4::new_translation(&Vector3::<f32>::new(
                self.position[0],
                self.position[1],
                self.position[2],
            )) * Matrix4::new_nonuniform_scaling(&Vector3::new(scale_x, scale_y, scale_z)))
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

    pub fn get_size(&self) -> &Size2D {
        &self.size
    }

    pub fn vertices(&self) -> &[Vertex2D] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix.into()
    }
}

impl Renderable for Triangle {
    type VertexType = Vertex2D;

    fn update(&mut self) {}

    fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix.into()
    }

    fn vertices(&self) -> &[Self::VertexType] {
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
            self.size.width, self.size.height, self.position[0], self.position[1], self.position[2]
        )
    }
}
