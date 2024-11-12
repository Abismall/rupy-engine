use std::collections::HashMap;

use crate::{
    core::error::AppError,
    log_debug, log_error,
    texture::{load_texture_by_name, write_texture_to_queue, TextureFile},
};

pub struct TextureManager {
    textures: HashMap<u64, TextureFile>,
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub fn get(&self, cache_key: u64) -> std::option::Option<&TextureFile> {
        self.textures.get(&cache_key)
    }

    pub fn insert(&mut self, cache_key: u64, texture: TextureFile) {
        self.textures.insert(cache_key, texture);
    }

    pub fn contains(&self, id: &u64) -> bool {
        self.textures.contains_key(id)
    }

    pub fn load(
        &self,
        device: &wgpu::Device,
        name: String,
        format: wgpu::TextureFormat,
    ) -> Result<TextureFile, AppError> {
        load_texture_by_name(&device, &name, format)
    }

    pub async fn write(&self, queue: &wgpu::Queue) -> Result<(), AppError> {
        for texture in self.textures.values() {
            log_debug!("Writing texture: {:?}", texture.rows_per_image);
            if let Err(e) = write_texture_to_queue(&queue, &texture, 0, None, None) {
                log_error!("{:?}", e);
            };
        }
        Ok(())
    }
}
pub async fn setup_texture_manager(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    mut texture_manager: TextureManager,
    textures: Vec<(String, u64)>,
    format: wgpu::TextureFormat,
) -> Result<TextureManager, AppError> {
    for (name, id) in textures {
        if texture_manager.contains(&id) {
            log_error!("Texture '{}' is already cached. Skipping loading.", &name);
            continue;
        } else {
            log_debug!("Adding texture '{}' to cache with key {}.", &name, id);
        };

        match load_texture_by_name(&device, &name, format) {
            Ok(texture) => {
                texture_manager.insert(id, texture);
            }
            Err(e) => {
                log_error!("Failed to load texture '{}': {:?}", &name, e);
                return Err(e);
            }
        }
    }

    if let Err(write_err) = texture_manager.write(queue).await {
        log_error!("Failed to write textures to queue: {:?}", write_err);
        return Err(write_err);
    }
    Ok(texture_manager)
}
