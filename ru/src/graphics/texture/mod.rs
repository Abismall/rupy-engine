use crate::core::error::AppError;
use image::{GenericImageView, ImageBuffer, Rgba};
use wgpu::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
pub mod library;
pub mod loader;
pub mod sampler;

#[derive(Clone, Debug)]
pub struct TextureAttachment {
    pub file_path: String,
    pub dimension: TextureDimension,
    pub usage: TextureUsages,
    pub size: wgpu::Extent3d,
    pub format: TextureFormat,
    pub mip_level_count: u32,
    pub sample_count: u32,
}

pub const TEXTURE_DIR: &str = "static\\images";

pub fn create_texture(
    device: &wgpu::Device,
    file_path: String,
    dimension: TextureDimension,
    usage: TextureUsages,
    size: Extent3d,
    format: TextureFormat,
    mip_level_count: u32,
    sample_count: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(&file_path),
        size: size,
        mip_level_count,
        sample_count,
        dimension: dimension,
        format: format,
        usage: usage,
        view_formats: &[],
    })
}
pub fn create_extent_3d(
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
pub fn create_texture_view(
    texture: &wgpu::Texture,
    descriptor: Option<&wgpu::TextureViewDescriptor>,
) -> wgpu::TextureView {
    texture.create_view(descriptor.unwrap_or(&wgpu::TextureViewDescriptor::default()))
}

pub fn load_texture_image(file_path: &str) -> Result<image::DynamicImage, AppError> {
    let img = image::open(file_path)
        .map_err(|_| AppError::FileNotFoundError("Texture file not received".to_owned()))?;
    Ok(img)
}

pub fn create_texture_from_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    img: &image::DynamicImage,
) -> (wgpu::Texture, ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        size,
    );

    (texture, rgba)
}
