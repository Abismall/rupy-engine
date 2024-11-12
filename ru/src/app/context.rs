use super::DebugMode;
use crate::{
    core::{
        error::AppError,
        surface::{default_surface_configuration, RenderSurface},
    },
    ecs::{
        materials::MaterialManager,
        pipelines::PipelineManager,
        shaders::ShaderManager,
        textures::{setup_texture_manager, TextureManager},
        world::World,
    },
    gpu::{
        binding::BindGroupLayouts,
        global::{get_adapter, get_device, get_instance, get_queue},
        glyphon::GlyphonManager,
        RenderMode,
    },
    input::InputListener,
    log_debug, log_error, log_info,
    pipeline::{get_pipeline_label, setup::setup_pipelines},
    prelude::frame::FrameTime,
    texture::create_depth_texture_with_view,
};

use std::sync::Arc;

use wgpu::{DepthStencilState, TextureView};
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
    pub render_surface: RenderSurface,
    pub adapter: Arc<wgpu::Adapter>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,

    pub frame: FrameTime,

    pub depth_stencil_state: DepthStencilState,
    pub depth_texture_view: TextureView,
    pub world: World,

    pub last_mouse_position: winit::dpi::PhysicalPosition<f64>,

    pub debug_mode: DebugMode,
    pub render_mode: RenderMode,
    pub text_rendering_system: GlyphonManager,

    pub window: Arc<Window>,
}

impl RenderContext {
    pub async fn new(
        window: Arc<Window>,
        render_mode: RenderMode,
        debug_mode: DebugMode,
    ) -> Result<Self, AppError> {
        let (adapter, device, queue, instance) = Self::initialize().await;
        let surface = instance.create_surface(window.clone())?;
        let surface_config = default_surface_configuration(&surface, &adapter, &window);
        let depth_stencil_state = default_depth_stencil_state(None);
        let bind_group_layouts = BindGroupLayouts::new(&device);
        let mut shader_manager = ShaderManager::new();
        let mut pipeline_manager = PipelineManager::new();
        let material_manager = MaterialManager::new();
        let texture_manager = setup_texture_manager(
            &device,
            &queue,
            TextureManager::new(),
            vec![("default".to_string(), 14402189752926126668)],
            surface_config.format,
        )
        .await?;
        let _ = setup_pipelines(
            &mut pipeline_manager,
            &mut shader_manager,
            &device,
            surface_config.format,
            &bind_group_layouts.uniform_layout,
            &bind_group_layouts.texture_layout,
            &depth_stencil_state,
        );
        let render_surface = RenderSurface::new(window.clone(), surface, surface_config);
        let texture_format = render_surface.surface_config.format.clone();

        let text_rendering_system =
            GlyphonManager::new(&device, &queue, texture_format, depth_stencil_state.clone());
        let mut world = World::new(
            texture_manager,
            shader_manager,
            pipeline_manager,
            material_manager,
            bind_group_layouts,
            depth_stencil_state.clone(),
        );
        if let Err(e) = world.load_scene(&device, "default") {
            log_error!("Error loading world scene: {:?}", e);
        };
        let (.., depth_texture_view) =
            create_depth_texture_with_view(&device, &render_surface.surface_config);

        Ok(RenderContext {
            adapter,
            device,
            queue,
            render_surface,
            frame: FrameTime::new(),
            depth_texture_view,

            world,

            last_mouse_position: winit::dpi::PhysicalPosition::default(),
            debug_mode,
            render_mode,
            text_rendering_system,
            depth_stencil_state: depth_stencil_state,

            window,
        })
    }
    pub fn handle_input<T: 'static>(&mut self, event: &winit::event::Event<T>, delta_time: f32) {
        match event {
            winit::event::Event::DeviceEvent { event, .. } => {
                if let winit::event::DeviceEvent::MouseMotion { delta } = event {
                    self.world.camera.on_mouse_motion(*delta);
                }
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    self.world.camera.on_key_event(event, delta_time);
                }
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    self.world.camera.on_scroll(*delta);
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    self.world.camera.on_mouse_button(*button, *state);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
impl RenderContext {
    pub fn render_world(&mut self) {
        match self.render_surface.get_current_texture() {
            Ok(frame) => {
                let current_view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let use_depth = matches!(
                    self.render_mode,
                    RenderMode::LineColorWithDepth
                        | RenderMode::LineTextureWithDepth
                        | RenderMode::TriangleColorWithDepth
                        | RenderMode::TriangleTextureWithDepth
                );

                let depth_stencil_attachment = if use_depth {
                    Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: Default::default(),
                    })
                } else {
                    None
                };
                let (primitive, shading, depth) = self.render_mode.to_pipeline_config();
                let pipeline_label = get_pipeline_label(&primitive, &shading, &depth);
                let desc = wgpu::RenderPassDescriptor {
                    label: Some(&pipeline_label),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &current_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.05,
                                g: 0.05,
                                b: 0.05,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],

                    timestamp_writes: None,
                    occlusion_query_set: None,
                    depth_stencil_attachment,
                };
                let mut encoder =
                    self.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });
                {
                    {
                        let mut pass = encoder.begin_render_pass(&desc);

                        let _ =
                            self.world
                                .render(&self.device, &self.queue, &mut pass, pipeline_label);

                        if self.debug_mode != DebugMode::None {
                            self.text_rendering_system.render(
                                &mut pass,
                                use_depth,
                                &self.device,
                                &self.queue,
                                &self.render_surface.surface_config,
                            );
                        }
                    }

                    self.queue.submit(Some(encoder.finish()));
                    frame.present();
                }
            }

            Err(e) => {
                log_error!("Error rendering world: {:?}", e);
            }
        }
    }
}

impl RenderContext {
    async fn initialize() -> (
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
}
impl RenderContext {
    pub fn draw_debug_info(&mut self) {
        self.text_rendering_system.draw_debug_info(
            self.debug_mode,
            &self.frame,
            &self.world.camera,
        );
    }

    pub fn disable_depth_stencil(&mut self) {
        self.depth_stencil_state.depth_write_enabled = false;
    }

    pub fn enable_depth_stencil(&mut self) {
        self.depth_stencil_state.depth_write_enabled = true;
    }
    pub fn toggle_debug_mode(&mut self) {
        self.debug_mode = self.debug_mode.next();
        log_info!("Debug Mode: {:?}", self.debug_mode);
    }

    pub fn toggle_render_mode(&mut self) {
        self.render_mode = self.render_mode.next();
        log_debug!("Render Mode: {:?}", self.render_mode);

        if matches!(
            self.render_mode,
            RenderMode::LineColorWithDepth
                | RenderMode::TriangleColorWithDepth
                | RenderMode::TriangleTextureWithDepth
                | RenderMode::LineTextureWithDepth
        ) {
            self.enable_depth_stencil();
            self.world.camera.set_perspective_projection(
                std::f32::consts::FRAC_PI_2,
                16.0 / 9.0,
                0.1,
                100.0,
            );
        } else {
            self.disable_depth_stencil();
            self.world.camera.set_orthographic_projection(
                0.0,
                self.render_surface.surface_config.width as f32,
                0.0,
                self.render_surface.surface_config.height as f32,
                -1.0,
                1.0,
            );
        }
    }
}
