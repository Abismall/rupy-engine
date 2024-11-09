use super::state::{ColorTargetConfig, PrimitiveStateConfig};
use super::{
    line_list_colored_with_depth, line_list_colored_with_no_depth, line_list_textured_with_depth,
    line_list_textured_with_no_depth, triangle_list_colored_with_depth,
    triangle_list_colored_with_no_depth, triangle_list_textured_with_depth,
    triangle_list_textured_with_no_depth,
};
use crate::core::error::AppError;
use crate::log_debug;
use crate::pipeline::create_render_pipeline;
use crate::shader::library::ShaderLibrary;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{
    BindGroupLayout, BindGroupLayoutEntry, Device, PipelineLayoutDescriptor, PushConstantRange,
    RenderPipeline,
};
use wgpu::{ShaderModule, TextureFormat};
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

    pub fn add_pipeline(
        &mut self,
        name: String,
        device: &Device,
        texture_format: TextureFormat,
        vertex_shader: &ShaderModule,
        fragment_shader: &ShaderModule,
        depth_stencil: Option<&wgpu::DepthStencilState>,
        primitive_state_config: &PrimitiveStateConfig,
        color_target_config: &ColorTargetConfig,
        bind_group_layouts: &[&BindGroupLayout],
    ) {
        let render_pipeline = create_render_pipeline(
            device,
            &name,
            texture_format,
            bind_group_layouts,
            vertex_shader,
            fragment_shader,
            depth_stencil,
            primitive_state_config,
            color_target_config,
        );
        log_debug!("[PUT] {:?}", render_pipeline);

        self.pipelines.insert(name, render_pipeline.into());
    }
    pub fn get_or_create_pipeline(
        &mut self,
        name: String,
        device: &Device,
        texture_format: TextureFormat,
        vertex_shader: &ShaderModule,
        fragment_shader: &ShaderModule,
        depth_stencil: Option<wgpu::DepthStencilState>,
        primitive_state_config: PrimitiveStateConfig,
        color_target_config: ColorTargetConfig,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> std::option::Option<Arc<wgpu::RenderPipeline>> {
        if let Some(existing_pipeline) = self.pipelines.get(&name) {
            return Some(existing_pipeline.clone());
        }

        let render_pipeline = Arc::new(create_render_pipeline(
            device,
            &name,
            texture_format,
            bind_group_layouts,
            vertex_shader,
            fragment_shader,
            depth_stencil.as_ref(),
            &primitive_state_config,
            &color_target_config,
        ));

        self.pipelines.insert(name.clone(), render_pipeline.clone());

        self.pipelines.get(&name).cloned()
    }
    pub fn get_pipeline(&self, name: &str) -> Option<&RenderPipeline> {
        self.pipelines.get(name).map(|v| &**v)
    }

    pub fn create_bind_group_layout(
        &self,
        label: Option<&str>,
        device: &Device,
        entries: &[BindGroupLayoutEntry],
    ) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label, entries })
    }

    pub fn create_pipeline_layout(
        &self,
        label: Option<&str>,
        device: &Device,
        push_constant_ranges: &[PushConstantRange],
        bind_group_layouts: &[&BindGroupLayout],
    ) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label,
            bind_group_layouts,
            push_constant_ranges,
        })
    }
}

pub fn setup_pipeline_manager(
    device: &Device,
    shader_manager: &mut ShaderLibrary,
    pipeline_manager: &mut PipelineManager,
    texture_format: TextureFormat,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    color_bind_group_layout: wgpu::BindGroupLayout,
) -> Result<(), AppError> {
    triangle_list_textured_with_depth(
        pipeline_manager,
        shader_manager,
        device,
        texture_format,
        &uniform_bind_group_layout,
        &texture_bind_group_layout,
    );
    triangle_list_textured_with_no_depth(
        pipeline_manager,
        shader_manager,
        device,
        texture_format,
        &uniform_bind_group_layout,
        &texture_bind_group_layout,
    );

    line_list_textured_with_depth(
        pipeline_manager,
        shader_manager,
        device,
        texture_format,
        &uniform_bind_group_layout,
        &texture_bind_group_layout,
    );
    line_list_textured_with_no_depth(
        pipeline_manager,
        shader_manager,
        device,
        texture_format,
        &uniform_bind_group_layout,
        &texture_bind_group_layout,
    );

    triangle_list_colored_with_depth(
        pipeline_manager,
        shader_manager,
        device,
        texture_format,
        &uniform_bind_group_layout,
        &texture_bind_group_layout,
    );
    triangle_list_colored_with_no_depth(
        pipeline_manager,
        shader_manager,
        device,
        texture_format,
        &uniform_bind_group_layout,
        &texture_bind_group_layout,
    );

    line_list_colored_with_depth(
        pipeline_manager,
        shader_manager,
        device,
        texture_format,
        &uniform_bind_group_layout,
        &color_bind_group_layout,
    );
    line_list_colored_with_no_depth(
        pipeline_manager,
        shader_manager,
        device,
        texture_format,
        &uniform_bind_group_layout,
        &color_bind_group_layout,
    );

    Ok(())
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum RenderPipelineDepthOption {
    WithDepth,
    WithNoDepth,
}
pub enum RenderPipelineType {
    TriangleList(RenderPipelineDepthOption),
    LineList(RenderPipelineDepthOption),
}

impl RenderPipelineType {
    pub fn new(&self) -> RenderPipelineType {
        match self {
            RenderPipelineType::TriangleList(render_pipeline_depth_option) => {
                RenderPipelineType::TriangleList(*render_pipeline_depth_option)
            }
            RenderPipelineType::LineList(render_pipeline_depth_option) => {
                RenderPipelineType::LineList(*render_pipeline_depth_option)
            }
        }
    }
    pub fn build(
        &self,
        pipeline_manager: &mut PipelineManager,
        shader_manager: &mut ShaderLibrary,
        device: &Device,
        swapchain_format: TextureFormat,
        uniform_bind_group_layout: Arc<wgpu::BindGroupLayout>,
        texture_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    ) {
        match self {
            RenderPipelineType::TriangleList(render_pipeline_depth_option) => {
                if *render_pipeline_depth_option == RenderPipelineDepthOption::WithDepth {
                    triangle_list_textured_with_depth(
                        pipeline_manager,
                        shader_manager,
                        device,
                        swapchain_format,
                        &uniform_bind_group_layout,
                        &texture_bind_group_layout,
                    );
                } else if *render_pipeline_depth_option == RenderPipelineDepthOption::WithNoDepth {
                    triangle_list_textured_with_no_depth(
                        pipeline_manager,
                        shader_manager,
                        device,
                        swapchain_format,
                        &uniform_bind_group_layout,
                        &texture_bind_group_layout,
                    );
                }
            }
            RenderPipelineType::LineList(render_pipeline_depth_option) => {
                if *render_pipeline_depth_option == RenderPipelineDepthOption::WithDepth {
                    line_list_textured_with_depth(
                        pipeline_manager,
                        shader_manager,
                        device,
                        swapchain_format,
                        &uniform_bind_group_layout,
                        &texture_bind_group_layout,
                    );
                } else if *render_pipeline_depth_option == RenderPipelineDepthOption::WithNoDepth {
                    line_list_textured_with_no_depth(
                        pipeline_manager,
                        shader_manager,
                        device,
                        swapchain_format,
                        &uniform_bind_group_layout,
                        &texture_bind_group_layout,
                    );
                }
            }
        }
    }
}
