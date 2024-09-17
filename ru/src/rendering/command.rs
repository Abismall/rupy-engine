use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, Buffer, Device, RenderPass, RenderPipeline, ShaderStages, TextureFormat};

use crate::AppError;

use super::pipeline::RenderPipelineManager;

const SHADE_DIR_BASE_PATH: &str = "./rendering/shaders/";

/// A command that contains all the necessary state to render an object.
pub struct RenderCommand {
    vertex_buffer: Buffer,
    bind_group: BindGroup,
    vertex_count: u32,
    pipeline_id: String,
}

impl RenderCommand {
    /// Creates a new render command with the specified pipeline, vertex data, and bind group.
    pub fn new(
        device: &Device,
        pipeline_id: &str,
        vertex_data: &[f32],
        uniform_data: &[f32], // Uniform data to be passed to the shaders
    ) -> Result<Self, AppError> {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create a uniform buffer for the uniform data (e.g., transformation matrix).
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(uniform_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create a bind group layout and bind group.
        let bind_group_layout = RenderPipelineManager::create_bind_group_layout(device);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0, // This must match the binding in your shader layout
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Bind Group"),
        });

        let vertex_count = (vertex_data.len() / 6) as u32; // Assuming each vertex has 6 attributes.

        Ok(Self {
            pipeline_id: pipeline_id.to_string(),
            vertex_buffer,
            bind_group,
            vertex_count,
        })
    }

    /// Executes the command using the given render pass.
    pub fn execute(&self, render_pass: &mut RenderPass, pipeline: &RenderPipeline) {
        render_pass.set_pipeline(&pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex_count, 0..1);
    }
}
