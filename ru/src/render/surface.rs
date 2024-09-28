use std::sync::Arc;
use wgpu::{RenderPassColorAttachment, RenderPassDepthStencilAttachment, TextureView};

pub struct TargetSurface {
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub window: Arc<winit::window::Window>,
    pub current_texture: Option<wgpu::SurfaceTexture>,
    pub depth_texture: wgpu::Texture,
    pub depth_view: wgpu::TextureView,
}

impl TargetSurface {
    pub fn new(
        window: Arc<winit::window::Window>,
        device: &wgpu::Device,
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
    ) -> Self {
        let depth_texture = Self::create_depth_texture(device, &surface_config);
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        surface.configure(device, &surface_config);

        Self {
            surface,
            surface_config,
            window,
            current_texture: None,
            depth_texture,
            depth_view,
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32, device: &wgpu::Device) {
        self.surface_config.width = new_width;
        self.surface_config.height = new_height;
        self.surface.configure(device, &self.surface_config);

        self.depth_texture = Self::create_depth_texture(device, &self.surface_config);
        self.depth_view = self
            .depth_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
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
        }
    }
    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
    }
}

pub struct ViewWrapper {
    view: TextureView,
}

impl ViewWrapper {
    pub fn as_color_attachment(&self) -> RenderPassColorAttachment {
        RenderPassColorAttachment {
            view: &self.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                store: wgpu::StoreOp::Store,
            },
        }
    }

    pub fn as_depth_attachment(&self) -> RenderPassDepthStencilAttachment {
        RenderPassDepthStencilAttachment {
            view: &self.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }
    }
}
