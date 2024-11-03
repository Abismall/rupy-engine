use image::GenericImageView;
use wgpu::{TextureDimension, TextureFormat};

use super::{wgpu_texture_descriptor, wgpu_texture_view, TextureFile, TextureFileDescriptor};
use crate::{
    core::{error::AppError, files::FileSystem},
    log_error,
    texture::sampler::{create_sampler_from_type, SamplerType},
};

pub fn load_textures(
    folder_path: &str,
    extension: &str,
    format: TextureFormat,
    dimension: TextureDimension,
    mip_level_count: u32,
    sample_count: u32,
) -> Result<Vec<TextureFileDescriptor>, AppError> {
    let mut entries = Vec::new();
    for entry in
        FileSystem::list_files_with_extension(folder_path, std::ffi::OsStr::new(extension))?
    {
        let path = entry.as_path();
        let file_path = path.to_string_lossy().to_string();

        let image = image::open(&path)?;
        let (width, height) = image.dimensions();

        entries.push(TextureFileDescriptor {
            file_path,
            dimension,
            mip_level_count,
            sample_count,
            format,
            width,
            height,
            depth_or_array_layers: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });
    }
    Ok(entries)
}

pub fn load_texture(
    device: &wgpu::Device,
    path: &str,
    format: TextureFormat,
    dimension: TextureDimension,
    mip_level_count: u32,
    depth_or_array_layers: u32,
    sample_count: u32,
    origin: wgpu::Origin3d,
    aspect: wgpu::TextureAspect,
    mip_level: u32,
    offset: u64,
) -> Result<TextureFile, AppError> {
    let image = match FileSystem::load_image_file(&FileSystem::append_to_cargo_dir(path)) {
        Ok(img) => img,
        Err(e) => {
            log_error!("{:?}", e);
            return Err(e);
        }
    };

    let rgba = image.to_rgba8();
    let (width, height) = image.dimensions();

    let texture = device.create_texture(&wgpu_texture_descriptor(
        path,
        width,
        height,
        depth_or_array_layers,
        mip_level_count,
        sample_count,
        format,
        dimension,
    ));
    let view = wgpu_texture_view(&texture, None, aspect);
    let sampler = create_sampler_from_type(SamplerType::Textured)?;

    Ok(TextureFile {
        texture,
        view,
        sampler,
        rgba,
        offset,
        bytes_per_row: Some(4 * width),
        rows_per_image: Some(height),
        mip_level,
        origin,
        aspect,
    })
}
pub fn texture_load_files(
    folder_path: String,
    extension: String,
    dimension: TextureDimension,
    format: TextureFormat,
    mip_level_count: u32,
    sample_count: u32,
) -> Result<Vec<TextureFileDescriptor>, AppError> {
    Ok(load_textures(
        &FileSystem::append_to_cargo_dir(&folder_path),
        &extension,
        format,
        dimension,
        mip_level_count,
        sample_count,
    )?)
}

pub async fn texture_load_files_async(
    folder_path: String,
    extension: String,
    format: TextureFormat,
    dimension: TextureDimension,
    mip_level_count: u32,
    sample_count: u32,
) -> Result<Vec<TextureFileDescriptor>, AppError> {
    tokio::task::spawn_blocking(move || {
        load_textures(
            &folder_path,
            &extension,
            format,
            dimension,
            mip_level_count,
            sample_count,
        )
    })
    .await
    .map_err(|e| AppError::TaskJoinError(e))?
}
