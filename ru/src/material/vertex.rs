use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TexturedVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl TexturedVertex {
    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TexturedVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position attribute
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0, // Matches @location(0) in the shader
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Texture coordinate attribute
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1, // Matches @location(1) in the shader
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub trait VertexType {
    fn position(&self) -> [f32; 3];
    fn color(&self) -> [f32; 3]; // For textured vertices, you can use texture coordinates instead
}

// Implement VertexType for `Vertex`
impl VertexType for Vertex {
    fn position(&self) -> [f32; 3] {
        self.position
    }

    fn color(&self) -> [f32; 3] {
        self.color
    }
}

// Implement VertexType for `TexturedVertex`
impl VertexType for TexturedVertex {
    fn position(&self) -> [f32; 3] {
        self.position
    }

    fn color(&self) -> [f32; 3] {
        [1.0, 1.0, 1.0] // Placeholder, textured vertices might not use color
    }
}
