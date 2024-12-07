use crate::{
    core::{
        cache::{ComponentCacheKey, HashCache},
        error::AppError,
    },
    ecs::traits::Cache,
    graphics::{
        binding::BindGroupManager,
        textures::{manager::TextureManager, BindableTexture},
    },
};

use super::model::Material;

pub struct MaterialManager {
    materials: HashCache<Material>,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self {
            materials: HashCache::new(),
        }
    }

    pub fn create_material(
        &mut self,
        device: &wgpu::Device,
        name: &str,
        diffuse_texture_key: Option<ComponentCacheKey>,
        normal_texture_key: Option<ComponentCacheKey>,
        layout: &wgpu::BindGroupLayout,
        texture_manager: &TextureManager,
        bind_group_manager: &mut BindGroupManager,
    ) -> Result<ComponentCacheKey, AppError> {
        if let Some(diffuse_key) = diffuse_texture_key {
            if !texture_manager.contains(diffuse_key) {
                return Err(AppError::ResourceNotFound);
            }
        }
        if let Some(normal_key) = normal_texture_key {
            if !texture_manager.contains(normal_key) {
                return Err(AppError::ResourceNotFound);
            }
        }

        let id = ComponentCacheKey::from(name);

        let material = Material::new(
            device,
            name,
            layout,
            texture_manager,
            bind_group_manager,
            diffuse_texture_key,
            normal_texture_key,
        )
        .unwrap();
        self.put(id, material)?;

        Ok(id)
    }
}

impl Cache<Material> for MaterialManager {
    fn get(&self, id: ComponentCacheKey) -> Option<&Material> {
        self.materials.get(id)
    }

    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.materials.contains(id)
    }

    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut Material> {
        self.materials.get_mut(id)
    }

    fn get_or_create<F>(
        &mut self,
        id: ComponentCacheKey,
        create_fn: F,
    ) -> Result<&mut Material, AppError>
    where
        F: FnOnce() -> Result<Material, AppError>,
    {
        self.materials.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: ComponentCacheKey, resource: Material) -> Result<(), AppError> {
        self.materials.put(id, resource)
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.materials.remove(id);
    }
}
