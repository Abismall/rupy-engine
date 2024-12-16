use wgpu::BindGroupLayout;

use crate::graphics::textures::Texture;

pub fn create_hdr_pipeline_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("hdr_pipeline_bind_group_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    bind_group_layout
}
pub fn create_hdr_pipeline_bind_group(
    device: &wgpu::Device,
    layout: &BindGroupLayout,
    texture: &Texture,
) -> wgpu::BindGroup {
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("hdr_pipeline_bind_group"),
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
    });
    bind_group
}
