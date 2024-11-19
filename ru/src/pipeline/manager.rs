use std::{collections::HashMap, sync::Arc};

use wgpu::{DepthStencilState, Device, RenderPipeline, TextureFormat};

use crate::{
    log_debug,
    pipeline::{
        setup::{render_pipeline_2d, render_pipeline_3d},
        state::{DepthType, PrimitiveType, ShadingType},
    },
    shader::manager::ShaderManager,
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

    pub fn add_pipeline(&mut self, name: &str, pipeline: RenderPipeline) {
        log_debug!("[PUT] {}", name);
        self.pipelines.insert(name.into(), pipeline.into());
    }
    pub fn get_or_create_pipeline_3d(
        &mut self,
        name: &str,
        shader_manager: &mut ShaderManager,
        device: &Device,
        swapchain_format: TextureFormat,
        uniform_layout: &wgpu::BindGroupLayout,
        texture_layout: &wgpu::BindGroupLayout,
        primitive_type: PrimitiveType,
        shading_type: ShadingType,
        depth_type: DepthType,
        depth_stencil_state: &DepthStencilState,
    ) -> Arc<wgpu::RenderPipeline> {
        if let Some(existing_pipeline) = self.pipelines.get(name) {
            return existing_pipeline.clone();
        }
        let pipeline_3d = render_pipeline_3d(
            shader_manager,
            device,
            swapchain_format,
            uniform_layout,
            texture_layout,
            primitive_type,
            shading_type,
            depth_type,
            depth_stencil_state,
        )
        .expect(&format!("Render Pipeline: {:?}", name));
        let render_pipeline = Arc::new(pipeline_3d);

        self.pipelines
            .insert(name.to_string(), render_pipeline.clone());

        render_pipeline.into()
    }
    pub fn get_or_create_pipeline_2d(
        &mut self,
        name: &str,
        shader_manager: &mut ShaderManager,
        device: &Device,
        swapchain_format: TextureFormat,
        uniform_layout: &wgpu::BindGroupLayout,
        texture_layout: &wgpu::BindGroupLayout,
        primitive_type: &PrimitiveType,
        shading_type: &ShadingType,
        depth_type: &DepthType,
        depth_stencil_state: &DepthStencilState,
    ) -> Arc<wgpu::RenderPipeline> {
        if let Some(existing_pipeline) = self.pipelines.get(name) {
            return existing_pipeline.clone();
        }
        let pipeline_3d = render_pipeline_2d(
            shader_manager,
            device,
            &swapchain_format,
            uniform_layout,
            texture_layout,
            primitive_type,
            shading_type,
            depth_type,
            depth_stencil_state,
        )
        .expect(&format!("Render Pipeline: {:?}", name));
        let render_pipeline = Arc::new(pipeline_3d);

        self.pipelines
            .insert(name.to_string(), render_pipeline.clone());

        render_pipeline.into()
    }
    pub fn get_pipeline(&self, name: &str) -> Option<Arc<RenderPipeline>> {
        self.pipelines.get(name).cloned()
    }
}
