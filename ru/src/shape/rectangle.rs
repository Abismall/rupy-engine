use nalgebra::Matrix4;

use crate::{ecs::components::model::Vertex3D, traits::rendering::Renderable};

pub struct ShadedRectangle {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u16>,
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
impl Renderable for ShadedRectangle {
    type VertexType = Vertex3D;
    fn update(&mut self) {}

    fn model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }

    fn vertices(&self) -> &[Self::VertexType] {
        &self.vertices
    }

    fn indices(&self) -> &[u16] {
        &self.indices
    }
}
