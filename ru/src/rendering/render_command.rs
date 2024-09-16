use std::sync::Arc;
use wgpu::{Buffer, RenderPass, RenderPipeline};

pub struct RenderCommand {
    pub pipeline: Arc<RenderPipeline>,
    pub vertex_buffer: Arc<Buffer>,
    pub index_buffer: Option<Arc<Buffer>>,
    pub vertex_count: u32,
}

impl RenderCommand {
    pub fn new_triangle(pipeline: Arc<RenderPipeline>, vertex_buffer: Arc<Buffer>) -> Self {
        Self {
            pipeline,
            vertex_buffer,
            index_buffer: None,
            vertex_count: 3,
        }
    }

    pub fn execute(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex_count, 0..1);
    }
    pub fn get_pipeline(&self) -> &Arc<RenderPipeline> {
        &self.pipeline
    }

    pub fn get_vertex_buffer(&self) -> &Arc<Buffer> {
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> Option<&Arc<Buffer>> {
        self.index_buffer.as_ref()
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.vertex_count
    }
}
