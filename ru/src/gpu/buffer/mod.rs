use setup::BufferSetup;
use traits::VertexBuffer;
use wgpu::{util::DeviceExt, Buffer, VertexBufferLayout};

use crate::ecs::model::{Vertex2D, Vertex3D};
pub mod setup;
pub mod traits;

impl VertexBuffer for Vertex2D {
    fn vertex_buffer_layout(&self) -> VertexBufferLayout {
        Vertex2D::buffer_layout()
    }

    fn vertex_buffer(device: &wgpu::Device, vertices: &[Self]) -> Buffer
    where
        Self: Sized,
    {
        device.create_buffer_init(&BufferSetup::vertex_buffer_description::<Vertex2D>(
            vertices,
        ))
    }
}

impl VertexBuffer for Vertex3D {
    fn vertex_buffer_layout(&self) -> VertexBufferLayout {
        Vertex3D::buffer_layout()
    }

    fn vertex_buffer(device: &wgpu::Device, vertices: &[Self]) -> Buffer
    where
        Self: Sized,
    {
        device.create_buffer_init(&BufferSetup::vertex_buffer_description::<Vertex3D>(
            vertices,
        ))
    }
}
