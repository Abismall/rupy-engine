use color::Color;

use crate::geometry::Geometry;

pub(crate) mod color;
pub(crate) mod manager;
pub(crate) mod vertex;

use wgpu::{BindGroup, Buffer};

#[derive(Debug)]
pub struct Material {
    pub bind_group: BindGroup,
    pub uniform_buffer: Buffer,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub geometry: Geometry,
    pub color: Color,
}

impl Material {
    pub fn new(
        bind_group: BindGroup,
        uniform_buffer: Buffer,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        geometry: Geometry,
        color: Color,
    ) -> Self {
        Material {
            bind_group,
            uniform_buffer,
            vertex_buffer,
            index_buffer,
            geometry,
            color,
        }
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }
}
