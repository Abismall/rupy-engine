use wgpu::{Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

pub struct RenderSurface<'a> {
    pub surface: Surface<'a>,
    pub config: SurfaceConfiguration,
}

impl<'a> RenderSurface<'a> {
    pub fn new(
        surface: Surface<'a>,
        size: PhysicalSize<u32>,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
    ) -> Self {
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
        surface.configure(device, &config);
        Self { surface, config }
    }
    pub fn configure(&mut self, device: &wgpu::Device) {
        self.surface.configure(device, &self.config)
    }
    pub fn update_config_size<P: winit::dpi::Pixel>(
        &mut self,
        new_size: winit::dpi::PhysicalSize<P>,
    ) -> bool {
        let width: u32 = new_size.width.cast();
        let height: u32 = new_size.height.cast();
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            true
        } else {
            false
        }
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }
}
