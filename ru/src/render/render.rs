use wgpu::{
    BindGroup, DynamicOffset, IndexFormat, QuerySet, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassTimestampWrites, RenderPipeline,
};

use crate::shader::Mesh;

pub fn render(
    encoder: &mut wgpu::CommandEncoder,
    timestamp_writes: Option<RenderPassTimestampWrites>,
    color_attachments: &[Option<RenderPassColorAttachment>],
    occlusion_query_set: Option<&QuerySet>,
    bind_group: &BindGroup,
    offsets: &[DynamicOffset],
    mesh: &Mesh,
    depth_stencil_attachment: Option<RenderPassDepthStencilAttachment>,
    index_format: IndexFormat,
    base_vertex: i32,
    instances: u32,
    pipeline: &RenderPipeline,
) {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments,
        depth_stencil_attachment,
        timestamp_writes,
        occlusion_query_set,
    });

    render_pass.set_pipeline(&pipeline);
    render_pass.set_bind_group(0, bind_group, offsets);
    render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    render_pass.set_index_buffer(mesh.index_buffer.slice(..), index_format);
    render_pass.draw_indexed(0..mesh.index_count as u32, base_vertex, instances..1);
}
