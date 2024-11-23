pub mod bind_group;
pub mod buffer;
pub mod context;
pub mod data;
pub mod global;
pub mod glyphon;
pub mod render;
pub mod sampler;

use serde::{Deserialize, Serialize};

use crate::pipeline::{DepthType, PrimitiveType, ShadingType};

pub fn default_depth_stencil_state(format: Option<wgpu::TextureFormat>) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: format.unwrap_or(wgpu::TextureFormat::Depth32Float),
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
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
