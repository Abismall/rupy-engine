use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

use color::Color;

pub(crate) mod color;

pub(crate) mod material_manager;
pub(crate) mod vertex;

use wgpu::{BindGroup, Buffer};

use crate::geometry::Shape;

pub struct GeometricHasher;
impl GeometricHasher {
    pub fn hash(geometry: &Shape) -> u64 {
        let mut hasher = DefaultHasher::new();
        geometry.vertex_buffer_data().hash(&mut hasher);
        geometry.index_buffer_data().hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    pub bind_group: Arc<BindGroup>,
    pub uniform_buffer: Arc<Buffer>,
    pub geometry: Shape,
    pub color: Color,
}

impl Material {
    pub fn new(
        bind_group: BindGroup,
        uniform_buffer: Buffer,

        geometry: Shape,
        color: Color,
    ) -> Self {
        Material {
            bind_group: bind_group.into(),
            geometry,
            color,
            uniform_buffer: uniform_buffer.into(),
        }
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }
}
