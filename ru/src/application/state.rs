use crate::{
    camera::Camera,
    geometry,
    gpu::GPUGlobal,
    log_debug,
    material::{color::Color, material_manager::MaterialManager},
    render::{pass::RenderPhase, pipeline::PipelineManager, RenderSystem},
    scene::scene_manager::SceneManager,
    text::glyphon_manager::GlyphonManager,
    utilities::debug::DebugMetrics,
};

use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use wgpu::{CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureUsages};
use winit::window::Window;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    Normal,      // Regular rendering
    OutlineOnly, // Only render the outlines
    Freeze,      // Do not render any new frames
}

struct Settings {
    pub fps: bool,
    pub last_frame_time: bool,
    pub debug_mode: RenderMode,
}

impl Settings {
    pub fn new(fps: bool, last_frame_time: bool, debug_mode: RenderMode) -> Settings {
        Settings {
            fps,
            last_frame_time,
            debug_mode,
        }
    }

    pub fn toggle_fps(&mut self) {
        self.fps = !self.fps;
    }

    pub fn toggle_last_frame_time(&mut self) {
        self.last_frame_time = !self.last_frame_time;
    }

    pub fn cycle_debug_mode(&mut self) {
        self.debug_mode = match self.debug_mode {
            RenderMode::Normal => RenderMode::OutlineOnly,
            RenderMode::OutlineOnly => RenderMode::Freeze,
            RenderMode::Freeze => RenderMode::Normal,
        };
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            fps: false,
            last_frame_time: false,
            debug_mode: RenderMode::Normal,
        }
    }
}

pub(crate) struct ApplicationState {
    pub(crate) gpu: GPUGlobal,
    pub(crate) scene_manager: SceneManager,
    pub(crate) text_rendering_system: GlyphonManager,
    pub(crate) camera: Camera,
    pub(crate) render_system: RenderSystem,
    pub last_mouse_position: winit::dpi::PhysicalPosition<f64>,
    pub debug: DebugMetrics,
    settings: Settings,
    pub frame_duration: Duration,
}

impl ApplicationState {
    pub async fn new(window: Arc<Window>, instance_desc: Option<wgpu::InstanceDescriptor>) -> Self {
        let gpu = GPUGlobal::initialize(instance_desc).await.expect("GPU");
        let instance_binding = gpu.instance();
        let instance = instance_binding.read().expect("Instance");
        let adapter_binding = gpu.adapter();
        let adapter = adapter_binding.read().expect("Adapter");
        let device_binding = gpu.device();
        let queue_binding = gpu.queue();

        let surface = instance
            .create_surface(window.clone())
            .expect("Create surface");

        let surface_capabilities = surface.get_capabilities(&adapter);
        let target_fps = 200.0;
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps);
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_capabilities.formats[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: PresentMode::Mailbox,
            desired_maximum_frame_latency: 2,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: [].to_vec(),
        };
        let mut pipeline_manager = PipelineManager::new(&device_binding);
        pipeline_manager.create_main_pipeline(&device_binding, surface_capabilities.formats[0]);
        pipeline_manager.create_outline_pipeline(&device_binding, surface_capabilities.formats[0]);
        pipeline_manager.create_surface_pipeline(&device_binding, surface_capabilities.formats[0]);

        let material_manager = MaterialManager::new(&device_binding, &surface_config);

        let scene_manager =
            SceneManager::new(material_manager, Arc::new(RwLock::new(pipeline_manager)));
        let render_system = RenderSystem::new(
            window.clone(),
            Arc::clone(&device_binding),
            surface,
            surface_config,
        );

        let text_rendering_system = GlyphonManager::new(
            &device_binding,
            &queue_binding,
            render_system.target_surface.surface_config.format,
        );

        ApplicationState {
            gpu,
            scene_manager,
            text_rendering_system,
            camera: Camera::default(),
            last_mouse_position: winit::dpi::PhysicalPosition::default(),
            render_system,
            debug: DebugMetrics::new(),
            settings: Settings::default(),
            frame_duration,
        }
    }

    pub fn cycle_debug_mode(&mut self) {
        self.settings.cycle_debug_mode();
        self.scene_manager.set_debug_mode(self.settings.debug_mode);
        log_debug!("Debug mode set to {:?}", self.settings.debug_mode);
    }

    pub fn toggle_fps(&mut self) {
        self.settings.toggle_fps();
    }

    pub fn toggle_last_frame_time(&mut self) {
        self.settings.toggle_last_frame_time();
    }

    pub fn set_debug_mode_for_all(&mut self, enabled: RenderMode) {
        self.scene_manager.set_debug_mode_for_all(enabled);
    }

    fn fps(&mut self) {
        if self.settings.fps {
            self.text_rendering_system
                .draw_fps([10.0, 10.0], self.debug.fps);
        }
    }

    fn last_frame_time(&mut self) {
        if self.settings.last_frame_time {
            self.text_rendering_system
                .draw_frame_time([10.0, 20.0], self.debug.last_frame_time);
        }
    }
    pub fn add_test_objects(&mut self) {
        let geometry_cube = geometry::Shape::Cube(geometry::cube::CubeStructure::default());
        let geometry_sphere =
            geometry::Shape::Triangle(geometry::triangle::TriangleStructure::default());

        let position_cube = nalgebra::Vector3::new(3.0, 0.0, -5.0);
        let position_sphere = nalgebra::Vector3::new(0.0, 0.0, -5.0);

        let color_cube = Color::new(1.0, 0.0, 0.0, 1.0);
        let color_sphere = Color::new(0.0, 1.0, 0.0, 1.0);

        self.add_scene_object(geometry_cube, position_cube, color_cube);
        self.add_scene_object(geometry_sphere, position_sphere, color_sphere);
    }

    pub fn add_scene_object(
        &mut self,
        geometry: geometry::Shape,
        position: nalgebra::Vector3<f32>,
        color: Color,
    ) {
        let device_binding = self.gpu.device();
        self.scene_manager
            .add_object(position, geometry, color, None, &device_binding);
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let device_binding = self.gpu.device();
        self.render_system
            .target_surface
            .resize(new_width, new_height, &device_binding);
        self.scene_manager.materials.resize_depth_texture(
            &device_binding,
            &self.render_system.target_surface.surface_config,
        );

        self.camera
            .set_aspect_ratio(new_width as f32 / new_height as f32);
    }

    pub fn render_frame(&mut self) {
        if self
            .render_system
            .target_surface
            .acquire_current_texture()
            .is_err()
        {
            return;
        }

        self.fps();
        self.last_frame_time();

        if let Some(surface_texture) = &self.render_system.target_surface.current_texture {
            let color_view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let device_binding = self.gpu.device();
            let queue_binding = self.gpu.queue();

            let mut encoder =
                device_binding.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            let proj_matrix = self.camera.view_projection_matrix();

            self.render_system.command_buffer.commands.clear();

            self.scene_manager.populate_command_buffer(
                &mut self.render_system.command_buffer,
                &proj_matrix,
                Arc::new(&queue_binding),
                Arc::new(&device_binding),
            );

            let depth_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.render_system.target_surface.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            });

            RenderPhase::execute(
                &self.render_system.command_buffer.commands,
                &mut encoder,
                &color_view,
                depth_attachment.as_ref(),
                wgpu::Color::WHITE,
            );

            queue_binding.submit(Some(encoder.finish()));

            self.render_system.target_surface.present_texture();
            self.render_system.target_surface.window.request_redraw();
        }
    }
}
