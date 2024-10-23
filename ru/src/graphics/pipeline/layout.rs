use std::{collections::HashMap, sync::Arc};

use crate::{core::error::AppError, log_debug};

pub struct PipelineLayoutCache {
    pub layouts: HashMap<String, Arc<wgpu::PipelineLayout>>,
}
impl Default for PipelineLayoutCache {
    fn default() -> Self {
        Self {
            layouts: Default::default(),
        }
    }
}
impl PipelineLayoutCache {
    pub fn get_or_create(
        &mut self,
        device: &wgpu::Device,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        shader_module_path: String,
    ) -> Result<Arc<wgpu::PipelineLayout>, AppError> {
        let pipeline_layouts = &mut self.layouts;
        if let Some(cached) = pipeline_layouts.get(&shader_module_path) {
            return Ok(cached.clone());
        } else {
            log_debug!("Pipeline cache entry: {}", shader_module_path);
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });
            pipeline_layouts.insert(shader_module_path.clone(), Arc::new(pipeline_layout));
            match pipeline_layouts.get(&shader_module_path) {
                Some(cached) => Ok(cached.clone()),
                None => Err(AppError::PipelineLayoutCacheError(String::from(
                    "No matching layout in cache",
                ))),
            }
        }
    }
    pub fn add_layout(&mut self, layout: wgpu::PipelineLayout, cache_key: String) {
        if !self.layouts.contains_key(&cache_key) {
            self.layouts.insert(cache_key, Arc::new(layout));
        }
    }
    pub fn get_layout(
        &mut self,
        cache_key: String,
    ) -> Option<&std::sync::Arc<wgpu::PipelineLayout>> {
        self.layouts.get(&cache_key)
    }
}
