use std::{collections::HashMap, sync::Arc};

use wgpu::TextureViewDescriptor;

use crate::{
    core::error::AppError,
    gpu::binding::{
        groups::sampled_texture_bind_group, layouts::sampled_texture_bind_group_layout,
    },
    log_debug, log_error,
    texture::{load_texture_by_name, write_texture_to_queue, TextureFile},
};

pub struct TextureManager {
    textures: HashMap<u64, TextureFile>,
    bind_groups: HashMap<u64, Arc<wgpu::BindGroup>>,
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            textures: HashMap::new(),
            bind_groups: HashMap::new(),
        }
    }
    pub fn cleanup_unused(&mut self) {
        self.bind_groups
            .retain(|_, bind_group| Arc::strong_count(bind_group) > 1);
    }

    pub fn get_texture(&self, cache_key: u64) -> Option<&TextureFile> {
        self.textures.get(&cache_key)
    }

    pub fn get_bind_group(&self, cache_key: u64) -> Option<Arc<wgpu::BindGroup>> {
        self.bind_groups.get(&cache_key).cloned()
    }

    pub fn insert_texture(&mut self, cache_key: u64, texture: TextureFile) {
        self.textures.insert(cache_key, texture);
    }

    pub fn insert_bind_group(&mut self, cache_key: u64, bind_group: wgpu::BindGroup) {
        self.bind_groups.insert(cache_key, bind_group.into());
    }

    pub fn contains_texture(&self, id: &u64) -> bool {
        self.textures.contains_key(id)
    }

    pub fn contains_bind_group(&self, id: &u64) -> bool {
        self.bind_groups.contains_key(id)
    }

    pub fn load_texture(
        &self,
        device: &wgpu::Device,
        name: String,
        format: wgpu::TextureFormat,
    ) -> Result<TextureFile, AppError> {
        load_texture_by_name(&device, &name, format)
    }

    pub async fn write_textures(&self, queue: &wgpu::Queue) -> Result<(), AppError> {
        for texture in self.textures.values() {
            log_debug!("Writing texture: {:?}", texture.rows_per_image);
            if let Err(e) = write_texture_to_queue(&queue, &texture, 0, None, None) {
                log_error!("{:?}", e);
            };
        }
        Ok(())
    }

    pub fn get_or_create_bind_group(
        &mut self,
        device: &wgpu::Device,
        cache_key: u64,
    ) -> Result<Arc<wgpu::BindGroup>, AppError> {
        if let Some(bind_group) = self.bind_groups.get(&cache_key) {
            return Ok(bind_group.clone());
        }

        let file = self
            .textures
            .get(&cache_key)
            .ok_or_else(|| AppError::MissingTexture)?;
        let texture_view = &file.texture.create_view(&TextureViewDescriptor::default());
        let layout = sampled_texture_bind_group_layout(device);
        let bind_group = sampled_texture_bind_group(device, &layout, texture_view, &file.sampler);

        self.bind_groups.insert(cache_key, bind_group.into());
        Ok(self.bind_groups.get(&cache_key).unwrap().clone())
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
        if texture_manager.contains_texture(&id) {
            log_error!("Texture '{}' is already cached. Skipping loading.", &name);
            continue;
        } else {
            log_debug!("Adding texture '{}' to cache with key {}.", &name, id);
        };

        match texture_manager.load_texture(device, name.clone(), format) {
            Ok(texture) => {
                texture_manager.insert_texture(id, texture);

                texture_manager.get_or_create_bind_group(device, id)?;
            }
            Err(e) => {
                log_error!("Failed to load texture '{}': {:?}", &name, e);
                return Err(e);
            }
        }
    }

    if let Err(write_err) = texture_manager.write_textures(queue).await {
        log_error!("Failed to write textures to queue: {:?}", write_err);
        return Err(write_err);
    }
    Ok(texture_manager)
}
