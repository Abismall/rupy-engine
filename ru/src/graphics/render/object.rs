use wgpu::{DepthStencilState, Device, TextureFormat};

use crate::{
    core::error::AppError,
    pipeline::{
        descriptor::PipelineDescriptor, manager::PipelineManager, DepthType, PrimitiveType,
        ShadingType,
    },
    shader::manager::ShaderManager,
};

use super::cache::Cache;

#[derive(Debug)]
pub struct RenderObject {
    pub primitive_type: PrimitiveType,
    pub shading_type: ShadingType,
    pub depth_type: DepthType,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub vertex_count: u32,
    pub index_count: u32,
    pub vertex_layout: wgpu::VertexBufferLayout<'static>,
}

pub fn render_objects(
    objects: &[RenderObject],
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderManager,
    device: &Device,
    pass: &mut wgpu::RenderPass,
    uniform_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    swapchain_format: TextureFormat,
    depth_stencil_state: &DepthStencilState,
) -> Result<(), AppError> {
    for object in objects {
        let descriptor: PipelineDescriptor = object.into();
        let pipeline = pipeline_manager.get_or_create(
            pipeline_manager.hash_descriptor(&descriptor),
            || {
                create_pipeline(
                    &descriptor,
                    shader_manager,
                    device,
                    uniform_layout,
                    texture_layout,
                    swapchain_format,
                    depth_stencil_state,
                )
            },
        )?;
        pass.set_pipeline(&pipeline);

        pass.set_vertex_buffer(0, object.vertex_buffer.slice(..));
        if let Some(index_buffer) = &object.index_buffer {
            pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..object.index_count, 0, 0..1);
        } else {
            pass.draw(0..object.vertex_count, 0..1);
        }
    }
    Ok(())
}

impl Into<PipelineDescriptor> for RenderObject {
    fn into(self) -> PipelineDescriptor {
        PipelineDescriptor {
            primitive_type: self.primitive_type,
            shading_type: self.shading_type,
            depth_type: self.depth_type,
            vertex_layout: self.vertex_layout,
        }
    }
}
impl Into<PipelineDescriptor> for &RenderObject {
    fn into(self) -> PipelineDescriptor {
        PipelineDescriptor {
            primitive_type: PrimitiveType::from(self.primitive_type.clone()),
            shading_type: ShadingType::from(self.shading_type.clone()),
            depth_type: DepthType::from(self.depth_type.clone()),
            vertex_layout: self.vertex_layout.clone(),
        }
    }
}
