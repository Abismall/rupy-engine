use std::{mem, num::NonZero};

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayoutEntry, BindingResource, Buffer, Device, ShaderStages,
};

use crate::ecs::components::uniform::{ColorUniform, Uniforms};

pub fn uniform_bind_group_layout<'a>(device: &Device) -> wgpu::BindGroupLayout {
    let uniform_visibility = wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT;
    let dynamic_offset = false;
    let count: Option<std::num::NonZero<u32>> = None;
    let bbt = wgpu::BufferBindingType::Uniform;
    let uniform_binding_type = wgpu::BindingType::Buffer {
        ty: bbt,
        has_dynamic_offset: dynamic_offset,
        min_binding_size: NonZero::new(mem::size_of::<Uniforms>() as u64),
    };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(BindingGroups::UNIFORMS_LABEL),
        entries: &[bind_group_layout_entry(
            BindingLayouts::SAMPLER_BINDING.into(),
            uniform_visibility,
            uniform_binding_type,
            count,
        )],
    })
}

// Texture
pub fn texture_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    let texture_entry = bind_group_layout_entry(
        BindingLayouts::TEXTURE_BINDING.into(),
        wgpu::ShaderStages::FRAGMENT,
        wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        None,
    );

    let sampler_entry = bind_group_layout_entry(
        BindingLayouts::SAMPLER_BINDING.into(),
        wgpu::ShaderStages::FRAGMENT,
        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        None,
    );

    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(BindingGroups::TEXTURE_LABEL),
        entries: &[texture_entry, sampler_entry],
    })
}

pub fn uniform_bind_group(
    device: &Device,
    uniform_buffer: &Buffer,
    layout: &wgpu::BindGroupLayout,
) -> BindGroup {
    let resource = uniform_buffer.as_entire_binding();
    let entries = &[bind_group_entry(
        BindingLayouts::UNIFORMS_BINDING.into(),
        resource,
    )];
    let desc = bind_group_description(BindingGroups::UNIFORMS_LABEL, layout, entries);
    device.create_bind_group(&desc)
}

pub fn texture_bind_group(
    device: &Device,
    texture_view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> BindGroup {
    let layout = &texture_bind_group_layout(device);
    let view = BindingResource::TextureView(texture_view);
    let sampler = BindingResource::Sampler(sampler);

    let texture_view = bind_group_entry(BindingLayouts::TEXTURE_BINDING.into(), view);
    let texture_sampler = bind_group_entry(BindingLayouts::SAMPLER_BINDING.into(), sampler);
    let binding = [texture_view, texture_sampler];
    let desc = bind_group_description(BindingGroups::TEXTURE_LABEL, layout, &binding);
    device.create_bind_group(&desc)
}

pub fn bind_group_entry(binding: u32, resource: BindingResource) -> BindGroupEntry {
    BindGroupEntry { binding, resource }
}
pub fn bind_group_layout_entry(
    binding: u32,
    visibility: ShaderStages,
    ty: wgpu::BindingType,
    count: Option<NonZero<u32>>,
) -> BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty,
        count,
    }
}

pub fn bind_group_descriptor<'a>(
    label: &'a str,
    layout: &'a wgpu::BindGroupLayout,
    entries: &'a [BindGroupEntry<'a>],
) -> wgpu::BindGroupDescriptor<'a> {
    wgpu::BindGroupDescriptor {
        label: Some(label),
        layout,
        entries,
    }
}

pub fn bind_group_description<'a>(
    label: &'a str,
    layout: &'a wgpu::BindGroupLayout,
    entries: &'a [BindGroupEntry<'a>],
) -> wgpu::BindGroupDescriptor<'a> {
    wgpu::BindGroupDescriptor {
        label: Some(label),
        layout,
        entries,
    }
}

pub fn color_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
    let color_visibility = wgpu::ShaderStages::FRAGMENT;
    let dynamic_offset = false;
    let count: Option<std::num::NonZero<u32>> = None;
    let color_binding_type = wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: dynamic_offset,
        min_binding_size: NonZero::new(mem::size_of::<ColorUniform>() as u64),
    };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(BindingGroups::COLOR_LABEL),
        entries: &[bind_group_layout_entry(
            BindingLayouts::COLOR_BINDING.into(),
            color_visibility,
            color_binding_type,
            count,
        )],
    })
}
pub struct BindingLayouts;

impl BindingLayouts {
    pub const SAMPLER_BINDING: u16 = 0;
    pub const TEXTURE_BINDING: u16 = 1;
    pub const UNIFORMS_BINDING: u16 = 0;
    pub const COLOR_BINDING: u16 = 0;

    pub fn uniform(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        uniform_bind_group_layout(device)
    }
    pub fn texture(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        texture_bind_group_layout(device)
    }
    pub fn color(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        color_bind_group_layout(device)
    }
}

pub struct BindingGroups;

impl BindingGroups {
    pub const UNIFORMS_LABEL: &str = "UniformsBinding";
    pub const TEXTURE_LABEL: &str = "TextureBinding";
    pub const COLOR_LABEL: &str = "ColorBinding";

    pub fn uniform(
        device: &wgpu::Device,
        uniform_buffer: &wgpu::Buffer,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        uniform_bind_group(device, uniform_buffer, layout)
    }
    pub fn texture(
        device: &wgpu::Device,
        texture_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        texture_bind_group(device, texture_view, sampler)
    }
    pub fn color(
        device: &wgpu::Device,
        color_buffer: &wgpu::Buffer,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        color_bind_group(device, color_buffer, layout)
    }
}

pub fn color_bind_group(
    device: &Device,
    color_buffer: &Buffer,
    layout: &wgpu::BindGroupLayout,
) -> BindGroup {
    let resource = color_buffer.as_entire_binding();
    let entries = &[bind_group_entry(
        BindingLayouts::COLOR_BINDING.into(),
        resource,
    )];
    let desc = bind_group_description(BindingGroups::COLOR_LABEL, layout, entries);
    device.create_bind_group(&desc)
}
