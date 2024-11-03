use std::sync::Arc;
use wgpu::{
    Adapter, CompositeAlphaMode, Device, PresentMode, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages,
};
use winit::window::Window;

use crate::{app::coalesce_format, log_debug, prelude::helpers::window_inner_size_to_vector2};

pub struct RenderSurface {
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub window: Arc<Window>,
}

impl RenderSurface {
    pub fn new(
        window: Arc<Window>,
        device: &Device,
        adapter: &Adapter,
        surface: Surface<'static>,
    ) -> Self {
        let surface_config = default_surface_configuration(&surface, adapter, &window);
        configure_surface(&surface, device, &surface_config);
        Self {
            surface,
            surface_config,
            window,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, device: &Device) {
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        configure_surface(&self.surface, device, &self.surface_config);
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }
}
pub fn surface_configuration(
    format: TextureFormat,
    width: u32,
    height: u32,
    present_mode: PresentMode,
    alpha_mode: CompositeAlphaMode,
    usage: TextureUsages,
    view_formats: Vec<TextureFormat>,
    desired_maximum_frame_latency: u32,
) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage,
        format,
        width,
        height,
        present_mode,
        alpha_mode,
        view_formats,
        desired_maximum_frame_latency,
    }
}

pub fn default_surface_configuration(
    surface: &Surface,
    adapter: &Adapter,
    window: &Window,
) -> SurfaceConfiguration {
    let surface_size = window_inner_size_to_vector2(window);
    let surface_caps = surface.get_capabilities(adapter);
    let format = coalesce_format(&surface_caps);
    log_debug!("{:?}", format);
    surface_configuration(
        coalesce_format(&surface_caps),
        surface_size.x,
        surface_size.y,
        PresentMode::Mailbox,
        surface_caps.alpha_modes[0],
        TextureUsages::RENDER_ATTACHMENT,
        (&[format]).to_vec(),
        1,
    )
}

pub fn configure_surface(surface: &Surface, device: &Device, config: &SurfaceConfiguration) {
    surface.configure(device, &config);
}
