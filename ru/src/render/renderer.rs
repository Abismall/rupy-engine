use crossbeam::{channel, thread};
use std::sync::{Arc, RwLock};
use wgpu::{CommandEncoder, Device, Queue, Surface, SurfaceConfiguration};

use crate::{log_error, prelude::AppError};

pub struct Renderer {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface: Arc<RwLock<Surface<'static>>>,
    pub surface_config: Arc<RwLock<SurfaceConfiguration>>,
    command_sender: channel::Sender<CommandEncoder>,
}

impl Renderer {
    pub async fn new(
        instance: &wgpu::Instance,
        surface: wgpu::Surface<'static>,
    ) -> Result<Self, AppError> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| AppError::GpuAdapterNotFound)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await?;
        let surface_capabilities = surface.get_capabilities(&adapter);
        let supported_formats = surface_capabilities.formats;
        let swap_chain_format = supported_formats[0];

        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swap_chain_format,
            width: 800,
            height: 600,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: supported_formats,
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let (command_sender, command_receiver) = channel::unbounded();

        let device_arc = Arc::new(device);
        let queue_arc = Arc::new(queue);

        thread::scope(|s| {
            for _ in 0..4 {
                let command_sender = command_sender.clone();
                let device = Arc::clone(&device_arc);
                s.spawn(move |_| {
                    let command_encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Worker Command Encoder"),
                        });

                    if let Err(e) = command_sender.send(command_encoder) {
                        eprintln!("Failed to send command encoder: {:?}", e);
                    }
                });
            }

            for command_encoder in command_receiver.iter() {
                queue_arc.submit(Some(command_encoder.finish()));
            }
        })
        .map_err(|e| AppError::ThreadScopeError(format!("Thread scope error: {:?}", e)))?;

        Ok(Self {
            device: Arc::clone(&device_arc),
            queue: Arc::clone(&queue_arc),
            surface: Arc::new(RwLock::new(surface)),
            surface_config: Arc::new(RwLock::new(surface_config)),
            command_sender,
        })
    }

    pub fn resize(&self, width: u32, height: u32) {
        let mut config = self.surface_config.write().unwrap();
        config.width = width;
        config.height = height;

        let surface = self.surface.write().unwrap();
        surface.configure(&self.device, &config);
    }

    pub fn submit_command(&self, encoder: CommandEncoder) {
        if let Err(e) = self.command_sender.send(encoder) {
            log_error!("Failed to submit command encoder: {:?}", e);
        }
    }
}

fn worker_thread_function(renderer: Arc<Renderer>) {
    let encoder = renderer
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Worker Command Encoder"),
        });

    renderer.submit_command(encoder);
}
