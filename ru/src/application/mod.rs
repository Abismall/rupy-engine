pub mod bus;
pub mod event;
pub mod rupy;
use std::sync::Arc;

use crossbeam::channel::Sender;
use event::RupyAppEvent;
use wgpu::{
    CompositeAlphaMode, Device, PresentMode, SurfaceCapabilities, SurfaceConfiguration,
    TextureFormat, TextureUsages,
};
use winit::dpi::PhysicalSize;

use crate::{log_debug, prelude::AppError};

pub struct SurfaceWrapper {
    surface: wgpu::Surface<'static>,
    surface_config: SurfaceConfiguration,
}

impl SurfaceWrapper {
    const USAGE: TextureUsages = wgpu::TextureUsages::RENDER_ATTACHMENT;
    const ALPHA_MODE: CompositeAlphaMode = wgpu::CompositeAlphaMode::Auto;
    const PRESENT_MODE: PresentMode = PresentMode::Fifo;
    const MAXIMUM_FRAME_LATENCY: u32 = 2;
    pub fn new(surface: wgpu::Surface<'static>) -> Self {
        Self {
            surface,
            surface_config: SurfaceConfiguration {
                usage: Self::USAGE,
                format: TextureFormat::Rgba8UnormSrgb,
                present_mode: Self::PRESENT_MODE,
                desired_maximum_frame_latency: Self::MAXIMUM_FRAME_LATENCY,
                alpha_mode: Self::ALPHA_MODE,
                view_formats: Vec::new(),
                width: 0,
                height: 0,
            },
        }
    }
    pub fn configure(
        &mut self,
        device: &Device,
        size: PhysicalSize<u32>,
        surface_capabilities: SurfaceCapabilities,
        swap_chain_format: TextureFormat,
    ) -> Result<(), AppError> {
        self.surface_config = wgpu::SurfaceConfiguration {
            usage: Self::USAGE,
            format: swap_chain_format,
            width: size.width,
            height: size.height,
            present_mode: Self::PRESENT_MODE,
            alpha_mode: Self::ALPHA_MODE,
            view_formats: surface_capabilities.formats,
            desired_maximum_frame_latency: Self::MAXIMUM_FRAME_LATENCY,
        };
        self.surface.configure(device, &self.surface_config);
        Ok(())
    }
    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        new_size: PhysicalSize<u32>,
        tx: Arc<Sender<RupyAppEvent>>,
    ) -> Result<(), AppError> {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&device, &self.surface_config);
            let _ = tx
                .send(RupyAppEvent::WindowResized {
                    width: new_size.width,
                    height: new_size.height,
                })
                .ok();
        }
        Ok(())
    }
    pub fn get_config(self) -> wgpu::SurfaceConfiguration {
        self.surface_config
    }
    pub fn get_surface(&self) -> &wgpu::Surface<'static> {
        &self.surface
    }
}
