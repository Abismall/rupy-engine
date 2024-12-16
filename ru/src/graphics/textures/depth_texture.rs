use wgpu::{DepthStencilState, Device, Extent3d, TextureFormat};
use winit::dpi::PhysicalSize;

use super::Texture;

pub struct DepthTexture {
    pub current: Texture,
    pub size: Extent3d,
    pub stencil_state: wgpu::DepthStencilState,
}

impl DepthTexture {
    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
    pub const LABEL: &str = "depth_buffer";
    pub fn new(current: Texture, stencil_state: DepthStencilState) -> Self {
        let size = current.size;
        Self {
            current,
            size,
            stencil_state,
        }
    }
    pub fn resize<P: winit::dpi::Pixel>(&mut self, device: &Device, size: PhysicalSize<P>) {
        let resized_texture = Texture::create_depth_texture(&device, size, Self::LABEL);
        self.current = resized_texture;
    }
}
