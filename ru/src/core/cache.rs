use crate::{
    core::error::AppError,
    ecs::{entity::Entity, traits::Cache},
    prelude::helpers::string_to_u64,
};
use std::hash::Hash;
use std::{collections::HashMap, hash::Hasher};
pub trait HasCacheKey {
    fn key(suffixes: Vec<&str>) -> CacheKey;
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CacheKey(pub u64);

impl CacheKey {
    pub fn value(self) -> u64 {
        self.0
    }
}
impl From<u64> for CacheKey {
    fn from(value: u64) -> Self {
        CacheKey(value)
    }
}
impl Into<u64> for CacheKey {
    fn into(self) -> u64 {
        self.0
    }
}
impl From<&str> for CacheKey {
    fn from(name: &str) -> Self {
        CacheKey(string_to_u64(name))
    }
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl From<Entity> for CacheKey {
    fn from(entity: Entity) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        entity.hash(&mut hasher);
        CacheKey(hasher.finish())
    }
}
impl From<&Entity> for CacheKey {
    fn from(entity: &Entity) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        entity.hash(&mut hasher);
        CacheKey(hasher.finish())
    }
}
impl From<&String> for CacheKey {
    fn from(string: &String) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        string.hash(&mut hasher);
        CacheKey(hasher.finish())
    }
}
impl<R> Cache<R> for std::collections::HashMap<CacheKey, R> {
    fn get(&self, id: &CacheKey) -> Option<&R> {
        self.get(id)
    }
    fn contains(&self, id: &CacheKey) -> bool {
        self.contains_key(id)
    }
    fn get_mut(&mut self, id: &CacheKey) -> Option<&mut R> {
        self.get_mut(id)
    }

    fn get_or_create<F>(&mut self, id: CacheKey, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>,
    {
        if !self.contains_key(&id) {
            let resource = create_fn()?;
            self.insert(id, resource);
        }
        self.get_mut(&id).ok_or(AppError::ResourceNotFound(format!(
            "No cache entry found for key {}",
            id.value()
        )))
    }

    fn put(&mut self, id: CacheKey, resource: R) {
        self.insert(id, resource);
    }

    fn remove(&mut self, id: &CacheKey) {
        self.remove(id);
    }
}
#[derive(Debug, Default, Clone)]
pub struct HashCache<R> {
    cache: HashMap<CacheKey, R>,
}

impl<R> HashCache<R> {
    pub fn new() -> Self {
        HashCache {
            cache: HashMap::new(),
        }
    }
}

impl<R> Cache<R> for HashCache<R> {
    fn get(&self, id: &CacheKey) -> Option<&R> {
        self.cache.get(id)
    }
    fn contains(&self, id: &CacheKey) -> bool {
        self.cache.contains_key(&id)
    }
    fn get_mut(&mut self, id: &CacheKey) -> Option<&mut R> {
        self.cache.get_mut(id)
    }

    fn get_or_create<F>(&mut self, id: CacheKey, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>,
    {
        self.cache.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: CacheKey, resource: R) -> () {
        self.cache.insert(id, resource);
    }

    fn remove(&mut self, id: &CacheKey) {
        self.cache.remove(id);
    }
}
