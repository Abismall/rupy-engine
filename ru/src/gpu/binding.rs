use wgpu::{BindGroup, BindGroupEntry, BindingResource, Buffer, Device};

use crate::prelude::constant::{SAMPLER_BINDING_IDX, TEXTURE_BINDING_IDX, UNIFORM_BINDING_IDX};

pub fn uniform_bind_group_layout<'a>(device: &Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Uniform Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: UNIFORM_BINDING_IDX,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    })
}

pub fn texture_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("texture_bind_group_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: SAMPLER_BINDING_IDX,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: TEXTURE_BINDING_IDX,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
        ],
    })
}

pub fn uniform_bind_group(
    device: &Device,
    uniform_buffer: &Buffer,
    layout: &wgpu::BindGroupLayout,
) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("uniform_bind_group"),
        layout,
        entries: &[BindGroupEntry {
            binding: UNIFORM_BINDING_IDX,
            resource: uniform_buffer.as_entire_binding(),
        }],
    })
}

pub fn texture_bind_group(
    device: &Device,
    texture_view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> BindGroup {
    let layout = &texture_bind_group_layout(device);

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("texture_bind_group"),
        layout,
        entries: &[
            BindGroupEntry {
                binding: SAMPLER_BINDING_IDX,
                resource: BindingResource::Sampler(sampler),
            },
            BindGroupEntry {
                binding: TEXTURE_BINDING_IDX,
                resource: BindingResource::TextureView(texture_view),
            },
        ],
    })
}

pub struct BindGroupLayouts {
    pub uniform_layout: wgpu::BindGroupLayout,
    pub texture_layout: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniform_layout = uniform_bind_group_layout(device);

        let texture_layout = texture_bind_group_layout(device);

        Self {
            uniform_layout,
            texture_layout,
        }
    }
}
