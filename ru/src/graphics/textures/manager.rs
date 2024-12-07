use std::collections::HashMap;

use crate::{
    core::{cache::ComponentCacheKey, error::AppError},
    ecs::traits::Cache,
};

use super::Texture;

#[derive(Default, Debug)]
pub struct TextureManager {
    textures: HashMap<ComponentCacheKey, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}

impl Cache<Texture> for TextureManager {
    fn get(&self, id: ComponentCacheKey) -> Option<&Texture> {
        self.textures.get(&id)
    }

    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.textures.contains_key(&id)
    }

    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut Texture> {
        self.textures.get_mut(&id)
    }

    fn get_or_create<F>(
        &mut self,
        id: ComponentCacheKey,
        create_fn: F,
    ) -> Result<&mut Texture, AppError>
    where
        F: FnOnce() -> Result<Texture, AppError>,
    {
        if !self.textures.contains_key(&id) {
            let texture = create_fn()?;
            self.textures.insert(id, texture);
        }
        self.textures.get_mut(&id).ok_or(AppError::ResourceNotFound)
    }

    fn put(&mut self, id: ComponentCacheKey, resource: Texture) -> Result<(), AppError> {
        if self.textures.contains_key(&id) {
            return Err(AppError::DuplicateResource);
        }
        self.textures.insert(id, resource);
        Ok(())
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.textures.remove(&id);
    }
}
