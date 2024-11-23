use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Sub;

use bytemuck::Pod;
use bytemuck::Zeroable;
use serde::Deserialize;
use serde::Serialize;
use wgpu::util::DeviceExt;
use wgpu::Buffer;
use wgpu::VertexBufferLayout;

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

impl Uniforms {
    pub fn new(
        view_projection: [[f32; 4]; 4],
        model: [[f32; 4]; 4],
        color: [f32; 4],
        view_position: [f32; 3],
        light_position: [f32; 4],
    ) -> Self {
        Self {
            view_proj: view_projection,
            model,
            color,
            light_position,
            light_color: [1.0, 1.0, 1.0, 1.0],
            view_position: [view_position[0], view_position[1], view_position[2], 0.0],
            ambient_strength: 0.8,
            diffuse_strength: 12.0,
            specular_strength: 12.0,
            shininess: 32.0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug, Serialize, Deserialize)]
pub struct VertexStruct {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl VertexStruct {
    fn buffer_layout<'a>() -> VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<VertexStruct>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3, // position
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4, // color
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3, // normal
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>()
                        + mem::size_of::<[f32; 4]>()
                        + mem::size_of::<[f32; 3]>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2, // tex_coords
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    pub model: [[f32; 4]; 4],
    pub color: [f32; 4],
}
impl InstanceData {
    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance, // This indicates instanced rendering
            attributes: &[
                wgpu::VertexAttribute {
                    shader_location: 4, // Matches shader's instance model matrix
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    shader_location: 5,
                    offset: 16,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    shader_location: 6,
                    offset: 32,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    shader_location: 7,
                    offset: 48,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    shader_location: 8, // Instance color
                    offset: 64,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

impl Hash for VertexStruct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position
            .iter()
            .for_each(|&val| val.to_bits().hash(state));
        self.color.iter().for_each(|&val| val.to_bits().hash(state));
        self.normal
            .iter()
            .for_each(|&val| val.to_bits().hash(state));
        self.tex_coords
            .iter()
            .for_each(|&val| val.to_bits().hash(state));
    }
}

impl Sub for VertexStruct {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            position: [
                self.position[0] - rhs.position[0],
                self.position[1] - rhs.position[1],
                self.position[2] - rhs.position[2],
            ],
            color: [
                self.color[0] - rhs.color[0],
                self.color[1] - rhs.color[1],
                self.color[2] - rhs.color[2],
                self.color[3] - rhs.color[3],
            ],
            normal: [
                self.normal[0] - rhs.normal[0],
                self.normal[1] - rhs.normal[1],
                self.normal[2] - rhs.normal[2],
            ],
            tex_coords: [
                self.tex_coords[0] - rhs.tex_coords[0],
                self.tex_coords[1] - rhs.tex_coords[1],
            ],
        }
    }
}
