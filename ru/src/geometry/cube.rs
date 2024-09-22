use nalgebra::Matrix4;

use crate::graphics::vertex::Vertex;

pub struct Cube {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub model_matrix: Matrix4<f32>,
}

// impl Cube {
//     pub fn new(size: f32, position: [f32; 3], color: [f32; 3]) -> Self {
//         let half = size / 2.0;
//         let positions = [
//             // Front face
//             [-half, -half, half],
//             [half, -half, half],
//             [half, half, half],
//             [-half, half, half],
//             // Back face
//             [-half, -half, -half],
//             [half, -half, -half],
//             [half, half, -half],
//             [-half, half, -half],
//         ];

//         let normals = [
//             [0.0, 0.0, 1.0],  // Front
//             [0.0, 0.0, -1.0], // Back
//             [-1.0, 0.0, 0.0], // Left
//             [1.0, 0.0, 0.0],  // Right
//             [0.0, 1.0, 0.0],  // Top
//             [0.0, -1.0, 0.0], // Bottom
//         ];

//         let mut vertices = Vec::new();
//         let mut indices = Vec::new();

//         // Front face
//         vertices.push(Vertex {
//             position: positions[0],
//             color,
//             normal: normals[0],
//         });
//         vertices.push(Vertex {
//             position: positions[1],
//             color,
//             normal: normals[0],
//         });
//         vertices.push(Vertex {
//             position: positions[2],
//             color,
//             normal: normals[0],
//         });
//         vertices.push(Vertex {
//             position: positions[3],
//             color,
//             normal: normals[0],
//         });
//         indices.extend_from_slice(&[0, 1, 2, 2, 3, 0]);

//         // Back face
//         let base_index = vertices.len() as u32;
//         vertices.push(Vertex {
//             position: positions[5],
//             color,
//             normal: normals[1],
//         });
//         vertices.push(Vertex {
//             position: positions[4],
//             color,
//             normal: normals[1],
//         });
//         vertices.push(Vertex {
//             position: positions[7],
//             color,
//             normal: normals[1],
//         });
//         vertices.push(Vertex {
//             position: positions[6],
//             color,
//             normal: normals[1],
//         });
//         indices.extend_from_slice(&[
//             base_index,
//             base_index + 1,
//             base_index + 2,
//             base_index + 2,
//             base_index + 3,
//             base_index,
//         ]);

//         // Left face
//         let base_index = vertices.len() as u32;
//         vertices.push(Vertex {
//             position: positions[4],
//             color,
//             normal: normals[2],
//         });
//         vertices.push(Vertex {
//             position: positions[0],
//             color,
//             normal: normals[2],
//         });
//         vertices.push(Vertex {
//             position: positions[3],
//             color,
//             normal: normals[2],
//         });
//         vertices.push(Vertex {
//             position: positions[7],
//             color,
//             normal: normals[2],
//         });
//         indices.extend_from_slice(&[
//             base_index,
//             base_index + 1,
//             base_index + 2,
//             base_index + 2,
//             base_index + 3,
//             base_index,
//         ]);

//         // Right face
//         let base_index = vertices.len() as u32;
//         vertices.push(Vertex {
//             position: positions[1],
//             color,
//             normal: normals[3],
//         });
//         vertices.push(Vertex {
//             position: positions[5],
//             color,
//             normal: normals[3],
//         });
//         vertices.push(Vertex {
//             position: positions[6],
//             color,
//             normal: normals[3],
//         });
//         vertices.push(Vertex {
//             position: positions[2],
//             color,
//             normal: normals[3],
//         });
//         indices.extend_from_slice(&[
//             base_index,
//             base_index + 1,
//             base_index + 2,
//             base_index + 2,
//             base_index + 3,
//             base_index,
//         ]);

//         // Top face
//         let base_index = vertices.len() as u32;
//         vertices.push(Vertex {
//             position: positions[3],
//             color,
//             normal: normals[4],
//         });
//         vertices.push(Vertex {
//             position: positions[2],
//             color,
//             normal: normals[4],
//         });
//         vertices.push(Vertex {
//             position: positions[6],
//             color,
//             normal: normals[4],
//         });
//         vertices.push(Vertex {
//             position: positions[7],
//             color,
//             normal: normals[4],
//         });
//         indices.extend_from_slice(&[
//             base_index,
//             base_index + 1,
//             base_index + 2,
//             base_index + 2,
//             base_index + 3,
//             base_index,
//         ]);

//         // Bottom face
//         let base_index = vertices.len() as u32;
//         vertices.push(Vertex {
//             position: positions[4],
//             color,
//             normal: normals[5],
//         });
//         vertices.push(Vertex {
//             position: positions[5],
//             color,
//             normal: normals[5],
//         });
//         vertices.push(Vertex {
//             position: positions[1],
//             color,
//             normal: normals[5],
//         });
//         vertices.push(Vertex {
//             position: positions[0],
//             color,
//             normal: normals[5],
//         });
//         indices.extend_from_slice(&[
//             base_index,
//             base_index + 1,
//             base_index + 2,
//             base_index + 2,
//             base_index + 3,
//             base_index,
//         ]);

//         let model_matrix = Matrix4::new_translation(&position);

//         Self {
//             vertices,
//             indices,
//             model_matrix,
//         }
//     }
// }
