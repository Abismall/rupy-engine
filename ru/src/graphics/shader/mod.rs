pub mod shader_cache;
use std::{fs, path::Path};
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor};

pub fn create_shader_module_from_path<P: AsRef<Path>>(
    device: &Device,
    path: P,
) -> Result<ShaderModule, std::io::Error> {
    let shader_source = fs::read_to_string(path)?;

    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Shader Module"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });

    Ok(shader_module)
}
