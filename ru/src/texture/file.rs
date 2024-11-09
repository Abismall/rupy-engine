use image::GenericImageView;
use wgpu::{Device, TextureDescriptor};

use crate::{
    core::{error::AppError, files::FileSystem},
    log_error,
};

use super::config::TextureConfig;
#[derive(Debug)]
pub struct TextureFile {
    pub texture: wgpu::Texture,
    pub rgba: Vec<u8>,
    pub bytes_per_row: Option<u32>,
    pub rows_per_image: Option<u32>,
}
pub fn img_extent_3d(
    img: image::DynamicImage,
    depth_or_array_layers: Option<u32>,
) -> wgpu::Extent3d {
    let (width, height) = img.dimensions();
    let depth_or_array_layers = depth_or_array_layers.unwrap_or(1);
    wgpu::Extent3d {
        depth_or_array_layers,
        width,
        height,
    }
}

pub fn load_texture_image(file_path: &str) -> Result<image::DynamicImage, AppError> {
    match image::open(file_path) {
        Ok(img) => Ok(img),
        Err(e) => {
            log_error!("{:?}", e);
            return Err(AppError::from(e));
        }
    }
}

pub fn create_textures_from_configs(
    device: &Device,
    configs: Vec<TextureConfig>,
) -> Result<Vec<TextureFile>, AppError> {
    let mut textures = Vec::new();

    for config in configs {
        let path = FileSystem::get_image_file_path(&config.label)?;
        let image = match FileSystem::load_image_file(&path) {
            Ok(img) => img,
            Err(e) => {
                return Err(e);
            }
        };

        let rgba = image.to_rgba8();
        let (width, height) = image.dimensions();

        let desc = TextureDescriptor {
            label: Some(&config.label),
            size: config.size,
            mip_level_count: config.mip_level_count,
            sample_count: config.sample_count,
            dimension: config.dimension,
            format: config.format,
            usage: config.usage,
            view_formats: &config.view_formats,
        };

        let texture = device.create_texture(&desc);

        textures.push(TextureFile {
            texture,
            rgba: rgba.to_vec(),
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        });
    }

    Ok(textures)
}
