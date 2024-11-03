use super::{RupyShader, ShaderModuleBuilder};
use crate::core::error::AppError;
use crate::core::files::FileSystem;
use crate::log_debug;

use std::path::PathBuf;
use std::{collections::HashMap, sync::Arc};

const SHADERS_DIR_NAME: &str = "shaders";
const WGSL_SHADER_EXT: &str = "wgsl";

pub struct ShaderLibrary {
    pub shaders: HashMap<String, Arc<RupyShader>>,
    pub builder: ShaderModuleBuilder,
}

impl ShaderLibrary {
    pub fn new() -> Self {
        let builder = ShaderModuleBuilder::new();
        let shaders = HashMap::with_capacity(50);
        Self { builder, shaders }
    }
    fn dir_not_found<T>() -> Result<T, AppError> {
        Err(AppError::FileNotFoundError(
            "Shaders directory not found".into(),
        ))
    }
    fn find_shader_dir_path() -> std::result::Result<PathBuf, AppError> {
        match FileSystem::find_from_project_dir(SHADERS_DIR_NAME) {
            Some(dir) => Ok(dir),
            None => return ShaderLibrary::dir_not_found(),
        }
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
                log_debug!("New resource created!");
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
        let shader = ShaderModuleBuilder::from_path_string(device, shader_path, vs_main, fs_main)?;
        self.shaders
            .insert(shader_path.to_string(), Arc::new(shader.clone()));
        Ok(shader)
    }

    pub fn try_list_shader_file_paths() -> Result<Vec<String>, AppError> {
        let path_bufs = FileSystem::list_files_with_extension(
            &ShaderLibrary::find_shader_dir_path()?,
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
    let path_bufs = FileSystem::list_files_with_extension(
        &ShaderLibrary::find_shader_dir_path()?,
        WGSL_SHADER_EXT,
    )?;
    let paths: Vec<String> = path_bufs
        .iter()
        .filter_map(|path| path.to_str().map(|s| s.to_string()))
        .collect();

    Ok(paths)
}
