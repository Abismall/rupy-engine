use crate::{
    core::error::AppError,
    ecs::model::{Vertex2D, Vertex3D},
    gpu::InstanceData,
    prelude::constant::{WGSL_FRAGMENT_MAIN_DEFAULT, WGSL_VERTEX_MAIN_DEFAULT},
    shader::{manager::ShaderManager, module::RupyShader},
};
use wgpu::{DepthStencilState, Device, RenderPipeline, TextureFormat};

use super::{
    get_pipeline_label,
    manager::PipelineManager,
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
pub fn render_pipeline_2d(
    shader_manager: &mut ShaderManager,
    device: &Device,
    swapchain_format: &TextureFormat,
    uniform_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    primitive_type: &PrimitiveType,
    shading_type: &ShadingType,
    depth_type: &DepthType,
    depth_stencil_state: &DepthStencilState,
) -> Result<RenderPipeline, AppError> {
    let pipeline_label = &get_pipeline_label(primitive_type, shading_type, depth_type);

    let (vertex_shader_path, fragment_shader_path) = match shading_type {
        ShadingType::Color => (
            "static/shaders/unlit/default/2d_vert.wgsl",
            "static/shaders/unlit/default/2d_frag.wgsl",
        ),
        ShadingType::Texture => (
            "static/shaders/lit/default/2d_vert.wgsl",
            "static/shaders/lit/default/2d_frag.wgsl",
        ),
    };

    let (vertex_shader, fragment_shader) = load_shaders(
        shader_manager,
        device,
        vertex_shader_path,
        fragment_shader_path,
    )?;

    let primitive = match primitive_type {
        PrimitiveType::Line => PrimitiveStateConfig::Custom {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Line,
        },
        PrimitiveType::Triangle => PrimitiveStateConfig::TriangleList,
        PrimitiveType::Quad => PrimitiveStateConfig::TriangleList,
    };
    let targets = &[Some(
        ColorTargetConfig::AlphaBlend.to_color_target_state(*swapchain_format),
    )];
    let depth_stencil_state = match depth_type {
        DepthType::Depth => Some(depth_stencil_state),
        DepthType::NoDepth => None,
    };

    let layout = &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(pipeline_label),
        bind_group_layouts: &[&uniform_layout, &texture_layout],
        push_constant_ranges: &[],
    });

    Ok(
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(pipeline_label),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader.module,
                entry_point: WGSL_VERTEX_MAIN_DEFAULT,
                buffers: &[Vertex2D::buffer_layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader.module,
                entry_point: WGSL_FRAGMENT_MAIN_DEFAULT,
                targets,
                compilation_options: Default::default(),
            }),
            primitive: primitive.to_primitive_state(),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: Default::default(),
            depth_stencil: depth_stencil_state.cloned(),
        }),
    )
}
pub fn render_pipeline_3d(
    shader_manager: &mut ShaderManager,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    primitive_type: PrimitiveType,
    shading_type: ShadingType,
    depth_type: DepthType,
    depth_stencil_state: &DepthStencilState,
) -> Result<RenderPipeline, AppError> {
    let pipeline_label = get_pipeline_label(&primitive_type, &shading_type, &depth_type);

    let (vertex_shader_path, fragment_shader_path) = match shading_type {
        ShadingType::Color => (
            "static/shaders/unlit/default/3d_vert.wgsl",
            "static/shaders/unlit/default/3d_frag.wgsl",
        ),
        ShadingType::Texture => (
            "static/shaders/lit/default/3d_vert.wgsl",
            "static/shaders/lit/default/3d_frag.wgsl",
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

    let primitive = match primitive_type {
        PrimitiveType::Line => PrimitiveStateConfig::Custom {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Line,
        },
        PrimitiveType::Triangle => PrimitiveStateConfig::TriangleList,
        PrimitiveType::Quad => PrimitiveStateConfig::TriangleList,
    };
    let targets = &[Some(
        ColorTargetConfig::AlphaBlend.to_color_target_state(swapchain_format),
    )];
    let layout = match shading_type {
        ShadingType::Color => &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&pipeline_label),
            bind_group_layouts: &[&uniform_layout],
            push_constant_ranges: &[],
        }),
        ShadingType::Texture => &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&pipeline_label),
            bind_group_layouts: &[&uniform_layout, &texture_layout],
            push_constant_ranges: &[],
        }),
    };
    Ok(
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&pipeline_label),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader.module,
                entry_point: WGSL_VERTEX_MAIN_DEFAULT,
                buffers: &[Vertex3D::buffer_layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader.module,
                entry_point: WGSL_FRAGMENT_MAIN_DEFAULT,
                targets,
                compilation_options: Default::default(),
            }),
            primitive: primitive.to_primitive_state(),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: Default::default(),
            depth_stencil: depth_stencil_state.cloned(),
        }),
    )
}

pub fn setup_render_pipelines(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderManager,
    device: &Device,
    uniform_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    swapchain_format: &TextureFormat,
    depth_stencil_state: &DepthStencilState,
) -> Result<(), AppError> {
    let quad_color = render_pipeline_2d(
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        &PrimitiveType::Quad,
        &ShadingType::Color,
        &DepthType::NoDepth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(
            &PrimitiveType::Quad,
            &ShadingType::Color,
            &DepthType::NoDepth,
        ),
        quad_color,
    );
    let quad_texture = render_pipeline_2d(
        shader_manager,
        device,
        swapchain_format,
        &uniform_layout,
        &texture_layout,
        &PrimitiveType::Quad,
        &ShadingType::Texture,
        &DepthType::NoDepth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(
            &PrimitiveType::Quad,
            &ShadingType::Texture,
            &DepthType::NoDepth,
        ),
        quad_texture,
    );
    let line_texture_no_depth = render_pipeline_3d(
        shader_manager,
        device,
        *swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Line,
        ShadingType::Texture,
        DepthType::NoDepth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(
            &PrimitiveType::Line,
            &ShadingType::Texture,
            &DepthType::NoDepth,
        ),
        line_texture_no_depth,
    );
    let line_texture_with_depth = render_pipeline_3d(
        shader_manager,
        device,
        *swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Line,
        ShadingType::Texture,
        DepthType::Depth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(
            &PrimitiveType::Line,
            &ShadingType::Texture,
            &DepthType::Depth,
        ),
        line_texture_with_depth,
    );
    let triangle_texture_no_depth = render_pipeline_3d(
        shader_manager,
        device,
        *swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Triangle,
        ShadingType::Texture,
        DepthType::NoDepth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(
            &PrimitiveType::Triangle,
            &ShadingType::Texture,
            &DepthType::NoDepth,
        ),
        triangle_texture_no_depth,
    );
    let triangle_texture_with_depth = render_pipeline_3d(
        shader_manager,
        device,
        *swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Triangle,
        ShadingType::Texture,
        DepthType::Depth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(
            &PrimitiveType::Triangle,
            &ShadingType::Texture,
            &DepthType::Depth,
        ),
        triangle_texture_with_depth,
    );
    let line_color_no_depth = render_pipeline_3d(
        shader_manager,
        device,
        *swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Triangle,
        ShadingType::Color,
        DepthType::NoDepth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(
            &PrimitiveType::Line,
            &ShadingType::Color,
            &DepthType::NoDepth,
        ),
        line_color_no_depth,
    );
    let line_color_with_depth = render_pipeline_3d(
        shader_manager,
        device,
        *swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Line,
        ShadingType::Color,
        DepthType::Depth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(&PrimitiveType::Line, &ShadingType::Color, &DepthType::Depth),
        line_color_with_depth,
    );
    let triangle_color_no_depth = render_pipeline_3d(
        shader_manager,
        device,
        *swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Triangle,
        ShadingType::Color,
        DepthType::NoDepth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(
            &PrimitiveType::Triangle,
            &ShadingType::Color,
            &DepthType::NoDepth,
        ),
        triangle_color_no_depth,
    );
    let triangle_color_with_depth = render_pipeline_3d(
        shader_manager,
        device,
        *swapchain_format,
        &uniform_layout,
        &texture_layout,
        PrimitiveType::Triangle,
        ShadingType::Color,
        DepthType::Depth,
        depth_stencil_state,
    )?;
    pipeline_manager.add_pipeline(
        &get_pipeline_label(
            &PrimitiveType::Triangle,
            &ShadingType::Color,
            &DepthType::Depth,
        ),
        triangle_color_with_depth,
    );
    Ok(())
}
