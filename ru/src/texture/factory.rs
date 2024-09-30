use image::GenericImageView;
use std::{ops::Mul, path::Path, sync::Arc};
use wgpu::{ImageDataLayout, TextureDescriptor, TextureView, TextureViewDescriptor};

use crate::{files::FileSystem, AppError};

pub fn create_image_texture(
    device: &Arc<wgpu::Device>,
    path: &Path,
    desc: TextureDescriptor,
) -> Result<(wgpu::Texture, TextureView, ImageDataLayout), AppError> {
    let load_result = FileSystem::image_open(path)?;
    let (width, height) = load_result.dimensions();

    let texture = device.create_texture(&desc);
    let view = texture.create_view(&TextureViewDescriptor::default());
    let layout = wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(width.mul(4).into()),
        rows_per_image: Some(height.into()),
    };
    Ok((texture, view, layout))
}
