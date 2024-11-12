use nalgebra::{Matrix4, Vector3};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{
    ecs::components::model::Vertex3D,
    math::spatial::{GetValue, Height, Size3D, Width},
    traits::rendering::Renderable,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Cube {
    pub model_matrix: [[f32; 4]; 4],
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u16>,
    pub size: Size3D,
    pub position: [f32; 3],
}

impl Cube {
    pub fn new(
        width: u32,
        height: u32,
        depth: u32,
        position: [f32; 3],
        tex_coords: Option<[[f32; 2]; 24]>,
    ) -> Self {
        let size = Size3D::new(width, height, depth);
        let half_width = f32::from(size.size_2d.width) / 2.0;
        let half_height = f32::from(size.size_2d.height);
        let half_depth = f32::from(size.depth) / 2.0;

        let mut vertices = vec![
            // Front face
            Vertex3D {
                position: [-half_width, -half_height, half_depth],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex3D {
                position: [half_width, -half_height, half_depth],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex3D {
                position: [half_width, half_height, half_depth],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex3D {
                position: [-half_width, half_height, half_depth],
                color: [1.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Back face
            Vertex3D {
                position: [-half_width, -half_height, -half_depth],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex3D {
                position: [half_width, -half_height, -half_depth],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex3D {
                position: [half_width, half_height, -half_depth],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex3D {
                position: [-half_width, half_height, -half_depth],
                color: [0.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Left face
            Vertex3D {
                position: [-half_width, -half_height, -half_depth],
                color: [0.0, 0.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex3D {
                position: [-half_width, -half_height, half_depth],
                color: [0.0, 0.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex3D {
                position: [-half_width, half_height, half_depth],
                color: [0.0, 0.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex3D {
                position: [-half_width, half_height, -half_depth],
                color: [0.0, 0.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Right face
            Vertex3D {
                position: [half_width, -half_height, -half_depth],
                color: [1.0, 1.0, 0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex3D {
                position: [half_width, -half_height, half_depth],
                color: [1.0, 1.0, 0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex3D {
                position: [half_width, half_height, half_depth],
                color: [1.0, 1.0, 0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex3D {
                position: [half_width, half_height, -half_depth],
                color: [1.0, 1.0, 0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Top face
            Vertex3D {
                position: [-half_width, half_height, half_depth],
                color: [0.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex3D {
                position: [half_width, half_height, half_depth],
                color: [0.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex3D {
                position: [half_width, half_height, -half_depth],
                color: [0.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex3D {
                position: [-half_width, half_height, -half_depth],
                color: [0.0, 1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
            // Bottom face
            Vertex3D {
                position: [-half_width, -half_height, half_depth],
                color: [1.0, 0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 0.0], // Bottom-left
            },
            Vertex3D {
                position: [half_width, -half_height, half_depth],
                color: [1.0, 0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 0.0], // Bottom-right
            },
            Vertex3D {
                position: [half_width, -half_height, -half_depth],
                color: [1.0, 0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0], // Top-right
            },
            Vertex3D {
                position: [-half_width, -half_height, -half_depth],
                color: [1.0, 0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0], // Top-left
            },
        ];

        if let Some(coords) = tex_coords {
            for (i, vertex) in vertices.iter_mut().enumerate() {
                if i < coords.len() {
                    vertex.tex_coords = coords[i];
                }
            }
        }

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
                    half_height,
                    height as f32,
                    half_height,
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

    pub fn set_tex_coords(&mut self, tex_coords: &Vec<[f32; 2]>) {
        for (i, vertex) in self.vertices.iter_mut().enumerate() {
            if i < tex_coords.len() {
                vertex.tex_coords = tex_coords[i];
            }
        }
    }
    pub fn reset_tex_coords_default(&mut self) {
        let default_tex_coords = vec![
            // Front face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // Back face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // Left face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // Right face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // Top face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // Bottom face
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];
        self.set_tex_coords(&default_tex_coords);
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

    pub fn vertices(&self) -> &Vec<Vertex3D> {
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
    type VertexType = Vertex3D;

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
