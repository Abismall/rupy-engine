use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::core::{error::AppError, files::FileSystem};

#[derive(Debug, Deserialize, Serialize)]
pub struct TextureConfig {
    pub label: String,
    pub size: wgpu::Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: wgpu::TextureDimension,
    pub format: wgpu::TextureFormat,
    pub usage: wgpu::TextureUsages,
    pub view_formats: Vec<wgpu::TextureFormat>,
}

pub fn load_texture_config(path: &PathBuf) -> Result<TextureConfig, AppError> {
    let config_string = fs::read_to_string(path)?;
    let config: TextureConfig = serde_yaml::from_str(&config_string)?;
    Ok(config)
}

pub fn load_texture_configs_from_folder(
    folder_path: &str,
    extension: &str,
) -> Result<Vec<TextureConfig>, AppError> {
    let mut texture_configs = Vec::new();

    for entry in
        FileSystem::list_files_with_extension(folder_path, std::ffi::OsStr::new(extension))?
    {
        match load_texture_config(&entry) {
            Ok(config) => texture_configs.push(config),
            Err(e) => return Err(e),
        }
    }

    Ok(texture_configs)
}
