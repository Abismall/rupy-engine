use crate::geometry::Renderable;
use crate::material::vertex::Vertex;
use nalgebra::Matrix4;
#[derive(Debug, Clone)]
pub struct CubeStructure {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub model_matrix: Matrix4<f32>,
    pub is_textured: bool,
}

impl CubeStructure {
    pub fn new(size: f32, position: [f32; 3], color: [f32; 3], is_textured: bool) -> Self {
        let half = size / 2.0;
        let vertices = vec![
            Vertex {
                position: [-half, -half, half],
                color,
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [half, -half, half],
                color,
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [half, half, half],
                color,
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-half, half, half],
                color,
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-half, -half, -half],
                color,
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [half, -half, -half],
                color,
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [half, half, -half],
                color,
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-half, half, -half],
                color,
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-half, -half, -half],
                color,
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-half, -half, half],
                color,
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-half, half, half],
                color,
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-half, half, -half],
                color,
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [half, -half, -half],
                color,
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [half, -half, half],
                color,
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [half, half, half],
                color,
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [half, half, -half],
                color,
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-half, half, -half],
                color,
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-half, half, half],
                color,
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [half, half, half],
                color,
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [half, half, -half],
                color,
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-half, -half, -half],
                color,
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-half, -half, half],
                color,
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [half, -half, half],
                color,
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [half, -half, -half],
                color,
                normal: [0.0, -1.0, 0.0],
            },
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16,
            17, 18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
        ];

        let model_matrix = Matrix4::new_translation(&position.into());

        Self {
            vertices,
            indices,
            model_matrix,
            is_textured,
        }
    }
}

impl Renderable for CubeStructure {
    type VertexType = Vertex;

    fn update(&mut self) {}

    fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }

    fn vertices(&self) -> &[Self::VertexType] {
        &self.vertices
    }

    fn indices(&self) -> &[u32] {
        &self.indices
    }

    fn is_textured(&self) -> bool {
        self.is_textured
    }

    fn update_texture(&self, _queue: &wgpu::Queue) {
        if self.is_textured {}
    }
}

impl Default for CubeStructure {
    fn default() -> Self {
        Self::new(1.0, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], false)
    }
}
