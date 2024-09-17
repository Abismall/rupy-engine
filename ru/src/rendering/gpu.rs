use std::collections::VecDeque;

use crate::log_error;
use nalgebra::Vector2;
use wgpu::{
    self, Adapter, Backends, CommandBuffer, CommandEncoderDescriptor, Device, DeviceDescriptor,
    Extent3d, Features, Instance, InstanceDescriptor, Limits, LoadOp, Operations, PresentMode,
    Queue, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipeline, StoreOp, Surface, SurfaceConfiguration, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureViewDescriptor,
};
use winit::window::Window;

use super::command::RenderCommand;

/// Configuration for initializing a GPU
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

pub struct GPU;

impl GPU {
    /// Create an instance for GPU
    pub fn create_instance(backends: Backends) -> Instance {
        Instance::new(InstanceDescriptor {
            backends,
            ..Default::default()
        })
    }

    /// Create a rendering surface for the given window
    pub fn create_surface<'a>(instance: &'a Instance, window: &'a Window) -> Surface<'a> {
        instance.create_surface(window).unwrap()
    }

    /// Request a suitable GPU adapter
    pub async fn request_adapter<'a>(instance: &Instance, surface: &Surface<'a>) -> Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find a suitable GPU adapter!")
    }

    /// Request a device and queue from the adapter with the given configuration
    pub async fn request_device(
        adapter: &Adapter,
        gpu_config: &GpuConfig,
    ) -> (Device, wgpu::Queue) {
        adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Active Device"),
                    required_features: gpu_config.device_features,
                    required_limits: gpu_config
                        .device_limits
                        .clone()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to create a device!")
    }

    /// Get the texture format of the surface
    pub fn get_surface_format(surface: &Surface, adapter: &Adapter) -> TextureFormat {
        surface.get_capabilities(adapter).formats[0]
    }

    pub fn determine_samples(max_samples: u8, format: TextureFormat, adapter: &Adapter) -> u32 {
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

    pub fn default_surface_config(
        surface: &Surface,
        adapter: &Adapter,
        window: &Window,
    ) -> SurfaceConfiguration {
        let surface_size = GPU::compute_surface_size(window);
        let surface_caps = surface.get_capabilities(adapter);

        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: surface_size.x.max(1),
            height: surface_size.y.max(1),
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        }
    }

    /// Compute the surface size from the window dimensions
    pub fn compute_surface_size(window: &Window) -> Vector2<u32> {
        let window_size = window.inner_size();
        Vector2::new(window_size.width.max(1), window_size.height.max(1))
    }

    /// Resize the surface configuration based on the current window size
    pub fn resize_surface(surface: &Surface, device: &Device, window: &Window, adapter: &Adapter) {
        // Get the current size of the window
        let new_size = window.inner_size();

        // Create a new configuration with the updated size
        let mut config = GPU::default_surface_config(surface, adapter, window);
        config.width = new_size.width.max(1); // Ensure width is at least 1
        config.height = new_size.height.max(1); // Ensure height is at least 1

        // Reconfigure the surface with the updated configuration
        surface.configure(device, &config);
    }

    /// Submit the command buffers to the queue
    pub fn submit(queue: &wgpu::Queue, command_buffers: Vec<CommandBuffer>) {
        queue.submit(command_buffers);
    }
}

pub fn render_frame(
    device: &Device,
    pipeline: &RenderPipeline,
    queue: &Queue,
    surface: &Surface,
    surface_config: &SurfaceConfiguration,
    command_queue: &mut VecDeque<RenderCommand>,
    window: &Window,
) {
    // Acquire the next frame from the surface.
    let frame = match surface.get_current_texture() {
        Ok(frame) => frame,
        Err(e) => {
            log_error!("Failed to acquire next swap chain texture: {:?}", e);
            return;
        }
    };

    // Create a texture view for the current frame.
    let view = frame.texture.create_view(&TextureViewDescriptor::default());

    // Create a depth texture with the same device used for other operations.
    let depth_texture = device.create_texture(&TextureDescriptor {
        label: Some("Depth Texture"),
        size: Extent3d {
            width: surface_config.width,
            height: surface_config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Depth32Float,
        usage: TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    // Create a view for the depth texture.
    let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

    // Create a command encoder with the same device.
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    // Begin a render pass using the correct device context.
    {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(wgpu::Color::BLACK), // Clear to black
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0), // Clear depth to far (maximum value)
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Execute all commands from the queue using the render pass.
        for command in command_queue.iter() {
            command.execute(&mut render_pass, pipeline);
        }
    } // The render pass ends here when the scope ends.

    // Submit the command encoder commands to the queue.
    queue.submit(std::iter::once(encoder.finish()));

    // Present the frame to the screen.
    frame.present();

    // Request another redraw to continue rendering.
    window.request_redraw();
}
