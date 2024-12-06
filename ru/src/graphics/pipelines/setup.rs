use wgpu::{ColorTargetState, DepthStencilState, Device, RenderPipeline};

use crate::{
    core::error::AppError,
    graphics::shaders::{manager::ShaderManager, module::RupyShader},
};

use super::{
    get_pipeline_label, render_pipeline, DepthType, PrimitiveStateConfig, PrimitiveType,
    ShadingType,
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

pub fn create_pipeline(
    shader_manager: &mut ShaderManager,
    device: &Device,
    uniform_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    texture_layout: &wgpu::BindGroupLayout,
    primitive_type: PrimitiveType,
    shading_type: ShadingType,
    depth_type: DepthType,
    depth_stencil_state: &DepthStencilState,
    color_target: ColorTargetState,
) -> Result<RenderPipeline, AppError> {
    let label = get_pipeline_label(&primitive_type, &shading_type, &depth_type);

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

    Ok(render_pipeline(
        device,
        &label,
        uniform_layout,
        instance_layout,
        texture_layout,
        &vertex_shader.module,
        &fragment_shader.module,
        depth_stencil_state,
        color_target,
        primitive_state.to_primitive_state(),
    ))
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    topology: wgpu::PrimitiveTopology, // NEW!
    shader: wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(shader);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&format!("{:?}", shader)),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_layouts,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology, // NEW!
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual, // UDPATED!
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
        cache: None,
    })
}
