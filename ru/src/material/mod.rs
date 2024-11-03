pub mod manager;
use std::sync::Arc;

use wgpu::{BindGroup, Buffer};

use crate::shape::Geometry;
#[derive(Debug)]
pub struct Material {
    pub uniform_bind_group: BindGroup,
    pub uniform_buffer: Buffer,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub geometry: Geometry,
    pub color: [f32; 4],
    pub texture_bind_group: Option<Arc<BindGroup>>,
}

impl Material {
    pub fn new(
        uniform_bind_group: BindGroup,
        uniform_buffer: Buffer,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        geometry: Geometry,
        color: [f32; 4],
        texture_bind_group: Option<Arc<BindGroup>>,
    ) -> Self {
        Material {
            uniform_bind_group,
            uniform_buffer,
            vertex_buffer,
            index_buffer,
            geometry,
            color,
            texture_bind_group,
        }
    }

    pub fn set_color(&mut self, new_color: [f32; 4]) {
        self.color = new_color;
    }
}
