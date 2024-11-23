use std::num::NonZero;

use crate::prelude::constant::{
    INSTANCE_DATA_BINDING_IDX, SAMPLER_BINDING_IDX, TEXTURE_BINDING_IDX, UNIFORM_BINDING_IDX,
};
use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType,
    Buffer, BufferBindingType, Device,
};

use super::data::Uniforms;

pub struct BindGroupLayouts {
    pub instance_data_layout: BindGroupLayout,
    pub uniform_layout: BindGroupLayout,
    pub sampled_texture_layout: BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &Device, use_dynamic_offset: bool, min_binding_size: u64) -> Self {
        let instance_data_layout = instance_data_bind_group_layout(device);
        let uniform_layout =
            uniform_bind_group_layout(device, use_dynamic_offset, min_binding_size);
        let sampled_texture_layout = sampled_texture_bind_group_layout(device);
        Self {
            instance_data_layout,
            uniform_layout,
            sampled_texture_layout,
        }
    }
}

pub fn instance_data_bind_group(
    device: &Device,
    buffer: &Buffer,
    layout: &BindGroupLayout,
) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("instance_data_bind_group"),
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: INSTANCE_DATA_BINDING_IDX,
            resource: buffer.as_entire_binding(),
        }],
    })
}

pub fn uniform_bind_group(device: &Device, buffer: &Buffer, layout: &BindGroupLayout) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Uniform Bind Group"),
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: UNIFORM_BINDING_IDX,
            resource: buffer.as_entire_binding(),
        }],
    })
}

pub fn sampled_texture_bind_group(
    device: &Device,
    layout: &BindGroupLayout,
    texture_view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("sampled_texture_bind_group"),
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
pub fn instance_data_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("instance_data_bind_group_layout"),
        entries: &[BindGroupLayoutEntry {
            binding: INSTANCE_DATA_BINDING_IDX,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    })
}

pub fn uniform_bind_group_layout(
    device: &Device,
    use_dynamic_offset: bool,
    min_binding_size: u64,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("uniform_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: UNIFORM_BINDING_IDX, // 0
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: use_dynamic_offset,
                min_binding_size: NonZero::new(min_binding_size),
            },
            count: None,
        }],
    })
}

pub fn sampled_texture_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
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
