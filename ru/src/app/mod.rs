use wgpu::{SurfaceCapabilities, TextureFormat};

pub mod context;
pub mod handler;
pub mod renderer;
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
    Minimal,
    Verbose,
}

impl DebugMode {
    pub fn next(self) -> DebugMode {
        match self {
            DebugMode::None => DebugMode::Minimal,
            DebugMode::Minimal => DebugMode::Verbose,
            DebugMode::Verbose => DebugMode::None,
        }
    }
}
