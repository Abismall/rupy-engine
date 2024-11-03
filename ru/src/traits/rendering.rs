use nalgebra::Matrix4;

pub trait Renderable {
    type VertexType;

    fn update(&mut self);

    fn model_matrix(&self) -> Matrix4<f32>;

    fn vertices(&self) -> &[Self::VertexType];

    fn indices(&self) -> &[u16];
}
