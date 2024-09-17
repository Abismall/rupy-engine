use crate::AppError;
use std::{collections::HashMap, sync::Arc};
use wgpu::{
    Device, MultisampleState, RenderPipeline, ShaderModule, ShaderSource, TextureFormat,
    VertexBufferLayout,
};

/// Creates or retrieves an existing render pipeline by name.

pub struct RenderPipelineManager;

impl RenderPipelineManager {
    /// Loads a shader module from WGSL source code
    pub fn create_shader_module(
        device: &Device,
        shader_source: &str,
    ) -> Result<ShaderModule, AppError> {
        Ok(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(shader_source.into()),
        }))
    }

    /// Creates a bind group layout based on the shader requirements
    pub fn create_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    /// Creates a pipeline layout with the given bind group layout
    pub fn create_pipeline_layout(
        device: &Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        })
    }

    /// Defines the vertex buffer layout based on vertex structure
    pub fn define_vertex_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: (3 + 3) * std::mem::size_of::<f32>() as wgpu::BufferAddress, // 3 floats for position + 3 floats for color
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0, // Position attribute location
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 3 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
                    shader_location: 1, // Color attribute location
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }

    /// Creates the render pipeline using all components
    pub fn create_render_pipeline(
        device: &Device,
        pipeline_layout: &wgpu::PipelineLayout,
        vertex_shader_module: &ShaderModule,
        fragment_shader_module: &ShaderModule,
        vertex_layout: VertexBufferLayout<'static>,
        format: TextureFormat,
        multisample: MultisampleState,
    ) -> Result<RenderPipeline, AppError> {
        Ok(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(pipeline_layout),
                vertex: wgpu::VertexState {
                    module: vertex_shader_module,
                    entry_point: "main", // Entry point function in your vertex shader
                    buffers: &[vertex_layout],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: fragment_shader_module,
                    entry_point: "main", // Entry point function in your fragment shader
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::REPLACE), // Blending mode
                        write_mask: wgpu::ColorWrites::ALL,     // Which color channels to write
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // Draw triangles
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back), // Cull back-facing triangles
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float, // Using `Depth32Float` format
                    depth_write_enabled: true,                 // Enable writing to the depth buffer
                    depth_compare: wgpu::CompareFunction::Less, // Standard depth test
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample,
                multiview: None,
                cache: None,
            }),
        )
    }
}
