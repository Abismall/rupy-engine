use crate::graphics::data::VertexStruct;
use std::hash::Hasher;
use std::hash::{DefaultHasher, Hash};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Mesh {
    pub id: u64,
    pub vertices: Vec<VertexStruct>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn new(vertices: Vec<VertexStruct>, indices: Vec<u16>) -> Self {
        let id = Mesh::calculate_id(&vertices, &indices);
        Self {
            id,
            vertices,
            indices,
        }
    }

    pub fn calculate_id(vertices: &[VertexStruct], indices: &[u16]) -> u64 {
        let mut hasher = DefaultHasher::new();
        vertices.hash(&mut hasher);
        indices.hash(&mut hasher);
        hasher.finish()
    }
}
