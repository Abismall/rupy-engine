use crate::scene::components::{uniform::CameraUniforms, vertex::Vertex};
use wgpu::util::DeviceExt;

pub const LABEL_CAMERA_UNIFORM_BUFFER: &str = "Camera Uniform Buffer";
pub const LABEL_MODEL_UNIFORM_BUFFER: &str = "Model Uniform Buffer";
pub const LABEL_VERTEX_BUFFER: &str = "Vertex Buffer";
pub const LABEL_INDEX_BUFFER: &str = "Index Buffer";

pub fn camera_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&BufferScribe::camera_uniform_descriptor(
        std::mem::size_of::<CameraUniforms>() as wgpu::BufferAddress,
    ))
}

pub fn model_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&BufferScribe::model_uniform_descriptor(
        std::mem::size_of::<CameraUniforms>() as wgpu::BufferAddress,
    ))
}
pub fn vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
    device.create_buffer_init(&BufferScribe::vertex_descriptor(vertices))
}
pub fn index_buffer(device: &wgpu::Device, indices: &[u16]) -> wgpu::Buffer {
    device.create_buffer_init(&BufferScribe::index_descriptor(indices))
}

pub struct BufferScribe;

impl BufferScribe {
    pub fn camera_uniform_descriptor<'a>(buffer_byte_size: u64) -> wgpu::BufferDescriptor<'a> {
        wgpu::BufferDescriptor {
            label: Some(LABEL_CAMERA_UNIFORM_BUFFER),
            size: buffer_byte_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }
    }
    pub fn model_uniform_descriptor<'a>(buffer_byte_size: u64) -> wgpu::BufferDescriptor<'a> {
        wgpu::BufferDescriptor {
            label: Some(LABEL_MODEL_UNIFORM_BUFFER),
            size: buffer_byte_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }
    }
    pub fn vertex_descriptor<'a>(vertices: &'a [Vertex]) -> wgpu::util::BufferInitDescriptor<'a> {
        wgpu::util::BufferInitDescriptor {
            label: Some(LABEL_VERTEX_BUFFER),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }
    pub fn index_descriptor<'a>(indices: &'a [u16]) -> wgpu::util::BufferInitDescriptor<'a> {
        wgpu::util::BufferInitDescriptor {
            label: Some(LABEL_INDEX_BUFFER),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        }
    }
    pub fn model_uniform_buffer() {}
    pub fn camera_uniform_buffer() {}
}

pub fn create_buffer_init(
    device: &wgpu::Device,
    desc: &wgpu::util::BufferInitDescriptor<'_>,
) -> wgpu::Buffer {
    device.create_buffer_init(desc)
}

pub fn create_buffer(device: &wgpu::Device, desc: wgpu::BufferDescriptor<'_>) -> wgpu::Buffer {
    device.create_buffer(&desc)
}
