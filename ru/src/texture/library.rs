use std::{collections::HashMap, path::Path, sync::Arc};

use wgpu::{Device, Queue};

use crate::{core::error::AppError, log_error, prelude::helpers::string_to_u64};

use super::{
    config::{load_texture_configs_from_folder, TextureConfig},
    file::create_textures_from_configs,
    write_texture_to_queue, TextureFile,
};

pub struct TextureManager {
    textures: HashMap<u64, Arc<TextureFile>>,
    queue: Arc<Queue>,
    device: Arc<Device>,
}

impl TextureManager {
    pub fn new(queue: Arc<Queue>, device: Arc<Device>) -> Self {
        TextureManager {
            textures: HashMap::new(),
            queue,
            device,
        }
    }

    pub fn get_texture(&self, path: &str) -> Option<Arc<TextureFile>> {
        let file = Self::parse_path(path).and_then(|p| self.textures.get(&string_to_u64(p)));
        file.map(Arc::clone)
    }

    pub fn insert_texture(&mut self, path: String, texture: Arc<TextureFile>) {
        if let Some(file_name) = Path::new(&path).file_stem().and_then(|f| f.to_str()) {
            let cache_key = string_to_u64(file_name);
            if !self.textures.contains_key(&cache_key) {
                self.textures.insert(cache_key, texture);
            }
        } else {
            log_error!("Error: Invalid UTF-8 in file name for path '{}'", path);
        }
    }

    pub fn contains_file(&self, file_name: &str) -> bool {
        Self::parse_path(file_name).map_or(false, |path| {
            self.textures.contains_key(&string_to_u64(path))
        })
    }

    fn parse_path(path: &str) -> Option<&str> {
        Path::new(path).file_stem().and_then(|f| f.to_str())
    }

    pub fn load_texture_configs(
        &self,
        folder_path: &str,
        extension: &str,
    ) -> Result<Vec<TextureConfig>, AppError> {
        load_texture_configs_from_folder(folder_path, extension)
    }

    pub fn create_textures_from_configs(
        &mut self,
        configs: Vec<TextureConfig>,
    ) -> Result<Vec<TextureFile>, AppError> {
        let files = create_textures_from_configs(&self.device, configs)?;
        Ok(files)
    }

    pub async fn write_to_queue(&self) -> Result<(), AppError> {
        for texture in self.textures.values() {
            if let Err(e) = write_texture_to_queue(&self.queue, &texture, 0, None, None) {
                log_error!("{:?}", e);
            };
        }
        Ok(())
    }

    pub async fn async_load_and_create_textures(
        &mut self,
        folder_path: String,
        extension: String,
    ) -> Result<(), AppError> {
        let configs = tokio::task::spawn_blocking(move || {
            load_texture_configs_from_folder(&folder_path, &extension)
        })
        .await
        .map_err(|e| AppError::TaskJoinError(e))??;

        self.create_textures_from_configs(configs)?;

        Ok(())
    }
}
