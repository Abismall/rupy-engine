use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, Device,
    PipelineLayoutDescriptor, RenderPipeline, ShaderModule, ShaderStages, TextureFormat,
};

use crate::{object::vertex::Vertex, render::layout::BindGroupLayoutEntryEnum};

pub struct PipelineFactory;

impl PipelineFactory {
    pub fn create_bind_group_layout_entries(
        entries: &Vec<BindGroupLayoutEntryEnum>,
    ) -> Vec<wgpu::BindGroupLayoutEntry> {
        let mut layout_entries = Vec::new();
        for (index, resource) in entries.iter().enumerate() {
            let layout_entry = match resource {
                BindGroupLayoutEntryEnum::UniformBuffer(_) => wgpu::BindGroupLayoutEntry {
                    binding: index as u32,
                    visibility: ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntryEnum::Texture(_) => wgpu::BindGroupLayoutEntry {
                    binding: index as u32,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntryEnum::Sampler(_) => wgpu::BindGroupLayoutEntry {
                    binding: index as u32,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            };

            layout_entries.push(layout_entry);
        }
        layout_entries
    }

    pub fn create_bind_group_entries(
        entries: &Vec<BindGroupLayoutEntryEnum>,
    ) -> Vec<wgpu::BindGroupEntry> {
        let mut bind_group_entries = Vec::new();

        for (index, resource) in entries.iter().enumerate() {
            let bind_group_entry = match resource {
                BindGroupLayoutEntryEnum::UniformBuffer(buffer) => wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupLayoutEntryEnum::Texture(texture_view) => wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                BindGroupLayoutEntryEnum::Sampler(sampler) => wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            };

            bind_group_entries.push(bind_group_entry);
        }
        bind_group_entries
    }

    pub fn create_bind_group(
        device: &Device,
        entries: &Vec<BindGroupLayoutEntryEnum>,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            layout: &device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &&Self::create_bind_group_layout_entries(entries),
                label: Some("Dynamic Bind Group Layout"),
            }),
            entries: &Self::create_bind_group_entries(entries),
            label: Some("Dynamic Bind Group"),
        })
    }
    pub fn create_bind_group_layout(
        device: &Device,
        entries: &Vec<BindGroupLayoutEntryEnum>,
    ) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &Self::create_bind_group_layout_entries(entries),
            label: Some("Dynamic Bind Group Layout"),
        })
    }
    pub fn create_pipeline_with_depth(
        device: &Device,
        texture_format: TextureFormat,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        bind_group_layout: &BindGroupLayout,
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

        Self::create_pipeline(
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

    pub fn create_pipeline_without_depth(
        device: &Device,
        texture_format: TextureFormat,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        bind_group_layout: &BindGroupLayout,
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

        Self::create_pipeline(
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

    pub fn create_pipeline(
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
