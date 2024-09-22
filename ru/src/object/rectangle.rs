use nalgebra::Matrix4;

use super::{traits::Renderable, vertex::Vertex};
pub struct ShadedRectangle {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub model_matrix: Matrix4<f32>,
}

// impl ShadedRectangle {
//     pub fn new(size: [f32; 2], position: [f32; 3], color: [f32; 3]) -> Self {
//         let [width, height] = size;
//         let hw = width / 2.0;
//         let hh = height / 2.0;

//         let vertices = vec![
//             Vertex {
//                 position: [-hw, -hh, 0.0],
//                 color,
//                 normal: [0.0, 0.0, 1.0],
//             },
//             Vertex {
//                 position: [hw, -hh, 0.0],
//                 color,
//                 normal: [0.0, 0.0, 1.0],
//             },
//             Vertex {
//                 position: [hw, hh, 0.0],
//                 color,
//                 normal: [0.0, 0.0, 1.0],
//             },
//             Vertex {
//                 position: [-hw, hh, 0.0],
//                 color,
//                 normal: [0.0, 0.0, 1.0],
//             },
//         ];

//         let indices = vec![0, 1, 2, 2, 3, 0];

//         let model_matrix = Matrix4::from_translation(position.into());

//         Self {
//             vertices,
//             indices,
//             model_matrix,
//         }
//     }
// }
impl Renderable<Vertex> for ShadedRectangle {
    fn update(&mut self) {
        // Transformation or logic to update the object
    }

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
