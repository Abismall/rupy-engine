use crate::{
    core::error::AppError, graphics::texture::texture_cache::RupyTextureFileVec,
    prelude::TextureCache,
};

pub struct ResourceManager {
    pub textures: TextureCache,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            textures: TextureCache::default(),
        }
    }

    pub fn load_textures(&mut self, entries: RupyTextureFileVec) -> Result<(), AppError> {
        self.textures.load_texture_files(entries)
    }
}
