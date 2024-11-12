use std::{collections::HashMap, sync::Arc};

use wgpu::{Device, RenderPipeline, ShaderModule, TextureFormat};

use crate::{
    gpu::buffer::VertexBuffer,
    log_debug,
    pipeline::{
        render_pipeline,
        state::{ColorTargetConfig, PrimitiveStateConfig},
    },
};

#[derive(Debug)]
pub struct PipelineManager {
    pipelines: HashMap<String, Arc<RenderPipeline>>,
}

impl PipelineManager {
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new(),
        }
    }

    pub fn add_pipeline<T: VertexBuffer>(
        &mut self,
        name: &str,
        device: &Device,
        texture_format: TextureFormat,
        vertex_shader: &ShaderModule,
        fragment_shader: &ShaderModule,
        depth_stencil: Option<&wgpu::DepthStencilState>,
        primitive_state_config: &PrimitiveStateConfig,
        color_target_config: &ColorTargetConfig,
        uniform_layout: &wgpu::BindGroupLayout,
        texture_layout: &wgpu::BindGroupLayout,
    ) {
        let render_pipeline = render_pipeline::<T>(
            device,
            name,
            texture_format,
            uniform_layout,
            texture_layout,
            vertex_shader,
            fragment_shader,
            depth_stencil,
            primitive_state_config,
            color_target_config,
        );
        log_debug!("[PUT] {:?}", render_pipeline);

        self.pipelines.insert(name.into(), render_pipeline.into());
    }
    pub fn get_or_create_pipeline<T: VertexBuffer>(
        &mut self,
        name: String,
        device: &Device,
        texture_format: TextureFormat,
        vertex_shader: &ShaderModule,
        fragment_shader: &ShaderModule,
        depth_stencil: Option<wgpu::DepthStencilState>,
        primitive_state_config: PrimitiveStateConfig,
        color_target_config: ColorTargetConfig,
        uniform_layout: &wgpu::BindGroupLayout,
        texture_layout: &wgpu::BindGroupLayout,
    ) -> Arc<wgpu::RenderPipeline> {
        if let Some(existing_pipeline) = self.pipelines.get(&name) {
            return existing_pipeline.clone();
        }

        let render_pipeline = Arc::new(render_pipeline::<T>(
            device,
            &name,
            texture_format,
            uniform_layout,
            texture_layout,
            vertex_shader,
            fragment_shader,
            depth_stencil.as_ref(),
            &primitive_state_config,
            &color_target_config,
        ));

        self.pipelines.insert(name.clone(), render_pipeline.clone());

        render_pipeline
    }
    pub fn get_pipeline(&self, name: &str) -> Option<Arc<RenderPipeline>> {
        self.pipelines.get(name).cloned()
    }
}
