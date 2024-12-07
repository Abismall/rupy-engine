use crate::{
    core::{
        cache::{ComponentCacheKey, HashCache},
        error::AppError,
    },
    ecs::traits::Cache,
};

use super::Transform;

#[derive(Debug, Clone)]
pub struct TransformManager {
    cache: HashCache<Transform>,
}

impl TransformManager {
    pub fn new() -> Self {
        TransformManager {
            cache: HashCache::new(),
        }
    }

    pub fn insert(&mut self, id: ComponentCacheKey, transform: Transform) -> Result<(), AppError> {
        self.cache.put(id, transform)
    }

    pub fn get(&self, id: ComponentCacheKey) -> Option<&Transform> {
        self.cache.get(id)
    }

    pub fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut Transform> {
        self.cache.get_mut(id)
    }

    pub fn get_or_create<F>(
        &mut self,
        id: ComponentCacheKey,
        create_fn: F,
    ) -> Result<&mut Transform, AppError>
    where
        F: FnOnce() -> Result<Transform, AppError>,
    {
        self.cache.get_or_create(id, create_fn)
    }

    pub fn remove(&mut self, id: ComponentCacheKey) {
        self.cache.remove(id);
    }

    pub fn contains(&self, id: ComponentCacheKey) -> bool {
        self.cache.contains(id)
    }
}

impl Cache<Transform> for TransformManager {
    fn get(&self, id: ComponentCacheKey) -> Option<&Transform> {
        self.cache.get(id)
    }

    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.cache.contains(id)
    }

    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut Transform> {
        self.cache.get_mut(id)
    }

    fn get_or_create<F>(
        &mut self,
        id: ComponentCacheKey,
        create_fn: F,
    ) -> Result<&mut Transform, AppError>
    where
        F: FnOnce() -> Result<Transform, AppError>,
    {
        self.cache.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: ComponentCacheKey, resource: Transform) -> Result<(), AppError> {
        self.cache.put(id, resource)
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.cache.remove(id);
    }
}
