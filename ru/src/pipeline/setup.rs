use crate::{
    core::error::AppError,
    ecs::{
        components::model::{Vertex2D, Vertex3D},
        pipelines::PipelineManager,
        shaders::ShaderManager,
    },
    shader::module::RupyShader,
};
use wgpu::{DepthStencilState, Device, TextureFormat};

use super::{
    get_pipeline_label,
    state::{ColorTargetConfig, DepthType, PrimitiveStateConfig, PrimitiveType, ShadingType},
};

fn load_shaders(
    shader_manager: &mut ShaderManager,
    device: &Device,
    vertex_path: &str,
    fragment_path: &str,
) -> Result<(std::sync::Arc<RupyShader>, std::sync::Arc<RupyShader>), AppError> {
    let vertex_shader = shader_manager
        .get_or_create(device, vertex_path, "vs_main".into(), "fs_main".into())?
        .clone();
    let fragment_shader = shader_manager
        .get_or_create(device, fragment_path, "vs_main".into(), "fs_main".into())?
        .clone();

    Ok((vertex_shader, fragment_shader))
}
fn create_2d_pipeline(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderManager,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    primitive_type: &PrimitiveType,
    shading_type: &ShadingType,
    depth_type: &DepthType,
) -> Result<(), AppError> {
    let label = get_pipeline_label(primitive_type, shading_type, depth_type);

    let (vertex_shader_path, fragment_shader_path) = match shading_type {
        ShadingType::Color => (
            "static/shaders/2d_unlit_vertex.wgsl",
            "static/shaders/2d_unlit_fragment.wgsl",
        ),
        ShadingType::Texture => (
            "static/shaders/2d_lit_vertex.wgsl",
            "static/shaders/2d_lit_fragment.wgsl",
        ),
    };

    let (vertex_shader, fragment_shader) = load_shaders(
        shader_manager,
        device,
        vertex_shader_path,
        fragment_shader_path,
    )?;

    let primitive_state = match primitive_type {
        PrimitiveType::Line => PrimitiveStateConfig::Custom {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Line,
        },
        PrimitiveType::Triangle => PrimitiveStateConfig::TriangleList,
        PrimitiveType::Quad => PrimitiveStateConfig::TriangleList,
    };
    pipeline_manager.add_pipeline::<Vertex2D>(
        &label,
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
        None,
        &primitive_state,
        &ColorTargetConfig::Replace,
        &uniform_layout,
        &texture_layout,
    );

    Ok(())
}
fn create_pipeline(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderManager,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    primitive_type: PrimitiveType,
    shading_type: ShadingType,
    depth_type: DepthType,
    depth_stencil_state: &DepthStencilState,
) -> Result<(), AppError> {
    let label = get_pipeline_label(&primitive_type, &shading_type, &depth_type);

    let (vertex_shader_path, fragment_shader_path) = match shading_type {
        ShadingType::Color => (
            "static/shaders/unlit_vertex.wgsl",
            "static/shaders/unlit_fragment.wgsl",
        ),
        ShadingType::Texture => (
            "static/shaders/lit_vertex.wgsl",
            "static/shaders/lit_fragment.wgsl",
        ),
    };

    let (vertex_shader, fragment_shader) = load_shaders(
        shader_manager,
        device,
        vertex_shader_path,
        fragment_shader_path,
    )?;

    let depth_stencil_state = match depth_type {
        DepthType::Depth => Some(depth_stencil_state),
        DepthType::NoDepth => None,
    };

    let primitive_state = match primitive_type {
        PrimitiveType::Line => PrimitiveStateConfig::Custom {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Line,
        },
        PrimitiveType::Triangle => PrimitiveStateConfig::TriangleList,
        PrimitiveType::Quad => PrimitiveStateConfig::TriangleList,
    };

    pipeline_manager.add_pipeline::<Vertex3D>(
        &label,
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
        depth_stencil_state,
        &primitive_state,
        &ColorTargetConfig::AlphaBlend,
        &uniform_layout,
        &texture_layout,
    );

    Ok(())
}

pub fn setup_pipelines(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderManager,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    depth_stencil_state: &DepthStencilState,
) -> Result<(), AppError> {
    create_2d_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        &PrimitiveType::Quad,
        &ShadingType::Color,
        &DepthType::NoDepth,
    )?;
    create_2d_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        &PrimitiveType::Quad,
        &ShadingType::Texture,
        &DepthType::NoDepth,
    )?;

    create_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Line,
        ShadingType::Texture,
        DepthType::NoDepth,
        depth_stencil_state,
    )?;

    create_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Line,
        ShadingType::Texture,
        DepthType::Depth,
        depth_stencil_state,
    )?;

    create_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Triangle,
        ShadingType::Texture,
        DepthType::NoDepth,
        depth_stencil_state,
    )?;

    create_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Triangle,
        ShadingType::Texture,
        DepthType::Depth,
        depth_stencil_state,
    )?;

    create_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Line,
        ShadingType::Color,
        DepthType::NoDepth,
        depth_stencil_state,
    )?;

    create_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Line,
        ShadingType::Color,
        DepthType::Depth,
        depth_stencil_state,
    )?;

    create_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Triangle,
        ShadingType::Color,
        DepthType::NoDepth,
        depth_stencil_state,
    )?;

    create_pipeline(
        pipeline_manager,
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Triangle,
        ShadingType::Color,
        DepthType::Depth,
        depth_stencil_state,
    )?;

    Ok(())
}
