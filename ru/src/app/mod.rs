use wgpu::{SurfaceCapabilities, TextureFormat};

use crate::{camera::Camera, prelude::frame::FrameTime};
pub mod context;
pub mod handler;
pub mod rupy;
pub mod state;
pub fn coalesce_format<'a>(capabilities: &SurfaceCapabilities) -> TextureFormat {
    *capabilities
        .formats
        .iter()
        .find(|&&t| t.eq(&TextureFormat::Rgba8UnormSrgb) | t.eq(&TextureFormat::Rgba8Unorm))
        .unwrap_or(&TextureFormat::Rgba8Uint)
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DebugMode {
    None,
    FPS,
    Frame,
    Camera,
    Verbose,
}

impl DebugMode {
    pub fn next(self) -> DebugMode {
        match self {
            DebugMode::None => DebugMode::FPS,
            DebugMode::FPS => DebugMode::Frame,
            DebugMode::Frame => DebugMode::Camera,
            DebugMode::Camera => DebugMode::Verbose,
            DebugMode::Verbose => DebugMode::None,
        }
    }

    pub fn frame(frame_time: &FrameTime) -> String {
        let now = std::time::Instant::now();
        let frame_duration = now.duration_since(frame_time.last_frame_time);
        let frame_time_ms = frame_duration.as_secs_f32() * 1000.0;

        format!(
            "
            Delta Time: {}\n
            Frame Time: {:.6} ms\n
            ",
            frame_time.delta_time, frame_time_ms,
        )
    }

    pub fn fps(fps: f32) -> String {
        format!(
            "
        FPS: {:.2}\n
            ",
            fps
        )
    }

    pub fn camera(camera: &Camera) -> String {
        format!(
            "
            Position: {}\n
            Target: {}\n
            Panning: {}\n
            Aspect Ratio: {}\n
            ",
            camera.position, camera.target, camera.is_panning, camera.aspect_ratio,
        )
    }
}
