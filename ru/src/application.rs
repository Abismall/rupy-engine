use rand::Rng;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use wgpu::{util::DeviceExt, Buffer, BufferUsages};
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
        gpu::{Gpu, GpuConfig},
        render_command::RenderCommand,
    },
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
    pub(crate) render_pipeline: Option<Arc<wgpu::RenderPipeline>>,
    pub(crate) vertex_buffer: Option<Arc<Buffer>>,
    pub(crate) bind_group: Option<wgpu::BindGroup>,
    windows: HashMap<WindowId, Arc<Window>>,
    render_queue: VecDeque<RenderCommand>,
    start_time: std::time::Instant,
}

impl App {
    pub fn new(event_loop: &ActiveEventLoop, config: AppConfig) -> Self {
        let mut windows = HashMap::new();
        let gpu: Arc<Gpu>;

        match event_loop.create_window(config.window) {
            Ok(window) => {
                let window_id = window.id();

                let arc_win = Arc::new(window);

                let gpu_instance = pollster::block_on(Gpu::new(arc_win.clone(), config.gpu));
                gpu = Arc::new(gpu_instance);

                gpu.resume(&arc_win);

                windows.insert(window_id, arc_win);
            }
            Err(e) => {
                log_error!("Failed to create window: {}", e);
                return Self {
                    input: InputHandler::new(),
                    gpu: None,
                    bind_group: None,
                    vertex_buffer: None,
                    render_pipeline: None,
                    windows,
                    render_queue: VecDeque::new(),
                    start_time: std::time::Instant::now(),
                };
            }
        }

        Self {
            input: InputHandler::new(),
            gpu: Some(gpu),
            bind_group: None,
            vertex_buffer: None,
            render_pipeline: None,
            windows,
            render_queue: VecDeque::new(),
            start_time: std::time::Instant::now(),
        }
    }
    fn generate_animated_triangle_command(&mut self, gpu: &Gpu) {
        if self.render_pipeline.is_none() {
            let pipeline = gpu.create_render_pipeline();
            self.render_pipeline = Some(Arc::new(pipeline));

            let transform_matrix: [[f32; 4]; 4] = nalgebra::Matrix4::identity().into();
            let transform_buffer =
                gpu.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Transform Buffer"),
                        contents: bytemuck::cast_slice(&transform_matrix),
                        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                    });

            let bind_group_layout =
                gpu.device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some("Uniform Bind Group Layout"),
                        entries: &[wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        }],
                    });

            let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                }],
                label: Some("Uniform Bind Group"),
            });

            self.bind_group = Some(bind_group);
        }

        let elapsed = self.start_time.elapsed().as_secs_f32();
        let amplitude = 0.2;
        let speed = 2.0;

        let mut rng = rand::thread_rng();
        let vertex_data: Vec<f32> = vec![
            0.0,
            0.5 + amplitude * (elapsed * speed).sin(),
            0.0,
            rng.gen::<f32>(),
            rng.gen::<f32>(),
            rng.gen::<f32>(),
            -0.5,
            -0.5 + amplitude * (elapsed * speed).cos(),
            0.0,
            rng.gen::<f32>(),
            rng.gen::<f32>(),
            rng.gen::<f32>(),
            0.5,
            -0.5 + amplitude * (elapsed * speed).sin(),
            0.0,
            rng.gen::<f32>(),
            rng.gen::<f32>(),
            rng.gen::<f32>(),
        ];

        let vertex_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Animated Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex_data),
                usage: BufferUsages::VERTEX,
            });

        self.vertex_buffer = Some(Arc::new(vertex_buffer));

        if let (Some(pipeline), Some(vertex_buffer)) = (&self.render_pipeline, &self.vertex_buffer)
        {
            let render_command =
                RenderCommand::new_triangle(pipeline.clone(), vertex_buffer.clone());
            self.render_queue.push_back(render_command);
        }
    }

    fn execute_render_commands(&mut self, gpu: &Gpu, window: &Window) {
        if let Some(bind_group) = &self.bind_group {
            while let Some(command) = self.render_queue.pop_front() {
                gpu.render_with_command(&command, window, bind_group);
            }
        } else {
            log_error!("Bind group is missing; rendering cannot proceed.");
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
                    let window = app.windows.get(&id).cloned();
                    let gpu = app.gpu.as_ref().cloned();

                    if let (Some(window), Some(gpu)) = (window, gpu) {
                        app.generate_animated_triangle_command(&gpu);
                        app.execute_render_commands(&gpu, &window);
                    }
                }
                WindowEvent::Resized(new_size) => {
                    if let Some(gpu) = app.gpu.as_ref() {
                        gpu.resize(new_size);
                    }
                }
                ev => {
                    log_debug!("Window event: {:?}", ev);
                }
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
                DeviceEvent::MouseWheel { delta } => {}
                DeviceEvent::Button { button, state } => match state {
                    winit::event::ElementState::Pressed => {
                        if let Some(&window_id) = app.windows.keys().next() {
                            if let Some(window) = app.windows.get(&window_id) {
                                window.request_redraw();
                            }
                        }
                    }
                    winit::event::ElementState::Released => {}
                },
                _ => (),
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
                    let window = Arc::new(window);
                    app.windows.insert(window_id, window.clone());

                    if let Some(gpu) = app.gpu.as_ref() {
                        gpu.resume(&window);
                        log_debug!("GPU resuming for ID: {:?}", window_id);
                    }

                    if let Some(accessed_window) = app.windows.get(&window_id) {
                        accessed_window.request_redraw();
                        log_debug!("Requesting redraw for ID: {:?}", window_id);
                    }
                }
                Err(e) => {
                    log_error!("Failed to resume event loop and create a window: {}", e);
                }
            },
        }
    }
}
