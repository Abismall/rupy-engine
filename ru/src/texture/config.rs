use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::core::error::AppError;

use super::load_texture_image;

#[derive(Debug, Deserialize, Serialize)]
pub struct TextureConfig {
    pub name: String,
    pub id: u64,
    pub depth_or_array_layers: u32,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: wgpu::TextureDimension,
}

pub fn load_texture_config(path: &PathBuf) -> Result<TextureConfig, AppError> {
    let config_string = fs::read_to_string(path)?;
    let config: TextureConfig = serde_yaml::from_str(&config_string)?;
    Ok(config)
}

pub fn load_texture_config_from_folder(folder_path: &PathBuf) -> Result<TextureConfig, AppError> {
    let mut texture_path = folder_path.clone();

    texture_path.push("data.rupy");

    let config_string = fs::read_to_string(&texture_path)?;
    let config: TextureConfig = serde_yaml::from_str(&config_string)?;
    Ok(config)
}

pub fn load_texture_image_from_folder(
    folder_path: &PathBuf,
) -> Result<image::DynamicImage, AppError> {
    let mut image_path = folder_path.clone();
    image_path.push("image.png");

    load_texture_image(&image_path)
}
