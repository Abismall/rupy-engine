use crate::log_error;
use nalgebra::Vector2;
use std::sync::{Arc, Mutex};
use wgpu::{
    self, Adapter, Backends, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits,
    MultisampleState, PresentMode, SurfaceConfiguration, TextureFormat,
};
use winit::{dpi::PhysicalSize, window::Window};

use super::render_command::RenderCommand;

#[derive(Clone)]
pub struct GpuConfig {
    pub backends: Backends,
    pub device_features: Features,
    pub device_limits: Limits,
    pub max_samples: u8,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            backends: Backends::all(),
            device_features: Features::empty(),
            device_limits: Limits::downlevel_webgl2_defaults(),
            max_samples: 1,
        }
    }
}

pub(crate) struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::Surface<'static>,
    surface_size: Mutex<Vector2<u32>>,
    config: Mutex<SurfaceConfiguration>,
    format: TextureFormat,
    samples: u32,
    sample_state: MultisampleState,
    target_msaa: Mutex<Option<wgpu::Texture>>,
}

impl Gpu {
    pub async fn new(window: Arc<Window>, gpu_config: GpuConfig) -> Self {
        let instance = Instance::new(InstanceDescriptor {
            backends: gpu_config.backends,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find a suitable GPU adapter!");

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: gpu_config.device_features,
                    required_limits: gpu_config.device_limits.using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to create a device!");

        let surface_caps = surface.get_capabilities(&adapter);
        let format = surface_caps.formats[0];

        let samples = Gpu::determine_samples(gpu_config.max_samples, format, &adapter);

        let config = Gpu::default_config(&surface, &adapter, &window);
        let sample_state = MultisampleState {
            count: samples,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        surface.configure(&device, &config);

        Self {
            device,
            queue,
            adapter,
            surface,
            surface_size: Default::default(),
            config: Mutex::new(config),
            format,
            samples,
            sample_state,
            target_msaa: Default::default(),
        }
    }

    pub fn create_render_pipeline(&self) -> wgpu::RenderPipeline {
        // Define the vertex buffer layout, including position and color
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: (3 + 3) * std::mem::size_of::<f32>() as wgpu::BufferAddress, // 3 position + 3 color
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3, // Position attribute
                },
                wgpu::VertexAttribute {
                    offset: 3 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3, // Color attribute
                },
            ],
        };

        // Load the shaders
        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/shader.wgsl").into()),
            });

        // Create the pipeline layout
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        // Create the render pipeline
        self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[vertex_layout],
                    compilation_options: Default::default(), // Attach the vertex buffer layout here
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: self.sample_state,
                multiview: None,
                cache: None,
            })
    }
    pub fn render_with_command(
        &self,
        command: &RenderCommand,
        window: &Window,
        bind_group: &wgpu::BindGroup,
    ) {
        let frame = match self.start_frame() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Outdated) => {
                self.reconfigure(window);
                match self.start_frame() {
                    Ok(texture) => texture,
                    Err(e) => {
                        log_error!(
                            "Failed to acquire frame texture after reconfiguration: {:?}",
                            e
                        );
                        return;
                    }
                }
            }
            Err(e) => {
                log_error!("Failed to acquire frame texture: {:?}", e);
                return;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let msaa_view = if self.samples > 1 {
            let msaa_texture = self.create_msaa_texture();
            Some(msaa_texture)
        } else {
            None
        };

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("RenderPass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: msaa_view.as_ref().unwrap_or(&view),
                    resolve_target: if self.samples > 1 { Some(&view) } else { None },
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_bind_group(0, bind_group, &[]);

            command.execute(&mut render_pass);
        }

        self.submit(vec![encoder.finish()]);
        frame.present();
        window.request_redraw();
    }

    fn determine_samples(max_samples: u8, format: TextureFormat, adapter: &Adapter) -> u32 {
        let sample_flags = adapter.get_texture_format_features(format).flags;
        if max_samples >= 16
            && sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X16)
        {
            16
        } else if max_samples >= 8
            && sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X8)
        {
            8
        } else if max_samples >= 4
            && sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X4)
        {
            4
        } else if max_samples >= 2
            && sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X2)
        {
            2
        } else {
            1
        }
    }
    pub fn reconfigure(&self, window: &Window) {
        let current_size = Self::compute_surface_size(window);
        let config = Self::default_config(&self.surface, &self.adapter, window);

        self.surface.configure(&self.device, &config);
        self.update_surface_size(current_size);
        self.resize(PhysicalSize::new(config.width, config.height));
        self.update_config(config);
    }

    pub fn compute_surface_size(window: &Window) -> Vector2<u32> {
        let window_size = window.inner_size();
        Vector2::new(window_size.width.max(1), window_size.height.max(1))
    }

    pub fn default_config(
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
        window: &Window,
    ) -> wgpu::SurfaceConfiguration {
        let surface_size = Self::compute_surface_size(window);

        let surface_caps = surface.get_capabilities(adapter);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: surface_size.x.max(1),
            height: surface_size.y.max(1),
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 0,
        }
    }
    pub fn update_surface_size(&self, new_size: Vector2<u32>) {
        *self
            .surface_size
            .lock()
            .expect("Update Surface Size: failed to acquire mutex lock") = new_size;
    }
    pub fn surface_size(&self) -> Vector2<u32> {
        self.surface_size
            .lock()
            .expect("Surface Size: failed to acquire mutex lock")
            .clone()
    }
    pub fn update_config(&self, config: SurfaceConfiguration) {
        *self
            .config
            .lock()
            .expect("Update Config: failed to acquire mutex lock") = config;
    }
    pub fn samples(&self) -> u32 {
        self.samples
    }
    fn create_msaa_texture(&self) -> wgpu::TextureView {
        let size = self.surface_size();
        #[cfg(feature = "logging")]
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("MSAAtexture"),
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: self.samples,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn update_msaa(&self, size: Vector2<u32>) {
        let mut target_msaa = self
            .target_msaa
            .lock()
            .expect("Update MSAA: failed to acquire mutex lock");

        if self.samples() > 1 && (size != self.surface_size() || target_msaa.is_none()) {
            let msaa_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("MSAA Texture"),
                size: wgpu::Extent3d {
                    width: size.x,
                    height: size.y,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: self.samples(),
                dimension: wgpu::TextureDimension::D2,
                format: self.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });

            *target_msaa = Some(msaa_texture);
        }
    }

    pub fn resume(&self, window: &Window) {
        let config = Self::default_config(&self.surface, &self.adapter, &window);
        self.update_msaa(Vector2::new(config.width, config.height));
        self.reconfigure(window);
    }

    pub fn resize(&self, new_size: winit::dpi::PhysicalSize<u32>) {
        let mut config = self.config.lock().unwrap();
        config.width = new_size.width.max(1);
        config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &config);
    }

    pub fn start_frame(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    pub fn submit(&self, command_buffers: Vec<wgpu::CommandBuffer>) {
        self.queue.submit(command_buffers);
    }
}
