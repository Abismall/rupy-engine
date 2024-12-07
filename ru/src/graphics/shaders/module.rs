use std::path::Path;

use wgpu::{ShaderModule, ShaderModuleDescriptor};

use crate::core::{error::AppError, files::FileSystem};

#[derive(Debug)]
pub struct RupyShader {
    pub module: ShaderModule,
    pub vs_main: String,
    pub fs_main: String,
    pub path: String,
}

pub fn read_shader_source_from_path<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
) -> Result<std::string::String, std::io::Error> {
    let source_data_string = FileSystem::read_to_string(path)?;

    Ok(source_data_string)
}

pub fn create_shader_modules(
    device: &wgpu::Device,
    vert_path: &str,
    frag_path: &str,
) -> Result<
    (
        (wgpu::ShaderModule, std::string::String),
        (wgpu::ShaderModule, std::string::String),
    ),
    AppError,
> {
    let (vertex_shader_src, fragment_shader_src) = (
        read_shader_source_from_path(vert_path)?,
        read_shader_source_from_path(frag_path)?,
    );
    let vert_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader Module"),
        source: wgpu::ShaderSource::Wgsl(vertex_shader_src.clone().into()),
    });
    let frag_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader Module"),
        source: wgpu::ShaderSource::Wgsl(fragment_shader_src.clone().into()),
    });
    Ok((
        (vert_module, vertex_shader_src),
        (frag_module, fragment_shader_src),
    ))
}
