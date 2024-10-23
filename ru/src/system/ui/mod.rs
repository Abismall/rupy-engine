use std::sync::Arc;
pub mod component;
pub mod layout;

use component::UIComponent;
use layout::UILayout;
use wgpu::{BindGroup, RenderPipeline};

#[derive(Debug)]
pub struct Menu {
    pub name: String,
    pub layout: UILayout,
    pub pipeline: Option<Arc<RenderPipeline>>,
    pub bind_group: Option<Arc<BindGroup>>,
}

impl Menu {
    pub fn new(name: String, window_size: (f32, f32)) -> Self {
        let layout = UILayout::new(window_size);
        Self {
            name,
            layout,
            pipeline: None,
            bind_group: None,
        }
    }
    pub fn set_pipeline(&mut self, pipeline: Arc<wgpu::RenderPipeline>) {
        self.pipeline = Some(pipeline);
    }
    pub fn set_bind_group(&mut self, bind_group: Arc<wgpu::BindGroup>) {
        self.bind_group = Some(bind_group);
    }
    pub fn add_component(&mut self, component: UIComponent) {
        self.layout.add_component(component);
    }

    pub fn resize_window(&mut self, new_size: (f32, f32)) {
        self.layout.resize(&new_size);
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if let Some(pipeline) = &self.pipeline {
            render_pass.set_pipeline(pipeline);

            if let Some(bind_group) = &self.bind_group {
                render_pass.set_bind_group(1, bind_group, &[]);
            };

            for component in self.layout.components.iter() {
                render_pass.set_vertex_buffer(0, component.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(component.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                render_pass.draw_indexed(0..component.num_indices, 0, 0..1);
            }
        };
    }
}
