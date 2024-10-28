use std::{collections::HashMap, sync::Arc};
use wgpu::{PipelineLayout, RenderPipeline};

use super::key::CacheKey;
pub struct PipelineLayouts {
    pub container: HashMap<u64, Arc<PipelineLayout>>,
}
impl PipelineLayouts {
    pub fn new() -> Self {
        Self {
            container: HashMap::new(),
        }
    }
    pub fn contains(&self, cache_key: &CacheKey) -> bool {
        self.container.contains_key(&cache_key.as_hash())
    }
    pub fn get(&self, cache_key: &CacheKey) -> Option<Arc<PipelineLayout>> {
        self.container.get(&cache_key.as_hash()).cloned()
    }
    pub fn put(&mut self, layout: Arc<wgpu::PipelineLayout>, key: &CacheKey) {
        let hash = key.as_hash();
        if !self.container.contains_key(&hash) {
            self.container.insert(hash, layout);
        }
    }
    pub fn flush(&mut self) {
        self.container.clear();
    }
}
impl Default for PipelineLayouts {
    fn default() -> Self {
        Self {
            container: HashMap::new(),
        }
    }
}
pub struct Pipelines {
    pub container: HashMap<u64, Arc<RenderPipeline>>,
    pub layouts: PipelineLayouts,
}

impl Default for Pipelines {
    fn default() -> Self {
        Self {
            container: HashMap::new(),
            layouts: PipelineLayouts::new(),
        }
    }
}

impl Pipelines {
    pub fn new() -> Self {
        Self {
            container: HashMap::new(),
            layouts: PipelineLayouts::new(),
        }
    }
    pub fn contains(&self, cache_key: &CacheKey) -> bool {
        self.container.contains_key(&cache_key.as_hash())
    }
    pub fn get(&self, cache_key: &CacheKey) -> Option<std::sync::Arc<wgpu::RenderPipeline>> {
        self.container.get(&cache_key.as_hash()).map(|v| v.clone())
    }
    pub fn remove(
        &mut self,
        cache_key: &CacheKey,
    ) -> std::option::Option<std::sync::Arc<wgpu::RenderPipeline>> {
        self.container.remove(&cache_key.as_hash())
    }
    pub fn put(&mut self, pipeline: std::sync::Arc<wgpu::RenderPipeline>, cache_key: CacheKey) {
        if !self.container.contains_key(&cache_key.as_hash()) {
            self.container.insert(cache_key.as_hash(), pipeline);
        }
    }
    pub fn flush(&mut self) {
        self.container.clear();
    }
}
