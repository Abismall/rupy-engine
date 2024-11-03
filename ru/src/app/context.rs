use std::{sync::Arc, time::Duration};

use super::DebugMode;
use crate::{
    camera::{Camera, Frustum},
    core::{
        error::AppError,
        surface::{default_surface_configuration, RenderSurface},
    },
    ecs::components::uniform::{ColorUniform, ModelUniform, Uniforms, ViewProjectionMatrix},
    graphics::{
        binding::{uniform_bind_group, uniform_bind_group_layout, BindingGroups, BindingLayouts},
        buffer::uniform_buffer,
        global::{get_adapter, get_device, get_instance, get_queue},
        glyphon::GlyphonManager,
        RenderMode,
    },
    log_debug, log_error,
    pipeline::cache::setup_pipeline_manager,
    prelude::frame::FrameTime,
    shape::{cube::Cube, Geometry},
    texture::{create_depth_textures, library::TextureFileCache},
};
use crate::{material::manager::MaterialManager, pipeline::cache::PipelineManager};
use nalgebra::Vector3;
use vecmath::mat4_id;
use wgpu::{util::DeviceExt, Origin3d};
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

pub struct RenderContext {
    pub material_manager: MaterialManager,
    pub text_rendering_system: GlyphonManager,
    pub pipeline_manager: Arc<PipelineManager>,
    pub texture_manager: Arc<TextureFileCache>,
    pub uniform_bind_group: Arc<wgpu::BindGroup>,
    pub uniform_buffer: Arc<wgpu::Buffer>,
    pub depth_stencil_state: wgpu::DepthStencilState,
    pub camera: Camera,
    pub mode: RenderMode,
    pub debug: DebugMode,
    pub frame: FrameTime,
    pub frustum: Option<Frustum>,
    pub render_surface: RenderSurface,
    pub last_mouse_position: winit::dpi::PhysicalPosition<f64>,
    pub adapter: Arc<wgpu::Adapter>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub window: Arc<Window>,
    pub(crate) depth_texture: Option<Arc<wgpu::Texture>>,
    pub(crate) depth_texture_view: Option<Arc<wgpu::TextureView>>,
    pub uniform_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    pub texture_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    pub color_bind_group_layout: Arc<wgpu::BindGroupLayout>,
    pub(crate) _target_fps: u64,
    pub(crate) _frame_duration: Duration,
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

            let color = ColorUniform {
                rgba: [1.0, 5.0, 5.0, 1.0],
            };
            let color_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Color Buffer"),
                    contents: bytemuck::cast_slice(&[color]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

            let color_bind_group =
                BindingGroups::color(&self.device, &color_buffer, &self.color_bind_group_layout);

            let pipeline = match self.pipeline_manager.get_pipeline("lines") {
                Some(lines) => lines,
                None => {
                    log_debug!("Pipeline 'Lines' not found");
                    return;
                }
            };

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Lines"),
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
            render_pass.set_bind_group(1, &color_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        } else {
            log_debug!("Frustum not available for drawing");
        }
    }
}
impl RenderContext {
    pub async fn new(window: Arc<Window>, mode: RenderMode, debug: DebugMode) -> Self {
        let target_fps = 300;
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
        let instance = get_instance().expect("Instance");
        let adapter = get_adapter().expect("Adapter");
        let device = get_device().expect("Device");
        let queue = get_queue().expect("Queue");

        let surface = instance
            .create_surface(window.clone())
            .expect("Create surface");

        let surface_config = default_surface_configuration(&surface, &adapter, &window);
        let camera = Camera::default();
        let uniform_buffer = uniform_buffer(
            &device,
            &Uniforms {
                model: ModelUniform { matrix: mat4_id() },
                color: ColorUniform {
                    rgba: [1.0 as f32, 1.0 as f32, 1.0 as f32, 1.0 as f32],
                },
                view_projection: ViewProjectionMatrix {
                    matrix: Default::default(),
                },
            },
        );

        let uniform_bind_group = uniform_bind_group(
            &device,
            &uniform_buffer,
            &uniform_bind_group_layout(&device),
        );
        let uniform_bind_group_layout = Arc::new(BindingLayouts::uniform(&device));
        let texture_bind_group_layout = Arc::new(BindingLayouts::texture(&device));
        let color_bind_group_layout = Arc::new(BindingLayouts::color(&device));
        let pipeline_manager = Arc::new(
            setup_pipeline_manager(
                &device,
                surface_config.format,
                &uniform_bind_group_layout,
                &texture_bind_group_layout,
                &color_bind_group_layout,
            )
            .expect("Pipeline Manager"),
        );
        let mut texture_manager = TextureFileCache::new();

        let mut material_manager = MaterialManager::new(pipeline_manager.clone(), &device);
        let render_surface = RenderSurface::new(window.clone(), &device, &adapter, surface);

        let text_rendering_system = GlyphonManager::new(
            &device,
            &queue,
            render_surface.surface_config.format,
            default_depth_stencil_state(None),
        );

        let _ = material_manager.create_material(
            Geometry::Cube(Cube::new(10, 10, 10, Vector3::new(0.0, 0.0, 1.0))),
            camera.view_projection_matrix(),
            [1.0, 1.0, 1.0, 1.0],
            "cube".into(),
            None,
        );

        let _ = material_manager.set_material_texture(
            "cube",
            "static/images/missing.png",
            &mut texture_manager,
            surface_config.format,
            wgpu::TextureDimension::D2,
            1,
            1,
            1,
            Origin3d::ZERO,
            wgpu::TextureAspect::All,
            0,
            0,
        );

        if let Err(e) = texture_manager.write_to_queue(&queue).await {
            log_error!("Failed to write texture to queue: {:?}", e);
        }
        RenderContext {
            _target_fps: target_fps,
            _frame_duration: frame_duration,
            material_manager,
            text_rendering_system,
            uniform_bind_group: uniform_bind_group.into(),
            uniform_buffer: uniform_buffer.into(),
            camera,
            frame: FrameTime::new(),
            frustum: Some(Frustum::default()),
            mode,
            last_mouse_position: winit::dpi::PhysicalPosition::default(),
            pipeline_manager,
            render_surface,
            depth_texture: None,
            uniform_bind_group_layout,
            texture_bind_group_layout,
            color_bind_group_layout,
            depth_texture_view: None,
            adapter,
            device,
            queue,
            window,
            debug,
            texture_manager: texture_manager.into(),
            depth_stencil_state: default_depth_stencil_state(None),
        }
    }
    pub fn disable_depth_stencil(&mut self) {
        self.depth_texture = None;
        self.depth_texture_view = None;

        let state = false;
        self.depth_stencil_state.depth_write_enabled = state;
        self.text_rendering_system.set_use_depth(state);
    }
    pub fn enable_depth_stencil(&mut self) {
        let (depth_texture, depth_texture_view) =
            create_depth_textures(&self.device, &self.render_surface.surface_config);

        self.depth_texture = Some(depth_texture.into());
        self.depth_texture_view = Some(depth_texture_view.into());

        let state = true;

        self.depth_stencil_state.depth_write_enabled = state;
        self.text_rendering_system.set_use_depth(state);
    }

    pub fn draw_debug(&mut self) {
        self.text_rendering_system.draw_debug_info(
            self.debug,
            &self.frame,
            self.frame.fps,
            &self.camera,
        );
        self.text_rendering_system
            .draw_fps([20.0, 40.0], self.frame.fps);
    }

    pub fn set_render_mode(&mut self, render_mode: RenderMode) {
        let inner_size = self.window.inner_size();
        match render_mode {
            RenderMode::Flat | RenderMode::WireNoDepth => {
                self.camera.set_orthographic_projection(
                    0.0,
                    inner_size.width as f32,
                    0.0,
                    inner_size.height as f32,
                    -1.0,
                    1.0,
                );
                self.disable_depth_stencil();
            }
            RenderMode::Depth | RenderMode::WireWithDepth => {
                self.camera.set_perspective_projection(
                    std::f32::consts::FRAC_PI_2,
                    16.0 / 9.0,
                    1.0,
                    100.0,
                );
                self.enable_depth_stencil();
            }
        }
    }

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
                let proj_matrix = self.camera.view_projection_matrix();

                let (pipeline_label, use_depth) = match self.mode {
                    RenderMode::Flat => ("flat", false),
                    RenderMode::Depth => ("depth", true),
                    RenderMode::WireNoDepth => ("wire_no_depth", false),
                    RenderMode::WireWithDepth => ("wire", true),
                };

                let depth_stencil_attachment = if use_depth {
                    self.depth_texture_view.as_ref().map(|depth_view| {
                        wgpu::RenderPassDepthStencilAttachment {
                            view: depth_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: Default::default(),
                        }
                    })
                } else {
                    None
                };

                let render_pass_desc = wgpu::RenderPassDescriptor {
                    label: Some(pipeline_label),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.5,
                                g: 0.5,
                                b: 0.5,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: depth_stencil_attachment.clone(),
                    timestamp_writes: Default::default(),
                    occlusion_query_set: Default::default(),
                };

                if let Some(pipeline) = self.pipeline_manager.get_pipeline(pipeline_label) {
                    let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

                    if let Some(material) = self.material_manager.materials().get("cube") {
                        let model_matrix = material.geometry.model_matrix();
                        queue_binding.write_buffer(
                            &material.uniform_buffer,
                            0,
                            bytemuck::cast_slice(&[Uniforms {
                                model: model_matrix.into(),
                                view_projection: proj_matrix.into(),
                                color: material.color.into(),
                            }]),
                        );

                        render_pass.set_pipeline(&pipeline);
                        render_pass.set_bind_group(0, &material.uniform_bind_group, &[]);
                        if let Some(texture_bind_group) = &material.texture_bind_group {
                            render_pass.set_bind_group(1, &texture_bind_group, &[]);
                        }
                        render_pass.set_vertex_buffer(0, material.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(
                            material.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );
                        render_pass.draw_indexed(0..material.geometry.num_indices(), 0, 0..1);
                    }
                }

                if self.debug != DebugMode::None {
                    self.text_rendering_system.render(
                        &mut encoder,
                        &self.device,
                        &self.queue,
                        &view,
                        depth_stencil_attachment.as_ref(),
                        &self.render_surface.surface_config,
                        &self.debug,
                    );
                }
                if self.mode == RenderMode::Depth || self.mode == RenderMode::WireWithDepth {
                    self.draw_frustum(&mut encoder, &view, depth_stencil_attachment.as_ref());
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

    pub fn resize(&mut self, new_width: u32, new_height: u32, device: &wgpu::Device) {
        self.render_surface.resize(new_width, new_height, &device);
    }

    pub fn change_material_color(
        &mut self,
        material_name: &str,
        new_color: [f32; 4],
        queue: &wgpu::Queue,
    ) {
        self.material_manager.set_material_color(
            material_name,
            new_color,
            &queue,
            self.camera.projection_matrix().into(),
        );
    }
}
