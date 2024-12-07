use wgpu::RenderPipeline;

use crate::{
    core::cache::{ComponentCacheKey, HashCache},
    ecs::traits::Cache,
};

#[derive(Debug)]
pub struct PipelineManager {
    cache: HashCache<RenderPipeline>,
}

impl PipelineManager {
    pub fn new() -> Self {
        Self {
            cache: HashCache::new(),
        }
    }
}

impl Cache<RenderPipeline> for PipelineManager {
    fn get(&self, id: ComponentCacheKey) -> Option<&RenderPipeline> {
        self.cache.get(id)
    }

    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut RenderPipeline> {
        self.cache.get_mut(id)
    }

    fn get_or_create<F>(
        &mut self,
        id: ComponentCacheKey,
        create_fn: F,
    ) -> Result<&mut RenderPipeline, crate::prelude::error::AppError>
    where
        F: FnOnce() -> Result<RenderPipeline, crate::prelude::error::AppError>,
    {
        self.cache.get_or_create(id, create_fn)
    }

    fn put(
        &mut self,
        id: ComponentCacheKey,
        resource: RenderPipeline,
    ) -> Result<(), crate::prelude::error::AppError> {
        self.cache.put(id, resource)
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.cache.remove(id);
    }

    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.cache.contains(id)
    }
}
