use serde::{Deserialize, Serialize};

use crate::pipeline::state::{DepthType, PrimitiveType, ShadingType};

pub mod binding;
pub mod buffer;
pub mod global;
pub mod glyphon;
pub mod sampler;

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
