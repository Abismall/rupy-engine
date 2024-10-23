use crate::{
    core::error::AppError, graphics::binding::global_bind_group_layout_cache::get_bind_group_layout,
};

use crate::graphics::shader::create_shader_module_from_path;
use crate::log_debug;
use crate::scene::components::vertex::Vertex;
use std::{collections::HashMap, sync::Arc};
use wgpu::ShaderModule;

use super::cache_key::PipelineCacheKey;

pub struct PipelineCache {
    pub pipelines:
        HashMap<PipelineCacheKey, (Arc<wgpu::RenderPipeline>, Vec<Arc<wgpu::BindGroupLayout>>)>,
    pub shader_modules: HashMap<String, Arc<wgpu::ShaderModule>>,
}

impl Default for PipelineCache {
    fn default() -> Self {
        Self {
            pipelines: Default::default(),
            shader_modules: Default::default(),
        }
    }
}

impl PipelineCache {
    pub fn get_or_create_pipeline(
        &mut self,
        device: &wgpu::Device,
        key: PipelineCacheKey,
        swap_chain_format: wgpu::TextureFormat,
    ) -> Result<Arc<wgpu::RenderPipeline>, AppError> {
        if let Some((cached_pipeline, _)) = self.pipelines.get(&key) {
            return Ok(cached_pipeline.clone());
        }

        log_debug!("Creating new pipeline for key: {:?}", key.shader_path);

        let shader_module = if let Some(module) = self.shader_modules.get(&key.shader_path) {
            module.clone()
        } else {
            let module = Arc::new(create_shader_module_from_path(device, &key.shader_path)?);
            self.shader_modules
                .insert(key.shader_path.clone(), module.clone());
            module
        };

        let bind_group_layouts_arc: Vec<Arc<wgpu::BindGroupLayout>> = key
            .bind_group_layout_labels
            .iter()
            .map(|label| {
                get_bind_group_layout(label)
                    .expect(&format!("BindGroupLayout '{}' not found in cache", label))
                    .clone()
            })
            .collect();

        let bind_group_layouts: Vec<&wgpu::BindGroupLayout> = bind_group_layouts_arc
            .iter()
            .map(|arc| arc.as_ref())
            .collect();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &bind_group_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("Render Pipeline: {}", key.shader_path)),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: &key.vertex_entry_point,
                buffers: &[Vertex::vertex_buffer_layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: &key.fragment_entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: swap_chain_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: key.topology,
                strip_index_format: Some(wgpu::IndexFormat::Uint16),
                front_face: key.front_face,
                cull_mode: key.cull_mode,
                polygon_mode: key.polygon_mode,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: Default::default(),
        });

        let pipeline = Arc::new(pipeline);
        self.pipelines
            .insert(key.clone(), (pipeline.clone(), bind_group_layouts_arc));
        Ok(pipeline)
    }

    pub fn add_pipeline(
        &mut self,
        pipeline: wgpu::RenderPipeline,
        key: PipelineCacheKey,
        bind_group_layouts: Vec<Arc<wgpu::BindGroupLayout>>,
    ) {
        if !self.pipelines.contains_key(&key) {
            self.pipelines
                .insert(key, (Arc::new(pipeline), bind_group_layouts));
        }
    }

    pub fn add_shader_module(&mut self, module: wgpu::ShaderModule, shader_path: String) {
        if !self.shader_modules.contains_key(&shader_path) {
            self.shader_modules.insert(shader_path, Arc::new(module));
        }
    }

    pub fn load_shader(
        &mut self,
        device: &wgpu::Device,
        path: &str,
    ) -> Result<Arc<ShaderModule>, AppError> {
        if let Some(module) = self.shader_modules.get(path) {
            return Ok(module.clone());
        }

        let shader_module = Arc::new(create_shader_module_from_path(device, path)?);
        self.shader_modules
            .insert(path.to_string(), shader_module.clone());
        Ok(shader_module)
    }
}
