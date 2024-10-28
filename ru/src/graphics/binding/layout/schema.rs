use crate::{
    graphics::binding::layout::{
        BINDING_CAMERA_UNIFORM, BINDING_MODEL_UNIFORM, BINDING_SAMPLER, BINDING_TEXTURE,
    },
    scene::components::uniform::{CameraUniforms, ObjectUniforms},
};

use super::{LABEL_CAMERA_BIND_GROUP, LABEL_MODEL_BIND_GROUP, LABEL_TEXTURE_BIND_GROUP};
pub struct BindGroupLayoutScribe;

impl BindGroupLayoutScribe {
    pub fn camera_uniform_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        camera_uniform_bind_group_layout(device)
    }

    pub fn camera_uniform_entries() -> Vec<wgpu::BindGroupLayoutEntry> {
        camera_uniform_bind_group_entries()
    }

    pub fn model_uniform_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        model_uniform_bind_group_layout(device)
    }

    pub fn model_uniform_entries() -> Vec<wgpu::BindGroupLayoutEntry> {
        model_uniform_bind_group_entries()
    }

    pub fn texture_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        texture_bind_group_layout(device)
    }

    pub fn texture_entries() -> Vec<wgpu::BindGroupLayoutEntry> {
        texture_bind_group_entries()
    }
}

/// Computes the minimum binding size for a given object type `T`
/// by calculating its size in bytes as `NonZeroU64`.
///
/// Returns `None` if the size of `T` is 0, otherwise returns `Some(NonZeroU64)`.
pub fn mem_byte_size<T>() -> Option<std::num::NonZeroU64> {
    std::num::NonZeroU64::new(std::mem::size_of::<T>() as u64)
}

pub fn create_bind_group_layout(
    device: &wgpu::Device,
    desc: &wgpu::BindGroupLayoutDescriptor,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(desc)
}

/// Entries for the camera uniform buffer layout.
fn camera_uniform_bind_group_entries() -> Vec<wgpu::BindGroupLayoutEntry> {
    vec![wgpu::BindGroupLayoutEntry {
        binding: BINDING_CAMERA_UNIFORM,
        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: mem_byte_size::<CameraUniforms>(),
        },
        count: None,
    }]
}

/// Creates the layout for the camera uniform buffer.
fn camera_uniform_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    create_bind_group_layout(
        device,
        &wgpu::BindGroupLayoutDescriptor {
            label: Some(LABEL_CAMERA_BIND_GROUP),
            entries: &camera_uniform_bind_group_entries(),
        },
    )
}

/// Entries for the model uniform buffer layout.
fn model_uniform_bind_group_entries() -> Vec<wgpu::BindGroupLayoutEntry> {
    vec![wgpu::BindGroupLayoutEntry {
        binding: BINDING_MODEL_UNIFORM,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: mem_byte_size::<ObjectUniforms>(),
        },
        count: None,
    }]
}

/// Creates the layout for the model uniform buffer.
fn model_uniform_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    create_bind_group_layout(
        device,
        &wgpu::BindGroupLayoutDescriptor {
            label: Some(LABEL_MODEL_BIND_GROUP),
            entries: &model_uniform_bind_group_entries(),
        },
    )
}

/// Entries for the texture and sampler binding layout.
fn texture_bind_group_entries() -> Vec<wgpu::BindGroupLayoutEntry> {
    vec![
        wgpu::BindGroupLayoutEntry {
            binding: BINDING_TEXTURE,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                multisampled: false,
                view_dimension: wgpu::TextureViewDimension::D2,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: BINDING_SAMPLER,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        },
    ]
}

/// Creates the layout for texture and sampler bindings.
fn texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    create_bind_group_layout(
        device,
        &wgpu::BindGroupLayoutDescriptor {
            label: Some(LABEL_TEXTURE_BIND_GROUP),
            entries: &texture_bind_group_entries(),
        },
    )
}
