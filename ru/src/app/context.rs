use std::{collections::HashMap, sync::Arc};

use super::DebugMode;
use crate::{
    camera::{Camera, Frustum},
    core::{
        error::AppError,
        surface::{default_surface_configuration, RenderSurface},
    },
    ecs::{
        components::{
            create_model_matrix,
            model::{Material, Mesh},
            uniform::{Transform, UniformColor, Uniforms},
        },
        systems::{materials::MaterialManager, world::World},
    },
    graphics::{
        binding::{
            color_bind_group, color_bind_group_layout, uniform_bind_group,
            uniform_bind_group_layout, BindingLayouts,
        },
        buffer::{color_buffer, uniform_buffer},
        global::{get_adapter, get_device, get_instance, get_queue},
        glyphon::GlyphonManager,
        RenderMode, LINE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL,
    },
    log_debug, log_error,
    pipeline::cache::{setup_pipeline_manager, PipelineManager},
    prelude::frame::FrameTime,
    shader::library::ShaderLibrary,
    texture::{create_depth_textures, library::TextureManager},
};
use glyphon::Resolution;

use wgpu::{util::DeviceExt, Color, DepthStencilState, Operations};
use winit::window::Window;

pub fn default_depth_stencil_state(format: Option<wgpu::TextureFormat>) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: format.unwrap_or(wgpu::TextureFormat::Depth32Float),
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    }
}

pub struct LayoutBindings {
    pub uniform: wgpu::BindGroupLayout,
    pub texture: wgpu::BindGroupLayout,
    pub color: wgpu::BindGroupLayout,
}

pub struct RenderContext {
    pub render_surface: RenderSurface,
    pub adapter: Arc<wgpu::Adapter>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,

    pub frame: FrameTime,
    pub depth_texture: Option<Arc<wgpu::Texture>>,
    pub depth_texture_view: Option<Arc<wgpu::TextureView>>,
    pub frustum: Option<Frustum>,
    pub depth_ops: Operations<f32>,
    pub depth_stencil_state: DepthStencilState,
    pub world: World,

    pub camera: Camera,
    pub last_mouse_position: winit::dpi::PhysicalPosition<f64>,

    pub material_manager: MaterialManager,
    pub pipeline_manager: PipelineManager,
    pub texture_manager: TextureManager,

    pub shader_manager: ShaderLibrary,
    pub debug: DebugMode,
    pub mode: RenderMode,
    pub text_rendering_system: GlyphonManager,
    pub ops: Operations<Color>,
    pub color_bind_group: wgpu::BindGroup,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub color_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
    pub color_uniform_buffer: wgpu::Buffer,
    pub window: Arc<Window>,
}
impl RenderContext {
    pub fn update_frustum(&mut self) {
        let vp_matrix = self.camera.view_projection_matrix();
        self.frustum = Some(Frustum::from_view_projection_matrix(&vp_matrix));
    }

    pub fn draw_frustum<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        depth_stencil_attachment: Option<&wgpu::RenderPassDepthStencilAttachment<'a>>,
    ) {
        if let Some(frustum) = &self.frustum {
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Frustum Vertex Buffer"),
                    contents: bytemuck::bytes_of(frustum),
                    usage: wgpu::BufferUsages::VERTEX,
                });

            let indices = [
                0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
            ];
            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Frustum Index Buffer"),
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

            let pipeline = match self
                .pipeline_manager
                .get_pipeline(LINE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL)
            {
                Some(lines) => lines,
                None => {
                    log_debug!("Pipeline not found");
                    return;
                }
            };

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(LINE_LIST_COLORED_DEPTH_VIEW_PIPELINE_LABEL),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: depth_stencil_attachment.cloned(),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_bind_group(1, &self.color_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        } else {
            log_debug!("Frustum not available for drawing");
        }
    }
}

impl RenderContext {
    pub async fn new(
        window: Arc<Window>,
        mode: RenderMode,
        debug: DebugMode,
    ) -> Result<Self, AppError> {
        let (adapter, device, queue, instance) = Self::initialize_graphics().await;
        let surface = instance.create_surface(window.clone())?;
        let surface_config = default_surface_configuration(&surface, &adapter, &window);

        let mut shader_manager = ShaderLibrary::new();
        let mut pipeline_manager = PipelineManager::new();
        let _ = Self::initialize_pipeline_manager(
            &device,
            surface_config.format,
            &mut shader_manager,
            &mut pipeline_manager,
        )?;
        let material_manager = MaterialManager::new();
        let texture_manager = TextureManager::new(queue.clone(), device.clone());

        let render_surface = RenderSurface::new(window.clone(), surface, surface_config);

        let color_uniform_buffer = color_buffer(&device, UniformColor::default());
        let color_bind_group = color_bind_group(
            &device,
            &color_uniform_buffer,
            &color_bind_group_layout(&device),
        );

        let (uniform_group, uniform_buffer, layout_bindings) = Self::setup_uniforms(&device);

        let uniform_bind_group = uniform_group;
        let uniform_buffer = uniform_buffer;

        let camera = Camera::default();
        let frustum = Frustum::default();
        let text_rendering_system = GlyphonManager::new(
            &device,
            &queue,
            render_surface.surface_config.format,
            default_depth_stencil_state(None),
        );

        let world = World::new();

        Ok(RenderContext {
            adapter,
            device,
            queue,
            render_surface,
            frame: FrameTime::new(),
            depth_texture: None,
            depth_texture_view: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
            frustum: Some(frustum),
            depth_ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            },
            world,
            camera,
            last_mouse_position: winit::dpi::PhysicalPosition::default(),
            material_manager,
            pipeline_manager,
            texture_manager,
            shader_manager,
            debug,
            mode,
            text_rendering_system,
            depth_stencil_state: default_depth_stencil_state(None),
            window,

            uniform_bind_group_layout: layout_bindings.uniform,
            texture_bind_group_layout: layout_bindings.texture,
            color_bind_group_layout: layout_bindings.color,
            uniform_bind_group,
            uniform_buffer,
            color_bind_group,
            color_uniform_buffer,
        })
    }

    pub fn set_render_mode(&mut self, render_mode: RenderMode) {
        self.mode = render_mode;
        match render_mode {
            RenderMode::LineListDepthView | RenderMode::TriangleListDepthView => {
                self.enable_depth_stencil();
            }
            RenderMode::LineListNoDepth | RenderMode::TriangleListNoDepth => {
                self.disable_depth_stencil();
            }
        }
    }

    pub fn resize(
        render_surface: &mut RenderSurface,
        text_rendering_system: &mut GlyphonManager,
        new_width: u32,
        new_height: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        render_surface.resize(new_width, new_height, &device);
        text_rendering_system.reconfigure(
            &queue,
            Resolution {
                width: new_width,
                height: new_height,
            },
        );
    }
}
impl RenderContext {
    pub fn render_frame(&mut self) -> Result<(), AppError> {
        match self.render_surface.get_current_texture() {
            Ok(frame) => {
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let device_binding = &self.device;
                let queue_binding = &self.queue;

                let mut encoder =
                    device_binding.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                let render_pass_desc = wgpu::RenderPassDescriptor {
                    label: Some(self.mode.pipeline_label()),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: self.ops,
                    })],
                    depth_stencil_attachment: if self.mode.use_depth_stencil() {
                        self.depth_texture_view.as_ref().map(|depth_view| {
                            wgpu::RenderPassDepthStencilAttachment {
                                view: depth_view,
                                depth_ops: Some(self.depth_ops),
                                stencil_ops: Default::default(),
                            }
                        })
                    } else {
                        None
                    },
                    ..Default::default()
                };

                let pipeline = match self
                    .pipeline_manager
                    .get_pipeline(self.mode.pipeline_label())
                {
                    Some(pipe) => pipe,
                    None => return Ok(()),
                };

                {
                    let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
                    render_pass.set_pipeline(pipeline);

                    self.world.query3::<Transform, Mesh, Material>(
                        |_entity, transform, mesh, material| {
                            if mesh.geometry.num_indices() > 0
                                && mesh.geometry.vertex_buffer_data().len() > 0
                            {
                                let model_matrix = create_model_matrix(
                                    transform.position,
                                    transform.rotation,
                                    transform.scale,
                                );

                                let proj_matrix = self.camera.view_projection_matrix();

                                let uniform_data = Uniforms {
                                    model: model_matrix.into(),
                                    view_projection: proj_matrix.into(),
                                    color: material.color.into(),
                                };

                                self.queue.write_buffer(
                                    &self.uniform_buffer,
                                    0,
                                    bytemuck::cast_slice(&[uniform_data]),
                                );
                                render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

                                // if let Some(texture_bind_group) = &material.texture_id {
                                //     render_pass.set_bind_group(1, texture_bind_group, &[]);
                                // } else {
                                //     render_pass.set_bind_group(1, &self.color_bind_group, &[]);
                                // }
                                // render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                                // render_pass.set_index_buffer(
                                //     mesh.index_buffer.slice(..),
                                //     wgpu::IndexFormat::Uint16,
                                // );

                                // render_pass.draw_indexed(0..mesh.indices_count, 0, 0..1);
                            }
                        },
                    );
                }

                queue_binding.submit(Some(encoder.finish()));
                frame.present();
                self.text_rendering_system.clear_buffer();
            }
            Err(e) => {
                log_error!("Failed to get frame texture: {:?}", e);
            }
        }
        Ok(())
    }
}

impl RenderContext {
    async fn initialize_graphics() -> (
        Arc<wgpu::Adapter>,
        Arc<wgpu::Device>,
        Arc<wgpu::Queue>,
        Arc<wgpu::Instance>,
    ) {
        let instance = get_instance().expect("Instance");
        let adapter = get_adapter().expect("Adapter");
        let device = get_device().expect("Device");
        let queue = get_queue().expect("Queue");
        (adapter, device, queue, instance)
    }

    pub fn initialize_pipeline_manager(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        shader_manager: &mut ShaderLibrary,
        pipeline_manager: &mut PipelineManager,
    ) -> Result<(), AppError> {
        Ok(setup_pipeline_manager(
            &device,
            shader_manager,
            pipeline_manager,
            format,
            BindingLayouts::uniform(&device),
            BindingLayouts::texture(&device),
            BindingLayouts::color(&device),
        )?)
    }

    fn setup_uniforms(device: &wgpu::Device) -> (wgpu::BindGroup, wgpu::Buffer, LayoutBindings) {
        let uniform_buffer = uniform_buffer(device, &Uniforms::default());
        let uniform_bind_group_layout = uniform_bind_group_layout(device);
        let uniform_bind_group =
            uniform_bind_group(device, &uniform_buffer, &uniform_bind_group_layout);

        (
            uniform_bind_group,
            uniform_buffer,
            LayoutBindings {
                uniform: uniform_bind_group_layout,
                texture: BindingLayouts::texture(device),
                color: BindingLayouts::color(device),
            },
        )
    }
}
impl RenderContext {
    pub fn change_material_color(&mut self, material_name: &str, new_color: [f32; 4]) {
        let _ = self
            .material_manager
            .set_material_color(material_name, new_color);
    }
    pub fn draw_debug(&mut self) {
        self.text_rendering_system.draw_debug_info(
            self.debug,
            &self.frame,
            self.frame.fps,
            &self.camera,
        );
    }

    pub fn disable_depth_stencil(&mut self) {
        let inner_size = self.window.inner_size();
        self.depth_texture = None;
        self.depth_texture_view = None;
        self.depth_stencil_state.depth_write_enabled = false;
        self.camera.set_orthographic_projection(
            0.0,
            inner_size.width as f32,
            0.0,
            inner_size.height as f32,
            -1.0,
            1.0,
        );
        self.text_rendering_system.set_use_depth(false);
    }

    pub fn enable_depth_stencil(&mut self) {
        let (depth_texture, depth_texture_view) =
            create_depth_textures(&self.device, &self.render_surface.surface_config);
        self.depth_texture = Some(Arc::new(depth_texture));
        self.depth_texture_view = Some(Arc::new(depth_texture_view));
        self.depth_stencil_state.depth_write_enabled = true;

        self.camera
            .set_perspective_projection(std::f32::consts::FRAC_PI_2, 16.0 / 9.0, 1.0, 100.0);
        self.text_rendering_system.set_use_depth(true);
    }
}
