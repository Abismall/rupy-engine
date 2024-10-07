use std::{collections::HashMap, sync::Arc};

use crossbeam::channel::Sender;

use pollster::FutureExt;
use wgpu::{core::instance::RequestAdapterOptions, Adapter, Device, Queue};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{WindowAttributes, WindowId},
};

#[cfg(feature = "logging")]
use crate::rupyLogger;
use crate::{
    camera::{perspective::CameraPreset, Camera},
    input::{manager::InputManager, InputEvent},
    log_debug, log_info, log_warning,
    prelude::{AppError, CameraPerspective},
    render::window::{RupyWindow, WindowManager},
    rupyLogger::LogFactory,
};

use super::{
    event::{EventProxyTrait, RupyAppEvent},
    SurfaceWrapper,
};

pub struct Rupy {
    #[cfg(feature = "logging")]
    logger: rupyLogger::LogFactory,
    pub(crate) tx: Arc<Sender<RupyAppEvent>>,
    input: InputManager,
    adapter: Option<Arc<Adapter>>,
    device: Option<Arc<Device>>,
    queue: Option<Arc<Queue>>,
    windows: HashMap<WindowId, WindowAttributes>,
    surface: Option<SurfaceWrapper>,
    camera: Camera,
    perspective: CameraPerspective,
    proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>,
}

impl Rupy {
    pub fn new(
        tx: Arc<Sender<RupyAppEvent>>,
        input: InputManager,
        proxy: Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>,
    ) -> Self {
        Rupy {
            tx,
            input,
            proxy,
            camera: Camera::new(None),
            perspective: CameraPerspective::from_preset(CameraPreset::Standard),
            windows: HashMap::new(),
            logger: LogFactory::default(),
            adapter: None,
            device: None,
            queue: None,
            surface: None,
        }
    }
    fn is_initialized(&self) -> bool {
        if self.windows.is_empty()
            || self.device.is_none()
            || self.queue.is_none()
            || self.surface.is_none()
        {
            false
        } else {
            true
        }
    }
    fn toggle_audio(&mut self) {
        self.tx.send(RupyAppEvent::ToggleAudio).ok();
    }
    fn transmitter(&mut self) -> Arc<Sender<RupyAppEvent>> {
        self.tx.clone()
    }
    fn register_window(&mut self, window_id: WindowId, attributes: WindowAttributes) {
        if self.windows.contains_key(&window_id) {
            return;
        } else {
            self.windows.insert(window_id, attributes);
        }
    }
    pub fn set_event_proxy(
        &mut self,
        event_proxy: &Arc<dyn EventProxyTrait<RupyAppEvent> + Send + Sync>,
    ) {
        self.input.set_event_proxy(event_proxy.clone());
    }
    pub async fn initialize(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) -> Result<Arc<winit::window::Window>, AppError> {
        let window = Arc::new(self.create_window(RupyWindow::Main, event_loop, None)?);
        let id = window.id();
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Adapter"); // TODO: Fix

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await?;
        self.device = Some(Arc::new(device));
        self.adapter = Some(Arc::new(adapter));
        self.queue = Some(Arc::new(queue));
        self.surface = Some(SurfaceWrapper::new(surface));
        self.register_window(id, WindowAttributes::default().with_inner_size(size));

        Ok(window)
    }

    pub fn shutdown(&self, grace_period_secs: u64) {
        let tx_clone = self.tx.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(grace_period_secs));
            let _ = tx_clone.send(RupyAppEvent::CloseRequested);

            std::process::exit(0);
        });
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) {}
}

impl ApplicationHandler<RupyAppEvent> for Rupy {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.shutdown(0);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.input.handle_event(InputEvent::Key(event));
            }
            WindowEvent::Resized(size) => {
                if let Some(device) = &self.device {
                    if let Some(adapter) = &self.adapter {
                        if let Some(surface) = &mut self.surface {
                            let tx_resize = self.tx.clone();
                            let _ =
                                Self::configure_surface(size, &surface.surface, &adapter, device);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: RupyAppEvent) {
        match event {
            RupyAppEvent::CloseRequested => {
                event_loop.exit();
                log_debug!("Bye!");
                std::process::exit(0);
            }

            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.is_initialized() {
            if let Err(e) = self.initialize(event_loop).block_on() {
                log_warning!("Failed to initialize state on resume: {:?}", e);
            }
        }
    }
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        log_info!("Event loop exit: {:?}", _event_loop);
    }
}
