use wgpu::{
    Device, Extent3d, Texture, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

pub struct WGPUTextureConfig {
    size: Extent3d,
    mip_level_count: u32,
    sample_count: u32,
    format: TextureFormat,
    usage: TextureUsages,
    dimension: TextureDimension,
    view_descriptor: Option<TextureViewDescriptor<'static>>,
}

impl<'a> WGPUTextureConfig {
    pub fn new(
        size: Extent3d,
        mip_level_count: u32,
        sample_count: u32,
        format: TextureFormat,
        usage: TextureUsages,
        dimension: TextureDimension,
        view_descriptor: Option<TextureViewDescriptor<'static>>,
    ) -> WGPUTextureConfig {
        WGPUTextureConfig {
            size,
            mip_level_count,
            sample_count,
            format,
            usage,
            dimension,
            view_descriptor,
        }
    }
}

pub fn wgpu_texture(
    label: Option<&str>,
    device: &Device,
    config: WGPUTextureConfig,
) -> (Texture, Option<TextureView>) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: config.size,
        mip_level_count: config.mip_level_count,
        sample_count: config.sample_count,
        dimension: config.dimension,
        format: config.format,
        usage: config.usage,
        view_formats: &[],
        label,
    });
    let texture_view = match config.view_descriptor {
        Some(descriptor) => {
            let texture_view = texture.create_view(&descriptor);
            return (texture, Some(texture_view));
        }
        None => None,
    };
    (texture, texture_view)
}
