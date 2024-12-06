use wgpu::RenderPipeline;

use crate::{core::cache::HashCache, ecs::traits::Cache, prelude::helpers::string_to_u64};

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
    fn get(&self, id: u64) -> Option<&RenderPipeline> {
        self.cache.get(id)
    }

    fn get_mut(&mut self, id: u64) -> Option<&mut RenderPipeline> {
        self.cache.get_mut(id)
    }

    fn get_or_create<F>(
        &mut self,
        id: u64,
        create_fn: F,
    ) -> Result<&mut RenderPipeline, crate::prelude::error::AppError>
    where
        F: FnOnce() -> Result<RenderPipeline, crate::prelude::error::AppError>,
    {
        self.cache.get_or_create(id, create_fn)
    }

    fn put(
        &mut self,
        id: u64,
        resource: RenderPipeline,
    ) -> Result<(), crate::prelude::error::AppError> {
        self.cache.put(id, resource)
    }

    fn remove(&mut self, id: u64) {
        self.cache.remove(id);
    }

    fn contains(&self, id: u64) -> bool {
        self.cache.contains(id)
    }
}
