use std::{collections::HashMap, path::Path, sync::Arc};

use wgpu::{Origin3d, TextureAspect, TextureDimension, TextureFormat};

use crate::{
    core::error::AppError,
    graphics::{
        binding::texture_bind_group,
        global::{get_device, get_queue},
    },
    log_error,
};

use super::{loader::load_texture, write_texture_to_queue, TextureFile};

pub struct TextureFileCache {
    pub texture_files: HashMap<String, (Arc<TextureFile>, Arc<wgpu::BindGroup>)>,
}

impl Default for TextureFileCache {
    fn default() -> Self {
        Self {
            texture_files: HashMap::default(),
        }
    }
}

impl TextureFileCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_cache_entry(
        &self,
        path: &str,
    ) -> std::option::Option<(std::sync::Arc<TextureFile>, std::sync::Arc<wgpu::BindGroup>)> {
        match Self::parse_path(path) {
            Some(p) => self.texture_files.get(p).cloned(),
            None => self.texture_files.get(path).cloned(),
        }
    }

    pub fn insert_texture(
        &mut self,
        path: String,
        texture: (std::sync::Arc<TextureFile>, std::sync::Arc<wgpu::BindGroup>),
    ) {
        if let Some(file_name) = Path::new(&path).file_stem().and_then(|f| f.to_str()) {
            if !self.texture_files.contains_key(file_name) {
                self.texture_files.insert(file_name.to_string(), texture);
            }
        } else {
            log_error!("Error: Invalid UTF-8 in file name for path '{}'", path);
        }
    }

    pub fn contains_file(&self, file_name: &str) -> bool {
        Self::parse_path(file_name).map_or(false, |path| self.texture_files.contains_key(path))
    }

    fn parse_path(path: &str) -> Option<&str> {
        Path::new(path).file_stem().and_then(|f| f.to_str())
    }

    pub fn get_or_load_texture(
        &mut self,
        path: &str,
        format: TextureFormat,
        dimension: TextureDimension,
        mip_level_count: u32,
        depth_or_array_layers: u32,
        sample_count: u32,
        origin: Origin3d,
        aspect: TextureAspect,
        mip_level: u32,
        offset: u64,
    ) -> Result<(std::sync::Arc<TextureFile>, std::sync::Arc<wgpu::BindGroup>), AppError> {
        if let Some(cached_texture) = self.get_cache_entry(path) {
            return Ok(cached_texture);
        }
        let device = get_device()?;
        let texture = Arc::new(load_texture(
            &device,
            path,
            format,
            dimension,
            mip_level_count,
            depth_or_array_layers,
            sample_count,
            origin,
            aspect,
            mip_level,
            offset,
        )?);
        let texture_bind_group = texture_bind_group(&device, &texture.view, &texture.sampler);
        drop(device);
        let queue = &get_queue()?;
        if let Err(e) = write_texture_to_queue(queue, &texture) {
            log_error!("{:?}", e);
        };
        self.insert_texture(path.to_string(), (texture, Arc::new(texture_bind_group)));

        Ok(self.get_cache_entry(path).unwrap())
    }

    pub async fn write_to_queue(&self, queue: &wgpu::Queue) -> Result<(), AppError> {
        for (data, ..) in self.texture_files.values() {
            if let Err(e) = write_texture_to_queue(queue, &data) {
                log_error!("{:?}", e);
            };
        }
        Ok(())
    }
}
