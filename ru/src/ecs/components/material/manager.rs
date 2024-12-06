use crate::{
    core::{
        cache::{CacheId, HashCache},
        error::AppError,
    },
    ecs::traits::Cache,
    graphics::textures::Texture,
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
        diffuse_texture: Texture,
        normal_texture: Texture,
        layout: &wgpu::BindGroupLayout,
    ) -> Result<CacheId, AppError> {
        let id = CacheId::from(name);
        let material = Material::new(device, name, diffuse_texture, normal_texture, layout)?;
        self.put(id.value(), material)?;
        Ok(id)
    }
}

impl Cache<Material> for MaterialManager {
    fn get(&self, id: u64) -> Option<&Material> {
        self.materials.get(id)
    }

    fn contains(&self, id: u64) -> bool {
        self.materials.contains(id)
    }

    fn get_mut(&mut self, id: u64) -> Option<&mut Material> {
        self.materials.get_mut(id)
    }

    fn get_or_create<F>(&mut self, id: u64, create_fn: F) -> Result<&mut Material, AppError>
    where
        F: FnOnce() -> Result<Material, AppError>,
    {
        self.materials.get_or_create(id, create_fn)
    }

    fn put(&mut self, id: u64, resource: Material) -> Result<(), AppError> {
        self.materials.put(id, resource)
    }

    fn remove(&mut self, id: u64) {
        self.materials.remove(id);
    }
}
