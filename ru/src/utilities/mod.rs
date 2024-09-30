use wgpu::{Adapter, Device, DeviceDescriptor, Instance};
use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position},
    window::{Fullscreen, Window, WindowAttributes},
};

use crate::{config::gpu::GpuConfig, math::Vec2};

pub mod debug;
pub mod logger;

pub const TITLE: &str = "RuPy";

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
    gpu: &GpuConfig,
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

pub fn window_inner_size_to_vector2(window: &Window) -> Vec2 {
    let window_size = window.inner_size();
    [
        window_size.width.max(1) as f32,
        window_size.height.max(1) as f32,
    ]
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
