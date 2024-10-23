#[derive(Debug)]
pub struct RenderResources {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub texture_bind_group: Option<wgpu::BindGroup>,
}
