use std::sync::Arc;

use vecmath::col_mat4_mul;

use crate::{
    math::{
        mat4_id,
        spatial::{new_nonuniform_scaling, new_rotation},
        vector::vec3_to_mat4_translation,
        Mat4,
    },
    render::traits::Renderable,
};

use super::shape::Shape;

#[derive(Debug, Clone)]
pub struct Object {
    pub model_matrix: Mat4,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub shape: Arc<Shape>,
    pub index_buffer_key: u64,
    pub vertex_buffer_key: u64,
    pub bind_group_layout_key: u64,
    pub bind_group_key: u64,
}

impl Object {
    pub fn new(
        position: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
        shape: Arc<Shape>,
        bind_group_key: u64,
        bind_group_layout_key: u64,
        vertex_buffer_key: u64,
        index_buffer_key: u64,
    ) -> Self {
        Object {
            model_matrix: mat4_id(),
            position,
            rotation,
            scale,
            shape,
            bind_group_layout_key,
            bind_group_key,
            index_buffer_key,
            vertex_buffer_key,
        }
    }

    pub fn update_model_matrix(&mut self) {
        let translation = vec3_to_mat4_translation(self.position);
        let rotation = new_rotation(self.rotation);
        let scaling = new_nonuniform_scaling(self.scale);
        self.model_matrix = col_mat4_mul(col_mat4_mul(translation, rotation), scaling);
    }
}
impl Renderable for Object {
    fn vertex_buffer_data(&self) -> &[u8] {
        bytemuck::cast_slice(&self.shape.vertices)
    }

    fn index_buffer_data(&self) -> &[u32] {
        bytemuck::cast_slice(&self.shape.indices)
    }

    fn num_indices(&self) -> u32 {
        self.shape.indices.len() as u32
    }

    fn is_textured(&self) -> bool {
        self.shape.is_textured
    }

    fn update(&mut self) {
        self.update_model_matrix();
    }
}
