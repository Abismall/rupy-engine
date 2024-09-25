use std::sync::Arc;
use wgpu::{Device, Surface, SurfaceConfiguration};
use winit::window::Window;

pub struct RenderSurface {
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub window: Arc<Window>,
}

impl RenderSurface {
    pub fn new(
        window: Arc<Window>,
        device: &Device,
        surface: Surface<'static>,
        surface_config: SurfaceConfiguration,
    ) -> Self {
        surface.configure(device, &surface_config);
        Self {
            surface,
            surface_config,
            window,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, device: &Device) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(device, &self.surface_config);
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }
}
