use std::sync::Arc;
use wgpu::{Device, Surface, SurfaceConfiguration, SurfaceTexture, TextureView};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    core::error::AppError,
    graphics::gpu::{get_device, get_instance},
    log_error,
};
#[derive(Debug)]
pub struct SurfaceWrapper {
    pub current: wgpu::Surface<'static>,
    pub target_window: Option<Arc<Window>>,
    pub config: wgpu::SurfaceConfiguration,
    pub desired_maximum_latency: u32,
    pub alpha_mode: wgpu::CompositeAlphaMode,
    pub present_mode: wgpu::PresentMode,
}

impl SurfaceWrapper {
    pub fn new(config: SurfaceConfiguration, surface: wgpu::Surface<'static>) -> Self {
        SurfaceWrapper {
            config,
            current: surface,
            target_window: None,
            desired_maximum_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            present_mode: wgpu::PresentMode::Fifo,
        }
    }
    fn create_surface(&mut self, window: Arc<Window>) -> std::result::Result<(), AppError> {
        let instance = get_instance()?;
        let surface = instance
            .create_surface(window)
            .map_err(|e| AppError::CreateSurfaceError(e))?;

        self.current = surface;
        Ok(())
    }
    pub fn get(&self) -> &Surface<'static> {
        &self.current
    }
    fn handle_surface_lost(
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Result<wgpu::SurfaceTexture, AppError> {
        surface.configure(device, config);
        surface
            .get_current_texture()
            .map_err(AppError::SurfaceError)
    }

    pub fn get_current_surface_texture(
        &mut self,
        device: &Device,
    ) -> Result<SurfaceTexture, AppError> {
        match &self.target_window {
            Some(w) => {
                if let Err(e) = self.create_surface(w.clone()) {
                    return Err(e);
                }
            }
            None => return Err(AppError::NoActiveWindowError),
        }

        let texture = match self.current.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost) => {
                Self::handle_surface_lost(&self.current, device, &self.config)?
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                log_error!("Out of memory");
                std::process::exit(1);
            }
            Err(e) => {
                log_error!("Failed to acquire next surface texture: {:?}", e);
                return Err(AppError::SurfaceError(e));
            }
        };
        Ok(texture)
    }
    pub fn create_texture_view(
        &self,
        surface_texture: &SurfaceTexture,
        texture_view_description: &wgpu::TextureViewDescriptor,
    ) -> Result<TextureView, AppError> {
        let view = surface_texture
            .texture
            .create_view(texture_view_description);
        Ok(view)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Result<(), AppError> {
        let _ = self.configure(
            None,
            None,
            None,
            None,
            Some(size.width.max(1)),
            Some(size.height.max(1)),
        );
        Ok(())
    }

    pub fn configure(
        &mut self,
        swap_chain_format: Option<wgpu::TextureFormat>,
        alpha_mode: Option<wgpu::CompositeAlphaMode>,
        present_mode: Option<wgpu::PresentMode>,
        desired_maximum_latency: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<(), AppError> {
        self.config.format = swap_chain_format.unwrap_or(self.config.format);
        self.alpha_mode = alpha_mode.unwrap_or(self.alpha_mode);
        self.present_mode = present_mode.unwrap_or(self.present_mode);
        self.desired_maximum_latency =
            desired_maximum_latency.unwrap_or(self.desired_maximum_latency);

        self.config.width = width.unwrap_or(self.config.width).max(1);
        self.config.height = height.unwrap_or(self.config.height).max(1);

        let device = get_device()?;
        self.current.configure(&device, &self.config);

        Ok(())
    }
}
