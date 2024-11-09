pub mod cache;
pub mod key;
pub mod state;

use cache::PipelineManager;
use state::{ColorTargetConfig, PrimitiveStateConfig};
use wgpu::{Device, TextureFormat};

use crate::{
    app::context::default_depth_stencil_state,
    ecs::components::vertex::Vertex,
    graphics::{
        LINE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL, LINE_LIST_COLORED_PIPELINE_LABEL,
        LINE_LIST_TEXTURED_DEPTH_VIEW_PIPELINE_LABEL, LINE_LIST_TEXTURED_PIPELINE_LABEL,
        TRIANGLE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL, TRIANGLE_LIST_COLORED_PIPELINE_LABEL,
        TRIANGLE_LIST_TEXTURED_DEPTH_VIEW_PIPELINE_LABEL, TRIANGLE_LIST_TEXTURED_PIPELINE_LABEL,
    },
    shader::library::ShaderLibrary,
};

pub fn create_render_pipeline(
    device: &wgpu::Device,
    name: &String,
    texture_format: wgpu::TextureFormat,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    depth_stencil: Option<&wgpu::DepthStencilState>,
    primitive_state_config: &PrimitiveStateConfig,
    color_target_config: &ColorTargetConfig,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&name),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(name),
                bind_group_layouts,
                push_constant_ranges: &[],
            }),
        ),
        vertex: wgpu::VertexState {
            module: vertex_shader,
            entry_point: "vs_main",
            buffers: &[Vertex::buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: fragment_shader,
            entry_point: "fs_main",
            targets: &[Some(
                color_target_config.to_color_target_state(texture_format),
            )],
            compilation_options: Default::default(),
        }),
        primitive: primitive_state_config.to_primitive_state(),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: Default::default(),
        depth_stencil: depth_stencil.cloned(),
    })
}

pub fn line_list_textured_with_depth(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderLibrary,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) {
    let vertex_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/lit_vertex.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    let fragment_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/lit_fragment.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");

    let depth_stencil_state = Some(default_depth_stencil_state(None));

    pipeline_manager.add_pipeline(
        LINE_LIST_TEXTURED_DEPTH_VIEW_PIPELINE_LABEL.into(),
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
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
}

pub fn line_list_textured_with_no_depth(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderLibrary,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) {
    let vertex_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/lit_vertex.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    let fragment_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/lit_fragment.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");

    pipeline_manager.add_pipeline(
        LINE_LIST_TEXTURED_PIPELINE_LABEL.into(),
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
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
}

pub fn triangle_list_textured_with_depth(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderLibrary,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) {
    let vertex_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/lit_vertex.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    let fragment_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/lit_fragment.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");

    let depth_stencil_state = Some(default_depth_stencil_state(None));

    pipeline_manager.add_pipeline(
        TRIANGLE_LIST_TEXTURED_DEPTH_VIEW_PIPELINE_LABEL.into(),
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
        depth_stencil_state.as_ref(),
        &PrimitiveStateConfig::TriangleList,
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &texture_bind_group_layout],
    );
}

pub fn triangle_list_textured_with_no_depth(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderLibrary,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) {
    let vertex_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/lit_vertex.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    let fragment_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/lit_fragment.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    pipeline_manager.add_pipeline(
        TRIANGLE_LIST_TEXTURED_PIPELINE_LABEL.into(),
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
        None,
        &PrimitiveStateConfig::TriangleList,
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &texture_bind_group_layout],
    );
}
pub fn line_list_colored_with_depth(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderLibrary,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    color_bind_group_layout: &wgpu::BindGroupLayout,
) {
    let vertex_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/unlit_vertex.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    let fragment_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/unlit_fragment.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");

    let depth_stencil_state = Some(default_depth_stencil_state(None));

    pipeline_manager.add_pipeline(
        LINE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL.into(),
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
        depth_stencil_state.as_ref(),
        &PrimitiveStateConfig::Custom {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Line,
        },
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &color_bind_group_layout],
    );
}

pub fn line_list_colored_with_no_depth(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderLibrary,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    color_bind_group_layout: &wgpu::BindGroupLayout,
) {
    let vertex_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/unlit_vertex.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    let fragment_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/unlit_fragment.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");

    pipeline_manager.add_pipeline(
        LINE_LIST_COLORED_PIPELINE_LABEL.into(),
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
        None,
        &PrimitiveStateConfig::Custom {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Line,
        },
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &color_bind_group_layout],
    );
}

pub fn triangle_list_colored_with_depth(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderLibrary,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    color_bind_group_layout: &wgpu::BindGroupLayout,
) {
    let vertex_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/unlit_vertex.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    let fragment_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/unlit_fragment.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");

    let depth_stencil_state = Some(default_depth_stencil_state(None));

    pipeline_manager.add_pipeline(
        TRIANGLE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL.into(),
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
        depth_stencil_state.as_ref(),
        &PrimitiveStateConfig::TriangleList,
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &color_bind_group_layout],
    );
}

pub fn triangle_list_colored_with_no_depth(
    pipeline_manager: &mut PipelineManager,
    shader_manager: &mut ShaderLibrary,
    device: &Device,
    swapchain_format: TextureFormat,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    color_bind_group_layout: &wgpu::BindGroupLayout,
) {
    let vertex_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/unlit_vertex.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    let fragment_shader = shader_manager
        .get_or_create(
            device,
            "static/shaders/unlit_fragment.wgsl",
            "vs_main".into(),
            "fs_main".into(),
        )
        .expect("Failed to load shaders");
    pipeline_manager.add_pipeline(
        TRIANGLE_LIST_COLORED_PIPELINE_LABEL.into(),
        device,
        swapchain_format,
        &vertex_shader.module,
        &fragment_shader.module,
        None,
        &PrimitiveStateConfig::TriangleList,
        &ColorTargetConfig::Replace,
        &[&uniform_bind_group_layout, &color_bind_group_layout],
    )
}
