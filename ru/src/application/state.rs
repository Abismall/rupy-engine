use std::sync::{Arc, RwLock};

use wgpu::util::DeviceExt;
use wgpu::{BindGroup, Buffer, InstanceDescriptor, ShaderModule, SurfaceConfiguration};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::camera::entity::Camera;
use crate::camera::perspective::CameraPerspective;
use crate::config::screen::ScreenConfig;
use crate::config::settings::Settings;
use crate::log_debug;
use crate::math::mat4_id;
use crate::object::buffer::BufferManager;

use crate::object::object::ObjectDescription;
use crate::object::{Mesh, Uniforms, Vertex};
use crate::pipeline::cache::PipelineCache;
use crate::scene::scene::Scene;
use crate::scene::SceneDescription;
use crate::utilities::debug::DebugMetrics;

pub struct ApplicationState {
    buffer_manager: Arc<RwLock<BufferManager>>,
    pipeline_cache: Arc<RwLock<PipelineCache>>,
    pub uniform_buffer: Option<wgpu::Buffer>,
    pub camera: Camera,
    pub perspective: CameraPerspective,
    pub screen_config: ScreenConfig,
    pub settings: Settings,
    pub debug_metrics: DebugMetrics,
    pub scene: Option<Scene>,                   // Scene is now non-generic
    pub device: Arc<wgpu::Device>,              // GPU device
    pub queue: Arc<wgpu::Queue>,                // Command queue
    pub surface: wgpu::Surface<'static>,        // Surface to render to
    pub swap_chain_format: wgpu::TextureFormat, // Format used for rendering
    pub surface_config: SurfaceConfiguration,   // Surface configuration
}
fn create_triangle_mesh() -> Mesh {
    let vertices = vec![
        Vertex {
            position: [0.0, 0.5, 0.0],
            color: [1.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
    ];

    let indices = vec![0, 1, 2];

    Mesh {
        vertices,
        indices,
        vertex_buffer: None,
        index_buffer: None,
    }
}
fn create_cube_mesh() -> Mesh {
    let vertices = vec![
        // Front face
        Vertex {
            position: [-0.5, -0.5, 0.5],
            color: [1.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            color: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.5],
            color: [0.0, 0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            color: [1.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        },
        // Back face
        Vertex {
            position: [-0.5, -0.5, -0.5],
            color: [1.0, 0.0, 0.0],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            color: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            color: [0.0, 0.0, 1.0],
            normal: [0.0, 0.0, -1.0],
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            color: [1.0, 1.0, 0.0],
            normal: [0.0, 0.0, -1.0],
        },
    ];

    let indices = vec![
        0, 1, 2, 2, 3, 0, // Front face
        4, 5, 6, 6, 7, 4, // Back face
        3, 2, 6, 6, 7, 3, // Top face
        0, 1, 5, 5, 4, 0, // Bottom face
        0, 3, 7, 7, 4, 0, // Left face
        1, 2, 6, 6, 5, 1, // Right face
    ];

    Mesh {
        vertices,
        indices,
        vertex_buffer: None,
        index_buffer: None,
    }
}
fn create_scene_description() -> SceneDescription {
    // Create a triangle and a cube
    let triangle_mesh = Box::new(create_triangle_mesh());
    let cube_mesh = Box::new(create_cube_mesh());

    // Define object descriptions
    let mut object_desc_vec = Vec::new();

    // Add triangle object
    object_desc_vec.push(ObjectDescription {
        position: [0.0, 0.0, 0.0],
        rotation: [0.0, 0.0, 0.0],
        scale: [1.0, 1.0, 1.0],
        mesh: triangle_mesh, // Use Boxed Mesh
        pipeline_cache_key: 1,
    });

    // Add cube object
    object_desc_vec.push(ObjectDescription {
        position: [2.0, 0.0, 0.0], // Offset position for the cube
        rotation: [0.0, 0.0, 0.0],
        scale: [1.0, 1.0, 1.0],
        mesh: cube_mesh, // Use Boxed Mesh
        pipeline_cache_key: 2,
    });

    // Create scene description
    SceneDescription {
        objects: object_desc_vec,
        camera_position: [0.0, 0.0, 5.0],
    }
}

impl ApplicationState {
    pub async fn new(el: &ActiveEventLoop) -> Self {
        // Initialize GPU, Device, and Surface
        let instance = wgpu::Instance::new(InstanceDescriptor::default());
        let window = el.create_window(WindowAttributes::default()).unwrap();
        let window_size = window.inner_size();
        let surface = instance
            .create_surface(window)
            .expect("Failed to get surface from GPU");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

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
            .await
            .expect("Failed to create device");

        // Use the texture format provided by the surface capabilities
        let surface_caps = surface.get_capabilities(&adapter);
        let swap_chain_format = surface_caps.formats[0]; // Use the first available format

        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swap_chain_format, // Use the correct format here
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![], // Use an empty vec if view formats are not necessary
        };

        surface.configure(&device, &surface_config);

        let buffer_manager = Arc::new(RwLock::new(BufferManager::new()));
        let pipeline_cache = Arc::new(RwLock::new(PipelineCache::new()));

        let camera = Camera::new([0.0, 0.0, 5.0]);

        ApplicationState {
            buffer_manager,
            pipeline_cache,
            camera,
            screen_config: ScreenConfig::default(),
            settings: Settings::default(),
            debug_metrics: DebugMetrics::new(),
            scene: None,
            device: Arc::new(device),
            queue: Arc::new(queue),
            surface,
            surface_config,
            swap_chain_format, // Store the correct format for later use
            perspective: CameraPerspective {
                fov: 45.0,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: window_size.width as f32 / window_size.height as f32,
            },
            uniform_buffer: None,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }
    pub fn load_scene(&mut self, scene_description: SceneDescription) -> Scene {
        // Create the scene from the provided description
        Scene::from_description(scene_description)
    }

    pub fn create_and_render_scene(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        swapchain_format: wgpu::TextureFormat,
        output_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        vertex_shader_src: &ShaderModule,
        fragment_shader_src: &ShaderModule,
        perspective: &CameraPerspective,
    ) {
        log_debug!("Creating and rendering scene!");
        // Create or update global uniform buffer
        self.create_global_uniform_buffer(device, perspective);
        let scene_desc = create_scene_description();

        let scene = self.load_scene(scene_desc);

        self.scene = Some(scene);
        // Update the uniform buffer with the latest camera data
        let buffer = match &self.uniform_buffer {
            Some(buffer) => buffer,
            None => {
                &device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Uniform Buffer"),
                    size: 400, // Example size, adjust as needed
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                })
            }
        };
        self.update_global_uniform_buffer(queue, &buffer, perspective);
        // Create global bind group
        let mut pipeline_cache = self.pipeline_cache.write().unwrap();
        let global_bind_group = self.create_global_bind_group(device, &mut pipeline_cache, &buffer);

        // Render the scene
        if let Some(scene) = &mut self.scene {
            log_debug!("Found scene");
            scene.render_scene_objects(
                device,
                &mut pipeline_cache,
                swapchain_format,
                vertex_shader_src,
                fragment_shader_src,
                encoder,
                output_view,
                &global_bind_group,
            );
        } else {
            log_debug!("Scene is None");
        }
    }
    pub fn create_global_uniform_buffer(
        &mut self,
        device: &wgpu::Device,
        perspective: &CameraPerspective,
    ) {
        let global_uniforms = Uniforms {
            model: mat4_id(), // Typically identity, used per object
            view: self.camera.view_matrix(),
            projection: self.camera.projection_matrix(perspective),
        };

        self.uniform_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Global Uniform Buffer"),
                contents: bytemuck::cast_slice(&[global_uniforms]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
        );
    }
    pub fn update_global_uniform_buffer(
        &self,
        queue: &wgpu::Queue,
        uniform_buffer: &wgpu::Buffer,
        perspective: &CameraPerspective,
    ) {
        let global_uniforms = Uniforms {
            model: mat4_id(), // Typically identity, used per object
            view: self.camera.view_matrix(),
            projection: self.camera.projection_matrix(perspective),
        };

        queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[global_uniforms]));
    }

    pub fn create_global_bind_group(
        &self,
        device: &wgpu::Device,
        pipeline_cache: &mut PipelineCache,
        global_uniform_buffer: &wgpu::Buffer,
    ) -> Arc<wgpu::BindGroup> {
        // Create a bind group layout if it doesn't already exist in the cache
        let bind_group_layout = pipeline_cache.get_or_create_bind_group_layout(
            device,
            0, // Unique key for global uniforms bind group layout
            &[wgpu::BindGroupLayoutEntry {
                binding: 0,                             // Ensure this matches the binding in your shader
                visibility: wgpu::ShaderStages::VERTEX, // Used in vertex shader
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        );

        // Correctly create the bind group with an entry for the uniform buffer
        pipeline_cache.get_or_create_bind_group(
            device,
            0,
            Arc::clone(&bind_group_layout),
            &[wgpu::BindGroupEntry {
                binding: 0, // This must match the binding index in the layout
                resource: global_uniform_buffer.as_entire_binding(),
            }],
        )
    }
}
