use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{BindGroup, BindGroupLayout, Device, RenderPipeline, ShaderModule};

use crate::{log_debug, object::Vertex};

pub struct PipelineCache {
    pipelines: HashMap<u64, Arc<RenderPipeline>>,
    bind_group_layouts: HashMap<u64, Arc<BindGroupLayout>>,
    bind_groups: HashMap<u64, Arc<BindGroup>>,
}

impl PipelineCache {
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new(),
            bind_group_layouts: HashMap::new(),
            bind_groups: HashMap::new(),
        }
    }

    pub fn get_or_create_pipeline(
        &mut self,
        device: &wgpu::Device,
        pipeline_cache_key: u64,
        vertex_shader_src: &ShaderModule,
        fragment_shader_src: &ShaderModule,
        bind_group_layout: &wgpu::BindGroupLayout, // This should include global bindings
        swap_chain_format: wgpu::TextureFormat,
    ) -> Arc<wgpu::RenderPipeline> {
        get_or_create_pipeline(
            self,
            device,
            pipeline_cache_key,
            vertex_shader_src,
            fragment_shader_src,
            bind_group_layout,
            swap_chain_format,
        )
    }
    pub fn get_or_create_bind_group_layout(
        &mut self,
        device: &Device,
        bind_group_cache_key: u64,
        entries: &[wgpu::BindGroupLayoutEntry],
    ) -> Arc<BindGroupLayout> {
        get_or_create_bind_group_layout(self, device, bind_group_cache_key, entries)
    }

    pub fn get_or_create_bind_group(
        &mut self,
        device: &Device,
        bind_group_cache_key: u64,
        layout: Arc<BindGroupLayout>,
        entries: &[wgpu::BindGroupEntry], // This is likely passed as an empty array
    ) -> Arc<BindGroup> {
        get_or_create_bind_group(self, device, bind_group_cache_key, layout, entries)
    }
}
pub fn get_or_create_pipeline(
    cache: &mut PipelineCache,
    device: &wgpu::Device,
    pipeline_cache_key: u64,
    vertex_shader_src: &ShaderModule,
    fragment_shader_src: &ShaderModule,
    bind_group_layout: &wgpu::BindGroupLayout, // This should include global bindings
    swap_chain_format: wgpu::TextureFormat,
) -> Arc<wgpu::RenderPipeline> {
    if let Some(pipeline) = cache.pipelines.get(&pipeline_cache_key) {
        return Arc::clone(pipeline);
    }

    // Create pipeline layout with the bind group layout
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout], // Ensure this includes the layout for group(0)
        push_constant_ranges: &[],
    });

    let pipeline = Arc::new(
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: vertex_shader_src,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: fragment_shader_src,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: swap_chain_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                polygon_mode: wgpu::PolygonMode::Fill,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: Default::default(),
        }),
    );
    cache
        .pipelines
        .insert(pipeline_cache_key, Arc::clone(&pipeline));

    pipeline
}

pub fn get_or_create_bind_group_layout(
    cache: &mut PipelineCache,
    device: &Device,
    bind_group_cache_key: u64,
    entries: &[wgpu::BindGroupLayoutEntry],
) -> Arc<BindGroupLayout> {
    // Check if the bind group layout is cached
    if let Some(layout) = cache.bind_group_layouts.get(&bind_group_cache_key) {
        log_debug!("Pipeline layout found from cache: {:?}", layout);
        return Arc::clone(layout);
    }

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Global Bind Group Layout"),
        entries: entries,
    });

    let layout = Arc::new(bind_group_layout);
    cache
        .bind_group_layouts
        .insert(bind_group_cache_key, Arc::clone(&layout));

    layout
}
pub fn get_or_create_bind_group(
    cache: &mut PipelineCache,
    device: &Device,
    bind_group_cache_key: u64,
    layout: Arc<BindGroupLayout>,
    entries: &[wgpu::BindGroupEntry], // This is likely passed as an empty array
) -> Arc<BindGroup> {
    // Check if the bind group is cached
    if let Some(bind_group) = cache.bind_groups.get(&bind_group_cache_key) {
        log_debug!("Pipeline bind group found from cache: {:?}", layout);
        return Arc::clone(bind_group);
    }

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout,
        entries, // This needs to have valid bind group entries
        label: Some("Bind Group"),
    });

    let bind_group_arc = Arc::new(bind_group);
    cache
        .bind_groups
        .insert(bind_group_cache_key, Arc::clone(&bind_group_arc));

    bind_group_arc
}
