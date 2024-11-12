use nalgebra::Matrix4;

use crate::{ecs::components::model::Vertex3D, traits::rendering::Renderable};

pub struct ShadedSphere {
    pub vertices: Vec<Vertex3D>,    // A Vec to hold vertices
    pub indices: Vec<u16>,          // A Vec to hold indices
    pub model_matrix: Matrix4<f32>, // Matrix4 to store the transformation matrix
}

// impl ShadedSphere {
//     pub fn new(
//         radius: f32,
//         latitude_bands: u32,
//         longitude_bands: u32,
//         position: [f32; 3],
//         color: [f32; 3],
//     ) -> Self {
//         let mut vertices = Vec::new();
//         let mut indices = Vec::new();

//         for lat_number in 0..=latitude_bands {
//             let theta = lat_number as f32 * std::f32::consts::PI / latitude_bands as f32;
//             let sin_theta = theta.sin();
//             let cos_theta = theta.cos();

//             for long_number in 0..=longitude_bands {
//                 let phi = long_number as f32 * 2.0 * std::f32::consts::PI / longitude_bands as f32;
//                 let sin_phi = phi.sin();
//                 let cos_phi = phi.cos();

//                 let x = cos_phi * sin_theta;
//                 let y = cos_theta;
//                 let z = sin_phi * sin_theta;

//                 let normal = [x, y, z];
//                 let position = [radius * x, radius * y, radius * z];

//                 vertices.push(Vertex {
//                     position,
//                     color,
//                     normal,
//                 });
//             }
//         }

//         for lat_number in 0..latitude_bands {
//             for long_number in 0..longitude_bands {
//                 let first = (lat_number * (longitude_bands + 1)) + long_number;
//                 let second = first + longitude_bands + 1;

//                 indices.extend_from_slice(&[
//                     first as u32,
//                     second as u32,
//                     (first + 1) as u32,
//                     second as u32,
//                     (second + 1) as u32,
//                     (first + 1) as u32,
//                 ]);
//             }
//         }

//         let model_matrix = Matrix4::from_translation(Vector3::from(position));

//         Self {
//             vertices,
//             indices,
//             model_matrix,
//         }
//     }
// }
impl Renderable for ShadedSphere {
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
