use std::sync::Arc;
use wgpu::{Adapter, CompositeAlphaMode, Device, Queue, TextureUsages};

use crate::{log_debug, log_error, prelude::AppError};

pub struct GpuContext {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub adapter: Arc<Adapter>,
}

impl GpuContext {
    pub async fn initialize() -> Result<GpuContext, AppError> {
        log_debug!("Initializing GPU context.");

        let gpu_context = GpuContext::new().await?;

        log_debug!("GPU context successfully initialized.");
        Ok(gpu_context)
    }

    pub fn get(&self) -> Arc<GpuContext> {
        Arc::new(GpuContext {
            device: self.device.clone(),
            queue: self.queue.clone(),
            adapter: self.adapter.clone(),
        })
    }

    pub async fn new() -> Result<Self, AppError> {
        log_debug!("Creating new GPU context.");
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        log_debug!("Requesting adapter.");
        let option_adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await;
        let adapter = option_adapter.ok_or(AppError::GpuAdapterNotFound)?;

        log_debug!("Requesting device and queue.");
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .map_err(AppError::GpuDeviceCreationError)?;

        log_debug!("Device and queue created successfully.");
        Ok(Self {
            device: device.into(),
            queue: queue.into(),
            adapter: adapter.into(),
        })
    }
}

use wgpu::{Surface, SurfaceCapabilities, SurfaceConfiguration, TextureFormat};
use winit::dpi::PhysicalSize;

pub struct GpuResources {
    pub context: GpuContext,
    pub surface: Option<Surface<'static>>,
    pub surface_configuration: Option<SurfaceConfiguration>,
    pub swap_chain_format: Option<TextureFormat>,
}

impl GpuResources {
    pub fn new(context: GpuContext) -> Self {
        Self {
            context,
            surface: None,
            surface_configuration: None,
            swap_chain_format: None,
        }
    }
    pub fn configure_surface(&mut self, size: PhysicalSize<u32>) -> Result<(), AppError> {
        if size.width == 0 || size.height == 0 {
            log::error!("Cannot reconfigure surface with zero width or height.");
            return Err(AppError::SurfaceConfigurationError);
        }

        if let Some(surface) = &self.surface {
            let swap_chain_format = match self.swap_chain_format {
                Some(format) => format,
                None => {
                    log::error!("Swap chain format not initialized.");
                    return Err(AppError::SurfaceConfigurationError);
                }
            };

            let surface_configuration = SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: swap_chain_format,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: CompositeAlphaMode::Inherit,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            surface.configure(&self.context.device, &surface_configuration);
            self.surface_configuration = Some(surface_configuration);
            Ok(())
        } else {
            log_error!("No surface available to configure.");

            Err(AppError::SurfaceConfigurationError)
        }
    }
    pub fn initialize_surface(
        &mut self,
        size: PhysicalSize<u32>,
        surface_capabilities: SurfaceCapabilities,
    ) -> Result<(), AppError> {
        if let Some(surface) = &self.surface {
            let swap_chain_format = self
                .swap_chain_format
                .unwrap_or(surface_capabilities.formats[0]);
            let surface_configuration = SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: swap_chain_format,
                width: size.width,
                height: size.height,
                present_mode: surface_capabilities.present_modes[0],
                desired_maximum_frame_latency: 2,
                alpha_mode: CompositeAlphaMode::Auto,
                view_formats: Vec::new(),
            };

            surface.configure(&self.context.device, &surface_configuration);

            self.surface_configuration = Some(surface_configuration);
            self.swap_chain_format = Some(swap_chain_format);
            Ok(())
        } else {
            Err(AppError::GpuInitializationError)
        }
    }

    pub fn reconfigure_surface(
        &mut self,
        size: PhysicalSize<u32>,
        surface_capabilities: SurfaceCapabilities,
    ) -> Result<(), AppError> {
        if let Some(surface) = &self.surface {
            let swap_chain_format = self
                .swap_chain_format
                .unwrap_or(surface_capabilities.formats[0]);
            let mut surface_configuration =
                self.surface_configuration
                    .clone()
                    .unwrap_or_else(|| SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format: swap_chain_format,
                        width: size.width,
                        height: size.height,
                        present_mode: surface_capabilities.present_modes[0],
                        desired_maximum_frame_latency: 2,
                        alpha_mode: CompositeAlphaMode::Auto,
                        view_formats: Vec::new(),
                    });

            surface_configuration.width = size.width;
            surface_configuration.height = size.height;

            surface.configure(&self.context.device, &surface_configuration);
            self.surface_configuration = Some(surface_configuration);
            Ok(())
        } else {
            Err(AppError::SurfaceConfigurationError)
        }
    }
}

pub fn get_default_surface_configuration(size: PhysicalSize<u32>) -> wgpu::SurfaceConfiguration {
    const DEFAULT_MAX_FRAME_LATENCY: u32 = 2;
    wgpu::SurfaceConfiguration {
        usage: TextureUsages::all(),
        format: TextureFormat::Rgba8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
        desired_maximum_frame_latency: DEFAULT_MAX_FRAME_LATENCY,
    }
}
