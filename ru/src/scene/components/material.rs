use crate::core::error::AppError;
use crate::graphics::{
    binding::global_bind_group_layout_cache::get_bind_group_layout,
    pipeline::{cache_key::PipelineCacheKey, pipeline_cache::PipelineCache},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum MaterialType {
    Unlit,
    Textured,
    Default,
}

impl MaterialType {
    pub fn get_bind_group_layout_labels(&self) -> Vec<String> {
        match self {
            MaterialType::Default => vec![String::from("GlobalUniforms")],
            MaterialType::Unlit => vec![
                String::from("GlobalUniforms"),
                String::from("ModelUniforms"),
            ],
            MaterialType::Textured => vec![
                String::from("GlobalUniforms"),
                String::from("ModelUniforms"),
                String::from("Texture"),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    pub material_type: MaterialType,
    pub shader_path: String,
    pub vertex_entry_point: String,
    pub fragment_entry_point: String,
    pub bind_group_layouts: Vec<Arc<wgpu::BindGroupLayout>>,
    pub topology: wgpu::PrimitiveTopology,
    pub front_face: wgpu::FrontFace,
    pub cull_mode: Option<wgpu::Face>,
    pub polygon_mode: wgpu::PolygonMode,
    pub depth_stencil_state: Option<wgpu::DepthStencilState>,
    pub pipeline: Option<Arc<wgpu::RenderPipeline>>,
}

impl Material {
    pub fn new_with_type(
        material_type: MaterialType,
        device: &wgpu::Device,
        pipeline_cache: &mut PipelineCache,
        shader_module_path: &str,
        vertex_entry_point: &str,
        fragment_entry_point: &str,
        topology: wgpu::PrimitiveTopology,
        front_face: wgpu::FrontFace,
        cull_mode: Option<wgpu::Face>,
        polygon_mode: wgpu::PolygonMode,
        depth_stencil_state: Option<wgpu::DepthStencilState>,
        swap_chain_format: wgpu::TextureFormat,
    ) -> Result<Self, AppError> {
        let bind_group_layout_labels = material_type.get_bind_group_layout_labels();

        let pipeline_key = PipelineCacheKey {
            shader_path: String::from(shader_module_path),
            vertex_entry_point: String::from(vertex_entry_point),
            fragment_entry_point: String::from(fragment_entry_point),
            bind_group_layout_labels: bind_group_layout_labels.clone(),
            topology,
            front_face,
            cull_mode,
            polygon_mode,
        };

        let pipeline = pipeline_cache.get_or_create_pipeline(
            device,
            pipeline_key.clone(),
            swap_chain_format,
        )?;

        let bind_group_layouts: Vec<Arc<wgpu::BindGroupLayout>> = bind_group_layout_labels
            .iter()
            .map(|label| {
                get_bind_group_layout(label)
                    .expect(&format!("BindGroupLayout '{}' not found in cache", label))
                    .clone()
            })
            .collect();

        Ok(Self {
            material_type,
            shader_path: String::from(shader_module_path),
            vertex_entry_point: String::from(vertex_entry_point),
            fragment_entry_point: String::from(fragment_entry_point),
            bind_group_layouts,
            topology,
            front_face,
            cull_mode,
            polygon_mode,
            depth_stencil_state,
            pipeline: Some(pipeline),
        })
    }
}
