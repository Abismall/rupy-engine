use crate::{
    core::error::AppError,
    ecs::traits::Cache,
    graphics::{
        binding::BindGroupLayouts,
        shaders::{manager::ShaderManager, module::RupyShader},
    },
    prelude::{
        cache::CacheKey,
        constant::{WGSL_FS_MAIN, WGSL_VS_MAIN},
    },
};
use wgpu::{BlendState, CommandEncoder, Device, PipelineLayout, TextureFormat, TextureView};
use winit::dpi::PhysicalSize;
pub trait PipelineBase {
    fn create_layout(device: &Device, bind_group_layouts: &BindGroupLayouts) -> PipelineLayout;
    fn new(
        device: &wgpu::Device,
        format: TextureFormat,
        width: u32,
        height: u32,
        shader_manager: &mut ShaderManager,
        bind_group_layouts: &BindGroupLayouts,
    ) -> Self;
    fn resize<P: winit::dpi::Pixel>(
        &mut self,
        device: &Device,
        new_size: PhysicalSize<P>,
        bind_group_layouts: &BindGroupLayouts,
    );
    fn view(&self) -> &TextureView;
    fn format(&self) -> TextureFormat;
    fn process(&self, encoder: &mut CommandEncoder, output: &TextureView);
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    topology: wgpu::PrimitiveTopology,
    shader_path: &str,
    shader_manager: &mut ShaderManager,
) -> Result<wgpu::RenderPipeline, AppError> {
    let key = CacheKey::from(shader_path);

    let shader = shader_manager.shaders.get_or_create(key, || {
        if let Ok(shader) = RupyShader::load(device, shader_path) {
            Ok(shader)
        } else {
            Err(AppError::ResourceNotFound(format!(
                "No shader found for {}",
                shader_path
            )))
        }
    })?;

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(shader_path),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader.module,
            entry_point: &WGSL_VS_MAIN,
            buffers: vertex_layouts,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader.module,
            entry_point: &WGSL_FS_MAIN,
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology,
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
            depth_compare: wgpu::CompareFunction::LessEqual,
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
    });

    Ok(pipeline)
}
