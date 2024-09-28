use super::command::RenderCommand;
use wgpu::{
    CommandEncoder, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, TextureView,
};

pub struct RenderPhase;

impl RenderPhase {
    pub fn execute(
        command_buffer: &Vec<RenderCommand>,
        encoder: &mut CommandEncoder,
        output_view: &TextureView,
        depth_stencil_attachment: Option<&RenderPassDepthStencilAttachment>,
        clear_color: wgpu::Color,
    ) {
        let color_attachment = RenderPassColorAttachment {
            view: output_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color),
                store: wgpu::StoreOp::Store,
            },
        };

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            timestamp_writes: None,
            occlusion_query_set: None,
            depth_stencil_attachment: depth_stencil_attachment.cloned(),
        });

        for command in command_buffer {
            render_pass.set_pipeline(&command.pipeline);

            render_pass.set_bind_group(0, &command.uniform_data, &[]);

            render_pass.set_vertex_buffer(0, command.vertex_buffer.slice(..));
            render_pass.set_index_buffer(command.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..command.index_count, 0, 0..1);
        }

        drop(render_pass);
    }
}
