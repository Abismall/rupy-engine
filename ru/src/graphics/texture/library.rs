use std::{path::Path, sync::Arc};

use image::{ImageBuffer, Rgba};
use wgpu::{TextureDimension, TextureUsages};

use crate::{core::error::AppError, graphics::gpu::get_queue, log_error};

use super::loader::{load_texture_file, texture_write};
pub struct CachedTexture {
    pub texture: wgpu::Texture,
    pub rgba: ImageBuffer<Rgba<u8>, Vec<u8>>,
}
pub struct TextureLibrary {
    pub textures: std::collections::HashMap<String, Arc<CachedTexture>>,
}
impl Default for TextureLibrary {
    fn default() -> Self {
        Self {
            textures: Default::default(),
        }
    }
}
impl TextureLibrary {
    pub fn new() -> Self {
        Self {
            textures: Default::default(),
        }
    }
    pub fn get_cache_entry(&self, path: &str) -> Option<Arc<CachedTexture>> {
        match Self::parse_path(path) {
            Some(p) => self.textures.get(p).map(|v| v.clone()),
            None => self.textures.get(path).map(|v| v.clone()),
        }
    }
    pub fn insert_texture(&mut self, path: String, texture: Arc<CachedTexture>) {
        if let Some(file_name) = Path::new(&path).file_stem() {
            if let Some(file_name_str) = file_name.to_str() {
                if self.textures.contains_key(file_name_str) {
                    return;
                } else {
                    self.textures.insert(file_name_str.to_string(), texture);
                }
            } else {
                log_error!("Error: Invalid UTF-8 in file name");
            }
        } else {
            log_error!("Error: Could not extract file name");
        }
    }
    pub fn contains_file(&self, file_name: &str) -> bool {
        match Self::parse_path(file_name) {
            Some(path) => self.textures.contains_key(path),
            None => false,
        }
    }
    fn parse_path(path: &str) -> Option<&str> {
        if let Some(file_name) = Path::new(path).file_stem() {
            if let Some(file_name_str) = file_name.to_str() {
                return Some(file_name_str.as_ref());
            }
        }
        None
    }

    pub fn get_or_load_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        dimension: TextureDimension,
        mip_level_count: u32,
        sample_count: u32,
        usage: TextureUsages,
    ) -> Result<std::sync::Arc<CachedTexture>, AppError> {
        if let Some(cached_texture) = self.get_cache_entry(path) {
            return Ok(cached_texture);
        }

        let (texture, rgba) = load_texture_file(
            device,
            path,
            dimension,
            usage,
            mip_level_count,
            sample_count,
        )?;

        texture_write(
            &texture,
            wgpu::Origin3d::ZERO,
            wgpu::TextureAspect::All,
            &rgba,
            texture.size(),
            1,
            &queue,
        );

        let texture = Arc::new(CachedTexture { texture, rgba });
        self.insert_texture(path.to_string(), texture.clone());

        Ok(texture)
    }
    pub async fn write_to_queue(&self) -> Result<(), AppError> {
        let queue = &get_queue()?;
        for (_key, value) in self.textures.iter() {
            texture_write(
                &value.texture,
                wgpu::Origin3d::ZERO,
                wgpu::TextureAspect::All,
                &value.rgba,
                value.texture.size(),
                1,
                queue,
            );
        }
        Ok(())
    }
}
