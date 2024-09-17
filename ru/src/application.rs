use pollster::FutureExt;
use rand::Rng;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use wgpu::{Buffer, BufferUsages, Device, Instance, Queue, RenderPipeline, SurfaceConfiguration};
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
    log_debug, log_error, log_info,
    logger::LogFactory,
    rendering::{
        command::RenderCommand,
        gpu::{render_frame, GpuConfig, GPU},
        pipeline::RenderPipelineManager,
        queue::RenderCommandQueue,
    },
    window::build_window_attributes,
};

pub struct AppConfig {
    pub window: winit::window::WindowAttributes,
    pub(crate) gpu_config: GpuConfig,
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
            gpu_config: GpuConfig::default(),
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

    pub fn gpu(mut self, gpu_config: GpuConfig) -> Self {
        self.gpu_config = gpu_config;
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
    Uninitialized { config: AppConfig },
    Initialized(App),
}

pub struct App {
    render_pipelines: HashMap<String, Arc<RenderPipeline>>,
    vertex_buffer: Option<Arc<Buffer>>,
    windows: HashMap<WindowId, Arc<Window>>,
    target_id: WindowId,
    instance: Arc<Instance>,
    render_queue: VecDeque<RenderCommand>,
    start_time: std::time::Instant,
    device: Option<Device>,
    queue: Option<Queue>,
    surface_config: Option<SurfaceConfiguration>,
    command_queue: RenderCommandQueue,
}

impl App {
    pub fn new(event_loop: &ActiveEventLoop, config: AppConfig) -> Self {
        let mut windows = HashMap::new();
        let window = event_loop.create_window(config.window).unwrap();
        let window_id = window.id();
        let arc_win = Arc::new(window);
        let arc_clone = arc_win.clone();
        windows.insert(window_id, arc_win);
        let instance = Arc::new(GPU::create_instance(config.gpu_config.backends));

        let mut app = Self {
            vertex_buffer: None,
            instance,
            render_pipelines: HashMap::default(),
            windows,
            target_id: window_id,
            render_queue: VecDeque::new(),
            start_time: std::time::Instant::now(),
            device: None,
            queue: None,
            surface_config: None,
            command_queue: RenderCommandQueue::new(),
        };
        app.initialize_rendering(&arc_clone).block_on();
        app
    }

    /// Initializes the GPU, device, queue, surface, and render pipeline.
    pub async fn initialize_rendering(&mut self, window: &Window) {
        let instance = GPU::create_instance(GpuConfig::default().backends);
        let surface = GPU::create_surface(&instance, window);

        // Request an adapter and device asynchronously
        let adapter = GPU::request_adapter(&instance, &surface).await;
        let (device, queue) = GPU::request_device(&adapter, &GpuConfig::default()).await;

        // Configure the surface
        let surface_format = GPU::get_surface_format(&surface, &adapter);
        let surface_config = GPU::default_surface_config(&surface, &adapter, window);
        surface.configure(&device, &surface_config);

        // Set up shaders and pipeline
        let vertex_shader = RenderPipelineManager::create_shader_module(
            &device,
            include_str!("./rendering/shaders/vertex_shader.wgsl"),
        )
        .expect("Failed to create vertex shader module.");

        let fragment_shader = RenderPipelineManager::create_shader_module(
            &device,
            include_str!("./rendering/shaders/fragment_shader.wgsl"),
        )
        .expect("Failed to create fragment shader module.");

        let bind_group_layout = RenderPipelineManager::create_bind_group_layout(&device);
        let pipeline_layout =
            RenderPipelineManager::create_pipeline_layout(&device, &bind_group_layout);
        let vertex_layout = RenderPipelineManager::define_vertex_layout();

        let render_pipeline = RenderPipelineManager::create_render_pipeline(
            &device,
            &pipeline_layout,
            &vertex_shader,
            &fragment_shader,
            vertex_layout,
            surface_format,
            wgpu::MultisampleState::default(),
        )
        .expect("Failed to create render pipeline.");

        // Store initialized resources
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface_config = Some(surface_config);
        self.render_pipelines
            .insert("default_pipeline".to_string(), Arc::new(render_pipeline));
    }

    /// Executes the render commands within the render loop.
    pub fn render(&mut self) {
        // Ensure device, queue, and pipeline are available
        let device = self.device.as_ref().expect("Device not initialized.");
        let queue = self.queue.as_ref().expect("Queue not initialized.");
        let window = self
            .windows
            .get(&self.target_id)
            .expect("Target window not found");
        let surface_config = self
            .surface_config
            .as_ref()
            .expect("Surface configuration not initialized.");

        let pipeline = self
            .render_pipelines
            .get("default_pipeline")
            .expect("Render pipeline not found.")
            .as_ref();
        let surface = GPU::create_surface(&self.instance, window);
        // Create a render frame and execute all commands
        render_frame(
            device,
            pipeline,
            queue,
            &surface,
            surface_config,
            &mut self.command_queue.commands,
            window,
        );
    }
}

impl ApplicationHandler for AppState {
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match self {
            AppState::Initialized(app) => match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    // Execute render on redraw request
                    if let Some(window) = app.windows.get(&app.target_id) {
                        log_debug!("Rendering to window: {:?}", window.id());
                        // app.render();
                    }
                }
                WindowEvent::Resized(_size) => {
                    // Handle resizing the surface
                    if let Some(device) = &app.device {
                        if let Some(window) = app.windows.get(&app.target_id) {
                            // let surface = GPU::create_surface(&app.instance, window);
                            // let adapter = GPU::request_adapter(&app.instance, &surface).block_on();
                            // GPU::resize_surface(&surface, device, window, &adapter);
                        }
                    }
                }
                ev => {
                    log_debug!("Window event: {:?}", ev);
                }
            },
            AppState::Uninitialized { .. } => {
                log_debug!("Received window event while uninitialized.");
            }
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            AppState::Uninitialized { config } => {
                let config = std::mem::take(config);
                let app = App::new(event_loop, config);

                *self = AppState::Initialized(app);
            }
            AppState::Initialized(app) => {
                if let Some(target_window) = app.windows.get(&app.target_id) {
                    log_debug!("Requesting redraw for ID: {:?}", app.target_id);
                }
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        if let AppState::Initialized(app) = self {
            match event {
                DeviceEvent::Key(event) => InputHandler::key(&event),
                DeviceEvent::MouseMotion { delta } => InputHandler::mousemotion(delta),
                DeviceEvent::MouseWheel { delta } => {}
                DeviceEvent::Button { button, state } => match state {
                    winit::event::ElementState::Pressed => {}
                    winit::event::ElementState::Released => {}
                },
                _ => (),
            }
        }
    }
}
