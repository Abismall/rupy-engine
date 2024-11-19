use super::DebugMode;
use crate::{
    camera::{frustum::Frustum, projection::ProjectionMode, Camera},
    core::{
        error::AppError,
        surface::{default_surface_configuration, RenderSurface},
    },
    ecs::{
        buffer::BufferManager, materials::manager::MaterialManager, storage::ComponentManager,
        world::World,
    },
    gpu::{
        binding::{
            groups::uniform_bind_group,
            layouts::{sampled_texture_bind_group_layout, uniform_bind_group_layout},
        },
        buffer::setup::BufferSetup,
        global::{get_adapter, get_device, get_instance, get_queue},
        glyphon::GlyphonRender,
        uniform::Uniforms,
        RenderBatch, RenderMode,
    },
    input::InputListener,
    log_error, log_info,
    pipeline::{get_pipeline_label, manager::PipelineManager, setup::setup_render_pipelines},
    prelude::{
        constant::{
            ORTHOGRAPHIC_FAR, ORTHOGRAPHIC_NEAR, PERSPECTIVE_FAR, PERSPECTIVE_NEAR, ZERO_F32,
        },
        frame::FrameTime,
    },
    shader::manager::ShaderManager,
    texture::{
        depth_texture_with_view,
        manager::{setup_texture_manager, TextureManager},
    },
};

use std::sync::Arc;

use nalgebra::Matrix4;
use wgpu::{rwh::HasDisplayHandle, BindGroup, DepthStencilState, TextureView};
use winit::{dpi::PhysicalSize, window::Window};

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
    pub camera: Camera,
    pub frame: FrameTime,

    pub depth_stencil_state: DepthStencilState,
    pub depth_texture: TextureView,
    pub render_batch: RenderBatch,
    pub uniform_data: Uniforms,
    pub world: World,

    pub last_mouse_position: winit::dpi::PhysicalPosition<f64>,

    pub debug_mode: DebugMode,
    pub render_mode: RenderMode,
    pub glyphons: GlyphonRender,
    uniform_bind_group: BindGroup,

    global_uniform_buffer: wgpu::Buffer,
    _pipelines: PipelineManager,
    _shaders: ShaderManager,
    pub window: Arc<Window>,
}

impl RenderContext {
    pub async fn new(
        window: Arc<Window>,
        camera: Camera,
        render_mode: RenderMode,
        debug_mode: DebugMode,
    ) -> Result<Self, AppError> {
        let (adapter, device, queue, instance) = Self::initialize().await;
        let surface = instance.create_surface(window.clone())?;
        let surface_config = default_surface_configuration(&surface, &adapter, &window);
        let depth_stencil_state = default_depth_stencil_state(None);
        let uniform_layout = uniform_bind_group_layout(&device, false);
        let texture_layout = sampled_texture_bind_group_layout(&device);
        let uniform_data = Uniforms::new(
            camera.view_projection_matrix().into(),
            Matrix4::identity().into(),
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [10.0, 10.0, 10.0, 1.0],
            camera.position().into(),
            0.5,
            0.8,
            1.0,
            32.0,
        );
        let mut shaders = ShaderManager::new();
        let mut pipelines = PipelineManager::new();
        let material_manager = MaterialManager::new();
        let component_manager = ComponentManager::new();
        let buffer_manager = BufferManager::new();
        let texture_manager = setup_texture_manager(
            &device,
            &queue,
            TextureManager::new(),
            vec![("default".to_string(), 14402189752926126668)],
            surface_config.format,
        )
        .await?;
        let render_surface = RenderSurface::new(window.clone(), surface, surface_config);
        let swapchain_format = render_surface.surface_config.format;
        setup_render_pipelines(
            &mut pipelines,
            &mut shaders,
            &device,
            &uniform_layout,
            &texture_layout,
            &swapchain_format,
            &depth_stencil_state,
        )?;
        let text_update_interval = 3;
        let glyphons = GlyphonRender::new(
            &device,
            &queue,
            text_update_interval,
            swapchain_format,
            &depth_stencil_state,
        );
        let mut world = World::new(
            texture_manager,
            material_manager,
            component_manager,
            buffer_manager,
        );
        if let Err(e) = world.load_scene(&device, "default") {
            log_error!("Error loading world scene: {:?}", e);
        };
        let (.., depth_texture) = depth_texture_with_view(&device, &render_surface.surface_config);

        let global_uniform_buffer =
            BufferSetup::uniform_buffer(&device, std::mem::size_of::<Uniforms>() as u64);
        let uniform_bind_group =
            uniform_bind_group(&device, &global_uniform_buffer, &uniform_layout, None);

        Ok(RenderContext {
            camera,
            adapter,
            device,
            queue,
            render_surface,
            frame: FrameTime::new(),
            render_batch: RenderBatch::default(),
            depth_texture,
            uniform_bind_group,
            uniform_data,

            world,

            last_mouse_position: winit::dpi::PhysicalPosition::default(),
            debug_mode,
            render_mode,
            glyphons,
            global_uniform_buffer,
            depth_stencil_state,
            _pipelines: pipelines,
            _shaders: shaders,

            window,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.render_surface.surface_config.width = size.width;
        self.render_surface.surface_config.height = size.height;
        self.render_surface
            .resize(size.width, size.height, &self.device);

        if self.depth_stencil_state.is_depth_enabled() {
            self.set_depth_texture();
        }
    }
    pub fn set_depth_texture(&mut self) {
        let surface_config = &self.render_surface.surface_config;
        let device = &self.device;

        let (.., depth_texture) = depth_texture_with_view(device, surface_config);

        self.depth_texture = depth_texture;
    }
    pub fn surface_config_size(&self) -> (f32, f32) {
        (
            self.render_surface.surface_config.width as f32,
            self.render_surface.surface_config.height as f32,
        )
    }

    pub fn aspect_ratio(&self) -> f32 {
        let inner_size: PhysicalSize<f32> = self.window.inner_size().cast();
        inner_size.width / inner_size.height
    }

    pub fn handle_input<T: 'static>(&mut self, event: &winit::event::Event<T>, delta_time: f32) {
        match event {
            winit::event::Event::DeviceEvent { event, .. } => {
                if let winit::event::DeviceEvent::MouseMotion { delta } = event {
                    self.camera.on_mouse_motion(*delta);
                }
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    self.camera.on_key_event(event, delta_time);
                }
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    self.camera.on_scroll(*delta);
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    self.camera.on_mouse_button(*button, *state);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
impl RenderContext {
    pub fn get_pipeline(
        &self,
        name: &str,
    ) -> Result<std::sync::Arc<wgpu::RenderPipeline>, AppError> {
        Ok(self._pipelines.get_pipeline(&name).ok_or_else(|| {
            AppError::PipelineNotFoundError(format!("Pipeline '{}' not found", name))
        })?)
    }

    pub fn render_world(&mut self) -> Result<(), AppError> {
        match self.render_surface.get_current_texture() {
            Ok(frame) => {
                let current_view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder =
                    self.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                let use_depth = matches!(
                    self.render_mode,
                    RenderMode::LineColorWithDepth
                        | RenderMode::LineTextureWithDepth
                        | RenderMode::TriangleColorWithDepth
                        | RenderMode::TriangleTextureWithDepth
                );
                let depth_stencil_attachment = if use_depth {
                    Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture,
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

                {
                    let mut pass = encoder.begin_render_pass(&desc);
                    pass.set_bind_group(0, &self.uniform_bind_group, &[]);
                    let pipeline = self.get_pipeline(&pipeline_label)?;
                    pass.set_pipeline(&pipeline);

                    {
                        self.world.render(
                            &mut pass,
                            &self.device,
                            &Frustum::from_view_projection_matrix(
                                &self.camera.view_projection_matrix(),
                            ),
                        )?;
                    }

                    if self.debug_mode != DebugMode::None
                        && self.frame.frame_count % self.glyphons.interval == 0
                    {
                        self.glyphons.render(
                            &mut pass,
                            &self.device,
                            &self.queue,
                            &self.render_surface.surface_config,
                            use_depth,
                        );
                        self.glyphons.clear_buffer();
                    }
                }
                self.queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Err(e) => return Err(AppError::from(e)),
        }

        Ok(())
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
    pub fn update_debug_info(&mut self) {
        if self.frame.frame_count % self.glyphons.interval == 0 {
            self.glyphons.debug(self.debug_mode, &self.frame);
        }
    }
    pub fn compute_frame_metrics(&mut self) {
        self.frame.compute();
    }

    pub fn update_camera_uniform(&mut self) {
        let camera_position = self.camera.position();
        self.uniform_data.view_proj = self.camera.view_projection_matrix();
        self.uniform_data.view_position = camera_position;

        self.queue.write_buffer(
            &self.global_uniform_buffer,
            0,
            bytemuck::bytes_of(&self.uniform_data),
        );
    }

    pub fn redraw(&mut self) {
        if self.window.display_handle().is_ok() {
            self.window.request_redraw();
        }
    }

    pub fn set_next_debug_mode(&mut self) {
        self.debug_mode = self.debug_mode.next();
        log_info!("Debug Mode: {:?}", self.debug_mode);
    }

    pub fn set_next_render_mode(&mut self) {
        self.render_mode = self.render_mode.next();
        if matches!(
            self.render_mode,
            RenderMode::LineColorWithDepth
                | RenderMode::LineTextureWithDepth
                | RenderMode::TriangleColorWithDepth
                | RenderMode::TriangleTextureWithDepth
        ) {
            self.camera.set_projection_mode(ProjectionMode::Perspective(
                std::f32::consts::FRAC_PI_4,
                self.aspect_ratio(),
                PERSPECTIVE_NEAR,
                PERSPECTIVE_FAR,
            ))
        } else {
            let (width, height) = self.surface_config_size();
            self.camera
                .set_projection_mode(ProjectionMode::Orthographic(
                    ZERO_F32,
                    width,
                    ZERO_F32,
                    height,
                    ORTHOGRAPHIC_NEAR,
                    ORTHOGRAPHIC_FAR,
                ))
        };
    }
}
