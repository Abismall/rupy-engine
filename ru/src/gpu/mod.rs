use std::{collections::HashMap, mem, sync::Arc};

use buffer::setup::BufferSetup;
use serde::{Deserialize, Serialize};
use wgpu::Buffer;

use crate::pipeline::state::{DepthType, PrimitiveType, ShadingType};

pub mod binding;
pub mod buffer;
pub mod global;
pub mod glyphon;
pub mod sampler;
pub mod uniform;
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    pub model: [[f32; 4]; 4],
    pub color: [f32; 4],
}
impl InstanceData {
    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4, // First row of instance model matrix
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4, // Second row
                },
                wgpu::VertexAttribute {
                    offset: (2 * mem::size_of::<[f32; 4]>()) as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4, // Third row
                },
                wgpu::VertexAttribute {
                    offset: (3 * mem::size_of::<[f32; 4]>()) as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4, // Fourth row
                },
            ],
        }
    }
}

#[derive(Debug, Default)]
pub struct RenderBatch {
    pub mesh_instances: HashMap<u64, Vec<InstanceData>>,
    pub mesh_buffers: HashMap<u64, wgpu::Buffer>,
    pub texture_bindings: HashMap<u64, Arc<wgpu::BindGroup>>,
    pub texture_ids: HashMap<u64, u64>,
}
impl RenderBatch {
    pub fn get_instance_offset(&self, mesh_id: u64) -> u32 {
        self.mesh_instances
            .keys()
            .take_while(|&id| id != &mesh_id)
            .map(|id| self.mesh_instances[id].len() as u32)
            .sum()
    }
    pub fn get_instance_data(&self) -> Vec<InstanceData> {
        self.mesh_instances
            .iter()
            .flat_map(|(_, instances)| instances)
            .cloned()
            .collect()
    }
    pub fn instance_buffer(&self, device: &wgpu::Device) -> Buffer {
        BufferSetup::instance_buffer(device, &self.get_instance_data())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderMode {
    LineTextureNoDepth,
    LineTextureWithDepth,
    TriangleTextureNoDepth,
    TriangleTextureWithDepth,
    LineColorNoDepth,
    LineColorWithDepth,
    TriangleColorNoDepth,
    TriangleColorWithDepth,
    QuadTexture,
    QuadColor,
}

impl RenderMode {
    pub fn next(self) -> RenderMode {
        use RenderMode::*;
        match self {
            LineColorNoDepth => LineColorWithDepth,
            LineColorWithDepth => TriangleColorNoDepth,
            TriangleColorNoDepth => TriangleColorWithDepth,
            TriangleColorWithDepth => TriangleTextureNoDepth,
            TriangleTextureNoDepth => TriangleTextureWithDepth,
            TriangleTextureWithDepth => LineTextureNoDepth,
            LineTextureNoDepth => LineTextureWithDepth,
            LineTextureWithDepth => QuadColor,
            QuadColor => QuadTexture,
            QuadTexture => LineColorNoDepth,
        }
    }
    pub fn is_textured(&self) -> bool {
        matches!(
            self,
            RenderMode::LineTextureNoDepth
                | RenderMode::LineTextureWithDepth
                | RenderMode::TriangleTextureNoDepth
                | RenderMode::TriangleTextureWithDepth
                | RenderMode::QuadTexture
        )
    }

    pub fn is_colored(&self) -> bool {
        matches!(
            self,
            RenderMode::LineColorNoDepth
                | RenderMode::LineColorWithDepth
                | RenderMode::TriangleColorNoDepth
                | RenderMode::TriangleColorWithDepth
                | RenderMode::QuadColor
        )
    }
    pub fn to_pipeline_config(self) -> (PrimitiveType, ShadingType, DepthType) {
        match self {
            RenderMode::LineTextureWithDepth => {
                (PrimitiveType::Line, ShadingType::Texture, DepthType::Depth)
            }
            RenderMode::LineTextureNoDepth => (
                PrimitiveType::Line,
                ShadingType::Texture,
                DepthType::NoDepth,
            ),
            RenderMode::TriangleTextureWithDepth => (
                PrimitiveType::Triangle,
                ShadingType::Texture,
                DepthType::Depth,
            ),
            RenderMode::TriangleTextureNoDepth => (
                PrimitiveType::Triangle,
                ShadingType::Texture,
                DepthType::NoDepth,
            ),
            RenderMode::LineColorWithDepth => {
                (PrimitiveType::Line, ShadingType::Color, DepthType::Depth)
            }
            RenderMode::LineColorNoDepth => {
                (PrimitiveType::Line, ShadingType::Color, DepthType::NoDepth)
            }
            RenderMode::TriangleColorWithDepth => (
                PrimitiveType::Triangle,
                ShadingType::Color,
                DepthType::Depth,
            ),
            RenderMode::TriangleColorNoDepth => (
                PrimitiveType::Triangle,
                ShadingType::Color,
                DepthType::NoDepth,
            ),

            RenderMode::QuadColor => (PrimitiveType::Quad, ShadingType::Color, DepthType::NoDepth),
            RenderMode::QuadTexture => (
                PrimitiveType::Quad,
                ShadingType::Texture,
                DepthType::NoDepth,
            ),
        }
    }
}
