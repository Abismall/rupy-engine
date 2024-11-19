use std::num::NonZeroU64;

use wgpu::Device;

use crate::{
    gpu::uniform::Uniforms,
    prelude::constant::{
        INSTANCE_DATA_BINDING_IDX, SAMPLER_BINDING_IDX, TEXTURE_BINDING_IDX, UNIFORM_BINDING_IDX,
    },
};
pub fn instance_data_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("instance_data_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: INSTANCE_DATA_BINDING_IDX, // 0
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: Some(
                    NonZeroU64::new(std::mem::size_of::<crate::gpu::InstanceData>() as u64)
                        .unwrap(),
                ),
            },
            count: None,
        }],
    })
}

pub fn uniform_bind_group_layout(
    device: &Device,
    use_dynamic_offset: bool,
) -> wgpu::BindGroupLayout {
    let min_binding_size = NonZeroU64::new(std::mem::size_of::<Uniforms>() as u64);
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("uniform_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: UNIFORM_BINDING_IDX, // 0
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: use_dynamic_offset,
                min_binding_size,
            },
            count: None,
        }],
    })
}

pub fn sampled_texture_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    let view_dimension = wgpu::TextureViewDimension::D2;
    let sample_type = wgpu::TextureSampleType::Float { filterable: true };
    let multisampled = false;

    let sampler = wgpu::BindGroupLayoutEntry {
        binding: SAMPLER_BINDING_IDX, // 0
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        count: None,
    };
    let texture = wgpu::BindGroupLayoutEntry {
        binding: TEXTURE_BINDING_IDX, // 1
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            multisampled,
            view_dimension,
            sample_type,
        },
        count: None,
    };
    let description = wgpu::BindGroupLayoutDescriptor {
        label: Some("texture_bind_group_layout"),
        entries: &[sampler, texture],
    };
    device.create_bind_group_layout(&description)
}
