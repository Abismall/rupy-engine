use vecmath::col_mat4_mul;

use crate::prelude::{
    mat4_id, new_nonuniform_scaling, new_rotation, vec3_to_mat4_translation, Mat4,
};

use super::traits::Renderable;

pub struct Object {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub mesh: Box<dyn Renderable>,
    pub pipeline_cache_key: u64,
    pub model_matrix: Mat4,
}

impl Object {
    pub fn new(
        position: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
        mesh: Box<dyn Renderable>,
        pipeline_cache_key: u64,
    ) -> Self {
        let mut object = Object {
            position,
            rotation,
            scale,
            mesh,
            pipeline_cache_key,
            model_matrix: mat4_id(),
        };
        object.update_model_matrix();
        object
    }

    pub fn update_model_matrix(&mut self) {
        let translation = vec3_to_mat4_translation(self.position);
        let rotation = new_rotation(self.rotation);
        let scaling = new_nonuniform_scaling(self.scale);
        self.model_matrix = col_mat4_mul(col_mat4_mul(translation, rotation), scaling);
    }

    pub fn update(
        &mut self,
        delta_time: f64,
        update_strategies: &[Box<dyn Fn(&mut Self, f64) + Send + Sync>],
    ) {
        for strategy in update_strategies {
            strategy(self, delta_time);
        }
        self.update_model_matrix();
    }

    pub fn render_object<'a>(
        &mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        pipeline: &'a wgpu::RenderPipeline,
        global_bind_group: &'a wgpu::BindGroup,
    ) {
        self.mesh.render(render_pass, pipeline, global_bind_group);
    }
}

pub struct ObjectDescription {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub rotation_speed: [f32; 3],
    pub scale: [f32; 3],
    pub mesh: Box<dyn Renderable>,
    pub pipeline_cache_key: u64,
}
