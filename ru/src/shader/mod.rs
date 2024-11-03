pub mod library;
use naga::front::wgsl;
use std::{path::Path, sync::Arc};
use wgpu::{util::DeviceExt, ShaderModule, ShaderModuleDescriptor};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataType {
    None,
    Float,
    Float2,
    Float3,
    Float4,
    Mat3,
    Mat4,
    Int,
    Int2,
    Int3,
    Int4,
    Bool,
}

pub fn get_component_count(data_type: DataType) -> u32 {
    match data_type {
        DataType::Float => 1,
        DataType::Float2 => 2,
        DataType::Float3 => 3,
        DataType::Float4 => 4,
        DataType::Mat3 => 3,
        DataType::Mat4 => 4,
        DataType::Int => 1,
        DataType::Int2 => 2,
        DataType::Int3 => 3,
        DataType::Int4 => 4,
        DataType::Bool => 1,
        DataType::None => panic!("Invalid DataType: None"),
    }
}
#[derive(Debug)]
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: usize,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[u32], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Mesh {
            vertex_buffer,
            index_buffer,
            index_count: indices.len(),
        }
    }
}
pub fn shader_data_type_size(data_type: DataType) -> u32 {
    match data_type {
        DataType::Float => 4,
        DataType::Float2 => 4 * 2,
        DataType::Float3 => 4 * 3,
        DataType::Float4 => 4 * 4,
        DataType::Mat3 => 4 * 3 * 3,
        DataType::Mat4 => 4 * 4 * 4,
        DataType::Int => 4,
        DataType::Int2 => 4 * 2,
        DataType::Int3 => 4 * 3,
        DataType::Int4 => 4 * 4,
        DataType::Bool => 1,
        DataType::None => {
            panic!("Unknown ShaderDataType: None is not a valid shader data type!");
        }
    }
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
    let vertex_shader_module = create_shader_module_from_path(device, v_path).unwrap();

    let fragment_shader_module = create_shader_module_from_path(device, f_path).unwrap();

    Ok((vertex_shader_module, fragment_shader_module))
}
