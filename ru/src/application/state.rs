use crate::camera::Camera;
use crate::material::manager::MaterialManager;
use crate::render::pipeline_manager::{setup_pipeline_manager, PipelineManager};
use crate::render::surface::RenderSurface;
use crate::text::glyphon_manager::GlyphonManager;
use crate::utilities::debug::DebugMetrics;
use crate::{geometry, log_debug, log_error};
use crate::{gpu::GPUGlobal, material::color::Color};

use std::default;
use std::sync::{Arc, RwLock};

use wgpu::{CompositeAlphaMode, PresentMode, TextureUsages};
use winit::window::Window;

pub(crate) struct ApplicationState {
    pub(crate) gpu: GPUGlobal,
    pub(crate) material_manager: MaterialManager,
    pub(crate) text_rendering_system: GlyphonManager,
    pub(crate) pipeline_manager: Arc<RwLock<PipelineManager>>,
    pub(crate) debug: DebugMetrics,
    pub(crate) camera: Camera,
    pub(crate) render_surface: RenderSurface, // Use the RenderSurface here
    pub last_mouse_position: winit::dpi::PhysicalPosition<f64>,
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

        let surface_config = wgpu::SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_capabilities.formats[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: PresentMode::Mailbox,
            desired_maximum_frame_latency: 2,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: [].to_vec(),
        };
        let pipeline_manager = Arc::new(RwLock::new(setup_pipeline_manager(
            &device_binding,
            surface_capabilities.formats[0],
        )));

        // Clone the Arc so both MaterialManager and ApplicationState can use it
        let mut material_manager = MaterialManager::new(
            &device_binding,
            *surface_capabilities
                .formats
                .iter()
                .find(|&&f| {
                    f == wgpu::TextureFormat::Bgra8UnormSrgb
                        || f == wgpu::TextureFormat::Rgba8UnormSrgb
                })
                .unwrap_or(&surface_capabilities.formats[0]),
            pipeline_manager.clone(), // Clone the Arc
            &surface_config,
        );
        let render_surface =
            RenderSurface::new(window.clone(), &device_binding, surface, surface_config);

        let text_rendering_system = GlyphonManager::new(
            &device_binding,
            &queue_binding,
            render_surface.surface_config.format,
        );

        // Create the pipeline manager

        material_manager.create_material(
            &device_binding,
            geometry::Geometry::ShadedTriangle(
                crate::geometry::triangle::ShadedTriangleStructure::new(
                    5,
                    5,
                    [1.0, 1.0, 1.0].into(),
                ),
            ),
            Color::PINK,
            "triangle".to_owned(),
        );

        let debug = DebugMetrics::new();

        ApplicationState {
            gpu,
            material_manager,
            text_rendering_system,
            camera: Camera::default(),
            last_mouse_position: winit::dpi::PhysicalPosition::default(),
            pipeline_manager, // Use the original Arc here
            debug,
            render_surface,
        }
    }
    pub fn draw_debug(&mut self) {
        self.text_rendering_system
            .draw_frame_time([10.0, 10.0], self.debug.last_frame_time);
        self.text_rendering_system
            .draw_fps([10.0, 20.0], self.debug.fps);
    }
    pub fn render_frame(&mut self) {
        match self.render_surface.get_current_texture() {
            Ok(frame) => {
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let device_binding = self.gpu.device();
                let queue_binding = self.gpu.queue();

                let mut encoder =
                    device_binding.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                let proj_matrix = self.camera.view_projection_matrix();
                log_debug!("Proj matrix: {:?}", proj_matrix);
                // Lock the pipeline_manager to access the pipeline
                let pipeline_manager = self.pipeline_manager.read().unwrap();
                let selected_pipeline = "RenderPipeline"; // In a real scenario, this might be changed dynamically
                if let Some(pipeline) = pipeline_manager.get_pipeline(selected_pipeline) {
                    log_debug!("Starting material render");
                    self.material_manager.render(
                        &mut encoder,
                        &view,
                        "triangle",
                        proj_matrix,
                        &queue_binding,
                        pipeline,
                    );
                    log_debug!("Done material render");
                }

                // Rendering text, etc.
                self.text_rendering_system.render(
                    &mut encoder,
                    &device_binding,
                    &queue_binding,
                    &view,
                    &self.material_manager.depth_texture_view,
                    &self.render_surface.surface_config,
                );

                queue_binding.submit(Some(encoder.finish()));
                frame.present();
                self.text_rendering_system.clear_buffer();
                let result = wgpu::Device::poll(&device_binding, wgpu::MaintainBase::Poll);
                log_debug!("Result: {:?}", result.is_queue_empty());
            }
            Err(e) => {
                log_error!("Failed to get frame texture: {:?}", e);
            }
        }
    }
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let device_binding = self.gpu.device();
        self.render_surface
            .resize(new_width, new_height, &device_binding);
    }

    pub fn change_material_color(&mut self, material_name: &str, new_color: Color) {
        let queue_binding = self.gpu.queue();
        self.material_manager.set_material_color(
            material_name,
            Color::from(new_color),
            &queue_binding,
            self.camera.projection_matrix().into(),
        );
    }
}
