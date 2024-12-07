use std::sync::Arc;

use crate::{
    core::{cache::ComponentCacheKey, error::AppError},
    ecs::{components::IntoComponentCacheKey, traits::Cache},
    graphics::{
        binding::BindGroupManager,
        textures::{manager::TextureManager, BindableTexture, Texture},
    },
};
#[derive(Debug)]
pub struct Material {
    pub diffuse_texture_key: Option<ComponentCacheKey>,
    pub normal_texture_key: Option<ComponentCacheKey>,
    pub bind_group_key: ComponentCacheKey,
}

impl IntoComponentCacheKey for Material {
    fn into_cache_key(&self) -> ComponentCacheKey {
        let diffuse_value = self.diffuse_texture_key.map_or(0, |key| key.value());
        let normal_value = self.normal_texture_key.map_or(0, |key| key.value());
        let bind_group_value = self.bind_group_key.value();

        ComponentCacheKey::from(diffuse_value ^ normal_value ^ bind_group_value)
    }
}
impl Material {
    // pub fn resolve<'a>(
    //     &self,
    //     texture_manager: &'a TextureManager,
    //     bind_group_manager: &'a BindGroupManager,
    // ) -> Result<ResolvedMaterial<'a>, AppError> {
    //     let diffuse_texture = if let Some(key) = self.diffuse_texture_key {
    //         Some(texture_manager.get(key).ok_or(AppError::ResourceNotFound)?)
    //     } else {
    //         None
    //     };

    //     let normal_texture = if let Some(key) = self.normal_texture_key {
    //         Some(texture_manager.get(key).ok_or(AppError::ResourceNotFound)?)
    //     } else {
    //         None
    //     };

    //     let bind_group = bind_group_manager
    //         .get(self.bind_group_key)
    //         .ok_or(AppError::ResourceNotFound)?;

    //     Ok(ResolvedMaterial {
    //         diffuse_texture: diffuse_texture.as_ref(),
    //         normal_texture: normal_texture.as_ref(),
    //         bind_group: &bind_group,
    //     })
    // }

    pub fn new(
        device: &wgpu::Device,
        name: &str,
        layout: &wgpu::BindGroupLayout,
        texture_manager: &TextureManager,
        bind_group_manager: &mut BindGroupManager,
        diffuse_texture_key: Option<ComponentCacheKey>,
        normal_texture_key: Option<ComponentCacheKey>,
    ) -> Result<Self, AppError> {
        let mut entries = Vec::new();

        if let Some(key) = diffuse_texture_key {
            // Extract resources before the borrow ends
            let diffuse_texture = texture_manager.get(key).ok_or(AppError::ResourceNotFound)?;
            let diffuse_view = diffuse_texture.view().clone();
            let diffuse_sampler = diffuse_texture.sampler().clone();
            entries.push(wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_view),
            });
            entries.push(wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
            });
        }

        if let Some(key) = normal_texture_key {
            // Extract resources before the borrow ends
            let normal_texture = texture_manager.get(key).ok_or(AppError::ResourceNotFound)?;
            let normal_view = normal_texture.view().clone();
            let normal_sampler = normal_texture.sampler().clone();
            entries.push(wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&normal_view),
            });
            entries.push(wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Sampler(&normal_sampler),
            });
        }

        // Create the bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &entries,
            label: Some("material_bind_group"),
        });

        // Generate a key for the bind group
        let bind_group_key = ComponentCacheKey::from(name); // Create a unique key for the bind group

        bind_group_manager.put(bind_group_key, bind_group.into())?;

        Ok(Self {
            diffuse_texture_key,
            normal_texture_key,
            bind_group_key,
        })
    }
}

pub struct ResolvedMaterial<'a> {
    pub diffuse_texture: Option<&'a Arc<Texture>>,
    pub normal_texture: Option<&'a Arc<Texture>>,
    pub bind_group: &'a wgpu::BindGroup,
}
