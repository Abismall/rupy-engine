pub mod hdr;
pub mod manager;
pub mod setup;
pub fn get_pipeline_label(
    primitive: &PrimitiveType,
    shading: &ShadingType,
    depth: &DepthType,
) -> String {
    format!("[{:?}, {:?}, {:?}]", primitive, shading, depth)
}
#[derive(Debug)]
pub struct PipelineDescriptor {
    pub primitive_type: PrimitiveType,
    pub shading_type: ShadingType,
    pub depth_type: DepthType,
}

impl PipelineDescriptor {
    pub fn new(
        primitive_type: PrimitiveType,
        shading_type: ShadingType,
        depth_type: DepthType,
    ) -> Self {
        Self {
            primitive_type,
            shading_type,
            depth_type,
        }
    }
}

pub fn render_pipeline(
    device: &wgpu::Device,
    name: &str,
    uniform_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    depth_stencil: Option<&wgpu::DepthStencilState>,
    color_target: ColorTargetState,
    primitive_type: PrimitiveState,
) -> wgpu::RenderPipeline {
    let layout = &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(name),
        bind_group_layouts: &[&uniform_layout, &instance_layout, &texture_layout],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(name),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: vertex_shader,
            entry_point: WGSL_VERTEX_MAIN_DEFAULT,
            buffers: &[VertexTexture::desc()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: fragment_shader,
            entry_point: WGSL_FRAGMENT_MAIN_DEFAULT,
            targets: &[Some(color_target)],
            compilation_options: Default::default(),
        }),
        primitive: primitive_type,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: Default::default(),
        depth_stencil: depth_stencil.cloned(),
    })
}
#[derive(Debug, PartialEq)]
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

use crate::{
    graphics::model::VertexTexture,
    prelude::constant::{WGSL_FRAGMENT_MAIN_DEFAULT, WGSL_VERTEX_MAIN_DEFAULT},
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
