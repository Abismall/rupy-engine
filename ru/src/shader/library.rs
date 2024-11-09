use super::{RupyShader, ShaderModules};
use crate::core::error::AppError;
use crate::core::files::FileSystem;

use std::{collections::HashMap, sync::Arc};

const WGSL_SHADER_EXT: &str = "wgsl";
#[derive(Debug)]
pub struct ShaderLibrary {
    pub shaders: HashMap<String, Arc<RupyShader>>,
    pub builder: ShaderModules,
}

impl ShaderLibrary {
    pub fn new() -> Self {
        let builder = ShaderModules::new();
        let shaders = HashMap::with_capacity(50);
        Self { builder, shaders }
    }

    pub async fn async_load_shaders(&mut self, device: &wgpu::Device) -> Result<(), AppError> {
        for path in Self::try_list_shader_file_paths()? {
            self.insert_shader_from_path(device, &path, "vs_main".into(), "fs_main".into())?;
        }
        Ok(())
    }

    pub fn load_shaders(&mut self, device: &wgpu::Device) -> Result<(), AppError> {
        for shader_path in Self::try_list_shader_file_paths()? {
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
    ) -> Result<Arc<RupyShader>, AppError> {
        match self.shaders.get(path) {
            Some(cached) => Ok(Arc::clone(cached)),
            None => {
                if let Err(e) = self.insert_shader_from_path(device, path, vs_main, fs_main) {
                    Err(e)
                } else {
                    Ok(self.shaders.get(path).unwrap().clone())
                }
            }
        }
    }
    pub fn insert_shader_from_path(
        &mut self,
        device: &wgpu::Device,
        shader_path: &str,
        vs_main: String,
        fs_main: String,
    ) -> Result<RupyShader, AppError> {
        let shader = ShaderModules::from_path_string(device, shader_path, vs_main, fs_main)?;
        self.shaders
            .insert(shader_path.to_string(), Arc::new(shader.clone()));
        Ok(shader)
    }

    pub fn try_list_shader_file_paths() -> Result<Vec<String>, AppError> {
        let path_bufs = FileSystem::list_files_with_extension(
            &FileSystem::get_shaders_dir()?,
            WGSL_SHADER_EXT,
        )?;
        let paths: Vec<String> = path_bufs
            .iter()
            .filter_map(|path| path.to_str().map(|s| s.to_string()))
            .collect();

        Ok(paths)
    }
}
pub fn try_list_shader_file_paths() -> std::result::Result<Vec<String>, AppError> {
    let path_bufs =
        FileSystem::list_files_with_extension(&FileSystem::get_shaders_dir()?, WGSL_SHADER_EXT)?;
    let paths: Vec<String> = path_bufs
        .iter()
        .filter_map(|path| path.to_str().map(|s| s.to_string()))
        .collect();

    Ok(paths)
}
