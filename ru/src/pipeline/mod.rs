pub mod setup;
pub mod state;
use state::{ColorTargetConfig, DepthType, PrimitiveStateConfig, PrimitiveType, ShadingType};

use crate::{
    gpu::buffer::VertexBuffer,
    prelude::constant::{WGSL_FRAGMENT_MAIN_DEFAULT, WGSL_VERTEX_MAIN_DEFAULT},
};

pub fn get_pipeline_label(
    primitive: &PrimitiveType,
    shading: &ShadingType,
    depth: &DepthType,
) -> String {
    format!("[{:?}, {:?}, {:?}]", primitive, shading, depth)
}

pub fn render_pipeline<T: VertexBuffer>(
    device: &wgpu::Device,
    name: &str,
    texture_format: wgpu::TextureFormat,
    uniform_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    depth_stencil: Option<&wgpu::DepthStencilState>,
    primitive_state_config: &PrimitiveStateConfig,
    color_target_config: &ColorTargetConfig,
) -> wgpu::RenderPipeline {
    let layout = &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(name),
        bind_group_layouts: &[&uniform_layout, &texture_layout],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(name),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: vertex_shader,
            entry_point: WGSL_VERTEX_MAIN_DEFAULT,
            buffers: &[T::buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: fragment_shader,
            entry_point: WGSL_FRAGMENT_MAIN_DEFAULT,
            targets: &[Some(
                color_target_config.to_color_target_state(texture_format),
            )],
            compilation_options: Default::default(),
        }),
        primitive: primitive_state_config.to_primitive_state(),
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
