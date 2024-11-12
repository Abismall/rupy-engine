#[derive(Debug)]
pub enum PrimitiveType {
    Line,
    Triangle,
    Quad,
}

#[derive(Debug)]
pub enum ShadingType {
    Color,
    Texture,
}

#[derive(Debug)]
pub enum DepthType {
    Depth,
    NoDepth,
}
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FrontFace, PolygonMode, PrimitiveState,
    PrimitiveTopology, TextureFormat,
};

pub enum PrimitiveStateConfig {
    TriangleList,
    TriangleStrip,
    LineList,
    LineStrip,
    PointList,
    Custom {
        topology: PrimitiveTopology,
        front_face: FrontFace,
        cull_mode: Option<Face>,
        polygon_mode: PolygonMode,
    },
}

impl PrimitiveStateConfig {
    pub fn to_primitive_state(&self) -> PrimitiveState {
        match *self {
            PrimitiveStateConfig::TriangleList => PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            PrimitiveStateConfig::TriangleStrip => PrimitiveState {
                topology: PrimitiveTopology::TriangleStrip,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            PrimitiveStateConfig::LineList => PrimitiveState {
                topology: PrimitiveTopology::LineList,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            PrimitiveStateConfig::LineStrip => PrimitiveState {
                topology: PrimitiveTopology::LineStrip,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            PrimitiveStateConfig::PointList => PrimitiveState {
                topology: PrimitiveTopology::PointList,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            PrimitiveStateConfig::Custom {
                topology,
                front_face,
                cull_mode,
                polygon_mode,
            } => PrimitiveState {
                topology,
                front_face,
                cull_mode,
                polygon_mode,
                ..Default::default()
            },
        }
    }
}

pub enum ColorTargetConfig {
    Replace,
    Add,
    AlphaBlend,
    Custom {
        blend_state: Option<BlendState>,
        write_mask: ColorWrites,
    },
}

impl ColorTargetConfig {
    pub fn to_color_target_state(&self, format: TextureFormat) -> ColorTargetState {
        match *self {
            ColorTargetConfig::Replace => ColorTargetState {
                format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            },
            ColorTargetConfig::Add => ColorTargetState {
                format,
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            },
            ColorTargetConfig::AlphaBlend => ColorTargetState {
                format,
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            },
            ColorTargetConfig::Custom {
                blend_state,
                write_mask,
            } => ColorTargetState {
                format,
                blend: blend_state,
                write_mask,
            },
        }
    }
}
