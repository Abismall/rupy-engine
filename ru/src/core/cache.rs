use crate::{
    core::error::AppError,
    ecs::{entity::Entity, traits::Cache},
    prelude::helpers::string_to_u64,
};
use std::hash::Hash;
use std::{collections::HashMap, hash::Hasher};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ComponentCacheKey(pub u64);

impl ComponentCacheKey {
    pub fn value(self) -> u64 {
        self.0
    }
}
impl From<u64> for ComponentCacheKey {
    fn from(value: u64) -> Self {
        ComponentCacheKey(value)
    }
}
impl Into<u64> for ComponentCacheKey {
    fn into(self) -> u64 {
        self.0
    }
}
impl From<&str> for ComponentCacheKey {
    fn from(name: &str) -> Self {
        ComponentCacheKey(string_to_u64(name))
    }
}

impl Hash for ComponentCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl From<Entity> for ComponentCacheKey {
    fn from(entity: Entity) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        entity.hash(&mut hasher);
        ComponentCacheKey(hasher.finish())
    }
}

impl<R> Cache<R> for std::collections::HashMap<ComponentCacheKey, R> {
    fn get(&self, id: ComponentCacheKey) -> Option<&R> {
        self.get(&id)
    }
    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.contains_key(&id)
    }
    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut R> {
        self.get_mut(&id)
    }

    fn get_or_create<F>(&mut self, id: ComponentCacheKey, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>,
    {
        if !self.contains_key(&id) {
            let resource = create_fn()?;
            self.insert(id, resource);
        }
        self.get_mut(&id).ok_or(AppError::ResourceNotFound)
    }

    fn put(&mut self, id: ComponentCacheKey, resource: R) -> Result<(), AppError> {
        self.insert(id, resource);
        Ok(())
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.remove(&id);
    }
}
#[derive(Debug, Default, Clone)]
pub struct HashCache<R> {
    cache: HashMap<ComponentCacheKey, R>,
}

impl<R> HashCache<R> {
    pub fn new() -> Self {
        HashCache {
            cache: HashMap::new(),
        }
    }
}

impl<R> Cache<R> for HashCache<R> {
    fn get(&self, id: ComponentCacheKey) -> Option<&R> {
        self.cache.get(&id)
    }
    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.cache.contains_key(&id)
    }
    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut R> {
        self.cache.get_mut(&id)
    }

    fn get_or_create<F>(&mut self, id: ComponentCacheKey, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>,
    {
        self.cache.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: ComponentCacheKey, resource: R) -> Result<(), AppError> {
        self.cache.insert(id, resource);
        Ok(())
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.cache.remove(&id);
    }
}
