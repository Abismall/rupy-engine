use wgpu::util::DeviceExt;

use crate::material::vertex::Vertex;

use wgpu::{FragmentState, RenderPipelineDescriptor, VertexState};

pub fn create_render_pipeline(
    device: &wgpu::Device,
    texture_format: wgpu::TextureFormat,
    bind_group_layout: &wgpu::BindGroupLayout,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: vertex_shader,
            entry_point: "vs_main",
            buffers: &[Vertex::descriptor()],
            compilation_options: Default::default(),
        },
        fragment: Some(FragmentState {
            module: fragment_shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: texture_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float, // Make sure your depth format is supported by your device
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less, // Typical for 3D rendering
            stencil: wgpu::StencilState::default(),     // No stencil buffer for now
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: Default::default(),
    });

    render_pipeline
}

pub fn create_vertex_buffer<T: bytemuck::Pod>(
    device: &wgpu::Device,
    vertices: &[T],
) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}
