use wgpu::{SurfaceCapabilities, TextureFormat};
pub mod handler;
pub mod initialize;
pub mod resources;
pub mod rupy;
pub mod state;
pub mod worker;

pub fn coalesce_format<'a>(capabilities: SurfaceCapabilities) -> TextureFormat {
    *capabilities
        .formats
        .iter()
        .find(|&&t| t.eq(&TextureFormat::Bgra8UnormSrgb) | t.eq(&TextureFormat::Rgba8Unorm))
        .unwrap_or(&TextureFormat::Rgba8Uint)
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DebugMode {
    None,
    Frustum,
    Wireframe,
}

impl DebugMode {
    pub fn toggle(&mut self) {
        *self = match self {
            DebugMode::None => DebugMode::Frustum,
            DebugMode::Frustum => DebugMode::Wireframe,
            DebugMode::Wireframe => DebugMode::None,
        };
    }
}
