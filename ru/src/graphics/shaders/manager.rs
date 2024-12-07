use crate::core::cache::ComponentCacheKey;
use crate::core::error::AppError;
use crate::ecs::traits::Cache;

use std::collections::HashMap;

use super::module::RupyShader;

pub struct ShaderManager {
    shaders: HashMap<ComponentCacheKey, RupyShader>,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }

    pub fn load_shader(
        &mut self,
        device: &wgpu::Device,
        shader_id: ComponentCacheKey,
        shader_path: &str,
    ) -> Result<(), AppError> {
        let shader = super::loader::from_path_string(device, shader_path)?;
        self.put(shader_id, shader)?;
        Ok(())
    }

    pub fn list_shaders(&self) -> Vec<ComponentCacheKey> {
        self.shaders.keys().copied().collect()
    }
    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut RupyShader> {
        Some(self.shaders.get_mut(&id)?)
    }
}

impl Cache<RupyShader> for ShaderManager {
    fn get(&self, id: ComponentCacheKey) -> Option<&RupyShader> {
        self.shaders.get(&id)
    }

    fn contains(&self, id: ComponentCacheKey) -> bool {
        self.shaders.contains_key(&id)
    }

    fn put(
        &mut self,
        id: ComponentCacheKey,
        resource: RupyShader,
    ) -> std::result::Result<(), AppError> {
        if self.contains(id) {
            return Err(AppError::DuplicateResource);
        }
        self.shaders.insert(id, resource);
        Ok(())
    }

    fn remove(&mut self, id: ComponentCacheKey) {
        self.shaders.remove(&id);
    }
    fn get_or_create<F>(
        &mut self,
        shader_id: ComponentCacheKey,
        create_fn: F,
    ) -> std::result::Result<&mut RupyShader, AppError>
    where
        F: FnOnce() -> Result<RupyShader, AppError>,
    {
        if !self.contains(shader_id) {
            let shader = create_fn()?;
            self.put(shader_id, shader.into())?;
        }
        self.get_mut(shader_id).ok_or(AppError::ResourceNotFound)
    }

    fn get_mut(&mut self, id: ComponentCacheKey) -> Option<&mut RupyShader> {
        todo!()
    }
}
