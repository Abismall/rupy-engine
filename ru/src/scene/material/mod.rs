pub mod shaded;
pub mod textured;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MaterialStructure {
    pub bind_group_layout: Arc<wgpu::BindGroupLayout>,
    pub pipeline_layout: Arc<wgpu::PipelineLayout>,
    pub bind_group: Arc<wgpu::BindGroup>,
    pub pipeline: Arc<wgpu::RenderPipeline>,
}
impl Material for MaterialStructure {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
impl MaterialStructure {
    pub fn new(
        bind_group_layout: wgpu::BindGroupLayout,
        pipeline_layout: wgpu::PipelineLayout,
        bind_group: wgpu::BindGroup,
        pipeline: wgpu::RenderPipeline,
    ) -> Self {
        Self {
            pipeline_layout: Arc::new(pipeline_layout),
            bind_group_layout: Arc::new(bind_group_layout),
            bind_group: Arc::new(bind_group),
            pipeline: Arc::new(pipeline),
        }
    }
}

/// Represents a GPU-based material, defining the shader pipeline and associated bind groups.
pub trait Material {
    /// Returns the render pipeline associated with this material.
    fn pipeline(&self) -> &wgpu::RenderPipeline;

    /// Returns the bind group associated with this material, containing GPU resources like textures and buffers.
    fn bind_group(&self) -> &wgpu::BindGroup;
}
