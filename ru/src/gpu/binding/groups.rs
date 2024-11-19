use std::num::{NonZero, NonZeroU64};

use wgpu::{BindGroup, BindGroupEntry, BindGroupLayout, BindingResource, Buffer, Device};

use crate::{
    gpu::uniform::Uniforms,
    prelude::constant::{
        INSTANCE_DATA_BINDING_IDX, SAMPLER_BINDING_IDX, TEXTURE_BINDING_IDX, UNIFORM_BINDING_IDX,
    },
};
pub fn instance_data_bind_group(
    device: &Device,
    buffer: &Buffer,
    layout: &BindGroupLayout,
    offset: Option<u64>,
) -> BindGroup {
    let size = NonZeroU64::new(std::mem::size_of::<crate::gpu::InstanceData>() as u64);
    let binding = INSTANCE_DATA_BINDING_IDX;
    let offset = offset.unwrap_or(0);

    let resource = wgpu::BindingResource::Buffer(wgpu::BufferBinding {
        buffer,
        offset,
        size,
    });

    let buffer_binding = wgpu::BindGroupEntry { binding, resource };

    let description = wgpu::BindGroupDescriptor {
        label: Some("instance_data_bind_group"),
        layout,
        entries: &[buffer_binding],
    };

    device.create_bind_group(&description)
}

pub fn uniform_bind_group(
    device: &Device,
    buffer: &Buffer,
    layout: &BindGroupLayout,
    offset: Option<u64>,
) -> BindGroup {
    let size = NonZero::new(std::mem::size_of::<Uniforms>() as u64);
    let binding = UNIFORM_BINDING_IDX;
    let offset = offset.unwrap_or(0);
    let resource = wgpu::BindingResource::Buffer(wgpu::BufferBinding {
        buffer: &buffer,
        offset,
        size,
    });
    let buffer_binding = BindGroupEntry { binding, resource };
    let description = wgpu::BindGroupDescriptor {
        label: Some("uniform_bind_group"),
        layout,
        entries: &[buffer_binding],
    };
    let uniform_bind_group = device.create_bind_group(&description);
    uniform_bind_group
}

pub fn sampled_texture_bind_group(
    device: &Device,
    layout: &BindGroupLayout,
    texture_view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> BindGroup {
    let sampler = BindGroupEntry {
        binding: SAMPLER_BINDING_IDX,
        resource: BindingResource::Sampler(sampler),
    };
    let texture = BindGroupEntry {
        binding: TEXTURE_BINDING_IDX,
        resource: BindingResource::TextureView(texture_view),
    };
    let description = wgpu::BindGroupDescriptor {
        label: Some("sampled_texture_bind_group"),
        layout,
        entries: &[sampler, texture],
    };
    let texture_bind_group = device.create_bind_group(&description);
    texture_bind_group
}
