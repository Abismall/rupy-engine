use std::path::Path;

use naga::front::wgsl;

use crate::core::error::AppError;

use super::module::{create_shader_module_from_path, RupyShader};

#[derive(Debug)]
pub struct ShaderModuleLoader;
impl ShaderModuleLoader {
    pub fn new() -> Self {
        Self
    }
    pub fn load_wgsl_shader_to_naga_module(shader_code: &str) -> Result<naga::Module, AppError> {
        Ok(wgsl::parse_str(shader_code)?)
    }
    pub fn from_path_slice<P: AsRef<Path>>(
        device: &wgpu::Device,
        path: P,
        vs_main: &str,
        fs_main: &str,
    ) -> Result<RupyShader, AppError> {
        let path_string = path.as_ref().to_string_lossy().to_string();
        let (module, source_string) = create_shader_module_from_path(device, path)?;
        Ok(RupyShader {
            module: module.into(),
            source: source_string,
            path: path_string,
            fs_main: (*fs_main).to_string(),
            vs_main: (*vs_main).to_string(),
        })
    }
    pub fn from_path_string(
        device: &wgpu::Device,
        path: &str,
        vs_main: String,
        fs_main: String,
    ) -> Result<RupyShader, AppError> {
        let (module, source_string) = create_shader_module_from_path(device, path)?;
        Ok(RupyShader {
            module: module.into(),
            source: source_string,
            path: String::from(path),
            fs_main,
            vs_main,
        })
    }
}
