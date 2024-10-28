pub trait Renderable {
    fn vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout<'static>;
    fn index_format(&self) -> wgpu::IndexFormat;
    fn vertex_count(&self) -> u32;
    fn index_count(&self) -> u32;
    fn vertex_buffer(&self) -> &wgpu::Buffer;
    fn index_buffer(&self) -> &wgpu::Buffer;
}
