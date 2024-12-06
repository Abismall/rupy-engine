use bytemuck::{Pod, Zeroable};
use cgmath::SquareMatrix;

use crate::{
    camera::{projection::Projection, Camera},
    ecs::components::model::model::ModelVertex,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
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
impl Uniforms {
    pub fn new(
        view_projection: [[f32; 4]; 4],
        model: [[f32; 4]; 4],
        color: [f32; 4],
        view_position: [f32; 4],
        light_position: [f32; 4],
    ) -> Self {
        Self {
            view_proj: view_projection,
            model,
            color,
            light_position,
            light_color: [1.0, 1.0, 1.0, 1.0],
            view_position,
            ambient_strength: 0.8,
            diffuse_strength: 12.0,
            specular_strength: 12.0,
            shininess: 32.0,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexColor {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexTexture {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
impl VertexColor {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
impl VertexTexture {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<VertexTexture>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
pub enum VertexType {
    Textured(VertexTexture),
    Colored(VertexColor),
    Modeled(ModelVertex),
}

impl VertexType {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            VertexType::Textured(v) => bytemuck::cast_slice(std::slice::from_ref(v)),
            VertexType::Colored(v) => bytemuck::cast_slice(std::slice::from_ref(v)),
            VertexType::Modeled(v) => bytemuck::cast_slice(std::slice::from_ref(v)),
        }
    }

    pub fn as_pod(&self) -> Vec<u8> {
        match self {
            VertexType::Textured(data) => bytemuck::cast_slice(std::slice::from_ref(data)).to_vec(),
            VertexType::Colored(data) => bytemuck::cast_slice(std::slice::from_ref(data)).to_vec(),
            VertexType::Modeled(data) => bytemuck::cast_slice(std::slice::from_ref(data)).to_vec(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct CameraUniform {
    pub view_position: [f32; 4],
    pub view: [[f32; 4]; 4],
    pub view_proj: [[f32; 4]; 4],
    pub inv_proj: [[f32; 4]; 4],
    pub inv_view: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
            view: cgmath::Matrix4::identity().into(),
            inv_proj: cgmath::Matrix4::identity().into(),
            inv_view: cgmath::Matrix4::identity().into(),
        }
    }
    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.view_matrix()).into();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    pub _padding: u32,
    pub color: [f32; 3],
    pub _padding2: u32,
}
