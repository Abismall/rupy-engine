use wgpu::{Device, Surface, SurfaceConfiguration, TextureUsages};
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Debug)]
pub struct SurfaceWrapper {
    pub current: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
}

impl SurfaceWrapper {
    pub fn new(config: SurfaceConfiguration, surface: wgpu::Surface<'static>) -> Self {
        SurfaceWrapper {
            config,
            current: surface,
        }
    }

    pub fn get(&self) -> &Surface<'static> {
        &self.current
    }
    pub fn conform(&mut self, window: &Window, device: &Device) {
        let inner_size = window.inner_size();
        self.config.height = inner_size.height.max(1);
        self.config.width = inner_size.width.max(1);
        self.current.configure(device, &self.config);
    }
}

impl SurfaceWrapper {
    pub fn default_config(size: PhysicalSize<f32>) -> SurfaceConfiguration {
        SurfaceConfiguration {
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            present_mode: wgpu::PresentMode::Fifo,
            view_formats: (&[]).to_vec(),
            width: size.width as u32,
            height: size.height as u32,
            desired_maximum_frame_latency: 1,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            usage: TextureUsages::RENDER_ATTACHMENT,
        }
    }
}
