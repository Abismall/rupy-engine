use nalgebra::{Matrix4, Vector2, Vector3};
use serde::{Deserialize, Serialize};

use crate::ecs::model::{Vertex2D, Vertex3D};

#[derive(Clone, Copy, Debug)]
pub struct Cube3D {
    pub model_matrix: [[f32; 4]; 4],
    pub vertices: [Vertex3D; 24],
    pub indices: [u16; 36],
    pub size: [f32; 3],
    pub position: [f32; 3],
}
impl Cube3D {
    pub fn new(scale: f32, position: Vector3<f32>, tex_coords: Option<[[f32; 2]; 24]>) -> Self {
        let size = [1.0 * scale, 1.0 * scale, 1.0 * scale];
        let half_width = size[0] / 2.0;
        let half_height = size[1] / 2.0;
        let half_depth = size[2] / 2.0;

        let mut vertices = [
            // Front face
            Vertex3D {
                position: [
                    -half_width + position.x,
                    -half_height + position.y,
                    half_depth + position.z,
                ],
                color: [1.0, 0.0, 0.0, 1.0], // Red
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    -half_height + position.y,
                    half_depth + position.z,
                ],
                color: [0.0, 1.0, 0.0, 1.0], // Green
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    half_height + position.y,
                    half_depth + position.z,
                ],
                color: [0.0, 0.0, 1.0, 1.0], // Blue
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex3D {
                position: [
                    -half_width + position.x,
                    half_height + position.y,
                    half_depth + position.z,
                ],
                color: [1.0, 1.0, 0.0, 1.0], // Yellow
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            // Back face
            Vertex3D {
                position: [
                    -half_width + position.x,
                    -half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [1.0, 0.5, 0.0, 1.0], // Orange
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    -half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [0.5, 0.0, 1.0, 1.0], // Purple
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [0.0, 1.0, 1.0, 1.0], // Cyan
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex3D {
                position: [
                    -half_width + position.x,
                    half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [1.0, 0.0, 1.0, 1.0], // Magenta
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 1.0],
            },
            // Left face
            Vertex3D {
                position: [
                    -half_width + position.x,
                    -half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [0.5, 0.0, 0.5, 1.0], // Dark Purple
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex3D {
                position: [
                    -half_width + position.x,
                    -half_height + position.y,
                    half_depth + position.z,
                ],
                color: [0.5, 0.5, 0.0, 1.0], // Olive
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex3D {
                position: [
                    -half_width + position.x,
                    half_height + position.y,
                    half_depth + position.z,
                ],
                color: [0.5, 1.0, 0.5, 1.0], // Light Green
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex3D {
                position: [
                    -half_width + position.x,
                    half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [0.0, 0.5, 1.0, 1.0], // Light Blue
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            // Right face
            Vertex3D {
                position: [
                    half_width + position.x,
                    -half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [0.0, 0.0, 0.5, 1.0], // Dark Blue
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    -half_height + position.y,
                    half_depth + position.z,
                ],
                color: [0.5, 1.0, 0.0, 1.0], // Lime Green
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    half_height + position.y,
                    half_depth + position.z,
                ],
                color: [1.0, 0.5, 1.0, 1.0], // Light Magenta
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [0.5, 0.5, 1.0, 1.0], // Light Blue-Purple
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            // Top face
            Vertex3D {
                position: [
                    -half_width + position.x,
                    half_height + position.y,
                    half_depth + position.z,
                ],
                color: [0.0, 0.5, 0.5, 1.0], // Teal
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    half_height + position.y,
                    half_depth + position.z,
                ],
                color: [0.5, 1.0, 1.0, 1.0], // Light Cyan
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [1.0, 1.0, 0.5, 1.0], // Light Yellow
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex3D {
                position: [
                    -half_width + position.x,
                    half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [1.0, 0.5, 0.5, 1.0], // Salmon
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            // Bottom face
            Vertex3D {
                position: [
                    -half_width + position.x,
                    -half_height + position.y,
                    half_depth + position.z,
                ],
                color: [0.3, 0.0, 0.3, 1.0], // Deep Purple
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    -half_height + position.y,
                    half_depth + position.z,
                ],
                color: [0.3, 0.3, 0.0, 1.0], // Dark Olive
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex3D {
                position: [
                    half_width + position.x,
                    -half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [0.3, 0.3, 0.3, 1.0], // Gray
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex3D {
                position: [
                    -half_width + position.x,
                    -half_height + position.y,
                    -half_depth + position.z,
                ],
                color: [0.3, 0.0, 0.0, 1.0], // Dark Red
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
        ];
        if let Some(coords) = tex_coords {
            for (i, vertex) in vertices.iter_mut().enumerate() {
                vertex.tex_coords = coords[i];
            }
        }

        let indices = [
            0, 1, 2, 2, 3, 0, // Front
            4, 5, 6, 6, 7, 4, // Back
            8, 9, 10, 10, 11, 8, // Left
            12, 13, 14, 14, 15, 12, // Right
            16, 17, 18, 18, 19, 16, // Top
            20, 21, 22, 22, 23, 20, // Bottom
        ];

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

    pub fn set_position(&mut self, new_position: [f32; 3], scale: [f32; 3]) {
        self.position = new_position;
        let scale_x = scale[0] / 2.0;
        let scale_y = scale[1];
        let scale_z = scale[2] / 2.0;

        self.model_matrix =
            (Matrix4::new_translation(&Vector3::<f32>::new(
                self.position[0],
                self.position[1],
                self.position[2],
            )) * Matrix4::new_nonuniform_scaling(&Vector3::new(scale_x, scale_y, scale_z)))
            .into();
    }

    pub fn get_size(&self) -> &[f32; 3] {
        &self.size
    }

    pub fn vertices(&self) -> &[Vertex3D] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix.into()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cube2D {
    pub model_matrix: [[f32; 4]; 4],
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<u16>,
    pub size: [f32; 2],
    pub position: [f32; 2],
}

impl Cube2D {
    pub fn new(scale: f32, position: [f32; 2], tex_coords: Option<[[f32; 2]; 24]>) -> Self {
        let size = [1.0, 1.0];
        let half_width = size[0] / 2.0;
        let half_height = size[1];

        let mut vertices = vec![
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
        if let Some(coords) = tex_coords {
            for (i, vertex) in vertices.iter_mut().enumerate() {
                vertex.tex_coords = coords[i];
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

        Self {
            model_matrix: (Matrix4::new_translation(&Vector3::new(position[0], position[1], 0.0))
                * Matrix4::new_nonuniform_scaling(&Vector3::new(scale, scale, scale)))
            .into(),
            vertices,
            indices,
            position,
            size,
        }
    }

    pub fn set_position(&mut self, new_position: Vector2<f32>, scale: [f32; 3]) {
        self.position = new_position.into();
        let scale_x = scale[0] / 2.0;
        let scale_y = scale[1];
        let scale_z = scale[2] / 2.0;

        self.model_matrix =
            (Matrix4::new_translation(&Vector3::<f32>::new(
                self.position[0],
                self.position[1],
                self.position[2],
            )) * Matrix4::new_nonuniform_scaling(&Vector3::new(scale_x, scale_y, scale_z)))
            .into();
    }

    pub fn get_size(&self) -> [f32; 3] {
        [self.size[0], self.size[1], 0.0]
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
