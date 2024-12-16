pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexColor {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexTexture {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl VertexColor {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
}
impl Vertex for VertexColor {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<VertexColor>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
impl VertexTexture {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
}
impl Vertex for VertexTexture {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<VertexTexture>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
}
impl ModelVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3, 3 => Float32x3,  4 => Float32x3];
}
impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VertexType {
    Textured(VertexTexture),
    Colored(VertexColor),
    Modeled(ModelVertex),
}

impl VertexType {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            VertexType::Textured(v) => bytemuck::cast_slice(std::slice::from_ref(v)),
            VertexType::Colored(v) => bytemuck::cast_slice(std::slice::from_ref(v)),
            VertexType::Modeled(v) => bytemuck::cast_slice(std::slice::from_ref(v)),
        }
    }

    pub fn as_pod(&self) -> Vec<u8> {
        match self {
            VertexType::Textured(data) => bytemuck::cast_slice(std::slice::from_ref(data)).to_vec(),
            VertexType::Colored(data) => bytemuck::cast_slice(std::slice::from_ref(data)).to_vec(),
            VertexType::Modeled(data) => bytemuck::cast_slice(std::slice::from_ref(data)).to_vec(),
        }
    }
}
