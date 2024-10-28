use super::surface::SurfaceWrapper;
use crate::{core::error::AppError, graphics::gpu::get_instance, log_error};
use std::sync::Arc;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

pub struct WindowWrapper {
    pub active: Option<Arc<winit::window::Window>>,
    pub target: Option<SurfaceWrapper>,
    pub id: Option<WindowId>,
    pub device: Option<std::sync::Arc<wgpu::Device>>,
}
fn default_window_attributes() -> WindowAttributes {
    WindowAttributes::default()
        .with_inner_size(PhysicalSize {
            width: 800.0,
            height: 600.0,
        })
        .with_title("RupyEngine")
        .with_visible(false)
}

pub fn create_window(
    event_loop: &ActiveEventLoop,
    attr_override: WindowAttributes,
) -> Result<winit::window::Window, AppError> {
    match event_loop.create_window(attr_override) {
        Ok(w) => Ok(w),
        Err(e) => Err(e.into()),
    }
}
impl WindowWrapper {
    pub fn new() -> Self {
        Self {
            device: None,
            target: None,
            id: None,
            active: None,
        }
    }
    pub fn update(&mut self) {
        if self.device.is_some() {
            if self.active.is_some() {
                if self.target.is_some() {
                    self.target
                        .as_mut()
                        .unwrap()
                        .conform(self.active.as_ref().unwrap(), self.device.as_ref().unwrap());
                }
            }
        }
    }
    pub fn set_device(&mut self, device: std::sync::Arc<wgpu::Device>) {
        self.device = Some(device.clone());
    }
    pub fn set_surface(&mut self, surface: SurfaceWrapper) {
        self.target = Some(surface);
    }

    pub fn request_redraw(&self) -> Result<(), AppError> {
        if let Some(win) = &self.active {
            Ok(win.request_redraw())
        } else {
            Err(AppError::NoActiveWindowError)
        }
    }
    pub fn size(&self) -> std::option::Option<PhysicalSize<u32>> {
        if let Some(window) = &self.active {
            return Some(window.inner_size());
        }
        None
    }
    pub fn scale_factor(&self) -> f64 {
        if let Some(window) = &self.active {
            return window.scale_factor();
        }
        1.0
    }
    pub fn create_surface<'surface>(&mut self) -> Result<(), AppError> {
        if let Some(window) = &self.active {
            let surface = match get_instance()?.create_surface(window.clone()) {
                Ok(value) => value,
                Err(e) => {
                    log_error!("Failed to create surface: {:?}", e);
                    return Err(e.into());
                }
            };
            let size: PhysicalSize<f32> = window.inner_size().cast();
            let config = SurfaceWrapper::default_config(size);
            let surface_wrapper = SurfaceWrapper::new(config, surface);
            self.target = Some(surface_wrapper);

            Ok(())
        } else {
            log_error!("Failed to create surface, no active window!");
            Err(AppError::NoActiveWindowError)
        }
    }
    pub fn set_window(&mut self, event_loop: &ActiveEventLoop) -> Result<(), AppError> {
        self.active = Some(create_window(event_loop, default_window_attributes())?.into());
        Ok(())
    }

    pub fn set_visible(&self) {
        if let Some(window) = &self.active {
            window.set_visible(true);
        }
    }
    pub fn window_exists(&self) -> bool {
        self.active.is_some()
    }
}
