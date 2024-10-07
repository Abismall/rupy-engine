use image::GenericImageView;
use wgpu::util::TextureDataOrder;
use wgpu::{util::DeviceExt, Device, Queue, Texture as WgpuTexture}; // Ensure this is correctly imported

pub const NO_TEXTURE_IMG_PATH: &str = "../../static/images/missing.png";

pub struct Texture {
    pub texture: WgpuTexture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    /// Loads a texture from a file and creates the necessary GPU resources.
    /// The `data_order` argument specifies how the data should be arranged (MipMajor or LayerMajor).
    pub fn from_file(
        device: &Device,
        queue: &Queue,
        path: &str,
        data_order: TextureDataOrder,
        use_mipmaps: bool, // New argument to decide mipmaps
        array_layers: u32, // New argument for array layers
        sample_count: u32, // New argument for multisampling
    ) -> Result<Self, String> {
        // Load image from file
        let img = image::open(path).map_err(|e| format!("Failed to load texture: {:?}", e))?;
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        // Calculate mip level count
        let mip_level_count = if use_mipmaps {
            (dimensions.0.max(dimensions.1) as f32).log2().floor() as u32 + 1
        } else {
            1
        };

        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some(path),
                size: wgpu::Extent3d {
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: array_layers,
                },
                mip_level_count,
                sample_count,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            data_order,
            &rgba,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }

    /// Loads a "no texture" texture from a file (black and purple checkerboard).

    pub fn no_texture(device: &Device, queue: &Queue) -> Self {
        Texture::from_file(
            device,
            queue,
            NO_TEXTURE_IMG_PATH,
            TextureDataOrder::MipMajor,
            false,
            1,
            1,
        )
        .unwrap()
    }
}
