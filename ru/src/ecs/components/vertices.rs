use crate::ecs::model::{Vertex2D, Vertex3D};
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Sub;

impl Vertex3D {
    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex3D>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3, // position
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4, // color
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3, // normal
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>()
                        + mem::size_of::<[f32; 4]>()
                        + mem::size_of::<[f32; 3]>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2, // tex_coords
                },
            ],
        }
    }
}

impl Vertex2D {
    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex3D>() as wgpu::BufferAddress, // Match stride
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3, // position
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4, // color
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3, // normal, padding
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>()
                        + mem::size_of::<[f32; 4]>()
                        + mem::size_of::<[f32; 3]>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2, // tex_coords, padding
                },
            ],
        }
    }
}
impl From<Vertex2D> for Vertex3D {
    fn from(v: Vertex2D) -> Self {
        Vertex3D {
            position: [v.position[0], v.position[1], 0.0],
            color: v.color,
            normal: [1.0, 1.0, 1.0],
            tex_coords: v.tex_coords,
        }
    }
}
impl Hash for Vertex2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position
            .iter()
            .for_each(|&val| val.to_bits().hash(state));
        self.color.iter().for_each(|&val| val.to_bits().hash(state));
        self.tex_coords
            .iter()
            .for_each(|&val| val.to_bits().hash(state));
    }
}

impl Hash for Vertex3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position
            .iter()
            .for_each(|&val| val.to_bits().hash(state));
        self.color.iter().for_each(|&val| val.to_bits().hash(state));
        self.normal
            .iter()
            .for_each(|&val| val.to_bits().hash(state));
        self.tex_coords
            .iter()
            .for_each(|&val| val.to_bits().hash(state));
    }
}
impl Sub for Vertex2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            position: [
                self.position[0] - rhs.position[0],
                self.position[1] - rhs.position[1],
            ],
            color: [
                self.color[0] - rhs.color[0],
                self.color[1] - rhs.color[1],
                self.color[2] - rhs.color[2],
                self.color[3] - rhs.color[3],
            ],
            tex_coords: [
                self.tex_coords[0] - rhs.tex_coords[0],
                self.tex_coords[1] - rhs.tex_coords[1],
            ],
        }
    }
}

impl Sub for Vertex3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            position: [
                self.position[0] - rhs.position[0],
                self.position[1] - rhs.position[1],
                self.position[2] - rhs.position[2],
            ],
            color: [
                self.color[0] - rhs.color[0],
                self.color[1] - rhs.color[1],
                self.color[2] - rhs.color[2],
                self.color[3] - rhs.color[3],
            ],
            normal: [
                self.normal[0] - rhs.normal[0],
                self.normal[1] - rhs.normal[1],
                self.normal[2] - rhs.normal[2],
            ],
            tex_coords: [
                self.tex_coords[0] - rhs.tex_coords[0],
                self.tex_coords[1] - rhs.tex_coords[1],
            ],
        }
    }
}
