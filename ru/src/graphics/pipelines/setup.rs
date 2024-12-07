use crate::{
    core::{cache::ComponentCacheKey, error::AppError},
    ecs::{
        components::{instance::model::InstanceRaw, model::model::ModelVertex},
        traits::{Cache, Vertex},
    },
    graphics::{binding::BindGroupLayouts, shaders::manager::ShaderManager, PrimitiveTopology},
};
use wgpu::{ColorTargetState, DepthStencilState, Device, RenderPipeline};

use super::{
    get_pipeline_label, manager::PipelineManager, render_pipeline, DepthType, PrimitiveStateConfig,
    PrimitiveType, ShadingType,
};

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

    let vert_shader = shader_manager.load_shader(
        device,
        ComponentCacheKey::from(vertex_shader_path),
        vertex_shader_path,
    )?;
    let frag_shader = shader_manager.load_shader(
        device,
        ComponentCacheKey::from(fragment_shader_path),
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
        &vert_shader.module,
        &frag_shader.module,
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
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
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

        multiview: None,
        cache: None,
    })
}
pub fn setup_pipeline_manager(
    device: &wgpu::Device,
    depth_format: Option<wgpu::TextureFormat>,
    bind_group_layouts: &BindGroupLayouts,
    hdr_format: wgpu::TextureFormat,
) -> Result<PipelineManager, AppError> {
    let mut pipeline_manager = PipelineManager::new();

    let normal_shader = wgpu::ShaderModuleDescriptor {
        label: Some("Normal Shader"),
        source: wgpu::ShaderSource::Wgsl(
            include_str!("../../assets/shaders/core/normal.wgsl").into(),
        ),
    };

    let light_shader = wgpu::ShaderModuleDescriptor {
        label: Some("Light Shader"),
        source: wgpu::ShaderSource::Wgsl(
            include_str!("../../assets/shaders/core/lighting.wgsl").into(),
        ),
    };

    for topology in [
        PrimitiveTopology::PointList,
        PrimitiveTopology::LineList,
        PrimitiveTopology::LineStrip,
        PrimitiveTopology::TriangleList,
        PrimitiveTopology::TriangleStrip,
    ] {
        let topology_label = topology.label();

        let normal_pipeline_id =
            ComponentCacheKey::from(format!("{}_pipeline", topology_label).as_str());
        pipeline_manager.get_or_create(normal_pipeline_id, || {
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Normal Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &bind_group_layouts.texture_bind_group_layout,
                        &bind_group_layouts.camera_bind_group_layout,
                        &bind_group_layouts.light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

            Ok(create_render_pipeline(
                &device,
                &render_pipeline_layout,
                hdr_format,
                depth_format,
                &[ModelVertex::desc(), InstanceRaw::desc()],
                topology.to_wgpu_topology(),
                normal_shader.clone(),
            ))
        })?;

        let light_pipeline_id =
            ComponentCacheKey::from(format!("{}_light_pipeline", topology_label).as_str());
        pipeline_manager.get_or_create(light_pipeline_id, || {
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Light Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &bind_group_layouts.camera_bind_group_layout,
                        &bind_group_layouts.light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

            Ok(create_render_pipeline(
                &device,
                &render_pipeline_layout,
                hdr_format,
                depth_format,
                &[ModelVertex::desc()],
                topology.to_wgpu_topology(),
                light_shader.clone(),
            ))
        })?;
    }

    let sky_pipeline_id = ComponentCacheKey::from("sky_pipeline");
    pipeline_manager.get_or_create(sky_pipeline_id, || {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sky Pipeline Layout"),
            bind_group_layouts: &[
                &bind_group_layouts.camera_bind_group_layout,
                &bind_group_layouts.environment_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let shader = wgpu::include_wgsl!("../../assets/shaders/objects/skybox.wgsl");
        Ok(create_render_pipeline(
            &device,
            &layout,
            hdr_format,
            depth_format,
            &[],
            wgpu::PrimitiveTopology::TriangleList,
            shader,
        ))
    })?;

    Ok(pipeline_manager)
}
