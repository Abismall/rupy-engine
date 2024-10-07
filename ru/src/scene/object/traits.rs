use wgpu::{BindGroup, Buffer, RenderPass, RenderPipeline};

use crate::prelude::{Mat4, Quat, Vec3};

pub trait Renderable {
    /// Returns the vertex buffer for the object.
    fn vertex_buffer(&self) -> &Buffer;

    /// Returns the index buffer for the object.
    fn index_buffer(&self) -> &Buffer;

    /// Returns the number of indices to be rendered.
    fn num_indices(&self) -> u32;

    /// Checks if the object is textured.
    fn is_textured(&self) -> bool;

    fn update_model_uniform(&self, queue: &wgpu::Queue, model_matrix: &Mat4);

    /// Issues the draw call to render the object.
    fn render<'a>(
        &mut self,
        render_pass: &mut RenderPass<'a>,
        pipeline: &'a RenderPipeline,
        global_bind_group: &'a BindGroup,
    );
}

pub trait Transformable {
    /// Sets the object's position.
    fn set_position(&mut self, position: Vec3);

    /// Sets the object's rotation.
    fn set_rotation(&mut self, rotation: Quat<f32>);

    /// Sets the object's scale.
    fn set_scale(&mut self, scale: Vec3);

    /// Returns the current transformation matrix for the object.
    fn get_transform_matrix(&self) -> Mat4;
}
