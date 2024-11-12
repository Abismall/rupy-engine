use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages};

use crate::ecs::components::model::{Uniforms, Vertex2D, Vertex3D};

pub trait VertexBuffer: bytemuck::Pod + bytemuck::Zeroable + Sized {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
    fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Self]) -> Buffer
    where
        Self: Sized;
}
impl VertexBuffer for Vertex3D {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        Vertex3D::buffer_layout()
    }

    fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Self]) -> Buffer
    where
        Self: Sized,
    {
        let desc = vertex_descriptor(vertices);
        device.create_buffer_init(&desc)
    }
}

impl VertexBuffer for Vertex2D {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        Vertex2D::buffer_layout()
    }

    fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Self]) -> Buffer
    where
        Self: Sized,
    {
        let desc = vertex_descriptor(vertices);
        device.create_buffer_init(&desc)
    }
}

pub fn vertex_descriptor<'a, V: bytemuck::Pod>(
    vertices: &'a [V],
) -> wgpu::util::BufferInitDescriptor<'a> {
    wgpu::util::BufferInitDescriptor {
        label: Some("VertexBufferInitDescriptor"),
        contents: bytemuck::cast_slice(vertices),
        usage: BufferUsages::VERTEX,
    }
}

pub fn create_index_buffer<T: bytemuck::Pod>(device: &wgpu::Device, indices: &[T]) -> Buffer {
    let desc = wgpu::util::BufferInitDescriptor {
        label: Some("IndexBufferInitDescriptor"),
        contents: bytemuck::cast_slice(indices),
        usage: BufferUsages::INDEX,
    };
    device.create_buffer_init(&desc)
}

pub fn create_uniform_buffer(device: &wgpu::Device, uniforms: &Uniforms) -> Buffer {
    let binding = [*uniforms];
    let desc = wgpu::util::BufferInitDescriptor {
        label: Some("UniformBufferInitDescriptor"),
        contents: bytemuck::cast_slice(&binding),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    };
    device.create_buffer_init(&desc)
}

pub fn create_vertex_buffer<T: VertexBuffer>(device: &wgpu::Device, vertices: &[T]) -> Buffer {
    T::create_vertex_buffer(device, vertices)
}
