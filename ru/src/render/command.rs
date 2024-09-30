use std::sync::Arc;

use wgpu::{Buffer, RenderPipeline};

pub struct RenderCommand {
    pub vertex_buffer: Arc<Buffer>,
    pub index_buffer: Arc<Buffer>,
    pub index_count: u32,
    pub pipeline: Arc<RenderPipeline>,
    pub(crate) bind_group: Arc<wgpu::BindGroup>,
}

pub struct RenderCommandBuffer {
    pub commands: Vec<RenderCommand>,
}

impl RenderCommandBuffer {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn push_render_command(&mut self, command: RenderCommand) {
        self.commands.push(command);
    }
}
