use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3], color: [f32; 3], uv: [f32; 2]) -> Self {
        Vertex {
            position,
            normal,
            color,
            uv,
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
            0 => Float32x3, // position
            1 => Float32x3, // normal
            2 => Float32x3, // color
            3 => Float32x2  // uv
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}
