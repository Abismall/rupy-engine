use glyphon::{
    Attrs, Buffer, Cache, Color, Family, FontSystem, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer, Viewport,
};
use pollster::FutureExt;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wgpu::{
    rwh::HasDisplayHandle, CommandEncoderDescriptor, CompositeAlphaMode, Instance,
    InstanceDescriptor, LoadOp, MultisampleState, Operations, PresentMode,
    RenderPassColorAttachment, RenderPassDescriptor, SurfaceConfiguration, TextureFormat,
    TextureUsages, TextureViewDescriptor,
};
use winit::{
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

use crate::{
    constants::defaults::TITLE,
    core::instance::{
        adapter_request_device, default_window_attributes, instance_request_adapter, GPU,
    },
};

use super::{
    input::InputHandler,
    logger::LogFactory,
    menu::{Menu, MenuWrapper},
    views::{main_menu, MainMenu},
};

pub(crate) struct ApplicationState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: SurfaceConfiguration,
    window_attributes: winit::window::WindowAttributes,
    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    text_buffer: glyphon::Buffer,
    gpu: GPU,
    window: Arc<Window>,
}

impl ApplicationState {
    async fn new(
        window: Arc<Window>,
        gpu: GPU,
        title: Option<&str>,
        font_size: Option<f32>,
        line_height: Option<f32>,
    ) -> Self {
        let physical_size = window.inner_size();
        let window_attributes = default_window_attributes(None, title);

        let instance = Instance::new(InstanceDescriptor::default());
        let adapter = instance_request_adapter(
            &instance,
            None,
            wgpu::PowerPreference::HighPerformance,
            false,
        )
        .block_on();
        let (mut device, queue) =
            adapter_request_device(&adapter, &gpu, wgpu::MemoryHints::Performance).block_on();

        let surface = instance
            .create_surface(window.clone())
            .expect("Create surface");
        let swapchain_format = TextureFormat::Bgra8UnormSrgb;
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let mut font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        let mut atlas = TextAtlas::new(&device, &queue, &cache, swapchain_format);
        let text_renderer =
            TextRenderer::new(&mut atlas, &mut device, MultisampleState::default(), None);

        let f_size = match font_size {
            Some(size) => size,
            None => 32.0 as f32,
        };
        let l_height = match line_height {
            Some(height) => height,
            None => 42.0 as f32,
        };
        let text_buffer = Buffer::new(&mut font_system, glyphon::Metrics::new(f_size, l_height));

        Self {
            device,
            queue,
            surface,
            surface_config,
            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,
            window,
            window_attributes,

            gpu,
        }
    }

    pub fn set_text(&mut self, text: &str, font_size: glyphon::Metrics, font_family: Family) {
        self.text_buffer
            .set_metrics(&mut self.font_system, font_size);

        self.text_buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(font_family),
            Shaping::Advanced,
        );

        self.text_buffer
            .shape_until_scroll(&mut self.font_system, false);
    }

    pub fn render(&mut self) {
        self.viewport.update(
            &self.queue,
            Resolution {
                width: self.surface_config.width,
                height: self.surface_config.height,
            },
        );

        self.text_renderer
            .prepare(
                &self.device,
                &self.queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                [TextArea {
                    buffer: &self.text_buffer,
                    left: 10.0,
                    top: 10.0,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: self.surface_config.width as i32,
                        bottom: self.surface_config.height as i32,
                    },
                    default_color: Color::rgb(255, 255, 255),
                    custom_glyphs: &[],
                }],
                &mut self.swash_cache,
            )
            .expect("Failed to prepare text rendering");

        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to get current texture");
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.text_renderer
                .render(&self.atlas, &self.viewport, &mut pass)
                .expect("Failed to render text");
        }

        self.queue.submit(Some(encoder.finish()));

        frame.present();

        self.atlas.trim();
    }
}
impl winit::application::ApplicationHandler for Rupy {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let window_attributes: winit::window::WindowAttributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let gpu = GPU::default();
        self.state = Some(
            Some(pollster::block_on(ApplicationState::new(
                window,
                gpu,
                Some(Rupy::TITLE),
                None,
                None,
            )))
            .expect("Application State"),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(ref mut state) = self.state {
            if state.window.display_handle().is_ok() {
                match event {
                    WindowEvent::Resized(size) => {
                        state.surface_config.width = size.width;
                        state.surface_config.height = size.height;
                        state
                            .surface
                            .configure(&state.device, &state.surface_config);

                        state.window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        state.set_text(
                            "Hello world! 👋🦁",
                            glyphon::Metrics {
                                font_size: 30.0,
                                line_height: 42.0,
                            },
                            Family::SansSerif,
                        );
                        let _ = state.render();
                    }
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }
                    _ => {}
                }
            } else {
                self.initialize_state(Arc::new(
                    event_loop
                        .create_window(WindowAttributes::default())
                        .expect("Create Window"),
                ));
            }
        }
    }
}

pub struct Rupy {
    state: Option<ApplicationState>,
    #[cfg(feature = "logging")]
    pub logger: Option<LogFactory>,
    input: InputHandler,
    menu: Rc<RefCell<Menu<MainMenu, &'static str>>>,
}

impl Rupy {
    pub const TITLE: &str = TITLE;

    pub fn new() -> Self {
        let mut input = InputHandler::new();
        let menu = Rc::new(RefCell::new(main_menu()));
        menu.borrow_mut().activate();

        input.add_listener(Box::new(MenuWrapper::new(menu.clone())));

        Rupy {
            #[cfg(feature = "logging")]
            logger: Some(Default::default()),
            input,
            state: None,
            menu,
        }
    }

    pub fn initialize_state(&mut self, window: Arc<Window>) {
        let state = pollster::block_on(ApplicationState::new(
            window,
            GPU::default(),
            Some(Rupy::TITLE),
            None,
            None,
        ));
        self.state = Some(state);
    }
    pub fn window(self, window: Arc<Window>) {
        self.state.unwrap().window = window;
    }

    pub fn gpu(self, gpu: GPU) {
        self.state.unwrap().gpu = gpu;
    }

    #[cfg(feature = "logging")]
    pub fn logger(mut self, logger: Option<crate::core::logger::LogFactory>) -> Self {
        self.logger = logger;
        self
    }
}
