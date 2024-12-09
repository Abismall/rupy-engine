use nalgebra::Matrix4;

use crate::ecs::components::model::Vertex3D;

pub struct Hexagon {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u32>,
    pub model_matrix: Matrix4<f32>,
}

// impl Hexagon {
//     pub fn new(radius: f32, position: [f32; 3], color: [f32; 3]) -> Self {
//         let mut vertices = Vec::new();
//         let mut indices = Vec::new();

//         for i in 0..6 {
//             let angle_deg = 60.0 * i as f32;
//             let angle_rad = angle_deg.to_radians();
//             let x = radius * angle_rad.cos();
//             let y = radius * angle_rad.sin();
//             vertices.push(Vertex {
//                 position: [x, y, 0.0],
//                 color,
//                 normal: [0.0, 0.0, 1.0],
//             });
//         }

//         vertices.push(Vertex {
//             position: [0.0, 0.0, 0.0],
//             color,
//             normal: [0.0, 0.0, 1.0],
//         });

//         for i in 0..6 {
//             indices.push(6); // Center vertex index
//             indices.push(i);
//             indices.push((i + 1) % 6);
//         }

//         let model_matrix = Matrix4::from_translation(position.into());

//         Self {
//             vertices,
//             indices,
//             model_matrix,
//         }
//     }
// }
