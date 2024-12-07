use naga::front::wgsl;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::{
    core::{error::AppError, files::FileSystem},
    log_warning,
    prelude::constant::WGSL_SHADER_EXT,
};

use super::module::{read_shader_source_from_path, RupyShader};

pub fn load_wgsl_shader_to_naga_module(shader_code: &str) -> Result<naga::Module, AppError> {
    Ok(wgsl::parse_str(shader_code)?)
}
fn extract_main_fn_name(source: &str, annotation: &str) -> Option<String> {
    for line in source.lines() {
        if line.contains(annotation) {
            if let Some(start) = line.find("fn ") {
                let after_fn = &line[start + 3..];
                if let Some(end) = after_fn.find('(') {
                    return Some(after_fn[..end].trim().to_string());
                }
            }
        }
    }
    None
}

pub fn from_path_slice<P: AsRef<Path> + std::fmt::Debug>(
    device: &wgpu::Device,
    path: P,
) -> Result<RupyShader, AppError> {
    let path_string = path.as_ref().to_string_lossy().to_string();
    let source_string = read_shader_source_from_path(path)?;
    Ok(RupyShader {
        module: device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_string.into()),
        }),
        path: path_string,
        vs_main: "vs_main".to_string(),
        fs_main: "fs_main".to_string(),
    })
}
pub fn from_path_string(device: &wgpu::Device, path: &str) -> Result<RupyShader, AppError> {
    let source_string = read_shader_source_from_path(path)?;
    Ok(RupyShader {
        module: device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_string.into()),
        }),
        path: String::from(path),
        vs_main: "vs_main".to_string(),
        fs_main: "fs_main".to_string(),
    })
}
pub fn list_shader_file_paths() -> std::result::Result<Vec<String>, AppError> {
    let mut paths = Vec::with_capacity(50);
    let shader_dir = FileSystem::get_shaders_dir()?;
    let mut cur: DirEntry;
    for entry in WalkDir::new(shader_dir) {
        if paths.len() == paths.capacity() {
            return Ok(paths);
        } else {
            cur = entry?;
            if cur.file_type().is_file() && cur.path().extension() == Some(WGSL_SHADER_EXT.as_ref())
            {
                match cur.path().to_str() {
                    Some(path) => {
                        paths.push(path.to_string());
                    }
                    None => {
                        log_warning!("Failed to cast path '{:?}' to string", cur);
                    }
                }
            }
        }
    }

    Ok(paths)
}
