use ::serde::{Deserialize, Serialize};
use nalgebra::Quaternion;

use bytemuck::{Pod, Zeroable};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Material {
    pub texture_id: Option<u64>,
    pub color: [f32; 4],
    pub shininess: Option<f32>,
    pub ambient_strength: Option<f32>,
    pub diffuse_strength: Option<f32>,
    pub specular_strength: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}

#[derive(Debug, Clone)]
pub struct Mesh<T> {
    pub id: u64,
    pub vertices: Vec<T>,
    pub indices: Vec<u16>,
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: Quaternion<f32>,
    pub scale: [f32; 3],
    pub velocity: [f32; 3],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentVec<T> {
    pub(crate) data: Vec<Option<(u32, T)>>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug, Serialize, Deserialize)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug, Serialize, Deserialize)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}
