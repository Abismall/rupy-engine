use crate::{log_debug, render::layout::BindGroupLayoutEntryEnum};

use super::factory::PipelineFactory;
use std::{collections::HashMap, sync::Arc};
use wgpu::{BindGroup, BindGroupLayout, Device, TextureFormat};
#[derive(Debug)]

pub struct PipelineManager {
    pub pipelines: HashMap<u64, Arc<wgpu::RenderPipeline>>,
    pub bind_group_layouts: HashMap<u64, Arc<BindGroupLayout>>,
    pub bind_groups: HashMap<u64, Arc<BindGroup>>,
}

impl PipelineManager {
    pub fn new() -> Self {
        let instance = Self {
            pipelines: HashMap::new(),
            bind_group_layouts: HashMap::new(),
            bind_groups: HashMap::new(),
        };
        log_debug!("{:?}", instance);
        instance
    }

    pub fn get_or_create_pipeline(
        &mut self,
        device: &Device,
        pipeline_hash: u64,
        swapchain_format: TextureFormat,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        bind_group_layout_hash: u64,
        depth_enabled: bool,
    ) -> Arc<wgpu::RenderPipeline> {
        if let Some(pipeline) = self.pipelines.get(&pipeline_hash) {
            return Arc::clone(pipeline);
        }

        let bind_group_layout = self
            .bind_group_layouts
            .get(&bind_group_layout_hash)
            .expect("Bind group layout not found");

        let pipeline = if depth_enabled {
            PipelineFactory::create_pipeline_with_depth(
                device,
                swapchain_format,
                vertex_shader_src,
                fragment_shader_src,
                bind_group_layout,
                "Pipeline with Hash",
            )
        } else {
            PipelineFactory::create_pipeline_without_depth(
                device,
                swapchain_format,
                vertex_shader_src,
                fragment_shader_src,
                bind_group_layout,
                "Pipeline with Hash",
            )
        };

        let pipeline = Arc::new(pipeline);
        self.pipelines.insert(pipeline_hash, Arc::clone(&pipeline));
        pipeline
    }

    pub fn get_or_create_bind_group(
        &mut self,
        device: &Device,
        entries: &Vec<BindGroupLayoutEntryEnum>,
        bind_group_hash: u64,
    ) -> Arc<BindGroup> {
        if let Some(bind_group) = self.bind_groups.get(&bind_group_hash) {
            return Arc::clone(bind_group);
        }

        let bind_group = self
            .bind_groups
            .entry(bind_group_hash)
            .or_insert_with(|| Arc::new(PipelineFactory::create_bind_group(device, entries)));

        let bind_group: Arc<BindGroup> = Arc::clone(bind_group);

        self.bind_groups
            .insert(bind_group_hash, Arc::clone(&bind_group));
        bind_group
    }
}
