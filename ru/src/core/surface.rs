use wgpu::{
    CompositeAlphaMode, Device, PresentMode, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages,
};
use winit::dpi::PhysicalSize;

pub struct RenderSurface<'a> {
    pub surface: Surface<'a>,
    pub config: SurfaceConfiguration,
}

impl<'a> RenderSurface<'a> {
    pub fn new(surface: Surface<'a>, size: PhysicalSize<u32>, adapter: &wgpu::Adapter) -> Self {
        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![surface_format.add_srgb_suffix()],
            desired_maximum_frame_latency: 2,
        };
        Self { surface, config }
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&device, &self.config);
        }
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
        width: width.max(1),
        height: height.max(1),
        present_mode,
        alpha_mode,
        view_formats,
        desired_maximum_frame_latency,
    }
}
