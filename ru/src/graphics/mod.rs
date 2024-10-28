pub mod binding;
pub mod buffer;
pub mod glyphon;
pub mod gpu;
pub mod pipeline;
pub mod shader;
pub mod texture;

use crate::{scene::material::Material, traits::rendering::Renderable};

pub fn render_object<R: Renderable, M: Material>(
    render_pass: &mut wgpu::RenderPass,
    renderable: &R,
    material: &M,
) {
    render_pass.set_pipeline(material.pipeline());
    render_pass.set_bind_group(0, material.bind_group(), &[]);
    render_pass.set_vertex_buffer(0, renderable.vertex_buffer().slice(..));
    render_pass.set_index_buffer(
        renderable.index_buffer().slice(..),
        renderable.index_format(),
    );
    render_pass.draw_indexed(0..renderable.index_count(), 0, 0..1);
}
