use std::{collections::HashMap, sync::Arc};
use wgpu::{
    BindGroupLayout, BindGroupLayoutEntry, Device, PipelineLayoutDescriptor, RenderPipeline,
    ShaderModule, TextureFormat,
};

use crate::material::vertex::Vertex;

pub const DEFAULT_PIPELINE: &str = "RenderPipeline";
pub const OUTLINE_PIPELINE: &str = "OutlinePipeline";
pub const SURFACE_PIPELINE: &str = "SurfacePipeline";

pub struct PipelineManager {
    pub pipelines: HashMap<String, Arc<wgpu::RenderPipeline>>,
    pub bind_group_layout: BindGroupLayout,
}

impl PipelineManager {
    pub fn new(device: &Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        Self {
            pipelines: HashMap::new(),
            bind_group_layout,
        }
    }

    pub fn create_main_pipeline(&mut self, device: &Device, swapchain_format: TextureFormat) {
        let pipeline = self.create_pipeline_with_depth(
            device,
            &self.bind_group_layout,
            swapchain_format,
            include_str!("../../static/shader/vertex.wgsl"),
            include_str!("../../static/shader/fragment.wgsl"),
            DEFAULT_PIPELINE,
        );
        self.pipelines
            .insert(DEFAULT_PIPELINE.to_string(), Arc::new(pipeline));
    }

    pub fn create_outline_pipeline(&mut self, device: &Device, swapchain_format: TextureFormat) {
        let pipeline = self.create_pipeline_with_depth(
            device,
            &self.bind_group_layout,
            swapchain_format,
            include_str!("../../static/shader/outline.vertex.wgsl"),
            include_str!("../../static/shader/outline.fragment.wgsl"),
            OUTLINE_PIPELINE,
        );
        self.pipelines
            .insert(OUTLINE_PIPELINE.to_string(), Arc::new(pipeline));
    }

    pub fn create_surface_pipeline(&mut self, device: &Device, swapchain_format: TextureFormat) {
        let pipeline = self.create_pipeline_without_depth(
            device,
            &self.bind_group_layout,
            swapchain_format,
            include_str!("../../static/shader/transparent.vertex.wgsl"),
            include_str!("../../static/shader/transparent.fragment.wgsl"),
            SURFACE_PIPELINE,
        );
        self.pipelines
            .insert(SURFACE_PIPELINE.to_string(), Arc::new(pipeline));
    }

    pub fn get_pipeline(&self, name: &str) -> Option<Arc<wgpu::RenderPipeline>> {
        self.pipelines.get(name).cloned()
    }

    fn create_pipeline_with_depth(
        &self,
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        texture_format: TextureFormat,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        label: &str,
    ) -> RenderPipeline {
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(vertex_shader_src.into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(fragment_shader_src.into()),
        });

        self.create_pipeline(
            device,
            bind_group_layout,
            texture_format,
            &vertex_shader,
            &fragment_shader,
            wgpu::PolygonMode::Fill,
            Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            Some(label),
        )
    }

    fn create_pipeline_without_depth(
        &self,
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        texture_format: TextureFormat,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        label: &str,
    ) -> RenderPipeline {
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(vertex_shader_src.into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(fragment_shader_src.into()),
        });

        self.create_pipeline(
            device,
            bind_group_layout,
            texture_format,
            &vertex_shader,
            &fragment_shader,
            wgpu::PolygonMode::Fill,
            None,
            Some(label),
        )
    }

    fn create_pipeline(
        &self,
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        texture_format: TextureFormat,
        vertex_shader: &ShaderModule,
        fragment_shader: &ShaderModule,
        polygon_mode: wgpu::PolygonMode,
        depth_stencil: Option<wgpu::DepthStencilState>,
        label: Option<&str>,
    ) -> RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: vertex_shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                polygon_mode,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: Default::default(),
        })
    }
}
pub fn create_bind_group(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    uniform_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
        label: Some("Uniform Bind Group"),
    })
}
