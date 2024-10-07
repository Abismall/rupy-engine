use std::{collections::HashMap, sync::Arc, time::SystemTime};

use wgpu::SurfaceConfiguration;
use winit::{
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{log_info, AppError, Rupy, SystemEvent};

pub trait WindowManager {
    fn configure_surface(
        window: &Window,
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
    ) -> Result<SurfaceConfiguration, AppError>;
    fn create_window(
        &mut self,
        rupy_window: RupyWindow,
        el: &ActiveEventLoop,
        control_flow: Option<ControlFlow>,
    ) -> Result<Window, AppError>;

    fn destroy_window(&mut self, window_id: WindowId);
}

#[derive(Debug)]
pub enum RupyWindow {
    Settings,
    Main,
    ShaderSandbox,
}

impl RupyWindow {
    pub fn attributes(&self) -> WindowAttributes {
        match self {
            RupyWindow::Settings => WindowAttributes::default(),
            RupyWindow::Main => WindowAttributes::default(),
            RupyWindow::ShaderSandbox => WindowAttributes::default(),
        }
    }
}
impl WindowManager for Rupy {
    fn create_window(
        &mut self,
        rupy_window: RupyWindow,
        el: &ActiveEventLoop,
        control_flow: Option<ControlFlow>,
    ) -> Result<Window, AppError> {
        if let Some(flow) = control_flow {
            el.set_control_flow(flow);
        }

        let window_attributes = rupy_window.attributes();
        let window = el
            .create_window(window_attributes)
            .map_err(|e| AppError::from(e))?;
        let _ = self
            .tx
            .send(SystemEvent::In(super::bus::SystemEventIn::CreateWindow(
                window.id(),
            )));

        Ok(window)
    }

    fn destroy_window(&mut self, window_id: WindowId) {
        log_info!("Destroying window: {:?}", window_id);
        self.tx
            .send(SystemEvent::In(
                crate::application::bus::SystemEventIn::Shutdown,
            ))
            .expect("Failed to send DestroyWindow command");
    }
    fn configure_surface(
        window: &Window,
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
    ) -> Result<SurfaceConfiguration, AppError> {
        let window_size = window.inner_size();
        let surface_caps = surface.get_capabilities(adapter);
        let swap_chain_format = surface_caps.formats[0];

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swap_chain_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(device, &surface_config);
        Ok(surface_config)
    }
}

pub struct RupyWindowContext {
    main: bool,
    created: std::time::SystemTime,
}
impl RupyWindowContext {
    pub fn new(&self, main: bool, created: SystemTime) -> Arc<RupyWindowContext> {
        Arc::new(RupyWindowContext { main, created })
    }
}
pub fn register_window(window_id: &WindowId, windows: &mut HashMap<WindowId, RupyWindowContext>) {
    let id = window_id;
    if windows.contains_key(&id) {
        return;
    } else {
        windows.insert(
            *window_id,
            RupyWindowContext {
                main: windows.len() == 0,
                created: SystemTime::now(),
            },
        );
    }
}
