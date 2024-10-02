use wgpu::{BindGroup, Buffer, ShaderModule};

use crate::pipeline::cache::PipelineCache;

pub mod command;

pub trait Renderable {
    fn create_buffers(&mut self, device: &wgpu::Device);
    fn vertex_buffer(&self) -> &wgpu::Buffer;
    fn index_buffer(&self) -> &wgpu::Buffer;
    fn num_indices(&self) -> u32;
    fn is_textured(&self) -> bool;
    fn update(&mut self);
    fn render(
        &mut self,
        device: &wgpu::Device,
        pipeline_cache: &mut PipelineCache,
        swapchain_format: wgpu::TextureFormat,
        vertex_shader_src: &ShaderModule,
        fragment_shader_src: &ShaderModule,
        encoder: &mut wgpu::CommandEncoder,
        output_view: &wgpu::TextureView,
        global_bind_group: &wgpu::BindGroup,
    );
}
