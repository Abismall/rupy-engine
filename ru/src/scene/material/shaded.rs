use wgpu::{PipelineLayout, RenderPipeline};

use crate::{
    core::error::AppError,
    graphics::{
        binding::{group::BindGroupScribe, layout::schema::BindGroupLayoutScribe},
        pipeline::{
            LABEL_SHADED_COMPLEX_PIPELINE, LABEL_SHADED_SIMPLE_PIPELINE,
            LABEL_SHADED_SIMPLE_PIPELINE_LAYOUT,
        },
        shader::{RupyShader, ShaderModuleBuilder},
    },
    scene::components::vertex::Vertex,
    traits::buffers::VertexBuffer,
};

use super::Material;

pub struct ShadedMaterial {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
}

impl ShadedMaterial {
    const DEFAULT_SHADER: &str = "static/shaders/simple/material_shaded.wgsl";
    const VS_MAIN: &str = "vs_main";
    const FS_MAIN: &str = "fs_main";

    pub fn new(
        device: &wgpu::Device,
        model_uniform_buffer: std::sync::Arc<wgpu::Buffer>,
        shader: Option<RupyShader>,
    ) -> Result<Self, AppError> {
        let shader_module = match shader {
            Some(some) => some,
            None => ShaderModuleBuilder::from_path_string(
                device,
                Self::DEFAULT_SHADER,
                Self::VS_MAIN.into(),
                Self::FS_MAIN.into(),
            )?,
        };
        let layout = shaded_simple_pipeline_layout(device);
        let bind_group = BindGroupScribe::shaded_material_bind_group(device, &model_uniform_buffer);
        let pipeline = shaded_simple_pipeline(device, &shader_module, layout);

        Ok(Self {
            pipeline,
            bind_group,
        })
    }
}
impl Material for ShadedMaterial {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
pub fn shaded_simple_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(LABEL_SHADED_SIMPLE_PIPELINE_LAYOUT),
        bind_group_layouts: &[&BindGroupLayoutScribe::model_uniform_layout(device)],
        push_constant_ranges: &[],
    })
}
pub fn shaded_simple_pipeline(
    device: &wgpu::Device,
    shader: &RupyShader,
    layout: PipelineLayout,
) -> RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(LABEL_SHADED_SIMPLE_PIPELINE),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader.module,
            entry_point: "vs_main",
            buffers: &[Vertex::vertex_buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader.module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: Default::default(),
    })
}
pub fn shaded_complex_pipeline(
    device: &wgpu::Device,
    shader: &RupyShader,
    layout: PipelineLayout,
) -> RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(LABEL_SHADED_COMPLEX_PIPELINE),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader.module,
            entry_point: "vs_main",
            buffers: &[Vertex::vertex_buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader.module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: Default::default(),
    })
}
