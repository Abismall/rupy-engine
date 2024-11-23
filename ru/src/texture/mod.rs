pub mod config;
pub mod manager;
use crate::{
    core::{error::AppError, files::FileSystem},
    gpu::sampler::{sampler_from_type, SamplerType},
    log_debug, log_error,
};
use config::{load_texture_config_from_folder, load_texture_image_from_folder};
use image::GenericImageView;
use std::{path::PathBuf, sync::Arc};
use wgpu::{
    Device, Extent3d, Origin3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
pub fn load_texture_by_name(
    device: &Device,
    name: &str,
    format: wgpu::TextureFormat,
) -> Result<TextureFile, AppError> {
    let folder = FileSystem::get_texture_base_folder(&name)?;
    log_debug!("Folder: {:?}", folder);
    let config = load_texture_config_from_folder(&folder)?;
    let image = load_texture_image_from_folder(&folder)?;
    let rgba = image.to_rgba8();
    let extent3d = img_extent_3d(image, Some(config.depth_or_array_layers));
    let texture = device.create_texture(&TextureDescriptor {
        label: Some(&config.name),
        size: extent3d,
        mip_level_count: config.mip_level_count,
        sample_count: config.sample_count,
        dimension: config.dimension,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let sampler = sampler_from_type(device, &SamplerType::Textured)?;
    Ok(TextureFile {
        texture,
        id: config.id,
        rgba: rgba.to_vec(),
        bytes_per_row: Some(4 * extent3d.width),
        rows_per_image: Some(extent3d.height),
        sampler: sampler,
    })
}

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
        mip_level: 0,
        origin: origin.unwrap_or(Origin3d::ZERO),
        aspect: aspect.unwrap_or(wgpu::TextureAspect::default()),
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
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST,
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
        label: Some("create_texture_view"),
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
        label: Some("create_depth_texture_view"),
        format: Some(TextureFormat::Depth32Float),
        dimension,
        aspect,
        base_mip_level: 0,
        mip_level_count: Some(depth_texture.mip_level_count()),
        base_array_layer: 0,
        array_layer_count: Some(depth_texture.depth_or_array_layers()),
    })
}
pub fn depth_texture(
    device: &Arc<Device>,
    surface_config: &wgpu::SurfaceConfiguration,
    dimension: TextureDimension,
    mip_level_count: u32,
    sample_count: u32,
    depth_or_array_layers: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth_texture"),
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

pub fn depth_texture_with_view(
    device: &Arc<wgpu::Device>,
    surface_config: &wgpu::SurfaceConfiguration,
) -> (wgpu::Texture, wgpu::TextureView) {
    let depth_texture = depth_texture(
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

#[derive(Debug)]
pub struct TextureFile {
    pub texture: wgpu::Texture,
    pub id: u64,
    pub rgba: Vec<u8>,
    pub bytes_per_row: Option<u32>,
    pub rows_per_image: Option<u32>,
    pub sampler: wgpu::Sampler,
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

pub fn load_texture_image(img_path: &PathBuf) -> Result<image::DynamicImage, AppError> {
    match image::open(img_path) {
        Ok(img) => Ok(img),
        Err(e) => {
            log_error!("{:?}", e);
            return Err(AppError::from(e));
        }
    }
}
