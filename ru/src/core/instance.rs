use nalgebra::Vector2;
use wgpu::{
    Adapter, Backends, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, PresentMode, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages,
};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position};
use winit::window::{Fullscreen, WindowAttributes};

use crate::constants::defaults::TITLE;
use winit::window::Window;

#[derive(Clone)]
pub struct GPU {
    pub backends: Backends,
    pub device_features: Features,
    pub device_limits: Limits,
    pub max_samples: u8,
}

impl Default for GPU {
    fn default() -> Self {
        Self {
            backends: Backends::all(),
            device_features: Features::empty(),
            device_limits: Limits::downlevel_webgl2_defaults(),
            max_samples: 1,
        }
    }
}

pub fn wgpu_default_instance(backends: Backends) -> Instance {
    Instance::new(InstanceDescriptor {
        backends,
        ..Default::default()
    })
}

pub async fn instance_request_adapter(
    instance: &Instance,
    compatible_surface: Option<&wgpu::Surface<'static>>,
    power_preference: wgpu::PowerPreference,
    force_fallback_adapter: bool,
) -> Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference,
            compatible_surface,
            force_fallback_adapter,
        })
        .await
        .expect("Failed to find a suitable GPU adapter!")
}

pub async fn adapter_request_device(
    adapter: &Adapter,
    gpu: &GPU,
    memory_hints: wgpu::MemoryHints,
) -> (Device, wgpu::Queue) {
    adapter
        .request_device(
            &DeviceDescriptor {
                label: Some("Active Device"),
                required_features: gpu.device_features,
                required_limits: gpu.device_limits.clone().using_resolution(adapter.limits()),
                memory_hints,
            },
            None,
        )
        .await
        .expect("Failed to create a device!")
}

pub fn window_inner_size_to_vector2(window: &Window) -> Vector2<u32> {
    let window_size = window.inner_size();
    Vector2::new(window_size.width.max(1), window_size.height.max(1))
}

pub fn surface_configuration(
    format: TextureFormat,
    width: u32,
    height: u32,
    present_mode: PresentMode,
    alpha_mode: CompositeAlphaMode,
    usage: TextureUsages,
    view_formats: Vec<TextureFormat>,
    desired_maximum_frame_latency: u32,
) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage,
        format,
        width,
        height,
        present_mode,
        alpha_mode,
        view_formats,
        desired_maximum_frame_latency,
    }
}

pub fn default_surface_configuration(
    surface: &Surface,
    adapter: &Adapter,
    window: &Window,
) -> SurfaceConfiguration {
    let surface_size = window_inner_size_to_vector2(window);
    let surface_caps = surface.get_capabilities(adapter);

    surface_configuration(
        surface_caps.formats[0],
        surface_size.x,
        surface_size.y,
        PresentMode::Mailbox,
        surface_caps.alpha_modes[0],
        TextureUsages::RENDER_ATTACHMENT,
        surface_caps.formats.to_vec(),
        1,
    )
}

pub fn surface_capabilities(surface: &Surface, adapter: &Adapter) -> TextureFormat {
    let surface_caps = surface.get_capabilities(adapter);
    surface_caps.formats[0]
}

pub fn configure_surface(window: &Window, surface: &Surface, adapter: &Adapter, device: &Device) {
    surface.configure(
        device,
        &default_surface_configuration(surface, adapter, window),
    );
}

pub fn window_position_logical(
    window_size: LogicalSize<f64>,
    screen_size: LogicalSize<f64>,
) -> Position {
    let x = (screen_size.width - window_size.width) / 2.0;
    let y = (screen_size.height - window_size.height) / 2.0;

    Position::Logical(LogicalPosition::new(x, y))
}

pub fn window_position_physical(
    window_size: PhysicalSize<u32>,
    screen_size: PhysicalSize<u32>,
) -> Position {
    let x = (screen_size.width.saturating_sub(window_size.width)) / 2;
    let y = (screen_size.height.saturating_sub(window_size.height)) / 2;

    Position::Physical(PhysicalPosition::new(x as i32, y as i32))
}

pub fn default_window_attributes(
    fullscreen: Option<Fullscreen>,
    title: Option<&str>,
) -> WindowAttributes {
    WindowAttributes::default()
        .with_fullscreen(fullscreen)
        .with_title(title.unwrap_or(TITLE))
}

pub fn u32_to_physical_size(width: u32, height: u32) -> PhysicalSize<u32> {
    PhysicalSize::new(width, height)
}

pub fn f64_to_logical_size(width: f64, height: f64) -> LogicalSize<f64> {
    LogicalSize::new(width, height)
}