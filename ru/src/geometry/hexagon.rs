use nalgebra::Matrix4;

use crate::material::vertex::Vertex;

pub struct Hexagon {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub model_matrix: Matrix4<f32>,
}
