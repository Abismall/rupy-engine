use std::sync::Arc;

use crate::core::cache::HashCache;

use super::Texture;

#[derive(Debug)]
pub struct TextureManager {
    pub textures: HashCache<Arc<Texture>>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashCache::new(),
        }
    }
}
