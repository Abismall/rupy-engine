use crate::geometry::Renderable;
use crate::material::vertex::Vertex;
use nalgebra::Matrix4;
#[derive(Debug, Clone)]
pub struct RectangleStructure {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub model_matrix: Matrix4<f32>,
    pub is_textured: bool,
}

impl RectangleStructure {
    pub fn new(size: [f32; 2], position: [f32; 3], color: [f32; 3], is_textured: bool) -> Self {
        let [width, height] = size;
        let hw = width / 2.0;
        let hh = height / 2.0;

        let vertices = vec![
            Vertex {
                position: [-hw, -hh, 0.0],
                color,
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [hw, -hh, 0.0],
                color,
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [hw, hh, 0.0],
                color,
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-hw, hh, 0.0],
                color,
                normal: [0.0, 0.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        let model_matrix = Matrix4::new_translation(&position.into());

        Self {
            vertices,
            indices,
            model_matrix,
            is_textured,
        }
    }
}

impl Renderable for RectangleStructure {
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

impl Default for RectangleStructure {
    fn default() -> Self {
        Self::new([1.0, 1.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], false)
    }
}
