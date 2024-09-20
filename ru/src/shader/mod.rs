use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2],
}

fn vertex(pos: [i8; 3], tc: [i8; 2]) -> Vertex {
    Vertex {
        _pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
        _tex_coord: [tc[0] as f32, tc[1] as f32],
    }
}

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
