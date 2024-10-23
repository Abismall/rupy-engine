use std::hash::{DefaultHasher, Hash, Hasher};

use image::{ImageBuffer, Rgba};

use crate::{
    core::error::AppError,
    graphics::gpu::{get_device, get_queue},
    log_error,
    utilities::calculate_hash,
};

use super::{loader::load_texture_file, texture_write, RupyTextureFile, TEXTURE_DIR};

pub type RupyTextureFileVec = Vec<RupyTextureFile>;
pub struct CachedTexture {
    texture: wgpu::Texture,
    rgba: ImageBuffer<Rgba<u8>, Vec<u8>>,
}
pub struct TextureCache {
    pub textures: std::collections::HashMap<u64, CachedTexture>,
}
impl Default for TextureCache {
    fn default() -> Self {
        Self {
            textures: Default::default(),
        }
    }
}
impl TextureCache {
    pub fn load_texture_files(&mut self, entries: RupyTextureFileVec) -> Result<(), AppError> {
        let device = &get_device()?;
        for mut entry in entries {
            if !self.is_in_cache(&entry.file_path) {
                match load_texture_file(
                    device,
                    &entry.file_path,
                    entry.dimension,
                    entry.mip_level_count,
                    entry.sample_count,
                    entry.format,
                ) {
                    Ok((texture, rgba)) => {
                        self.textures.insert(
                            calculate_hash(&entry.file_path.split_off(TEXTURE_DIR.len() + 1)),
                            CachedTexture { texture, rgba },
                        );
                    }
                    Err(e) => {
                        log_error!("Failed to load texture: {}, Error: {}", entry.file_path, e);
                    }
                };
            }
        }
        Ok(())
    }
    pub fn write_to_queue(&self) -> Result<(), AppError> {
        let queue = &get_queue()?;
        for (_key, value) in self.textures.iter() {
            texture_write(
                &value.texture,
                wgpu::Origin3d::ZERO,
                wgpu::TextureAspect::All,
                &value.rgba,
                value.texture.size(),
                value.texture.mip_level_count(),
                queue,
            );
        }
        Ok(())
    }

    fn is_in_cache(&self, cache_key: &str) -> bool {
        self.textures
            .contains_key(&calculate_hash(&String::from(cache_key)))
    }

    fn add_cache_entry(&mut self, source: &str, texture: CachedTexture) {
        let key = calculate_hash(&String::from(source));
        self.textures.insert(key, texture);
    }

    pub fn get_cache_entry(&self, source: &str) -> Option<&CachedTexture> {
        self.textures.get(&calculate_hash(&String::from(source)))
    }
}
