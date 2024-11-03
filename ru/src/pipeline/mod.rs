pub mod cache;
pub mod key;
pub mod state;
// Debug
pub const LABEL_SHADED_SIMPLE_PIPELINE: &str = "Shaded Simple RenderPipeline";
pub const LABEL_SHADED_SIMPLE_PIPELINE_LAYOUT: &str = "Shaded Simple RenderPipeline Layout";

pub const LABEL_SHADED_COMPLEX_PIPELINE: &str = "Shaded Complex RenderPipeline";
pub const LABEL_SHADED_COMPLEX_PIPELINE_LAYOUT: &str = "Shaded Complex RenderPipeline Layout";

use state::{ColorTargetConfig, PrimitiveStateConfig};

use crate::ecs::components::vertex::Vertex;

pub fn create_render_pipeline(
    device: &wgpu::Device,
    name: &String,
    texture_format: wgpu::TextureFormat,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    depth_stencil: Option<&wgpu::DepthStencilState>,
    primitive_state_config: &PrimitiveStateConfig,
    color_target_config: &ColorTargetConfig,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(name),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&name),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: vertex_shader,
            entry_point: "vs_main",
            buffers: &[Vertex::buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: fragment_shader,
            entry_point: "fs_main",
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
