use wgpu::{CompareFunction, Device, TextureFormat, TextureView, TextureViewDescriptor};
use winit::dpi::PhysicalSize;

use super::textures::Texture;

pub struct DepthBuffer {
    pub depth_buffer: crate::graphics::textures::Texture,
    pub view: TextureView,
    pub stencil_state: wgpu::DepthStencilState,
}

impl DepthBuffer {
    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
    pub const LABEL: &str = "depth_buffer_texture";

    pub fn new(
        device: &Device,
        size: PhysicalSize<u32>,
        depth_compare: CompareFunction,
        depth_write_enabled: bool,
        label: &str,
    ) -> Self {
        let depth_buffer =
            crate::graphics::textures::Texture::create_depth_texture(&device, size, label);

        let view = Self::create_depth_buffer_texture_view(&depth_buffer);

        let stencil_state = wgpu::DepthStencilState {
            format: Self::DEPTH_FORMAT,
            depth_write_enabled,
            depth_compare,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };

        Self {
            depth_buffer,
            view,
            stencil_state,
        }
    }
    fn view_desc<'a>() -> TextureViewDescriptor<'a> {
        wgpu::TextureViewDescriptor::default()
    }
    fn create_depth_buffer_texture_view(depth_buffer: &Texture) -> TextureView {
        depth_buffer.texture.create_view(&Self::view_desc())
    }
    pub fn resize(&mut self, device: &Device, size: PhysicalSize<u32>) {
        self.depth_buffer =
            crate::graphics::textures::Texture::create_depth_texture(&device, size, Self::LABEL);
        self.view = Self::create_depth_buffer_texture_view(&self.depth_buffer)
    }
}
