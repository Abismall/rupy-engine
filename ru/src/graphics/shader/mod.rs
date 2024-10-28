pub mod library;
use naga::front::wgsl;
use std::{path::Path, sync::Arc};
use wgpu::{ShaderModule, ShaderModuleDescriptor};

use crate::core::{error::AppError, files::FileSystem};
#[derive(Debug, Clone)]
pub struct RupyShader {
    pub source: String,
    pub vs_main: String,
    pub fs_main: String,
    pub module: Arc<ShaderModule>,
    pub path: String,
}
impl RupyShader {
    pub fn show_source(&self) -> &str {
        &self.source
    }
}

pub struct ShaderModuleBuilder;
impl ShaderModuleBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn load_wgsl_shader_to_naga_module(shader_code: &str) -> Result<naga::Module, AppError> {
        wgsl::parse_str(shader_code).map_err(|e| AppError::from(e))
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

fn create_shader_module_from_path<P: AsRef<Path>>(
    device: &wgpu::Device,
    path: P,
) -> Result<(wgpu::ShaderModule, std::string::String), std::io::Error> {
    let source_data_string = FileSystem::read_to_string(path)?;
    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Shader Module"),
        source: wgpu::ShaderSource::Wgsl(source_data_string.clone().into()),
    });
    Ok((shader_module, source_data_string))
}
