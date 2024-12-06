use std::path::Path;

use wgpu::{ShaderModule, ShaderModuleDescriptor};

use crate::core::{error::AppError, files::FileSystem};

#[derive(Debug)]
pub struct RupyShader {
    pub source: String,
    pub vs_main: String,
    pub fs_main: String,
    pub module: ShaderModule,
    pub path: String,
}
impl RupyShader {
    pub fn show_source(&self) -> &str {
        &self.source
    }
}

pub fn create_shader_module_from_path<P: AsRef<Path> + std::fmt::Debug>(
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

pub fn create_shader_modules(
    device: &wgpu::Device,
    v_path: &str,
    f_path: &str,
) -> Result<
    (
        (wgpu::ShaderModule, std::string::String),
        (wgpu::ShaderModule, std::string::String),
    ),
    AppError,
> {
    let vertex_shader_module = create_shader_module_from_path(device, v_path)?;

    let fragment_shader_module = create_shader_module_from_path(device, f_path)?;

    Ok((vertex_shader_module, fragment_shader_module))
}
