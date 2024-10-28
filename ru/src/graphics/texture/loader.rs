use image::GenericImageView;
use wgpu::{Extent3d, TextureDimension, TextureFormat, TextureUsages};

use super::{create_texture, TextureAttachment};
use crate::core::{error::AppError, files::FileSystem};

pub fn read_image_format(image: image::DynamicImage) -> Result<TextureFormat, AppError> {
    Ok(match image.color() {
        image::ColorType::Rgb8 => TextureFormat::Rgba8UnormSrgb,
        image::ColorType::Rgba8 => TextureFormat::Rgba8UnormSrgb,
        image::ColorType::L8 => TextureFormat::R8Unorm,
        other_format => {
            return Err(AppError::UnsupportedImageFormat(format!(
                "{:?}",
                other_format
            )))
        }
    })
}
pub fn load_texture_files(
    folder_path: &str,
    extension: &str,
    dimension: TextureDimension,
    mip_level_count: u32,
    sample_count: u32,
) -> Result<Vec<TextureAttachment>, AppError> {
    let mut entries = Vec::new();
    for entry in
        FileSystem::list_files_with_extension(folder_path, std::ffi::OsStr::new(extension))?
    {
        let path = entry.as_path();
        let file_path = path.to_string_lossy().to_string();

        let image = image::open(&path)?;
        let (width, height) = image.dimensions();

        let format = match image.color() {
            image::ColorType::Rgb8 => TextureFormat::Rgba8UnormSrgb,
            image::ColorType::Rgba8 => TextureFormat::Rgba8UnormSrgb,
            image::ColorType::L8 => TextureFormat::R8Unorm,
            _format => return Err(AppError::UnsupportedImageFormat(format!("{:?}", _format))),
        };

        entries.push(TextureAttachment {
            file_path,
            dimension,
            mip_level_count,
            sample_count,
            format,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
        });
    }
    Ok(entries)
}

pub fn load_texture_file(
    device: &wgpu::Device,
    path: &str,
    dimension: wgpu::TextureDimension,
    usage: TextureUsages,
    mip_level_count: u32,
    sample_count: u32,
) -> Result<(wgpu::Texture, image::ImageBuffer<image::Rgba<u8>, Vec<u8>>), AppError> {
    let img = FileSystem::load_image_file(path)?;
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();
    let format = read_image_format(img)?;
    let texture = create_texture(
        device,
        path.into(),
        dimension,
        usage,
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        format,
        mip_level_count,
        sample_count,
    );
    Ok((texture, rgba))
}

pub fn load_textures_from_dir(
    folder_path: String,
    extension: String,
    dimension: TextureDimension,
    mip_level_count: u32,
    sample_count: u32,
) -> Result<Vec<TextureAttachment>, AppError> {
    Ok(load_texture_files(
        &FileSystem::append_to_cargo_dir(&folder_path),
        &extension,
        dimension,
        mip_level_count,
        sample_count,
    )?)
}

pub async fn async_load_textures_from_dir(
    folder_path: String,
    extension: String,
    dimension: TextureDimension,
    mip_level_count: u32,
    sample_count: u32,
) -> Result<Vec<TextureAttachment>, AppError> {
    tokio::task::spawn_blocking(move || {
        load_textures_from_dir(
            folder_path,
            extension,
            dimension,
            mip_level_count,
            sample_count,
        )
    })
    .await
    .map_err(|e| AppError::TaskJoinError(e))?
}

pub fn texture_write(
    texture: &wgpu::Texture,
    origin: wgpu::Origin3d,
    aspect: wgpu::TextureAspect,
    rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    size: wgpu::Extent3d,
    mip_level: u32,
    queue: &wgpu::Queue,
) {
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level,
            origin,
            aspect,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * size.width),
            rows_per_image: Some(size.height),
        },
        size,
    );
}
