use std::sync::Arc;

use nalgebra::Matrix4;
use wgpu::{
    Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, Surface,
    SurfaceConfiguration,
};
use winit::window::Window;

use crate::scene::scene_manager::SceneManager;

pub mod command;
pub mod pass;
pub mod pipeline;
pub mod surface;

pub struct RenderSystem {
    pub command_buffer: command::RenderCommandBuffer,
    pub pipeline_manager: pipeline::PipelineManager,
    pub target_surface: surface::TargetSurface,
}

impl RenderSystem {
    pub fn new(
        window: std::sync::Arc<Window>,
        device: std::sync::Arc<wgpu::Device>,
        target_surface: Surface<'static>,
        surface_config: SurfaceConfiguration,
    ) -> Self {
        let pipeline_manager = pipeline::PipelineManager::new(&device);
        let target_surface =
            surface::TargetSurface::new(window, &device, target_surface, surface_config);

        Self {
            command_buffer: command::RenderCommandBuffer::new(),
            pipeline_manager,
            target_surface,
        }
    }
    pub fn execute(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        queue: Arc<&wgpu::Queue>,
        device: Arc<&wgpu::Device>,
        view_proj_matrix: &Matrix4<f32>,
        scene_manager: &mut SceneManager,
    ) {
        if self.target_surface.acquire_current_texture().is_err() {
            return;
        }

        let view = self
            .target_surface
            .current_texture
            .as_ref()
            .unwrap()
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.command_buffer.commands.clear();
        scene_manager.populate_command_buffer(
            &mut self.command_buffer,
            view_proj_matrix,
            queue,
            device,
        );

        let color_attachment = RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE), // TODO: Use ScreenConfig to house the value
                store: wgpu::StoreOp::Store,
            },
        };

        let depth_attachment = RenderPassDepthStencilAttachment {
            view: &self.target_surface.depth_view,
            depth_ops: Some(Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: Some(depth_attachment),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        for command in &self.command_buffer.commands {
            render_pass.set_pipeline(&command.pipeline);
            render_pass.set_bind_group(0, &command.uniform_data, &[]);
            render_pass.set_vertex_buffer(0, command.vertex_buffer.slice(..));
            render_pass.set_index_buffer(command.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..command.index_count, 0, 0..1);
        }

        drop(render_pass);

        self.target_surface.present_texture();
    }
}
