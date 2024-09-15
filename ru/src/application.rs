use std::{collections::HashMap, sync::Arc};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, Position},
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    constants::defaults::SIZE_LARGE,
    input::InputHandler,
    log_debug, log_error,
    logger::LogFactory,
    rendering::gpu::{Gpu, GpuConfig},
    window::build_window_attributes,
};

pub struct AppConfig {
    pub window: winit::window::WindowAttributes,
    pub(crate) gpu: GpuConfig,
    pub scene_id: u32,
    #[cfg(target_os = "android")]
    pub android: AndroidApp,
    #[cfg(feature = "logging")]
    pub logger: Option<LogFactory>,
    #[cfg(target_arch = "wasm32")]
    pub canvas_attrs: FxHashMap<String, String>,
    #[cfg(target_arch = "wasm32")]
    pub auto_scale_canvas: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl AppConfig {
    pub const FIRST_SCENE_ID: u32 = 0;
    pub fn new(#[cfg(target_os = "android")] android: AndroidApp) -> Self {
        AppConfig {
            window: build_window_attributes(
                SIZE_LARGE,
                "Rupy",
                Position::new(LogicalPosition::new(300, 150)),
                false,
            ),
            gpu: GpuConfig::default(),
            scene_id: Self::FIRST_SCENE_ID,

            #[cfg(target_os = "android")]
            android,
            #[cfg(feature = "logging")]
            logger: Some(Default::default()),
            #[cfg(target_arch = "wasm32")]
            auto_scale_canvas: true,
            #[cfg(target_arch = "wasm32")]
            canvas_attrs: {
                let mut map = FxHashMap::default();
                map.insert("tabindex".into(), "0".into());
                map.insert("oncontextmenu".into(), "return false;".into());
                map.insert(
                    "style".into(),
                    "margin: auto; position: absolute; top: 0; bottom: 0; left: 0; right: 0;"
                        .into(),
                );
                map
            },
        }
    }

    pub fn window(mut self, window: winit::window::WindowAttributes) -> Self {
        self.window = window;
        self
    }

    pub fn gpu(mut self, gpu: GpuConfig) -> Self {
        self.gpu = gpu;
        self
    }

    pub fn scene_id(mut self, scene_id: u32) -> Self {
        self.scene_id = scene_id;
        self
    }

    #[cfg(feature = "logging")]
    pub fn logger(mut self, logger: Option<crate::logger::LogFactory>) -> Self {
        self.logger = logger;
        self
    }

    #[cfg(target_arch = "wasm32")]
    pub fn canvas_attr(mut self, key: String, value: String) -> Self {
        self.canvas_attrs.insert(key, value);
        self
    }

    #[cfg(target_arch = "wasm32")]
    pub fn auto_scale_canvas(mut self, auto_scale_canvas: bool) -> Self {
        self.auto_scale_canvas = auto_scale_canvas;
        self
    }
}

pub enum AppState {
    Setup { config: AppConfig },
    Run(App),
}

pub struct App {
    pub(crate) input: InputHandler,
    pub(crate) gpu: Option<Arc<Gpu>>,
    parent_window_id: Option<WindowId>,
    windows: HashMap<WindowId, Arc<Window>>,
}

impl App {
    pub fn new(event_loop: &ActiveEventLoop, config: AppConfig) -> Self {
        let mut parent_window_id = None;
        let mut windows = HashMap::new();
        let window_id: WindowId;

        let input = InputHandler::new();

        match event_loop.create_window(config.window) {
            Ok(window) => {
                window_id = window.id();
                parent_window_id = Some(window_id);
                let win_arc = Arc::new(window);

                let gpu = Arc::new(pollster::block_on(Gpu::new(win_arc.clone(), config.gpu)));
                gpu.resume(&win_arc);

                windows.insert(window_id, win_arc);

                return Self {
                    input,
                    gpu: Some(gpu),
                    parent_window_id,
                    windows,
                };
            }
            Err(e) => {
                log_error!("Failed to create window: {}", e);
                return Self {
                    input,
                    gpu: Default::default(),
                    parent_window_id,
                    windows,
                };
            }
        }
    }
}

impl ApplicationHandler for AppState {
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match self {
            AppState::Run(app) => match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    if let Some(window) = app.windows.get(&id) {
                        if let Some(gpu) = &app.gpu {
                            let pipe = gpu.create_render_pipeline();

                            gpu.render(&pipe, window);
                        }
                    }
                }
                WindowEvent::Resized(new_size) => {
                    log_debug!("Resized window: {:?}", new_size);
                }
                _ => (), // Unhandled window events
            },
            AppState::Setup { .. } => {
                log_debug!("Received window event while uninitialized.");
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        if let AppState::Run(app) = self {
            match event {
                DeviceEvent::Key(event) => app.input.key(&event),
                DeviceEvent::MouseMotion { delta } => app.input.mousemotion(delta),
                DeviceEvent::MouseWheel { delta } => {
                    log_debug!("Mouse {:?}", delta);
                }
                DeviceEvent::Button { button, state } => match state {
                    winit::event::ElementState::Pressed => {
                        log_debug!("Mouse {:?} {:?}", button, state);
                    }
                    winit::event::ElementState::Released => {
                        log_debug!("Mouse {:?} {:?}", button, state);
                    }
                },
                _ => (), // Unhandled device events
            }
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            AppState::Setup { config } => {
                let config = std::mem::take(config);
                let app = App::new(event_loop, config);
                *self = AppState::Run(app);
            }
            AppState::Run(app) => match event_loop.create_window(WindowAttributes::default()) {
                Ok(window) => {
                    let window_id = window.id();
                    app.parent_window_id = Some(window_id);
                    match &app.gpu {
                        Some(gpu) => {
                            gpu.resume(&window);
                        }
                        None => (),
                    }
                    app.windows.insert(window_id, Arc::new(window));

                    log_debug!("Resumed window with ID: {:?}", window_id);
                }
                Err(e) => {
                    log_error!("Failed to resume event loop and create a window: {}", e);
                }
            },
        }
    }
}
