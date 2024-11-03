use std::sync::Arc;

use crate::{core::error::AppError, log_error};
use image::{GenericImageView, ImageBuffer, Rgba};
use wgpu::{Device, Extent3d, TextureDimension, TextureFormat, TextureUsages};
pub mod library;
pub mod loader;
pub mod sampler;

pub const LABEL_TEXTURE: &str = "texture_attachment";
pub const LABEL_TEXTURE_VIEW: &str = "texture_view";

pub const LABEL_DEPTH_TEXTURE: &str = "depth_texture_attachment";
pub const LABEL_DEPTH_TEXTURE_VIEW: &str = "depth_texture_view";
#[derive(Clone, Debug)]
pub struct TextureFileDescriptor {
    pub file_path: String,
    pub dimension: TextureDimension,
    pub usage: TextureUsages,
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
    pub format: TextureFormat,
    pub mip_level_count: u32,
    pub sample_count: u32,
}

pub const TEXTURE_DIR: &str = "static\\images";
#[derive(Debug)]
pub struct TextureFile {
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    rgba: ImageBuffer<Rgba<u8>, Vec<u8>>,
    texture: wgpu::Texture,
    offset: u64,
    bytes_per_row: Option<u32>,
    rows_per_image: Option<u32>,
    mip_level: u32,
    origin: wgpu::Origin3d,
    aspect: wgpu::TextureAspect,
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

pub fn write_texture_to_queue(queue: &wgpu::Queue, data: &TextureFile) -> Result<(), AppError> {
    let data_layout = wgpu::ImageDataLayout {
        offset: data.offset,
        bytes_per_row: data.bytes_per_row,
        rows_per_image: data.rows_per_image,
    };
    let texture = wgpu::ImageCopyTexture {
        texture: &data.texture,
        mip_level: data.mip_level,
        origin: data.origin,
        aspect: data.aspect,
    };
    queue.write_texture(texture, &data.rgba, data_layout, data.texture.size());
    Ok(())
}
pub fn wgpu_texture_descriptor(
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
pub fn wgpu_texture_view(
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
pub fn wgpu_depth_texture_view(
    depth_texture: &wgpu::Texture,
    dimension: Option<wgpu::TextureViewDimension>,
    aspect: wgpu::TextureAspect,
) -> wgpu::TextureView {
    let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some(LABEL_TEXTURE_VIEW),
        format: Some(TextureFormat::Depth32Float),
        dimension,
        aspect,
        base_mip_level: 0,
        mip_level_count: Some(depth_texture.mip_level_count()),
        base_array_layer: 0,
        array_layer_count: Some(depth_texture.depth_or_array_layers()),
    });
    depth_texture_view
}
pub fn wgpu_depth_texture(
    device: &Arc<Device>,
    surface_config: &wgpu::SurfaceConfiguration,
    dimension: TextureDimension,
    mip_level_count: u32,
    sample_count: u32,
    depth_or_array_layers: u32,
) -> wgpu::Texture {
    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
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
    });
    depth_texture
}

pub fn set_depth_texture_size(texture: &mut wgpu::Texture, width: u32, height: u32) -> Extent3d {
    let mut size = texture.size();
    size.height = height.max(1);
    size.width = width.max(1);
    size
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
    let depth_texture_view = wgpu_depth_texture_view(
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
