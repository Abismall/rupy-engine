use super::create_render_pipeline;
use super::state::{ColorTargetConfig, PrimitiveStateConfig};
use crate::core::error::AppError;
use crate::log_debug;
use crate::shader::create_shader_modules;
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{
    BindGroupLayout, BindGroupLayoutEntry, Device, PipelineLayoutDescriptor, PushConstantRange,
    RenderPipeline,
};
use wgpu::{ShaderModule, TextureFormat};

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
        swapchain_format: TextureFormat,
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
            swapchain_format,
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
        swapchain_format: TextureFormat,
        vertex_shader: &ShaderModule,
        fragment_shader: &ShaderModule,
        depth_stencil: Option<wgpu::DepthStencilState>,
        primitive_state_config: PrimitiveStateConfig,
        color_target_config: ColorTargetConfig,
        uniform_bind_group_layout: Arc<wgpu::BindGroupLayout>,
        texture_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    ) -> std::option::Option<Arc<wgpu::RenderPipeline>> {
        if let Some(existing_pipeline) = self.pipelines.get(&name) {
            return Some(existing_pipeline.clone());
        }

        let render_pipeline = Arc::new(create_render_pipeline(
            device,
            &name,
            swapchain_format,
            &[&uniform_bind_group_layout, &texture_bind_group_layout],
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
    swapchain_format: TextureFormat,
    uniform_bind_group_layout: &Arc<wgpu::BindGroupLayout>,
    texture_bind_group_layout: &Arc<wgpu::BindGroupLayout>,
    color_bind_group_layout: &Arc<wgpu::BindGroupLayout>,
) -> Result<PipelineManager, AppError> {
    let mut pipeline_manager = PipelineManager::new();

    let (vertex_shader, fragment_shader) = create_shader_modules(
        device,
        "static/shaders/lit_vertex.wgsl",
        "static/shaders/lit_fragment.wgsl",
    )?;
    let (frustum_vert, frustum_frag) = create_shader_modules(
        device,
        "static/shaders/frustum_vertex.wgsl",
        "static/shaders/frustum_fragment.wgsl",
    )?;

    let depth_stencil_state = Some(wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth32Float,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    });

    pipeline_manager.add_pipeline(
        "depth".to_owned(),
        device,
        swapchain_format,
        &vertex_shader.0,
        &fragment_shader.0,
        depth_stencil_state.as_ref(),
        &PrimitiveStateConfig::TriangleList,
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &texture_bind_group_layout],
    );

    pipeline_manager.add_pipeline(
        "flat".to_owned(),
        device,
        swapchain_format,
        &vertex_shader.0,
        &fragment_shader.0,
        None,
        &PrimitiveStateConfig::TriangleList,
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &texture_bind_group_layout],
    );

    pipeline_manager.add_pipeline(
        "wire".to_owned(),
        device,
        swapchain_format,
        &vertex_shader.0,
        &fragment_shader.0,
        depth_stencil_state.as_ref(),
        &PrimitiveStateConfig::Custom {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Line,
        },
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &texture_bind_group_layout],
    );

    pipeline_manager.add_pipeline(
        "wire_no_depth".to_owned(),
        device,
        swapchain_format,
        &vertex_shader.0,
        &fragment_shader.0,
        None,
        &PrimitiveStateConfig::Custom {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Line,
        },
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &texture_bind_group_layout],
    );

    pipeline_manager.add_pipeline(
        "points".to_owned(),
        device,
        swapchain_format,
        &vertex_shader.0,
        &fragment_shader.0,
        depth_stencil_state.as_ref(),
        &PrimitiveStateConfig::PointList,
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &texture_bind_group_layout],
    );

    pipeline_manager.add_pipeline(
        "lines".to_owned(),
        device,
        swapchain_format,
        &frustum_vert.0,
        &frustum_frag.0,
        depth_stencil_state.as_ref(),
        &PrimitiveStateConfig::LineList,
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &color_bind_group_layout],
    );

    pipeline_manager.add_pipeline(
        "lines_no_depth".to_owned(),
        device,
        swapchain_format,
        &frustum_vert.0,
        &frustum_frag.0,
        None,
        &PrimitiveStateConfig::LineList,
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &color_bind_group_layout],
    );

    pipeline_manager.add_pipeline(
        "alphablended".to_owned(),
        device,
        swapchain_format,
        &vertex_shader.0,
        &fragment_shader.0,
        depth_stencil_state.as_ref(),
        &PrimitiveStateConfig::TriangleList,
        &ColorTargetConfig::AlphaBlend,
        &[&uniform_bind_group_layout, &texture_bind_group_layout],
    );

    pipeline_manager.add_pipeline(
        "alphablended_no_depth".to_owned(),
        device,
        swapchain_format,
        &vertex_shader.0,
        &fragment_shader.0,
        None,
        &PrimitiveStateConfig::TriangleList,
        &ColorTargetConfig::AlphaBlend,
        &[&uniform_bind_group_layout, &texture_bind_group_layout],
    );

    Ok(pipeline_manager)
}
