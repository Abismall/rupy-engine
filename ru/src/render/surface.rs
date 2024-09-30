use std::sync::{Arc, RwLock};

use wgpu::{SurfaceConfiguration, TextureDimension, TextureFormat};

use crate::log_error;

pub struct TargetSurface {
    pub surface: wgpu::Surface<'static>,
    pub window: Arc<winit::window::Window>,
    pub current_texture: Option<wgpu::SurfaceTexture>,
    pub depth_texture: wgpu::Texture,
    pub depth_view: wgpu::TextureView,
    pub surface_config: SurfaceConfiguration,
}

impl TargetSurface {
    pub fn new(
        window: Arc<winit::window::Window>,
        device: Arc<wgpu::Device>,
        surface_config: SurfaceConfiguration,
        surface: wgpu::Surface<'static>,
    ) -> Self {
        surface.configure(&device, &surface_config);
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            surface,
            window,
            current_texture: None,
            depth_texture,
            depth_view,
            surface_config,
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32, device: Arc<wgpu::Device>) {
        self.surface_config.width = new_width;
        self.surface_config.height = new_height;
        self.surface.configure(&device, &self.surface_config);
        let (texture, view) = create_depth_texture(
            &device,
            &self.surface_config,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        self.depth_texture = texture;
        self.depth_view = view;
    }

    pub fn acquire_current_texture(&mut self) -> Result<(), wgpu::SurfaceError> {
        let texture = self.surface.get_current_texture()?;
        self.current_texture = Some(texture);
        Ok(())
    }

    pub fn present_texture(&mut self) {
        if let Some(texture) = self.current_texture.take() {
            texture.present();
        } else {
            log_error!("No texture available to present.");
        }
    }
}
pub fn create_depth_texture(
    device: &wgpu::Device,
    sc_desc: &wgpu::SurfaceConfiguration,
    label: Option<&str>,
    usage: Option<wgpu::TextureUsages>,
    format: Option<TextureFormat>,
    dimension: Option<TextureDimension>,
    sample_count: Option<u32>,
    mip_level_count: Option<u32>,
) -> (wgpu::Texture, wgpu::TextureView) {
    let size = wgpu::Extent3d {
        width: sc_desc.width,
        height: sc_desc.height,
        depth_or_array_layers: 1,
    };

    let format = match format {
        Some(val) => val,
        None => wgpu::TextureFormat::Depth32Float,
    };
    let sample_count = match sample_count {
        Some(val) => val,
        None => 1,
    };
    let mip_level_count = match mip_level_count {
        Some(val) => val,
        None => 1,
    };
    let dimension = match dimension {
        Some(val) => val,
        None => wgpu::TextureDimension::D2,
    };
    let usage = match usage {
        Some(val) => val,
        None => wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label,
        size,
        mip_level_count,
        sample_count,
        dimension,
        format,
        usage,
        view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    (texture, view)
}
