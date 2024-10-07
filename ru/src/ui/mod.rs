pub mod container;
pub mod input;
pub mod menu;
use std::sync::Arc;
use winit::{dpi::PhysicalPosition, window::Window};

pub struct Ui<'window> {
    pub surface: wgpu::Surface<'window>,
    pub window: Arc<Window>,
}

impl<'window> Ui<'window> {
    pub async fn new(window: Arc<Window>) -> Ui<'window> {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let limits = wgpu::Limits::default();

        let mouse_coords = PhysicalPosition { x: 0.0, y: 0.0 };
        // let input_state = MenuMouseInputState {
        //     clicked: false,
        //     mouse_coords,
        // };
        let window_clone = window.clone();
        let surface = instance
            .create_surface(window_clone)
            .expect("can create surface");
        // let adapter = instance
        //     .request_adapter(&wgpu::RequestAdapterOptionsBase {
        //         power_preference: wgpu::PowerPreference::default(),
        //         force_fallback_adapter: false,
        //         compatible_surface: Some(&surface),
        //     })
        //     .await
        //     .expect("can create adapter");

        // let (device, queue) = adapter
        //     .request_device(
        //         &wgpu::DeviceDescriptor {
        //             label: None,
        //             required_features: Features::default(),
        //             required_limits: limits,
        //             memory_hints: wgpu::MemoryHints::Performance,
        //         },
        //         None,
        //     )
        //     .await
        //     .expect("can create a new device");

        // let config = surface
        //     .get_default_config(&adapter, size.width, size.height)
        //     .unwrap();

        // surface.configure(&device, &config);

        // let mut font_system =
        //     FontSystem::new_with_locale_and_db("en-US".into(), glyphon::fontdb::Database::new());
        // let font = include_bytes!("../../static/fonts/font.ttf");
        // font_system.db_mut().load_font_data(font.to_vec());

        // let text_cache = SwashCache::new();
        // let mut text_atlas = TextAtlas::new(&device, &queue, &Cache::new(&device), config.format);
        // let text_renderer = TextRenderer::new(
        //     &mut text_atlas,
        //     &device,
        //     wgpu::MultisampleState::default(),
        //     None,
        // );

        // let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        //     label: None,
        //     source: wgpu::ShaderSource::Wgsl(include_str!("../../static/shaders/gui.wgsl").into()),
        // });

        // let render_pipeline_layout =
        //     device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //         label: None,
        //         bind_group_layouts: &[],
        //         push_constant_ranges: &[],
        //     });

        // let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        //     label: None,
        //     layout: Some(&render_pipeline_layout),
        //     vertex: wgpu::VertexState {
        //         module: &shader,
        //         entry_point: "vertex",
        //         buffers: &[GuiVertex::desc()],
        //         compilation_options: Default::default(),
        //     },
        //     fragment: Some(wgpu::FragmentState {
        //         module: &shader,
        //         entry_point: "fragment",
        //         targets: &[Some(wgpu::ColorTargetState {
        //             format: config.format,
        //             blend: Some(wgpu::BlendState::REPLACE),
        //             write_mask: wgpu::ColorWrites::ALL,
        //         })],
        //         compilation_options: Default::default(),
        //     }),
        //     primitive: wgpu::PrimitiveState {
        //         topology: wgpu::PrimitiveTopology::TriangleList,
        //         strip_index_format: None,
        //         front_face: wgpu::FrontFace::Ccw,
        //         cull_mode: Some(wgpu::Face::Back),
        //         unclipped_depth: false,
        //         polygon_mode: wgpu::PolygonMode::Fill,
        //         conservative: false,
        //     },
        //     multisample: wgpu::MultisampleState::default(),
        //     depth_stencil: None,
        //     multiview: None,
        //     cache: None,
        // });

        Self {
            window: window.clone(),
            surface,
        }
    }
}
