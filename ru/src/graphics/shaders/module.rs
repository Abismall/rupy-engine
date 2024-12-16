use std::path::{Path, PathBuf};

use crate::core::{error::AppError, files::FileSystem};
use wgpu::ShaderModule;

#[derive(Debug)]
pub struct RupyShader {
    pub path: String,
    pub module: ShaderModule,
    pub source: String,
}
impl RupyShader {
    pub fn load(device: &wgpu::Device, path: &str) -> Result<RupyShader, AppError> {
        let (module, source) = RupyShader::create_shader_module(device, path)?;
        Ok(RupyShader {
            path: String::from(path),
            module,
            source,
        })
    }

    pub fn create_shader_module(
        device: &wgpu::Device,
        path: &str,
    ) -> Result<(wgpu::ShaderModule, std::string::String), AppError> {
        let shader_path_buf = shader_path(path)?;
        let src = read_shader_source_from_path(shader_path_buf)?;
        let source = wgpu::ShaderSource::Wgsl(src.clone().into());
        let label = Some("Shader Module");
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label, source });

        Ok((module, src))
    }
}

fn read_shader_source_from_path<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
) -> Result<std::string::String, std::io::Error> {
    let source_data_string = FileSystem::read_to_string(path)?;
    Ok(source_data_string)
}
fn shader_path(name: &str) -> Result<PathBuf, AppError> {
    FileSystem::get_shader_file_path(name)
}
