use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}

#[derive(Debug, Clone)]
pub struct Mesh<VertexType> {
    pub vertices: Vec<VertexType>,
    pub indices: Vec<u16>,
}

#[repr(C)]
#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize, Pod, Zeroable)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [[f32; 4]; 4],
    pub scale: [f32; 3],
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

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, Pod, Zeroable)]
pub struct Uniforms {
    pub model: [[f32; 4]; 4],     // 64 bytes
    pub view_proj: [[f32; 4]; 4], // 64 bytes
    pub color: [f32; 4],          // 16 bytes
    pub light_color: [f32; 4],    // 16 bytes
    pub light_position: [f32; 4], // 16 bytes
    pub view_position: [f32; 4],  // 16 bytes
    pub ambient_strength: f32,    // 4 bytes
    pub diffuse_strength: f32,    // 4 bytes
    pub specular_strength: f32,   // 4 bytes
    pub shininess: f32,           // 4 bytes
}
