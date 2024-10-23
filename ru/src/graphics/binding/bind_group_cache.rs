use std::sync::Arc;

use crate::core::error::AppError;

pub struct BindGroupCache {
    pub bind_groups: std::collections::HashMap<String, Arc<wgpu::BindGroup>>,
}
impl Default for BindGroupCache {
    fn default() -> Self {
        Self {
            bind_groups: Default::default(),
        }
    }
}
impl BindGroupCache {
    pub fn get_or_create(
        &mut self,
        device: &wgpu::Device,
        cache_key: String,
        desc: wgpu::BindGroupDescriptor,
    ) -> Result<std::sync::Arc<wgpu::BindGroup>, AppError> {
        self.bind_groups
            .entry(cache_key.clone())
            .or_insert_with(|| device.create_bind_group(&desc).into());

        match self.bind_groups.get(&cache_key) {
            Some(cached) => Ok(cached.clone()),
            None => Err(AppError::BindGroupCacheError(String::from(
                "No matching bind group in cache",
            ))),
        }
    }
    pub fn new_cache_entry(
        &mut self,
        device: &wgpu::Device,
        cache_key: String,
        desc: wgpu::BindGroupDescriptor,
    ) -> Result<&std::sync::Arc<wgpu::BindGroup>, AppError> {
        self.bind_groups
            .entry(cache_key.clone())
            .or_insert_with(|| device.create_bind_group(&desc).into());

        match self.bind_groups.get(&cache_key) {
            Some(cached) => Ok(cached),
            None => Err(AppError::BindGroupCacheError(String::from(
                "No matching bind group in cache",
            ))),
        }
    }
    pub fn add_bind_group(&mut self, cache_key: String, bind_group: wgpu::BindGroup) {
        if !self.bind_groups.contains_key(&cache_key) {
            self.bind_groups.insert(cache_key, Arc::new(bind_group));
        }
    }
}
