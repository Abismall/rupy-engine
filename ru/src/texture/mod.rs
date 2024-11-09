pub mod config;
pub mod file;
pub mod library;
pub mod sampler;

use crate::core::error::AppError;
use config::{load_texture_configs_from_folder, TextureConfig};
use file::TextureFile;
use std::sync::Arc;
use wgpu::{Device, Extent3d, Origin3d, TextureDimension, TextureFormat, TextureUsages};

pub const LABEL_TEXTURE: &str = "texture_attachment";
pub const LABEL_TEXTURE_VIEW: &str = "texture_view";

pub const LABEL_DEPTH_TEXTURE: &str = "depth_texture_attachment";
pub const LABEL_DEPTH_TEXTURE_VIEW: &str = "depth_texture_view";

pub const TEXTURE_DIR: &str = "static\\images";

pub fn write_texture_to_queue(
    queue: &wgpu::Queue,
    data: &TextureFile,
    offset: u64,
    origin: Option<Origin3d>,
    aspect: Option<wgpu::TextureAspect>,
) -> Result<(), AppError> {
    let data_layout = wgpu::ImageDataLayout {
        offset,
        bytes_per_row: data.bytes_per_row,
        rows_per_image: data.rows_per_image,
    };
    let texture = wgpu::ImageCopyTexture {
        texture: &data.texture,
        mip_level: data.texture.mip_level_count(),
        origin: origin.unwrap_or(Origin3d::ZERO),
        aspect: aspect.unwrap_or(wgpu::TextureAspect::All),
    };
    queue.write_texture(texture, &data.rgba, data_layout, data.texture.size());
    Ok(())
}
pub fn create_texture_desc(
    label: &str,
    width: u32,
    height: u32,
    depth_or_array_layers: u32,
    mip_level_count: u32,
    sample_count: u32,
    format: TextureFormat,
    dimension: TextureDimension,
) -> wgpu::TextureDescriptor {
    wgpu::TextureDescriptor {
        size: Extent3d {
            width,
            height,
            depth_or_array_layers,
        },
        mip_level_count,
        sample_count,
        dimension,
        format,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[],
        label: Some(label),
    }
}
pub fn create_texture_view(
    texture: &wgpu::Texture,
    dimension: Option<wgpu::TextureViewDimension>,
    aspect: wgpu::TextureAspect,
) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some(LABEL_TEXTURE_VIEW),
        format: Some(texture.format()),
        dimension,
        aspect,
        base_mip_level: 0,
        mip_level_count: Some(texture.mip_level_count()),
        base_array_layer: 0,
        array_layer_count: Some(texture.depth_or_array_layers()),
    })
}
pub fn create_depth_texture_view(
    depth_texture: &wgpu::Texture,
    dimension: Option<wgpu::TextureViewDimension>,
    aspect: wgpu::TextureAspect,
) -> wgpu::TextureView {
    depth_texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some(LABEL_TEXTURE_VIEW),
        format: Some(TextureFormat::Depth32Float),
        dimension,
        aspect,
        base_mip_level: 0,
        mip_level_count: Some(depth_texture.mip_level_count()),
        base_array_layer: 0,
        array_layer_count: Some(depth_texture.depth_or_array_layers()),
    })
}
pub fn wgpu_depth_texture(
    device: &Arc<Device>,
    surface_config: &wgpu::SurfaceConfiguration,
    dimension: TextureDimension,
    mip_level_count: u32,
    sample_count: u32,
    depth_or_array_layers: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(LABEL_DEPTH_TEXTURE),
        size: wgpu::Extent3d {
            width: surface_config.width,
            height: surface_config.height,
            depth_or_array_layers,
        },
        mip_level_count,
        sample_count,
        dimension,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}

pub fn set_depth_texture_size(texture: wgpu::Texture, width: u32, height: u32) -> wgpu::Texture {
    texture.size().height = height.max(1);
    texture.size().width = width.max(1);
    texture
}

pub fn create_depth_textures(
    device: &Arc<wgpu::Device>,
    surface_config: &wgpu::SurfaceConfiguration,
) -> (wgpu::Texture, wgpu::TextureView) {
    let depth_texture = wgpu_depth_texture(
        &device,
        &surface_config,
        wgpu::TextureDimension::D2,
        1,
        1,
        1,
    );
    let depth_texture_view = create_depth_texture_view(
        &depth_texture,
        Some(wgpu::TextureViewDimension::D2),
        wgpu::TextureAspect::DepthOnly,
    );
    (depth_texture, depth_texture_view)
}
pub fn resize_depth_texture(
    device: &Arc<wgpu::Device>,
    surface_config: &wgpu::SurfaceConfiguration,
) -> (
    std::option::Option<wgpu::Texture>,
    std::option::Option<wgpu::TextureView>,
) {
    let depth_texture = create_depth_textures(device, surface_config);
    (Some(depth_texture.0), Some(depth_texture.1))
}

pub async fn async_load_texture_config_files(
    folder_path: String,
    extension: String,
) -> Result<Vec<TextureConfig>, AppError> {
    tokio::task::spawn_blocking(move || load_texture_configs_from_folder(&folder_path, &extension))
        .await
        .map_err(|e| AppError::TaskJoinError(e))?
}
