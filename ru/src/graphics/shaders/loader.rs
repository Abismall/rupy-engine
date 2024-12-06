use naga::front::wgsl;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::{
    core::{error::AppError, files::FileSystem},
    log_warning,
    prelude::constant::WGSL_SHADER_EXT,
};

use super::module::{create_shader_module_from_path, RupyShader};

pub fn load_wgsl_shader_to_naga_module(shader_code: &str) -> Result<naga::Module, AppError> {
    Ok(wgsl::parse_str(shader_code)?)
}
pub fn from_path_slice<P: AsRef<Path> + std::fmt::Debug>(
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
