use crate::{
    core::error::AppError,
    graphics::textures::{BindableTexture, Texture},
};

pub fn create_material_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    diffuse_texture: Option<&Texture>,
    normal_texture: Option<&Texture>,
) -> Result<wgpu::BindGroup, AppError> {
    let mut entries = Vec::new();

    if let Some(diffuse) = diffuse_texture {
        entries.push(wgpu::BindGroupEntry {
            binding: entries.len() as u32,
            resource: wgpu::BindingResource::TextureView(diffuse.view()),
        });
        entries.push(wgpu::BindGroupEntry {
            binding: entries.len() as u32,
            resource: wgpu::BindingResource::Sampler(diffuse.sampler()),
        });
    }

    if let Some(normal) = normal_texture {
        entries.push(wgpu::BindGroupEntry {
            binding: entries.len() as u32,
            resource: wgpu::BindingResource::TextureView(normal.view()),
        });
        entries.push(wgpu::BindGroupEntry {
            binding: entries.len() as u32,
            resource: wgpu::BindingResource::Sampler(normal.sampler()),
        });
    }

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &entries,
        label: Some("material_bind_group"),
    });

    Ok(bind_group)
}
