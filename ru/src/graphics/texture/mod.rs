use image::GenericImageView;
pub mod loader;
pub mod sampler;
pub mod texture_cache;
#[derive(Clone, Debug)]
pub struct RupyTextureFile {
    pub file_path: String,
    pub dimension: wgpu::TextureDimension,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub format: wgpu::TextureFormat,
}
pub const TEXTURE_DIR: &str = "static\\images";

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
            texture,
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
pub fn create_texture(
    device: &wgpu::Device,
    label: Option<String>,
    dimension: wgpu::TextureDimension,
    format: wgpu::TextureFormat,
    size: wgpu::Extent3d,
    mip_level_count: u32,
    sample_count: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(&label.unwrap_or("Texture".to_string())),
        size,
        mip_level_count,
        sample_count,
        dimension,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
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
pub fn create_view(
    texture: &wgpu::Texture,
    descriptor: Option<&wgpu::TextureViewDescriptor>,
) -> wgpu::TextureView {
    texture.create_view(descriptor.unwrap_or(&wgpu::TextureViewDescriptor::default()))
}
