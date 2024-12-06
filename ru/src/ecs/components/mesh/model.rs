use crate::{ecs::components::model::model::ModelVertex, graphics::model::VertexType};

// pub struct Mesh {
//     pub name: String,
//     pub vertex_buffer: wgpu::Buffer,
//     pub index_buffer: wgpu::Buffer,
//     pub num_elements: u32,
//     pub material: usize,
// }
#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub num_elements: u32,
    pub material: usize,
}
