use std::collections::HashMap;

use wgpu::{
    BindGroupLayout, BindGroupLayoutEntry, Device, PipelineLayoutDescriptor, PushConstantRange,
    RenderPipeline,
};

use crate::shader::create_shader_modules;

use super::pipeline::create_render_pipeline;

use wgpu::{ShaderModule, TextureFormat};

pub struct PipelineManager {
    pipelines: HashMap<String, RenderPipeline>,
    pub bind_group_layout: BindGroupLayout, // Expose the layout for other systems
}

impl PipelineManager {
    pub fn new(device: &Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        Self {
            pipelines: HashMap::new(),
            bind_group_layout,
        }
    }

    /// Add a new render pipeline to the manager, using given shaders.
    pub fn add_pipeline(
        &mut self,
        name: String,
        device: &Device,
        swapchain_format: TextureFormat,
        vertex_shader: &ShaderModule,
        fragment_shader: &ShaderModule,
    ) {
        let render_pipeline = create_render_pipeline(
            device,
            swapchain_format,
            &self.bind_group_layout,
            vertex_shader,
            fragment_shader,
        );
        self.pipelines.insert(name.to_string(), render_pipeline);
    }
    pub fn get_or_create_pipeline(
        &mut self,
        name: String,
        device: &Device,
        swapchain_format: TextureFormat,
        vertex_shader: &ShaderModule,
        fragment_shader: &ShaderModule,
    ) -> Option<&RenderPipeline> {
        match self.pipelines.contains_key(&name) {
            true => match self.pipelines.get(&name) {
                Some(existing_pipeline) => return Some(existing_pipeline),
                None => None,
            },
            false => {
                let render_pipeline = create_render_pipeline(
                    device,
                    swapchain_format,
                    &self.bind_group_layout,
                    vertex_shader,
                    fragment_shader,
                );
                self.pipelines.insert(name.to_string(), render_pipeline);
                match self.pipelines.get(&name) {
                    Some(new_pipeline) => Some(new_pipeline),
                    None => None,
                }
            }
        }
    }
    /// Retrieve a specific render pipeline by its name.
    pub fn get_pipeline(&self, name: &str) -> Option<&RenderPipeline> {
        self.pipelines.get(name)
    }

    /// Utility to create a bind group layout based on provided entries.
    pub fn create_bind_group_layout(
        &self,
        label: Option<&str>,
        device: &Device,
        entries: &[BindGroupLayoutEntry],
    ) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label, entries })
    }

    /// Create a pipeline layout based on provided layouts and push constants.
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

pub fn setup_pipeline_manager(device: &Device, swapchain_format: TextureFormat) -> PipelineManager {
    let mut pipeline_manager = PipelineManager::new(device);

    let (vertex_shader, fragment_shader) = create_shader_modules(device);

    pipeline_manager.add_pipeline(
        "RenderPipeline".to_owned(),
        device,
        swapchain_format,
        &vertex_shader,
        &fragment_shader,
    );

    pipeline_manager
}
