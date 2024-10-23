use super::{create_extent_3d, create_texture, RupyTextureFile};
use crate::core::{error::AppError, files::FileSystem};
use crate::log_debug;

pub fn load_texture_files(
    folder_path: &str,
    extension: &str,
) -> Result<Vec<RupyTextureFile>, AppError> {
    let mut entries = Vec::new();
    for entry in
        FileSystem::list_files_with_extension(folder_path, std::ffi::OsStr::new(extension))?
    {
        log_debug!("Processing: {:?}", entry);
        let path = entry.as_path();
        let file_path = path.to_string_lossy().to_string();
        entries.push(RupyTextureFile {
            file_path,
            dimension: wgpu::TextureDimension::D3,
            mip_level_count: 1,
            sample_count: 1,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
        });
    }
    Ok(entries)
}
pub fn load_texture_file(
    device: &wgpu::Device,
    path: &str,
    dimension: wgpu::TextureDimension,
    mip_level_count: u32,
    sample_count: u32,
    format: wgpu::TextureFormat,
) -> Result<(wgpu::Texture, image::ImageBuffer<image::Rgba<u8>, Vec<u8>>), AppError> {
    let img = FileSystem::load_image_file(path)?;
    let rgba = img.to_rgba8();
    let size = create_extent_3d(img, Some(1));

    let texture = create_texture(
        device,
        Some(path.to_string()),
        dimension,
        format,
        size,
        mip_level_count,
        sample_count,
    );
    Ok((texture, rgba))
}

pub fn texture_file_cache_setup(
    folder_path: String,
    extension: String,
) -> Result<Vec<RupyTextureFile>, AppError> {
    Ok(load_texture_files(
        &FileSystem::append_to_cargo_dir(&folder_path),
        &extension,
    )?)
}
/// Async function to perform the texture file cache setup
pub async fn texture_file_cache_setup_task(
    folder_path: String,
    extension: String,
) -> Result<Vec<RupyTextureFile>, AppError> {
    // Call the synchronous texture_file_cache_setup function in an async context
    tokio::task::spawn_blocking(|| texture_file_cache_setup(folder_path, extension))
        .await
        .map_err(|e| AppError::TaskJoinError(e))?
}
