use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages};

use crate::ecs::components::uniform::{UniformColor, Uniforms};
use crate::ecs::components::vertex::Vertex;

pub const UNIFORM_BUFFER_LABEL: &str = "uniform_buffer";
pub const VERTEX_BUFFER_LABEL: &str = "vertex_buffer";
pub const INDEX_BUFFER_LABEL: &str = "index_buffer";
pub const UNIFORM_COLOR_BUFFER_LABEL: &str = "index_buffer";
pub trait UniformBuffer: bytemuck::Pod + bytemuck::Zeroable {
    fn create_uniform_buffer(device: &wgpu::Device, data: &Uniforms) -> Buffer {
        uniform_buffer(device, data)
    }

    fn create_static_uniform_buffer(device: &wgpu::Device, data: Uniforms) -> Buffer {
        let binding = [data];
        let desc = uniform_descriptor(&binding);
        init_buffer(device, desc)
    }
}

pub trait IndexBuffer: bytemuck::Pod + bytemuck::Zeroable {
    type IndexType: bytemuck::Pod + bytemuck::Zeroable;

    fn create_index_buffer(device: &wgpu::Device, indices: &[Self::IndexType]) -> Buffer {
        let desc = index_descriptor(indices);
        init_buffer(device, desc)
    }

    fn create_static_index_buffer(device: &wgpu::Device, indices: &[Self::IndexType]) -> Buffer {
        let desc = index_descriptor(indices);
        init_buffer(device, desc)
    }
}

pub trait VertexBuffer: bytemuck::Pod + bytemuck::Zeroable {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;

    fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> Buffer {
        let desc = vertex_descriptor(vertices);
        init_buffer(device, desc)
    }

    fn create_static_vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> Buffer {
        let desc = vertex_descriptor(vertices);
        init_buffer(device, desc)
    }
}

pub trait WgpuBufferBinding {
    fn buffer_binding<'a>(buffer: &'a Buffer, offset: u64) -> wgpu::BufferBinding<'a>;
}

pub fn init_buffer(device: &wgpu::Device, desc: wgpu::util::BufferInitDescriptor) -> Buffer {
    device.create_buffer_init(&desc)
}

pub fn vertex_descriptor<'a>(vertices: &'a [Vertex]) -> wgpu::util::BufferInitDescriptor<'a> {
    wgpu::util::BufferInitDescriptor {
        label: Some(VERTEX_BUFFER_LABEL),
        contents: bytemuck::cast_slice(vertices),
        usage: BufferUsages::VERTEX,
    }
}

pub fn index_descriptor<'a, T: bytemuck::Pod>(
    indices: &'a [T],
) -> wgpu::util::BufferInitDescriptor<'a> {
    wgpu::util::BufferInitDescriptor {
        label: Some(INDEX_BUFFER_LABEL),
        contents: bytemuck::cast_slice(indices),
        usage: BufferUsages::INDEX,
    }
}

pub fn uniform_descriptor<'a>(uniforms: &'a [Uniforms]) -> wgpu::util::BufferInitDescriptor<'a> {
    wgpu::util::BufferInitDescriptor {
        label: Some(UNIFORM_BUFFER_LABEL),
        contents: bytemuck::cast_slice(uniforms),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    }
}

pub fn vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> Buffer {
    let desc = vertex_descriptor(vertices);
    init_buffer(device, desc)
}

pub fn index_buffer<T: bytemuck::Pod>(device: &wgpu::Device, indices: &[T]) -> Buffer {
    let desc = index_descriptor(indices);
    init_buffer(device, desc)
}

pub fn uniform_buffer(device: &wgpu::Device, uniforms: &Uniforms) -> Buffer {
    let binding = [*uniforms];
    let desc = uniform_descriptor(&binding);
    init_buffer(device, desc)
}
pub fn color_buffer(device: &wgpu::Device, color: UniformColor) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(UNIFORM_COLOR_BUFFER_LABEL),
        contents: bytemuck::cast_slice(&[color]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}
impl IndexBuffer for u16 {
    type IndexType = u16;
    fn create_index_buffer(device: &wgpu::Device, indices: &[Self]) -> Buffer {
        let desc = index_descriptor(indices);
        init_buffer(device, desc)
    }
}
