use crate::{
    core::cache::{CacheKey, HashCache},
    ecs::traits::Cache,
};

use crate::core::error::AppError;

use super::module::RupyShader;

pub struct ShaderManager {
    pub shaders: HashCache<RupyShader>,
}

impl ShaderManager {
    pub fn new(device: &wgpu::Device, preload_paths: Vec<&str>) -> Result<Self, AppError> {
        let mut manager = Self {
            shaders: HashCache::new(),
        };
        for path in preload_paths {
            manager.load_shader(device, path, CacheKey::from(path))?;
        }

        Ok(manager)
    }

    fn load_shader(
        &mut self,
        device: &wgpu::Device,
        name: &str,
        key: CacheKey,
    ) -> Result<(), AppError> {
        self.shaders
            .get_or_create(key, || match RupyShader::load(device, name) {
                Ok(shader) => Ok(shader),
                Err(e) => Err(e),
            })?;

        Ok(())
    }
}
