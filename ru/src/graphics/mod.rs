use binding::BindGroupLayouts;
use pipelines::{manager::PipelineManager, setup::setup_pipeline_manager};
use shaders::manager::ShaderManager;
use textures::Texture;

use crate::{
    core::error::AppError,
    ecs::{
        components::{
            material::manager::MaterialManager, mesh::manager::MeshManager,
            model::manager::ModelManager, transform::manager::TransformManager,
        },
        systems::render::BufferManager,
    },
    log_error,
};

pub mod binding;
pub mod context;
pub mod depth;
pub mod geometry;
pub mod global;
pub mod glyphon;
pub mod model;
pub mod pipelines;
pub mod shaders;
pub mod textures;
pub struct ResourceManager {
    pub model_manager: ModelManager,
    pub mesh_manager: MeshManager,
    pub material_manager: MaterialManager,
    pub buffer_manager: BufferManager,
    pub transform_manager: TransformManager,
    pub pipeline_manager: PipelineManager,
    pub shader_manager: ShaderManager,
}
impl ResourceManager {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layouts: &BindGroupLayouts,
        hdr_format: wgpu::TextureFormat,
    ) -> Result<Self, AppError> {
        let pipeline_manager = match setup_pipeline_manager(
            device,
            Some(Texture::DEPTH_FORMAT),
            &bind_group_layouts,
            hdr_format,
        ) {
            Ok(pipelines) => pipelines,
            Err(e) => {
                log_error!("Failed to setup pipeline manager: {:?}", e);
                return Err(e);
            }
        };
        Ok(Self {
            pipeline_manager,
            material_manager: MaterialManager::new(),
            mesh_manager: MeshManager::new(),
            model_manager: ModelManager::new(),
            transform_manager: TransformManager::new(),
            buffer_manager: BufferManager::new(),
            shader_manager: ShaderManager::new(),
        })
    }
}
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

impl PrimitiveTopology {
    pub fn label(&self) -> &'static str {
        match self {
            PrimitiveTopology::PointList => "point_list",
            PrimitiveTopology::LineList => "line_list",
            PrimitiveTopology::LineStrip => "line_strip",
            PrimitiveTopology::TriangleList => "triangle_list",
            PrimitiveTopology::TriangleStrip => "triangle_strip",
        }
    }

    pub fn to_wgpu_topology(&self) -> wgpu::PrimitiveTopology {
        match self {
            PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
            PrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
            PrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
        }
    }
    pub fn next(self) -> PrimitiveTopology {
        use PrimitiveTopology::*;
        match self {
            LineList => PrimitiveTopology::LineStrip,
            LineStrip => PrimitiveTopology::TriangleList,
            TriangleList => PrimitiveTopology::TriangleStrip,
            TriangleStrip => PrimitiveTopology::PointList,
            PointList => PrimitiveTopology::LineList,
        }
    }
}
