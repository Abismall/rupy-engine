use std::{collections::HashMap, sync::Arc};

use wgpu::BindGroup;

use crate::graphics::pipeline::key::CacheKey;

pub struct BindGroups {
    pub container: HashMap<u64, Arc<BindGroup>>,
}
impl BindGroups {
    pub fn new() -> Self {
        Self {
            container: HashMap::new(),
        }
    }
    pub fn contains(&self, cache_key: &CacheKey) -> bool {
        self.container.contains_key(&cache_key.as_hash())
    }
    pub fn get(&self, cache_key: &CacheKey) -> Option<Arc<BindGroup>> {
        self.container.get(&cache_key.as_hash()).cloned()
    }
    pub fn remove(
        &mut self,
        cache_key: &CacheKey,
    ) -> std::option::Option<std::sync::Arc<wgpu::BindGroup>> {
        self.container.remove(&cache_key.as_hash())
    }
    pub fn put(&mut self, group: Arc<wgpu::BindGroup>, key: &CacheKey) {
        let hash = key.as_hash();
        if !self.container.contains_key(&hash) {
            self.container.insert(hash, group);
        }
    }
    pub fn flush(&mut self) {
        self.container.clear();
    }
}
impl Default for BindGroups {
    fn default() -> Self {
        Self {
            container: HashMap::new(),
        }
    }
}
