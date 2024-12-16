use wgpu::{TextureFormat, TextureView};

pub fn create_equirect_bind_group_layout(
    device: &wgpu::Device,
    texture_format: TextureFormat,
) -> wgpu::BindGroupLayout {
    let equirect_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("HdrLoader::equirect_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: texture_format,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                    },
                    count: None,
                },
            ],
        });
    equirect_bind_group_layout
}

pub fn create_equirect_bind_group(
    device: &wgpu::Device,
    src_texture_view: &TextureView,
    dst_texture_view: &TextureView,
    texture_format: TextureFormat,
) -> wgpu::BindGroup {
    let equirect_bind_group_layout = create_equirect_bind_group_layout(device, texture_format);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("HdrLoader::equirect_bind_group"),
        layout: &equirect_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(src_texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(dst_texture_view),
            },
        ],
    });
    bind_group
}
