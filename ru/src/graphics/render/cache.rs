use std::collections::HashMap;

use crate::{
    core::error::AppError,
    prelude::{helpers::string_to_u64, Labeled},
};

pub trait Cache<R> {
    fn get(&self, id: u64) -> Option<&R>;
    fn contains(&self, id: u64) -> bool;
    fn get_mut(&mut self, id: u64) -> Option<&mut R>;
    fn get_or_create<F>(&mut self, id: u64, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>;
    fn put(&mut self, id: u64, resource: R) -> Result<(), AppError>;
    fn remove(&mut self, id: u64);
    fn hash_descriptor<L: Labeled>(&self, desc: &L) -> u64;
}

impl<R> Cache<R> for std::collections::HashMap<u64, R> {
    fn get(&self, id: u64) -> Option<&R> {
        self.get(&id)
    }
    fn contains(&self, id: u64) -> bool {
        self.contains_key(&id)
    }
    fn get_mut(&mut self, id: u64) -> Option<&mut R> {
        self.get_mut(&id)
    }

    fn get_or_create<F>(&mut self, id: u64, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>,
    {
        if !self.contains_key(&id) {
            let resource = create_fn()?;
            self.insert(id, resource);
        }
        self.get_mut(&id).ok_or(AppError::ResourceNotFound)
    }

    fn put(&mut self, id: u64, resource: R) -> Result<(), AppError> {
        self.insert(id, resource);
        Ok(())
    }

    fn remove(&mut self, id: u64) {
        self.remove(&id);
    }

    fn hash_descriptor<L: Labeled>(&self, desc: &L) -> u64 {
        string_to_u64(&desc.label())
    }
}
#[derive(Debug, Default, Clone)]
pub struct HashCache<R> {
    cache: HashMap<u64, R>,
}

impl<R> HashCache<R> {
    pub fn new() -> Self {
        HashCache {
            cache: HashMap::new(),
        }
    }
}

impl<R> Cache<R> for HashCache<R> {
    fn get(&self, id: u64) -> Option<&R> {
        self.cache.get(&id)
    }
    fn contains(&self, id: u64) -> bool {
        self.cache.contains_key(&id)
    }
    fn get_mut(&mut self, id: u64) -> Option<&mut R> {
        self.cache.get_mut(&id)
    }

    fn get_or_create<F>(&mut self, id: u64, create_fn: F) -> Result<&mut R, AppError>
    where
        F: FnOnce() -> Result<R, AppError>,
    {
        self.cache.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: u64, resource: R) -> Result<(), AppError> {
        self.cache.insert(id, resource);
        Ok(())
    }

    fn remove(&mut self, id: u64) {
        self.cache.remove(&id);
    }

    fn hash_descriptor<L: Labeled>(&self, desc: &L) -> u64 {
        string_to_u64(&desc.label())
    }
}
