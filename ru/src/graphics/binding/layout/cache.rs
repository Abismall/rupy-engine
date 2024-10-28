use std::{collections::HashMap, sync::Arc};

use wgpu::BindGroupLayout;

use crate::graphics::pipeline::key::CacheKey;

use super::schema::BindGroupLayoutScribe;

pub struct BindGroupLayouts {
    pub container: HashMap<u64, Arc<BindGroupLayout>>,
    pub scribe: BindGroupLayoutScribe,
}
impl BindGroupLayouts {
    pub fn new() -> Self {
        Self {
            container: HashMap::new(),
            scribe: BindGroupLayoutScribe,
        }
    }
    pub fn contains(&self, cache_key: &CacheKey) -> bool {
        self.container.contains_key(&cache_key.as_hash())
    }
    pub fn get(&self, cache_key: &CacheKey) -> Option<Arc<BindGroupLayout>> {
        self.container.get(&cache_key.as_hash()).cloned()
    }
    pub fn remove(
        &mut self,
        cache_key: &CacheKey,
    ) -> std::option::Option<std::sync::Arc<wgpu::BindGroupLayout>> {
        self.container.remove(&cache_key.as_hash())
    }
    pub fn put(&mut self, layout: Arc<wgpu::BindGroupLayout>, key: &CacheKey) {
        let hash = key.as_hash();
        if !self.container.contains_key(&hash) {
            self.container.insert(hash, layout);
        }
    }
    pub fn flush(&mut self) {
        self.container.clear();
    }
}
impl Default for BindGroupLayouts {
    fn default() -> Self {
        Self {
            container: HashMap::new(),
            scribe: BindGroupLayoutScribe,
        }
    }
}
