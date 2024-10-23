use crossbeam::channel::Sender;
use glyphon::{Resolution, TextBounds};
use winit::{event_loop::ActiveEventLoop, window::WindowAttributes};

use super::{resources::ResourceManager, state::AppState, worker::RupyWorkerTask};
#[cfg(feature = "logging")]
use crate::rupyLogger;
use crate::{
    core::{error::AppError, time::Time},
    events::RupyAppEvent,
    graphics::gpu::{get_device, get_instance},
    log_info,
    model::{surface::SurfaceWrapper, window::WindowWrapper},
    scene::systems::render::RenderSystem,
    system::{
        glyphon::{get_text_bounds, GlyphonRender},
        input::manager::InputManager,
    },
    traits::bus::EventProxyTrait,
};
use std::sync::Arc;

pub struct Rupy {
    pub state: AppState,
    pub resources: ResourceManager,
    pub event_proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>,
    pub event_tx: Arc<Sender<RupyAppEvent>>,
    pub task_tx: Sender<RupyWorkerTask>,
    pub input: InputManager,
    #[cfg(feature = "logging")]
    pub logger: rupyLogger::factory::LogFactory,
    pub renderer: Option<RenderSystem>,
    pub glyphon: Option<GlyphonRender>,
    pub time: Time,
    pub device: Option<Arc<wgpu::Device>>,
    pub queue: Option<Arc<wgpu::Queue>>,
    pub surface: Option<SurfaceWrapper>,
    pub window: WindowWrapper,
}
impl Rupy {
    pub fn exit_process(&mut self, grace_period_secs: u32) {
        self.set_flag(AppState::SHUTDOWN);
        std::thread::spawn(move || {
            log_info!("Shutting down in {}", grace_period_secs);
            std::thread::sleep(std::time::Duration::from_secs(grace_period_secs.into()));
            log_info!("Shutting down now.");
            std::process::exit(0);
        });
    }
    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        attributes: WindowAttributes,
        title: String,
        width: f32,
        height: f32,
    ) {
        self.window
            .set_window(event_loop, attributes, title, width, height);
    }
    pub fn create_and_configure_surface(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), AppError> {
        let window = self
            .window
            .current
            .as_ref()
            .ok_or(AppError::NoActiveWindowError)?;
        let instance = get_instance()?;
        let surface = instance.create_surface(window.clone())?;

        let surface_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        let device: &Arc<wgpu::Device> = &get_device()?;
        surface.configure(device, &config);

        self.surface = Some(SurfaceWrapper::new(config, surface));
        log_info!(
            "Surface configured with width: {}, height: {}",
            width,
            height
        );

        Ok(())
    }

    pub fn send_event(&self, event: RupyAppEvent) -> std::result::Result<(), AppError> {
        self.event_tx.send(event).map_err(AppError::EventSendError)
    }
    pub fn send_task(&self, task: RupyWorkerTask) -> Result<(), AppError> {
        self.task_tx
            .send(task)
            .map_err(AppError::TaskQueueSendError)
    }
    pub fn set_flag(&mut self, flag: AppState) {
        self.state.set_flag(flag);
    }
    pub fn remove_flag(&mut self, flag: AppState) {
        self.state.remove_flag(flag);
    }
    pub fn contains_flag(&self, flag: AppState) -> bool {
        self.state.contains_flag(flag)
    }
}

impl Rupy {
    pub fn render_pass(&mut self) -> Result<(), AppError> {
        if let Some(surface) = &self.surface {
            let device = self
                .device
                .as_ref()
                .ok_or(AppError::InstanceInitializationError)?;
            let queue = self
                .queue
                .as_ref()
                .ok_or(AppError::InstanceInitializationError)?;

            let frame = surface
                .current
                .get_current_texture()
                .map_err(|e| AppError::SurfaceError(e))?;
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            let render_pass_descriptor = wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            };
            {
                let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);

                if let Some(glyphon) = &self.glyphon {
                    glyphon.render(&mut render_pass)?;
                }
            }

            queue.submit(Some(encoder.finish()));

            frame.present();
        }
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), AppError> {
        self.time.update();

        if let Some(glyphon) = &mut self.glyphon {
            let device = self
                .device
                .as_ref()
                .ok_or(AppError::InstanceInitializationError)?;
            let queue = self
                .queue
                .as_ref()
                .ok_or(AppError::InstanceInitializationError)?;

            let text = format!("{:#?}", self.time.debug(),);

            let inner_size = self.window.size();
            let text_bounds = get_text_bounds(
                inner_size.width as i32,
                inner_size.height as i32,
                Some(10),
                Some(10),
            );

            Self::prepare_text(
                device,
                queue,
                text,
                Resolution {
                    width: inner_size.width,
                    height: inner_size.height,
                },
                text_bounds,
                glyphon,
            );
        }

        Ok(())
    }

    pub(crate) fn prepare_text(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        text: String,
        resolution: Resolution,
        bounds: TextBounds,
        glyphon: &mut GlyphonRender,
    ) {
        glyphon.reconfigure(queue, resolution);
        glyphon.set_buffer_size((resolution.width as f32, resolution.height as f32));
        glyphon.set_buffer_text(&text);
        glyphon.prepare_text(&device, &queue, bounds).ok();
        glyphon.shape_until_scroll(false);
    }
}
