use nalgebra::Matrix4;

use crate::gpu::buffer::VertexBuffer;

pub trait Renderable {
    type VertexType: VertexBuffer;

    fn update(&mut self);

    fn model_matrix(&self) -> Matrix4<f32>;

    fn vertices(&self) -> &[Self::VertexType];

    fn indices(&self) -> &[u16];
}
