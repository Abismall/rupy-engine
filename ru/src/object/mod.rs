pub mod buffer;
pub mod color;
pub mod object;
pub mod shape;
use crate::{
    math::{Mat4, Vec3},
    pipeline::cache::PipelineCache,
    render::Renderable,
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

pub trait VertexData {
    fn position(&self) -> [f32; 3];
    fn normal(&self) -> [f32; 3];
    fn color(&self) -> [f32; 3];
}

pub trait HasTexture: VertexData {
    fn uv(&self) -> [f32; 2];
}
use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroup, ShaderModule};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct VertexTextured {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl VertexData for VertexTextured {
    fn position(&self) -> [f32; 3] {
        self.position
    }

    fn normal(&self) -> [f32; 3] {
        self.normal
    }

    fn color(&self) -> [f32; 3] {
        self.color
    }
}

impl HasTexture for VertexTextured {
    fn uv(&self) -> [f32; 2] {
        self.uv
    }
}

use wgpu::util::DeviceExt; // For `create_buffer_init`

pub struct Mesh {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>,
    pub(crate) vertex_buffer: Option<wgpu::Buffer>, // GPU Vertex Buffer
    pub(crate) index_buffer: Option<wgpu::Buffer>,  // GPU Index Buffer
}

impl Mesh {
    pub fn create_buffers(&mut self, device: &wgpu::Device) {
        // Create the vertex buffer
        self.vertex_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );

        // Create the index buffer
        self.index_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        );
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        self.vertex_buffer
            .as_ref()
            .expect("Vertex buffer not created")
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        self.index_buffer
            .as_ref()
            .expect("Index buffer not created")
    }

    pub fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }
}

impl Renderable for Mesh {
    fn create_buffers(&mut self, device: &wgpu::Device) {
        // Create the vertex buffer
        self.vertex_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );

        // Create the index buffer
        self.index_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        );
    }

    fn vertex_buffer(&self) -> &wgpu::Buffer {
        self.vertex_buffer
            .as_ref()
            .expect("Vertex buffer not created")
    }

    fn index_buffer(&self) -> &wgpu::Buffer {
        self.index_buffer
            .as_ref()
            .expect("Index buffer not created")
    }

    fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }

    fn is_textured(&self) -> bool {
        false // Adjust this if you add texture support
    }

    fn update(&mut self) {
        // Handle any per-frame updates here
    }

    fn render(
        &mut self,
        device: &wgpu::Device,
        pipeline_cache: &mut PipelineCache,
        swapchain_format: wgpu::TextureFormat,
        vertex_shader_src: &ShaderModule,
        fragment_shader_src: &ShaderModule,
        encoder: &mut wgpu::CommandEncoder,
        output_view: &wgpu::TextureView,
        global_bind_group: &wgpu::BindGroup,
    ) {
        // Mesh-specific rendering logic
    }
}

impl VertexData for Vertex {
    fn position(&self) -> [f32; 3] {
        self.position
    }

    fn normal(&self) -> [f32; 3] {
        self.normal
    }

    fn color(&self) -> [f32; 3] {
        self.color
    }
}
