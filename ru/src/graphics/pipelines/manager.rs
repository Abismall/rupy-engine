use wgpu::RenderPipeline;

use crate::{
    core::error::AppError,
    ecs::{components::instance::model::InstanceRaw, traits::Cache},
    graphics::{
        binding::BindGroupLayouts,
        shaders::manager::ShaderManager,
        vertex::{ModelVertex, Vertex},
        PrimitiveTopology,
    },
    prelude::cache::{CacheKey, HashCache},
};

use super::common::create_render_pipeline;

#[derive(Debug)]
pub struct PipelineManager {
    pub pipelines: HashCache<RenderPipeline>,
}

impl PipelineManager {
    pub fn new() -> Self {
        Self {
            pipelines: HashCache::new(),
        }
    }
    fn create_pipeline<F>(
        device: &wgpu::Device,
        layout_desc: &wgpu::PipelineLayoutDescriptor,
        hdr_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
        vertex_descs: &[wgpu::VertexBufferLayout],
        topology: wgpu::PrimitiveTopology,
        shader_path: &str,
        shader_manager: &mut ShaderManager,
        create_pipeline_fn: F,
    ) -> Result<RenderPipeline, AppError>
    where
        F: Fn(&wgpu::Device, &wgpu::PipelineLayoutDescriptor) -> wgpu::PipelineLayout,
    {
        let layout = create_pipeline_fn(device, layout_desc);
        create_render_pipeline(
            device,
            &layout,
            hdr_format,
            depth_format,
            vertex_descs,
            topology,
            shader_path,
            shader_manager,
        )
    }
    pub fn setup(
        device: &wgpu::Device,
        topologies: Vec<PrimitiveTopology>,
        depth_format: Option<wgpu::TextureFormat>,
        bind_group_layouts: &BindGroupLayouts,
        hdr_format: wgpu::TextureFormat,
        shader_manager: &mut ShaderManager,
    ) -> Result<PipelineManager, AppError> {
        let mut pipeline_manager = PipelineManager::new();

        for topology in topologies {
            let topology_label = topology.label();

            let normal_pipeline_id =
                CacheKey::from(format!("{}_pipeline", topology_label).as_str());
            let _ = pipeline_manager
                .pipelines
                .get_or_create(normal_pipeline_id, || {
                    let render_pipeline_layout =
                        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                            label: Some("Normal Render Pipeline Layout"),
                            bind_group_layouts: &[
                                &bind_group_layouts.texture_bind_group_layout,
                                &bind_group_layouts.camera_bind_group_layout,
                                &bind_group_layouts.light_bind_group_layout,
                                &bind_group_layouts.skybox_bind_group_layout,
                            ],
                            push_constant_ranges: &[],
                        });

                    Ok(create_render_pipeline(
                        &device,
                        &render_pipeline_layout,
                        hdr_format,
                        depth_format,
                        &[ModelVertex::desc(), InstanceRaw::desc()],
                        topology.to_wgpu_topology(),
                        "core/normal.wgsl",
                        shader_manager,
                    )
                    .expect("Normal pipeline"))
                });

            let light_pipeline_id =
                CacheKey::from(format!("{}_light_pipeline", topology_label).as_str());
            pipeline_manager
                .pipelines
                .get_or_create(light_pipeline_id, || {
                    let render_pipeline_layout =
                        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                            label: Some("Light Render Pipeline Layout"),
                            bind_group_layouts: &[
                                &bind_group_layouts.camera_bind_group_layout,
                                &bind_group_layouts.light_bind_group_layout,
                            ],
                            push_constant_ranges: &[],
                        });

                    Ok(create_render_pipeline(
                        &device,
                        &render_pipeline_layout,
                        hdr_format,
                        depth_format,
                        &[ModelVertex::desc()],
                        topology.to_wgpu_topology(),
                        "core/lighting.wgsl",
                        shader_manager,
                    )
                    .expect("Lighting pipeline"))
                })?;
        }

        let sky_pipeline_id = CacheKey::from("skybox");
        pipeline_manager
            .pipelines
            .get_or_create(sky_pipeline_id, || {
                let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Skybox Pipeline Layout"),
                    bind_group_layouts: &[
                        &bind_group_layouts.camera_bind_group_layout,
                        &bind_group_layouts.skybox_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

                Ok(create_render_pipeline(
                    &device,
                    &layout,
                    hdr_format,
                    depth_format,
                    &[],
                    wgpu::PrimitiveTopology::TriangleList,
                    "objects/skybox.wgsl",
                    shader_manager,
                )
                .expect("Skybox pipeline"))
            })?;

        Ok(pipeline_manager)
    }
}
