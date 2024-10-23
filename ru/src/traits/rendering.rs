use crate::{
    math::Vec3,
    prelude::{Mat4, Vec2, Vec4},
};

pub trait Position {
    fn position(&self) -> Vec3;
}

pub trait Color {
    fn color(&self) -> Vec4;
}
pub trait TextureMapping {
    fn uv(&self) -> Vec2;
}
pub trait ModelMatrix {
    fn model_matrix(&self) -> Mat4;
}

pub trait Renderable: Send + Sync {
    fn vertex_buffer(&self) -> &wgpu::Buffer;
    fn index_buffer(&self) -> &wgpu::Buffer;
    fn num_indices(&self) -> u32;
}
