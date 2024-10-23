use wgpu::util::DeviceExt;

pub trait UniformBuffer: bytemuck::Pod + bytemuck::Zeroable {
    fn create_uniform_buffer<T: bytemuck::Pod>(
        device: &wgpu::Device,
        data: &T,
        usage: wgpu::BufferUsages,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(data),
            usage: wgpu::BufferUsages::UNIFORM | usage,
        })
    }

    fn create_static_uniform_buffer<T: bytemuck::Pod>(
        device: &wgpu::Device,
        data: &T,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Static Uniform Buffer"),
            contents: bytemuck::bytes_of(data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }
}
pub trait IndexBuffer: bytemuck::Pod + bytemuck::Zeroable {
    type IndexType: bytemuck::Pod + bytemuck::Zeroable;

    fn create_index_buffer(
        device: &wgpu::Device,
        indices: &[Self::IndexType],
        usage: wgpu::BufferUsages,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage,
        })
    }

    fn create_static_index_buffer(
        device: &wgpu::Device,
        indices: &[Self::IndexType],
    ) -> wgpu::Buffer {
        Self::create_index_buffer(device, indices, wgpu::BufferUsages::INDEX)
    }
}
pub trait VertexBuffer: bytemuck::Pod + bytemuck::Zeroable {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;

    fn create_vertex_buffer(
        device: &wgpu::Device,
        vertices: &[Self],
        usage: wgpu::BufferUsages,
    ) -> wgpu::Buffer
    where
        Self: Sized,
    {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage,
        })
    }

    fn create_static_vertex_buffer(device: &wgpu::Device, vertices: &[Self]) -> wgpu::Buffer
    where
        Self: Sized,
    {
        Self::create_vertex_buffer(device, vertices, wgpu::BufferUsages::VERTEX)
    }
}
