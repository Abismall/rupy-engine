pub mod cache_key;
pub mod layout;
pub mod pipeline_cache;

use crate::scene::components::{material::Material, mesh::Mesh};

pub fn create_pipeline(
    device: &wgpu::Device,
    material: &Material,
    mesh: &Mesh,
    shader_module: &wgpu::ShaderModule,
    swap_chain_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &material
                    .bind_group_layouts
                    .iter()
                    .map(|l| &**l)
                    .collect::<Vec<_>>(),
                push_constant_ranges: &[],
            }),
        ),
        vertex: wgpu::VertexState {
            module: shader_module,
            entry_point: &material.vertex_entry_point,
            buffers: &[mesh.vertex_buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: &material.fragment_entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: swap_chain_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: material.topology,
            front_face: material.front_face,
            cull_mode: material.cull_mode,
            polygon_mode: material.polygon_mode,
            strip_index_format: None,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: material.depth_stencil_state.clone(),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}
