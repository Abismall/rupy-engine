pub mod schema;
use wgpu::{BindGroupEntry, BindingResource};

use super::layout::schema::BindGroupLayoutScribe;

pub const LABEL_SHADED_MATERIAL_BIND_GROUP: &str = "Shaded Material BindGroup";

pub mod cache;
pub struct BindGroupScribe;

impl BindGroupScribe {
    pub fn shaded_material_bind_group(
        device: &wgpu::Device,
        uniform_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&create_bind_group_description(
            &BindGroupLayoutScribe::model_uniform_layout(device),
            &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            Some(LABEL_SHADED_MATERIAL_BIND_GROUP),
        ))
    }
}

pub fn create_bind_group(
    device: &wgpu::Device,
    desc: &wgpu::BindGroupDescriptor,
) -> wgpu::BindGroup {
    device.create_bind_group(desc)
}

pub fn create_bind_group_description<'a>(
    layout: &'a wgpu::BindGroupLayout,
    entries: &'a [BindGroupEntry<'a>],
    label: Option<&'a str>,
) -> wgpu::BindGroupDescriptor<'a> {
    wgpu::BindGroupDescriptor {
        layout,
        entries,
        label,
    }
}

pub fn create_bind_group_entry(binding: u32, resource: BindingResource) -> BindGroupEntry {
    BindGroupEntry { binding, resource }
}
