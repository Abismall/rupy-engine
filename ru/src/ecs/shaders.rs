use crate::shader::module::{list_shader_file_paths, RupyShader};
use crate::{core::error::AppError, shader::loader::ShaderModuleLoader};
use std::collections::HashMap;

use std::sync::Arc;

#[derive(Debug)]
pub struct ShaderManager {
    pub shaders: HashMap<String, Arc<RupyShader>>,
    pub builder: ShaderModuleLoader,
}

impl ShaderManager {
    pub fn new() -> Self {
        let builder = ShaderModuleLoader::new();
        let shaders = HashMap::with_capacity(50);
        Self { builder, shaders }
    }

    pub async fn async_load_shaders(&mut self, device: &wgpu::Device) -> Result<(), AppError> {
        for path in list_shader_file_paths()? {
            self.insert_shader_from_path(device, &path, "vs_main".into(), "fs_main".into())?;
        }
        Ok(())
    }

    pub fn load_shaders(&mut self, device: &wgpu::Device) -> Result<(), AppError> {
        for shader_path in list_shader_file_paths()? {
            self.insert_shader_from_path(device, &shader_path, "vs_main".into(), "fs_main".into())?;
        }
        Ok(())
    }
    pub fn get_or_create(
        &mut self,
        device: &wgpu::Device,
        path: &str,
        vs_main: String,
        fs_main: String,
    ) -> Result<std::sync::Arc<RupyShader>, AppError> {
        if let Some(cached) = self.shaders.get(path) {
            return Ok(Arc::clone(cached));
        } else {
            match self.insert_shader_from_path(device, path, vs_main, fs_main) {
                Ok(new_shader) => return Ok(new_shader),
                Err(e) => return Err(e),
            }
        }
    }
    pub fn insert_shader_from_path(
        &mut self,
        device: &wgpu::Device,
        shader_path: &str,
        vs_main: String,
        fs_main: String,
    ) -> Result<Arc<RupyShader>, AppError> {
        let shader = Arc::new(ShaderModuleLoader::from_path_string(
            device,
            shader_path,
            vs_main,
            fs_main,
        )?);
        self.shaders.insert(shader_path.to_string(), shader.clone());
        Ok(shader)
    }
}
