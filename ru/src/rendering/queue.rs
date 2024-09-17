use std::collections::VecDeque;

use wgpu::{RenderPass, RenderPipeline};

use super::command::RenderCommand;

pub struct RenderCommandQueue {
    pub commands: VecDeque<RenderCommand>,
}

impl RenderCommandQueue {
    /// Creates a new render command queue.
    pub fn new() -> Self {
        Self {
            commands: VecDeque::new(),
        }
    }

    /// Adds a new command to the queue.
    pub fn add_command(&mut self, command: RenderCommand) {
        self.commands.push_back(command);
    }

    /// Executes all commands in the queue within the given render pass.
    pub fn execute_all(&mut self, render_pass: &mut RenderPass, mut pipeline: RenderPipeline) {
        for command in &self.commands {
            command.execute(render_pass, &mut pipeline);
        }
        self.commands.clear(); // Clear the queue after execution.
    }
}
